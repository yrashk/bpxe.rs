//! # Intermediate Throw Event flow node
use crate::bpmn::schema::{
    Cast, EventDefinitionType, FlowNodeType, IntermediateThrowEvent as Element,
};
use crate::event::ProcessEvent;
use crate::flow_node::{self, Action, FlowNode, IncomingIndex};
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};
use tokio::sync::broadcast;

/// Intermediate Throw Event flow node
pub struct IntermediateThrowEvent {
    element: Arc<Element>,
    state: State,
    waker: Option<Waker>,
    event_broadcaster: Option<broadcast::Sender<ProcessEvent>>,
}

impl IntermediateThrowEvent {
    /// Creates new Intermediate Throw Event flow node
    pub fn new(element: Element) -> Self {
        Self {
            element: Arc::new(element),
            state: State::Ready,
            waker: None,
            event_broadcaster: None,
        }
    }
}

/// Node state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum State {
    Ready,
    Complete,
    Done,
}

impl FlowNode for IntermediateThrowEvent {
    fn set_state(&mut self, state: flow_node::State) -> Result<(), flow_node::StateError> {
        match state {
            flow_node::State::IntermediateThrowEvent(state) => {
                self.state = state;
                Ok(())
            }
            _ => Err(flow_node::StateError::InvalidVariant),
        }
    }

    fn get_state(&mut self) -> flow_node::State {
        flow_node::State::IntermediateThrowEvent(self.state.clone())
    }

    fn element(&self) -> Box<dyn FlowNodeType> {
        Box::new(self.element.as_ref().clone())
    }

    fn incoming(&mut self, _index: IncomingIndex) {
        // Any incoming triggered is good enough to
        // complete IntermediateThrowEvent
        self.state = State::Complete;
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }

    fn set_process(&mut self, process: crate::process::Handle) {
        self.event_broadcaster.replace(process.event_broadcast());
    }
}

impl From<Element> for IntermediateThrowEvent {
    fn from(element: Element) -> Self {
        Self::new(element)
    }
}

impl Stream for IntermediateThrowEvent {
    type Item = Action;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.state {
            State::Ready => {
                self.waker.replace(cx.waker().clone());
                Poll::Pending
            }
            State::Complete => {
                self.state = State::Done;
                if let Some(event_broadcaster) = self.event_broadcaster.as_ref() {
                    if self.element.event_definitions.is_empty() {
                        let _ = event_broadcaster.send(ProcessEvent::NoneEvent);
                    } else {
                        for event_definition in &self.element.event_definitions {
                            if let Some(definition) = Cast::<dyn EventDefinitionType>::cast(
                                event_definition.clone().into_inner().as_ref(),
                            ) {
                                use std::convert::TryFrom;
                                if let Ok(event) = ProcessEvent::try_from(definition) {
                                    let _ = event_broadcaster.send(event);
                                }
                            }
                        }
                    }
                }
                Poll::Ready(Some(Action::Flow(
                    (0..self.element.outgoings().len()).collect(),
                )))
            }
            State::Done => {
                self.waker.replace(cx.waker().clone());
                self.state = State::Ready;
                Poll::Pending
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bpmn::parse;
    use crate::event::ProcessEvent;
    use crate::model;
    use crate::test::Mailbox;

    #[tokio::test]
    async fn throw_none_event() {
        let definitions = parse(include_str!("test_models/throw_none_event.bpmn")).unwrap();
        let model = model::Model::new(definitions).spawn().await;

        let handle = model.processes().await.unwrap().pop().unwrap();
        let mut mailbox = Mailbox::new(handle.event_receiver());

        assert!(handle.start().await.is_ok());

        assert!(
            mailbox
                .receive(|e| matches!(e, ProcessEvent::NoneEvent))
                .await
        );
    }

    #[tokio::test]
    async fn throw_signal_event() {
        let definitions = parse(include_str!("test_models/throw_signal_event.bpmn")).unwrap();
        let model = model::Model::new(definitions).spawn().await;

        let handle = model.processes().await.unwrap().pop().unwrap();
        let mut mailbox = Mailbox::new(handle.event_receiver());

        assert!(handle.start().await.is_ok());

        assert!(
            mailbox
                .receive(|e| matches!(e, ProcessEvent::SignalEvent { signal_ref } if signal_ref.as_ref().unwrap() == "sig1"))
                .await
        );
    }
}
