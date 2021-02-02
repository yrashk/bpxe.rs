//! Process scheduler
//!
//! This is where the magic happens
use super::{DataObjectContainer, DataObjectError, Handle, Log, Request, StartError};
use crate::bpmn::schema::{
    self, DocumentElementContainer, Element as E, Expr, FormalExpression, Process, ProcessType,
    SequenceFlow, SequenceFlowConditionExpression,
};
use crate::data_object::{self, DataObject};
use crate::event::ProcessEvent as Event;
use crate::flow_node;
use crate::language::{Engine as _, EngineContextProvider, MultiLanguageEngine};
use derive_more::{Deref, DerefMut};
use futures::stream::{Stream, StreamExt};
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use streamunordered::{StreamUnordered, StreamYield};
use tokio::sync::{broadcast, mpsc, oneshot, RwLock};
use tokio::task::{self};

pub(crate) struct Scheduler {
    receiver: mpsc::Receiver<Request>,
    process: Handle,
    flow_nodes: StreamUnordered<FlowNode>,
    // sequence flow => (token, index)
    flow_nodes_outgoing: HashMap<String, (usize, usize)>,
    // sequence flow => (token, index)
    flow_nodes_incoming: HashMap<String, (usize, usize)>,
    expression_evaluator: MultiLanguageEngine,
    element: Arc<Process>,
    log_broadcast: broadcast::Sender<Log>,
    data_objects: HashMap<String, DataObjectContainer>,
}

// FIXME: We're using this structure to be able to find flow nodes by their identifier
// in `FuturesUnordered` (`Scheduler.flow_nodes`). It's a linear search and is probably
// fine when there's a small number of flow nodes, but should it become large, this approach
// should probably be rethought.
#[derive(Deref, DerefMut)]
#[deref(forward)]
#[deref_mut(forward)]
struct FlowNode {
    #[deref(ignore)]
    #[deref_mut(ignore)]
    id: String,
    #[deref]
    #[deref_mut]
    node: Box<dyn flow_node::FlowNode>,
    #[deref(ignore)]
    #[deref_mut(ignore)]
    tokens: usize,
}

impl Stream for FlowNode {
    type Item = flow_node::Action;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.node.poll_next_unpin(cx)
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
        let mut flow_nodes = StreamUnordered::new();
        let mut flow_nodes_outgoing = HashMap::new();
        let mut flow_nodes_incoming = HashMap::new();

        for flow_node in process
            .element()
            .flow_elements()
            .iter()
            .map(|e| e.clone().into_inner())
            .filter_map(|e| {
                flow_node::new(e).map(|mut flow_node| {
                    flow_node.set_process(process.clone());
                    let e = flow_node.element();
                    FlowNode {
                        // FIXME: decide what should we do with flow nodes that don't have ID.
                        // They can't be connected with other nodes (there's no way to refer to
                        // them), but they can still be operational in a single flow node operation
                        // (even though this might be a degenerative case)
                        id: e.id().as_ref().unwrap_or(&"".to_string()).to_string(),
                        node: flow_node,
                        tokens: 0,
                    }
                })
            })
        {
            let element = flow_node.element();
            let token = flow_nodes.insert(flow_node);
            for (index, outgoing) in element.outgoings().iter().enumerate() {
                flow_nodes_outgoing.insert(outgoing.to_owned(), (token, index));
            }
            for (index, incoming) in element.incomings().iter().enumerate() {
                flow_nodes_incoming.insert(incoming.to_owned(), (token, index));
            }
        }

        let mut data_objects: HashMap<String, DataObjectContainer> = process
            .element()
            .flow_elements()
            .iter()
            .map(|e| e.clone().into_inner())
            .filter_map(|e| {
                if let Ok(schema::DataObject {
                    id: Some(id),
                    is_collection,
                    ..
                }) = e.downcast::<schema::DataObject>().map(|e| *e)
                {
                    let data_object = if let Some(true) = is_collection {
                        Box::new(data_object::Collection::new()) as Box<dyn DataObject>
                    } else {
                        Box::new(data_object::Empty) as Box<dyn DataObject>
                    };
                    Some((id, Arc::new(RwLock::new(data_object))))
                } else {
                    None
                }
            })
            .collect();

        // Now we can add data object references
        for (id, reference) in process
            .element()
            .flow_elements()
            .iter()
            .map(|e| e.clone().into_inner())
            .filter_map(|e| {
                if let Ok(schema::DataObjectReference {
                    id: Some(id),
                    data_object_ref: Some(reference),
                    ..
                }) = e.downcast::<schema::DataObjectReference>().map(|e| *e)
                {
                    Some((id, reference))
                } else {
                    None
                }
            })
        {
            if let Some(container) = data_objects.get(&reference).cloned() {
                data_objects.insert(id, container);
            }
        }

        let mut expression_evaluator = process.model().expression_engine_factory().create();

        if let Some(ref default_expression_language) =
            process.model().definitions().expression_language
        {
            expression_evaluator.set_default_namespace(default_expression_language);
        }

        let element = process.element();
        let log_broadcast = process.log_broadcast();

