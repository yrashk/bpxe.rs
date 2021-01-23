//! # Process
use crate::bpmn::schema::{
    DocumentElementContainer, Element as E, Expr, FlowNodeType, FormalExpression,
    Process as Element, ProcessType, SequenceFlow,
};
use crate::event::ProcessEvent as Event;
use crate::flow_node;
use crate::language::ExpressionEvaluator;
use crate::model;
use futures::future::FutureExt;
use futures::stream::{FuturesUnordered, Stream, StreamExt, StreamFuture};

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::sync::{broadcast, mpsc, oneshot};
use tokio::task::{self, JoinHandle};

use thiserror::Error;

/// Process start error
#[derive(Error, Debug, PartialEq)]
pub enum StartError {
    /// No startEvent element found
    #[error("no startEvent element found")]
    NoStartEvent,
    /// Response has not been received
    #[error("response has not been received")]
    NotReceived,
}

/// Process container
pub struct Process {
    element: Arc<Element>,
    model: model::Handle,
}

/// Control handle for a running process
#[derive(Clone)]
pub struct Handle {
    model: model::Handle,
    element: Arc<Element>,
    sender: mpsc::Sender<Request>,
    log_broadcast: broadcast::Sender<Log>,
    event_broadcast: broadcast::Sender<Event>,
}

enum Request {
    JoinHandle(JoinHandle<()>),
    Terminate(oneshot::Sender<Option<JoinHandle<()>>>),
    Start(oneshot::Sender<Result<(), StartError>>),
}

/// Process events
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Log {
    /// Flow node has received an incoming flow (activated for each incoming flow)
    FlowNodeIncoming {
        node: Box<dyn FlowNodeType>,
        incoming_index: flow_node::IncomingIndex,
    },
    /// Flow node execution has been completed
    FlowNodeCompleted { node: Box<dyn FlowNodeType> },
    /// No default path is available for a node
    NoDefaultPath { node: Box<dyn FlowNodeType> },
    /// Expression evaluation error
    ExpressionError { error: String },
    /// There are no more flow nodes to schedule, ever
    Done,
}

impl Process {
    /// Creates a new process container
    pub fn new(element: Element, model: model::Handle) -> Self {
        Self {
            element: Arc::new(element),
            model,
        }
    }

    /// Spawns process task
    pub async fn spawn(self) -> Handle {
        let (sender, receiver) = mpsc::channel(1);
        let (log_broadcast, _) = broadcast::channel(128);
        let (event_broadcast, _) = broadcast::channel(128);
        let element = self.element.clone();
        let handle = Handle {
            sender: sender.clone(),
            model: self.model.clone(),
            log_broadcast,
            event_broadcast,
            element,
        };

        let scheduler = Scheduler::new(receiver, handle.clone());

        let join_handle = task::spawn(async move { scheduler.run().await });

        let _ = sender.send(Request::JoinHandle(join_handle)).await;

        handle
    }
}

impl Handle {
    /// Request and wait for model execution termination
    pub async fn terminate(self) {
        let (sender, receiver) = oneshot::channel();
        let _ = self.sender.send(Request::Terminate(sender)).await;
        if let Ok(Some(handle)) = receiver.await {
            let _ = handle.await;
        }
    }

    /// Request explicit process start
    pub async fn start(&self) -> Result<(), StartError> {
        let (sender, receiver) = oneshot::channel();
        let _ = self.sender.send(Request::Start(sender)).await;
        if let Ok(result) = receiver.await {
            result
        } else {
            Err(StartError::NotReceived)
        }
    }

    /// Returns model handle
    pub fn model(&self) -> model::Handle {
        self.model.clone()
    }

    /// Returns log receiver
    pub fn log_receiver(&self) -> broadcast::Receiver<Log> {
        self.log_broadcast.subscribe()
    }

    /// Returns log broadcaster
    pub fn log_broadcast(&self) -> broadcast::Sender<Log> {
        self.log_broadcast.clone()
    }

    /// Returns `process` element
    pub fn element(&self) -> Arc<Element> {
        self.element.clone()
    }

