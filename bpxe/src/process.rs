//! # Process
use crate::bpmn::schema::{FlowNodeType, Process as Element};
use crate::data_object::DataObject;
use crate::event::ProcessEvent as Event;
use crate::flow_node;
use crate::model;
use crate::sys::task::{self, JoinHandle};
use serde::Serialize;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::{broadcast, mpsc, oneshot, RwLock};

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

/// Data object container
pub type DataObjectContainer = Arc<RwLock<Box<dyn DataObject>>>;

/// Data object retrieval error
#[derive(Error, Debug, PartialEq)]
pub enum DataObjectError {
    #[error("data object not found")]
    NotFound,
    #[error("response has not been received")]
    NotReceived,
}

pub(crate) enum Request {
    JoinHandle(JoinHandle<()>),
    Terminate(oneshot::Sender<Option<JoinHandle<()>>>),
    Start(oneshot::Sender<Result<(), StartError>>),
    DataObject(
        String,
        oneshot::Sender<Result<DataObjectContainer, DataObjectError>>,
    ),
}

/// Process events
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum Log {
    /// Flow node has received an incoming flow (activated for each incoming flow)
    FlowNodeIncoming {
        #[serde(serialize_with = "crate::serde::serialize_flow_node")]
        node: Box<dyn FlowNodeType>,
        incoming_index: flow_node::IncomingIndex,
    },
    /// Flow node execution has been completed
    FlowNodeCompleted {
        #[serde(serialize_with = "crate::serde::serialize_flow_node")]
        node: Box<dyn FlowNodeType>,
    },
    #[cfg(test)]
    /// Flow node report of tokens (for testing)
    FlowNodeTokens {
        #[serde(serialize_with = "crate::serde::serialize_flow_node")]
        node: Box<dyn FlowNodeType>,
        count: usize,
    },
    /// No default path is available for a node
    NoDefaultPath {
        #[serde(serialize_with = "crate::serde::serialize_flow_node")]
        node: Box<dyn FlowNodeType>,
    },
    /// Expression evaluation error
    ExpressionError { error: String },
    /// Script evaluation error
    ScriptError { error: String },
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

    /// Returns a data object container
    pub async fn data_object(&self, id: &str) -> Result<DataObjectContainer, DataObjectError> {
        let (sender, receiver) = oneshot::channel();
        let _ = self
            .sender
            .send(Request::DataObject(id.to_owned(), sender))
            .await;
        if let Ok(result) = receiver.await {
            result
        } else {
            Err(DataObjectError::NotReceived)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Log, StartError};
    use crate::bpmn::parse;
    use crate::bpmn::schema::*;
    use crate::model;
    use crate::test::*;
    use bpxe_internal_macros as bpxe_im;

    #[bpxe_im::test]
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

        model.terminate().await;
    }

    #[bpxe_im::test]
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

        model.terminate().await;
    }

    #[bpxe_im::test]
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

        model.terminate().await;
    }

    #[bpxe_im::test]
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

        model.terminate().await;
    }

    #[bpxe_im::test]
    async fn data_object() {
        use crate::data_object;
        use serde_json::json;
        let definitions = parse(include_str!("process/test_models/data_object.bpmn")).unwrap();
        let model = model::Model::new(definitions).spawn().await;
        let handle = model.processes().await.unwrap().pop().unwrap();
        let data_object = handle.data_object("data_object").await.unwrap();
        let read = data_object.read().await;
        // It should be empty
        assert!(read.downcast_ref::<data_object::Empty>().is_some());
        drop(read);
        let mut write = data_object.write().await;
        // Write something into it
        *write = Box::new(json!({"test": "passed"}));
        drop(write);
        // Get it again
        let data_object = handle.data_object("data_object").await.unwrap();
        let read = data_object.read().await;
        // It should have the value
        assert_eq!(
            read.downcast_ref::<serde_json::Value>().unwrap(),
            &json!({"test": "passed"})
        );
        // Since `data_object` is actual a data object reference to `DataObject`, let's check the
        // actual object.
        let data_object = handle.data_object("DataObject").await.unwrap();
        let read = data_object.read().await;
        // It should have the value
        assert_eq!(
            read.downcast_ref::<serde_json::Value>().unwrap(),
            &json!({"test": "passed"})
        );

        model.terminate().await;
    }
}