        Self {
            receiver,
            process,
            flow_nodes,
            flow_nodes_outgoing,
            flow_nodes_incoming,
            expression_evaluator,
            element,
            log_broadcast,
            data_objects,
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
                       Some(Request::DataObject(id, sender)) => {
                           self.get_data_object(&id, sender);
                       }
                       None => {}
                   },
               // Flow node processing
               next = self.flow_nodes.next() => {
                   if let Some(next) = next {
                           self.process_flow_node_next(next).await;
                   }
               }
            }
        }
    }

    async fn probe_sequence_flow(&mut self, seq_flow: &SequenceFlow) -> bool {
        if let Some(SequenceFlowConditionExpression(Expr::FormalExpression(
            ref expr @ FormalExpression { .. },
        ))) = seq_flow.condition_expression
        {
            let expr = expr.clone();
            match self
                .expression_evaluator
                .eval::<bool>(&expr, &mut self.expression_evaluator.new_context())
                .await
            {
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
    fn next_action(&mut self, action: Option<flow_node::Action>, token: usize) -> Control {
        if let Some(flow_node) = self.flow_nodes.get(token) {
            flow_node.element().incomings().iter().fold(
                Control::Proceed(action),
                |control, incoming| {
                    match control {
                        // once the action has been dropped, it's not checked against
                        // any other incoming flows
                        Control::Drop => control,
                        Control::Proceed(action) => {
                            let matching_predecessor = self.flow_nodes_outgoing.get(incoming);
                            if let Some((previous_token, index)) = matching_predecessor {
                                if let Some(incoming_node) =
                                    self.flow_nodes.get_mut(*previous_token)
                                {
                                    match incoming_node.handle_outgoing_action(*index, action) {
                                        None => Control::Drop,
                                        Some(action) => Control::Proceed(action),
                                    }
                                } else {
                                    Control::Proceed(action)
                                }
                            } else {
                                Control::Proceed(action)
                            }
                        }
                    }
                },
            )
        } else {
            Control::Drop
        }
    }

    async fn process_flow_node_next(&mut self, (next, token): (StreamYield<FlowNode>, usize)) {
        if self.flow_nodes.get(token).is_none() {
            // this shouldn't happen, but... (do nothing)
            // at least because of this check we can safely unwrap `flow_nodes.get*` below
            return;
        }
        if let StreamYield::Item(action) = next {
            let next_action = self.next_action(Some(action), token);
            match next_action {
                // We're good to proceed with the following probing action
                Control::Proceed(Some(flow_node::Action::ProbeOutgoingSequenceFlows(indices))) => {
                    let outgoings = self
                        .flow_nodes
                        .get(token)
                        .unwrap()
                        .element()
                        .outgoings()
                        .clone();
                    for index in indices {
                        let seq_flow = self
                            .element
                            .find_by_id(&outgoings[index])
                            .and_then(|seq_flow| seq_flow.downcast_ref::<SequenceFlow>())
                            .cloned();
                        if let Some(seq_flow) = seq_flow {
                            let success = self.probe_sequence_flow(&seq_flow).await;
                            self.flow_nodes
                                .get_mut(token)
                                .unwrap()
                                .sequence_flow(index, &seq_flow, success);
                        }
                    }
                }
                // We're good to proceed with the following flow action
                Control::Proceed(Some(flow_node::Action::Flow(ref indices))) => {
                    let el = self.flow_nodes.get(token).unwrap().element();
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
                                let node = &mut next_node.node;
                                // match target's node incoming index for this sequence flow
                                if let Some((_, index)) =
                                    self.flow_nodes_incoming.get(seq_flow.id.as_ref().unwrap())
                                {
                                    // there's an incoming
                                    let _ = self.log_broadcast.send(Log::FlowNodeIncoming {
                                        node: node.element().clone(),
                                        incoming_index: *index,
                                    });
                                    // increase the number of tokens by a number of added flows
                                    next_node.tokens += indices.len();
                                    // report it to the target node
                                    node.tokens(next_node.tokens);
                                    // and report the incoming
                                    node.incoming(*index);
                                }
                            }
                        }
                    }
                }
                // flow node completion
                Control::Proceed(Some(flow_node::Action::Complete)) => {
                    let _ = self.log_broadcast.send(Log::FlowNodeCompleted {
                        node: self.flow_nodes.get(token).unwrap().element().clone(),
                    });
                }
                // nothing, don't reschedule this flow node anymore
                Control::Proceed(None) => {
                    if self.flow_nodes.is_empty() {
                        let _ = self.log_broadcast.send(Log::Done);
                    }
                    Pin::new(&mut self.flow_nodes).remove(token);
                }
                // no action to be taken
                Control::Drop => {}
            }
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

    fn get_data_object(
        &self,
        id: &str,
        sender: oneshot::Sender<Result<DataObjectContainer, DataObjectError>>,
    ) {
        if let Some(container) = self.data_objects.get(id) {
            let _ = sender.send(Ok(container.clone()));
        } else {
            let _ = sender.send(Err(DataObjectError::NotFound));
        }
    }
}
