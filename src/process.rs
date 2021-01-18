//! # Process
use crate::bpmn::schema::{FlowElement, FlowNodeType, Process as Element, StartEvent};
use crate::event::start_event;
use crate::flow_node;
use crate::model;
use std::sync::Arc;
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
#[derive(Clone, Debug)]
pub struct Handle {
    model: model::Handle,
    element: Arc<Element>,
    sender: mpsc::Sender<Request>,
    event_broadcast: broadcast::Sender<Event>,
}

#[derive(Debug)]
enum Request {
    JoinHandle(JoinHandle<()>),
    Terminate(oneshot::Sender<Option<JoinHandle<()>>>),
    Start(oneshot::Sender<Result<(), StartError>>),
}

/// Process events
#[derive(Clone, Debug)]
pub enum Event {
    /// Flow node has been selected for execution
    FlowNodeSelected { node: Box<dyn FlowNodeType> },
    /// Flow node execution has been completed
    FlowNodeCompleted { node: Box<dyn FlowNodeType> },
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
        let (event_broadcast, _) = broadcast::channel(128);
        let element = self.element.clone();
        let handle = Handle {
            sender: sender.clone(),
            model: self.model.clone(),
            event_broadcast,
            element,
        };

        let handle_clone = handle.clone();

        let join_handle = task::spawn(async move { self.runner(receiver, handle_clone).await });

        let _ = sender.send(Request::JoinHandle(join_handle)).await;

        handle
    }

    // Main loop
    async fn runner(mut self, mut receiver: mpsc::Receiver<Request>, handle: Handle) {
        let mut join_handle = None;
        // Process requests until termination
        loop {
            tokio::select! {
            next = receiver.recv()  =>
                match next {
                    Some(Request::JoinHandle(handle)) => join_handle = Some(handle),
                    Some(Request::Terminate(sender)) => {
                        let _ = sender.send(join_handle.take());
                        return;
                    }
                    Some(Request::Start(sender)) => {
                        self.start(sender, handle.clone());
                    }
                    None => {}
                }
            }
        }
    }

    // Explicit process start
    fn start(&mut self, sender: oneshot::Sender<Result<(), StartError>>, handle: Handle) {
        let mut start_events = self
            .element
            .flow_elements
            .iter()
            .filter_map(|e| match e {
                FlowElement::StartEvent(start_event) => {
                    // Unwrapping here should be safe because we already know that `start_event`
                    // is a `StartEvent`
                    let (node, _) =
                        flow_node::make::<StartEvent, start_event::StartEvent>(start_event)
                            .unwrap();
                    Some(flow_node::spawn(
                        Box::new(start_event.clone()),
                        node,
                        handle.clone(),
                    ))
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        if start_events.is_empty() {
            let _ = sender.send(Err(StartError::NoStartEvent));
            return;
        }
        for start_event in start_events.drain(..) {
            task::spawn(async move {
                start_event.await;
            });
        }
        let _ = sender.send(Ok(()));
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

    /// Returns event receiver
    pub fn event_receiver(&self) -> broadcast::Receiver<Event> {
        self.event_broadcast.subscribe()
    }

    /// Returns event broadcaster
    pub(crate) fn event_broadcast(&self) -> broadcast::Sender<Event> {
        self.event_broadcast.clone()
    }

    /// Returns `process` element
    pub fn element(&self) -> Arc<Element> {
        self.element.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::{Event, Process as P, StartError};
    use crate::bpmn::schema::*;
    use crate::model;
    use crate::test::*;

    #[tokio::test]
    async fn no_start_event() {
        let mut proc1: Process = Default::default();
        proc1.id = Some("proc1".into());
        let mut definitions: Definitions = Default::default();
        definitions
            .root_elements
            .push(RootElement::Process(proc1.clone()));
        let model = model::Model::new(definitions).spawn().await;
        let handle = P::new(proc1, model).spawn().await;
        assert_eq!(handle.start().await, Err::<(), _>(StartError::NoStartEvent));
    }

    #[tokio::test]
    async fn single_start_event() {
        let mut proc1: Process = Default::default();
        proc1.id = Some("proc1".into());
        let mut start_event: StartEvent = Default::default();
        start_event.id = Some("start".into());
        proc1
            .flow_elements
            .push(FlowElement::StartEvent(start_event));
        let mut definitions: Definitions = Default::default();
        definitions
            .root_elements
            .push(RootElement::Process(proc1.clone()));
        let model = model::Model::new(definitions).spawn().await;

        let handle = P::new(proc1, model).spawn().await;
        let mut mailbox = Mailbox::new(handle.event_receiver());
        assert!(handle.start().await.is_ok());
        assert!(
            mailbox
                .receive(|e| if let Event::FlowNodeCompleted { node } = e {
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
        let mut proc1: Process = Default::default();
        proc1.id = Some("proc1".into());
        let mut start_event1: StartEvent = Default::default();
        start_event1.id = Some("start1".into());
        let mut start_event2: StartEvent = Default::default();
        start_event2.id = Some("start2".into());

        proc1
            .flow_elements
            .push(FlowElement::StartEvent(start_event1));

        proc1
            .flow_elements
            .push(FlowElement::StartEvent(start_event2));

        let mut definitions: Definitions = Default::default();
        definitions
            .root_elements
            .push(RootElement::Process(proc1.clone()));
        let model = model::Model::new(definitions).spawn().await;

        let handle = P::new(proc1, model).spawn().await;

        let mut mailbox = Mailbox::new(handle.event_receiver());
        assert!(handle.start().await.is_ok());

        assert!(
            mailbox
                .receive(|e| if let Event::FlowNodeCompleted { node } = e {
                    matches!(node.downcast_ref::<StartEvent>(),
                    Some(start_event) if start_event.id().as_ref().unwrap() == "start1")
                } else {
                    false
                })
                .await
        );

        assert!(
            mailbox
                .receive(|e| if let Event::FlowNodeCompleted { node } = e {
                    matches!(node.downcast_ref::<StartEvent>(),
                    Some(start_event) if start_event.id().as_ref().unwrap() == "start2")
                } else {
                    false
                })
                .await
        );
    }
}