    /// Returns event receiver
    pub fn event_receiver(&self) -> broadcast::Receiver<Event> {
        self.event_broadcast.subscribe()
    }

    /// Returns event broadcaster
    pub fn event_broadcast(&self) -> broadcast::Sender<Event> {
        self.event_broadcast.clone()
    }
}

struct Scheduler {
    receiver: mpsc::Receiver<Request>,
    process: Handle,
    flow_nodes: FuturesUnordered<NamedStreamFuture<Box<dyn flow_node::FlowNode>>>,
}

// FIXME: We're using this structure to be able to find flow nodes by their identifier
// in `FuturesUnordered` (`Scheduler.flow_nodes`). It's a linear search and is probably
// fine when there's a small number of flow nodes, but should it become large, this approach
// should probably be rethought.
struct NamedStreamFuture<T>(String, StreamFuture<T>);

use std::ops::{Deref, DerefMut};

impl<T> Deref for NamedStreamFuture<T>
where
    T: Unpin + Stream,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // FIXME: is there any better way to do this?
        // I *think* it's reasonable to assume it won't panic in runtime
        // because when it's used, scheduler is not doing anything with the future.
        // However, I am not confident in this.
        self.1.get_ref().unwrap()
    }
}

impl<T> DerefMut for NamedStreamFuture<T>
where
    T: Unpin + Stream,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        // FIXME: see above in `Deref` implementation
        self.1.get_mut().unwrap()
    }
}

impl<T> Future for NamedStreamFuture<T>
where
    T: Unpin + Stream,
{
    type Output = (String, <StreamFuture<T> as Future>::Output);

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.1.poll_unpin(cx).map(|v| (self.0.clone(), v))
    }
}

impl Scheduler {
    fn new(receiver: mpsc::Receiver<Request>, process: Handle) -> Self {
        let flow_nodes = process
            .element()
            .flow_elements()
            .iter()
            .map(|e| e.clone().into_inner())
            .filter_map(|e| {
                flow_node::new(e.as_ref()).map(|mut flow_node| {
                    flow_node.set_process(process.clone());
                    let e = flow_node.element();
                    NamedStreamFuture(
                        // FIXME: decide what should we do with flow nodes that don't have ID.
                        // They can't be connected with other nodes (there's no way to refer to
                        // them), but they can still be operational in a single flow node operation
                        // (even though this might be a degenerative case)
                        e.id().as_ref().unwrap_or(&"".to_string()).to_string(),
                        flow_node.into_future(),
                    )
                })
            })
            .collect();
        Self {
            receiver,
            process,
            flow_nodes,
        }
    }

