//! Process scheduler
//!
//! This is where the magic happens
use crate::bpmn::schema::{
    DocumentElementContainer, Element as E, Expr, FormalExpression, Process, ProcessType,
    SequenceFlow,
};
use crate::event::ProcessEvent as Event;
use crate::flow_node;
use crate::language::{Engine as _, MultiLanguageEngine};

use futures::future::FutureExt;
use futures::stream::{FuturesUnordered, StreamExt, StreamFuture};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use std::task::{Context, Poll};
use tokio::sync::{broadcast, mpsc, oneshot};
use tokio::task::{self};

use super::{Handle, Log, Request, StartError};

pub(crate) struct Scheduler {
    receiver: mpsc::Receiver<Request>,
    process: Handle,
    flow_nodes: FuturesUnordered<FlowNode>,
    expression_evaluator: MultiLanguageEngine<bool>,
    element: Arc<Process>,
    log_broadcast: broadcast::Sender<Log>,
}

// FIXME: We're using this structure to be able to find flow nodes by their identifier
// in `FuturesUnordered` (`Scheduler.flow_nodes`). It's a linear search and is probably
// fine when there's a small number of flow nodes, but should it become large, this approach
// should probably be rethought.
struct FlowNode {
    id: String,
    future: StreamFuture<Box<dyn flow_node::FlowNode>>,
    tokens: usize,
}

use std::ops::{Deref, DerefMut};

impl Deref for FlowNode {
    type Target = Box<dyn flow_node::FlowNode>;

    fn deref(&self) -> &Self::Target {
        // FIXME: is there any better way to do this?
        // I *think* it's reasonable to assume it won't panic in runtime
        // because when it's used, scheduler is not doing anything with the future.
        // However, I am not confident in this.
        self.future.get_ref().unwrap()
    }
}

impl DerefMut for FlowNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // FIXME: see above in `Deref` implementation
        self.future.get_mut().unwrap()
    }
}

/// This encapsulates an item produced by flow node (as a Stream)
struct Next {
    id: String,
    item: <StreamFuture<Box<dyn flow_node::FlowNode>> as Future>::Output,
    tokens: usize,
}

impl Future for FlowNode {
    type Output = Next;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.future.poll_unpin(cx).map(|v| Next {
            id: self.id.clone(),
            item: v,
            tokens: self.tokens,
        })
    }
}

/// Internal flow node scheduler control
enum Control {
    // Continue with this action
    Proceed(Option<flow_node::Action>),
    // Stop
    Drop,
}

impl Scheduler {
    pub(crate) fn new(receiver: mpsc::Receiver<Request>, process: Handle) -> Self {
        let flow_nodes = process
            .element()
            .flow_elements()
            .iter()
            .map(|e| e.clone().into_inner())
            .filter_map(|e| {
                flow_node::new(e.as_ref()).map(|mut flow_node| {
                    flow_node.set_process(process.clone());
                    let e = flow_node.element();
                    FlowNode {
                        // FIXME: decide what should we do with flow nodes that don't have ID.
                        // They can't be connected with other nodes (there's no way to refer to
                        // them), but they can still be operational in a single flow node operation
                        // (even though this might be a degenerative case)
                        id: e.id().as_ref().unwrap_or(&"".to_string()).to_string(),
                        future: flow_node.into_future(),
                        tokens: 0,
                    }
                })
            })
            .collect();

        let mut expression_evaluator = MultiLanguageEngine::new_with_builtin_engines();
        if let Some(ref default_expression_language) =
            process.model().definitions().expression_language
        {
            expression_evaluator.set_default_language(default_expression_language);
        }

        let element = process.element();
        let log_broadcast = process.log_broadcast();

        Self {
            receiver,
            process,
            flow_nodes,
            expression_evaluator,
            element,
            log_broadcast,
        }
    }

    // Main loop
    pub async fn run(mut self) {
        let mut join_handle = None;
        loop {
            task::yield_now().await;
            tokio::select! {
               // Handle request processing
               next = self.receiver.recv()  =>
                   match next {
                       Some(Request::JoinHandle(handle)) => join_handle = Some(handle),
                       Some(Request::Terminate(sender)) => {
                           let _ = sender.send(join_handle.take());
                           return;
                       }
                       Some(Request::Start(sender)) => {
                           self.start(sender);
                       }
                       None => {}
                   },
               // Flow node processing
               next = self.flow_nodes.next() => {
                   self.process_flow_node_next(next).await;
               }
            }
        }
    }

    async fn probe_sequence_flow(&mut self, seq_flow: &SequenceFlow) -> bool {
        if let Some(Expr::FormalExpression(ref expr @ FormalExpression { .. })) =
            seq_flow.condition_expression
        {
            let expr = expr.clone();
            match self.expression_evaluator.eval_expr(&expr).await {
                Ok(result) => result,
                Err(err) => {
                    let _ = self.log_broadcast.send(Log::ExpressionError {
                        error: format!("{:?}", err),
                    });
                    false
                }
            }
        } else {
            true
        }
    }

