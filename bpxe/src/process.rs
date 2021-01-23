//! # Process
use crate::bpmn::schema::{FlowNodeType, Process as Element};
use crate::event::ProcessEvent as Event;
use crate::flow_node;

use crate::model;

use std::sync::Arc;

use tokio::sync::{broadcast, mpsc, oneshot};
use tokio::task::{self, JoinHandle};

use thiserror::Error;

mod scheduler;
use scheduler::Scheduler;

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

pub(crate) enum Request {
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
    #[cfg(test)]
    /// Flow node report of tokens (for testing)
    FlowNodeTokens {
        node: Box<dyn FlowNodeType>,
        count: usize,
    },
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