    // Main loop
    async fn run(mut self) {
        let mut join_handle = None;
        let element = self.process.element();
        let log_broadcast = self.process.log_broadcast();
        let expression_evaluator: ExpressionEvaluator = Default::default();
        let default_expression_language = self
            .process
            .model()
            .definitions()
            .expression_language
            .clone();

        // This function is async even though nothing in it is asynchronous
        // at this moment. This is done with an expectation that expression
        // evaluation *might* become asynchronous in the future.
        async fn probe_sequence_flow(
            expression_evaluator: &ExpressionEvaluator,
            seq_flow: &SequenceFlow,
            default_expression_language: Option<&String>,
            log_broadcast: broadcast::Sender<Log>,
        ) -> bool {
            if let Some(Expr::FormalExpression(FormalExpression {
                content: Some(ref content),
                ..
            })) = seq_flow.condition_expression
            {
                match expression_evaluator.eval_expr(default_expression_language, content) {
                    Ok(result) => result,
                    Err(err) => {
                        let _ = log_broadcast.send(Log::ExpressionError {
                            error: format!("{:?}", err),
                        });
                        false
                    }
                }
            } else {
                true
            }
        }
        loop {
            task::yield_now().await;
            tokio::select! {
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
               next = self.flow_nodes.next() => {
                   if let Some((id, (action, mut flow_node))) = next  {
                       // Figure out if this action should be transformed, kept as is, or dropped
                       enum Control {
                           Proceed(Option<flow_node::Action>),
                           Drop
                       }
                       let control = flow_node.element().incomings().iter().
                           fold(Control::Proceed(action), |control, incoming| {
                               match control {
                                   // once the action has been dropped, it's not checked against
                                   // any other incoming flows
                                   Control::Drop => control,
                                   Control::Proceed(action) => {
                                       let mut matching_predecessor = self.flow_nodes.iter_mut().find(|node|
                                           node.element().outgoings().iter()
                                           .any(|outgoing| outgoing == incoming));
                                           if let Some(ref mut node) = matching_predecessor {
                                               // it's ok to unwrap here because we already know such
                                               // predecessor exists
                                               let index = node.element().outgoings().iter().
                                                   enumerate().find_map(|(i, name)| if name == incoming {
                                                       Some(i)
                                                   } else {
                                                       None
                                                   }).unwrap();
                                               match node.handle_outgoing_action(index, action) {
                                                   None => Control::Drop,
                                                   Some(action) => Control::Proceed(action),
                                                   }
                                           } else {
                                               Control::Proceed(action)
                                           }
                                   }
                               }
                           });
                       match control {
                           Control::Proceed(Some(flow_node::Action::ProbeOutgoingSequenceFlows(indices))) => {
                               let outgoings = flow_node.element().outgoings().clone();
                               for index in indices {
                                   let seq_flow = {
                                       element.find_by_id(&outgoings[index])
                                           .and_then(|seq_flow| seq_flow.downcast_ref::<SequenceFlow>())
                                   };
                                   if let Some(seq_flow) = seq_flow {
                                       let success = probe_sequence_flow(&expression_evaluator, &seq_flow,
                                           default_expression_language.as_ref(),
                                           log_broadcast.clone()).await;
                                       flow_node.sequence_flow(index, &seq_flow, success);
                                   }
                               }
                           }
                           Control::Proceed(Some(flow_node::Action::Flow(indices))) => {
                               let outgoings = flow_node.element().outgoings().clone();
                               for index in indices {
                                   // FIXME: see above about ID-less flow nodes
                                   let seq_flow = {
                                       element.find_by_id(&outgoings[index])
                                           .and_then(|seq_flow| seq_flow.downcast_ref::<SequenceFlow>())
                                   };

                                   if let Some(seq_flow) = seq_flow {
                                       let success = probe_sequence_flow(&expression_evaluator, &seq_flow,
                                           default_expression_language.as_ref(),
                                           log_broadcast.clone()).await;
                                       if success {
                                           for next_node in self.flow_nodes.iter_mut() {
                                               if next_node.0 == seq_flow.target_ref {
                                                   let target_node = &mut next_node.1;
                                                   let node = target_node.get_mut();
                                                   node.and_then(|node|
                                                       node.element().incomings().iter().enumerate().
                                                       find_map(|(index, incoming)|
                                                           if incoming == seq_flow.id.as_ref().unwrap() {
                                                               Some(index)
                                                           } else {
                                                               None
                                                           })
                                                       .map(|index| {
                                                           let _ = log_broadcast.send(Log::FlowNodeIncoming {
                                                               node: node.element().clone(),
                                                               incoming_index: index
                                                           });
                                                           node.incoming(index)
                                                       }));

                                               }
                                           }
                                       }
                                   }
                               }
                           }
                           Control::Proceed(Some(flow_node::Action::Complete)) => {
                               let _ = log_broadcast.send(Log::FlowNodeCompleted { node: flow_node.element().clone() });
                           }
                           Control::Proceed(None) => {
                               if self.flow_nodes.is_empty() {
                                   let _ = log_broadcast.send(Log::Done);
                               }
                               continue
                           }
                           Control::Drop => {}
                       }
                       // Reschedule the flow node
                       self.flow_nodes.push(NamedStreamFuture(id, flow_node.into_future()));
                   }
               },
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
}

#[cfg(test)]
mod tests {
    use super::{Log, StartError};
    use crate::bpmn::schema::*;
    use crate::model;
    use crate::test::*;

    #[tokio::test]
    async fn no_start_event() {
        let definitions = Definitions {
            root_elements: vec![Process {
                id: Some("proc1".into()),
                ..Default::default()
            }
            .into()],
            ..Default::default()
        };
        let model = model::Model::new(definitions).spawn().await;
        let handle = model.processes().await.unwrap().pop().unwrap();
        assert_eq!(handle.start().await, Err::<(), _>(StartError::NoStartEvent));
    }

    #[tokio::test]
    async fn single_start_event() {
        let definitions = Definitions {
            root_elements: vec![Process {
                id: Some("proc1".into()),
                flow_elements: vec![StartEvent {
                    id: Some("start".into()),
                    ..Default::default()
                }
                .into()],
                ..Default::default()
            }
            .into()],
            ..Default::default()
        };

        let model = model::Model::new(definitions).spawn().await;
        let handle = model.processes().await.unwrap().pop().unwrap();
        let mut mailbox = Mailbox::new(handle.log_receiver());
        assert!(handle.start().await.is_ok());
        assert!(
            mailbox
                .receive(|e| if let Log::FlowNodeCompleted { node } = e {
                    matches!(node.downcast_ref::<StartEvent>(),
                    Some(start_event) if start_event.id().as_ref().unwrap() == "start")
                } else {
                    false
                })
                .await
        );
    }

    #[tokio::test]
    async fn multiple_start_events() {
        let definitions = Definitions {
            root_elements: vec![Process {
                id: Some("proc1".into()),
                flow_elements: vec![
                    StartEvent {
                        id: Some("start1".into()),
                        ..Default::default()
                    }
                    .into(),
                    StartEvent {
                        id: Some("start2".into()),
                        ..Default::default()
                    }
                    .into(),
                ],
                ..Default::default()
            }
            .into()],
            ..Default::default()
        };
        let model = model::Model::new(definitions).spawn().await;

        let handle = model.processes().await.unwrap().pop().unwrap();

        let mut mailbox = Mailbox::new(handle.log_receiver());
        assert!(handle.start().await.is_ok());

        assert!(
            mailbox
                .receive(|e| if let Log::FlowNodeCompleted { node } = e {
                    matches!(node.downcast_ref::<StartEvent>(),
                    Some(start_event) if start_event.id().as_ref().unwrap() == "start1")
                } else {
                    false
                })
                .await
        );

        assert!(
            mailbox
                .receive(|e| if let Log::FlowNodeCompleted { node } = e {
                    matches!(node.downcast_ref::<StartEvent>(),
                    Some(start_event) if start_event.id().as_ref().unwrap() == "start2")
                } else {
                    false
                })
                .await
        );
    }

    #[tokio::test]
    async fn incoming_log() {
        let mut definitions = Definitions {
            root_elements: vec![Process {
                id: Some("proc1".into()),
                flow_elements: vec![
                    StartEvent {
                        id: Some("start".into()),
                        ..Default::default()
                    }
                    .into(),
                    EndEvent {
                        id: Some("end".into()),
                        ..Default::default()
                    }
                    .into(),
                ],
                ..Default::default()
            }
            .into()],
            ..Default::default()
        };

        definitions
            .find_by_id_mut("proc1")
            .unwrap()
            .downcast_mut::<Process>()
            .unwrap()
            .establish_sequence_flow("start", "end", "s1", None::<FormalExpression>)
            .unwrap();

        let model = model::Model::new(definitions).spawn().await;
        let handle = model.processes().await.unwrap().pop().unwrap();
        let mut mailbox = Mailbox::new(handle.log_receiver());
        assert!(handle.start().await.is_ok());
        assert!(
            mailbox
                .receive(|e| {
                    if let Log::FlowNodeIncoming {
                        node,
                        incoming_index: 0,
                    } = e
                    {
                        matches!(node.downcast_ref::<EndEvent>(),
                    Some(end_event) if end_event.id().as_ref().unwrap() == "end")
                    } else {
                        false
                    }
                })
                .await
        );
    }
}