    /// Figure out what should be the next course of action
    fn next_action(
        &mut self,
        action: Option<flow_node::Action>,
        flow_node: &dyn flow_node::FlowNode,
    ) -> Control {
        flow_node.element().incomings().iter().fold(
            Control::Proceed(action),
            |control, incoming| {
                match control {
                    // once the action has been dropped, it's not checked against
                    // any other incoming flows
                    Control::Drop => control,
                    Control::Proceed(action) => {
                        let mut matching_predecessor = self.flow_nodes.iter_mut().find(|node| {
                            node.element()
                                .outgoings()
                                .iter()
                                .any(|outgoing| outgoing == incoming)
                        });
                        if let Some(ref mut node) = matching_predecessor {
                            // it's ok to unwrap here because we already know such
                            // predecessor exists
                            let index = node
                                .element()
                                .outgoings()
                                .iter()
                                .enumerate()
                                .find_map(|(i, name)| if name == incoming { Some(i) } else { None })
                                .unwrap();
                            match node.handle_outgoing_action(index, action) {
                                None => Control::Drop,
                                Some(action) => Control::Proceed(action),
                            }
                        } else {
                            Control::Proceed(action)
                        }
                    }
                }
            },
        )
    }

    async fn process_flow_node_next(&mut self, next: Option<Next>) {
        if let Some(Next {
            id,
            item: (action, mut flow_node),
            tokens,
        }) = next
        {
            let next_action = self.next_action(action, &*flow_node);
            match next_action {
                // We're good to proceed with the following probing action
                Control::Proceed(Some(flow_node::Action::ProbeOutgoingSequenceFlows(indices))) => {
                    let outgoings = flow_node.element().outgoings().clone();
                    for index in indices {
                        let seq_flow = self
                            .element
                            .find_by_id(&outgoings[index])
                            .and_then(|seq_flow| seq_flow.downcast_ref::<SequenceFlow>())
                            .cloned();
                        if let Some(seq_flow) = seq_flow {
                            let success = self.probe_sequence_flow(&seq_flow).await;
                            flow_node.sequence_flow(index, &seq_flow, success);
                        }
                    }
                }
                // We're good to proceed with the following flow action
                Control::Proceed(Some(flow_node::Action::Flow(ref indices))) => {
                    let el = flow_node.element();
                    let outgoings = el.outgoings();
                    for index in indices {
                        let seq_flow = self
                            .element
                            .find_by_id(&outgoings[*index])
                            .and_then(|seq_flow| seq_flow.downcast_ref::<SequenceFlow>())
                            .cloned();
                        if let Some(seq_flow) = seq_flow {
                            let success = self.probe_sequence_flow(&seq_flow).await;
                            if !success {
                                continue;
                            }
                            if let Some(next_node) = self
                                .flow_nodes
                                .iter_mut()
                                .find(|next_node| next_node.id == seq_flow.target_ref)
                            {
                                let target_node = &mut next_node.future;
                                if let Some(node) = target_node.get_mut() {
                                    // match target's node incoming index for this sequence flow
                                    let index =
                                        node.element().incomings().iter().enumerate().find_map(
                                            |(index, incoming)| {
                                                if incoming == seq_flow.id.as_ref().unwrap() {
                                                    Some(index)
                                                } else {
                                                    None
                                                }
                                            },
                                        );
                                    // if there's one (and there better be!)
                                    if let Some(index) = index {
                                        // there's an incoming
                                        let _ = self.log_broadcast.send(Log::FlowNodeIncoming {
                                            node: node.element().clone(),
                                            incoming_index: index,
                                        });
                                        // increase the number of tokens by a number of added flows
                                        next_node.tokens += indices.len();
                                        // report it to the target node
                                        node.tokens(next_node.tokens);
                                        // and report the incoming
                                        node.incoming(index);
                                    }
                                }
                            }
                        }
                    }
                }
                // flow node completion
                Control::Proceed(Some(flow_node::Action::Complete)) => {
                    let _ = self.log_broadcast.send(Log::FlowNodeCompleted {
                        node: flow_node.element().clone(),
                    });
                }
                // nothing, don't reschedule this flow node anymore
                Control::Proceed(None) => {
                    if self.flow_nodes.is_empty() {
                        let _ = self.log_broadcast.send(Log::Done);
                    }
                    return;
                }
                // no action to be taken
                Control::Drop => {}
            }
            // Reschedule the flow node
            self.flow_nodes.push(FlowNode {
                id,
                future: flow_node.into_future(),
                tokens,
            });
        }
    }

    fn start(&mut self, sender: oneshot::Sender<Result<(), StartError>>) {
        if !self
            .process
            .element()
            .flow_elements()
            .iter()
            .map(|e| e.clone().into_inner())
            .any(|node| node.element() == E::StartEvent)
        {
            let _ = sender.send(Err(StartError::NoStartEvent));
        } else {
            let event_broadcast = self.process.event_broadcast();
            let _ = event_broadcast.send(Event::Start);
            let _ = sender.send(Ok(()));
        }
    }
}
