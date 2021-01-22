//! # Event-Based Gateway
use crate::bpmn::schema::{EventBasedGateway as Element, FlowNodeType};
use crate::flow_node::{self, Action, FlowNode, IncomingIndex, OutgoingIndex};
use crate::process;
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};

/// Event-Based Gateway flow node
pub struct Gateway {
    element: Arc<Element>,
    state: State,
    waker: Option<Waker>,
    process: Option<process::Handle>,
}

impl Gateway {
    /// Creates new Event-Based Gateway flow node
    pub fn new(element: Element) -> Self {
        let element = Arc::new(element);
        Self {
            element,
            state: State::Initialized,
            waker: None,
            process: None,
        }
    }
}

/// Node state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum State {
    Initialized,
    Ready,
    Flowed,
    Done,
}

impl FlowNode for Gateway {
    fn set_state(&mut self, state: flow_node::State) -> Result<(), flow_node::StateError> {
        match state {
            flow_node::State::EventBasedGateway(state) => {
                self.state = state;
                Ok(())
            }
            _ => Err(flow_node::StateError::InvalidVariant),
        }
    }

    fn set_process(&mut self, process: process::Handle) {
        self.process = Some(process);
    }

    fn get_state(&self) -> flow_node::State {
        flow_node::State::EventBasedGateway(self.state.clone())
    }

    fn element(&self) -> Box<dyn FlowNodeType> {
        Box::new(self.element.as_ref().clone())
    }

    fn incoming(&mut self, _index: IncomingIndex) {
        if let State::Initialized = self.state {
            self.state = State::Ready;
            if let Some(waker) = self.waker.take() {
                waker.wake();
            }
        }
    }

    fn handle_outgoing_action(
        &mut self,
        _index: OutgoingIndex,
        action: Option<Action>,
    ) -> Option<Option<Action>> {
        if let State::Flowed = self.state {
            if let Some(Action::Flow(_)) = &action {
                self.state = State::Done;
                if let Some(waker) = self.waker.take() {
                    waker.wake();
                }
            }
            Some(action)
        } else {
            None
        }
    }
}

impl From<Element> for Gateway {
    fn from(element: Element) -> Self {
        Self::new(element)
    }
}

impl Stream for Gateway {
    type Item = Action;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.state {
            State::Initialized => {
                self.waker.replace(cx.waker().clone());
                Poll::Pending
            }
            State::Ready => {
                self.waker.replace(cx.waker().clone());
                self.state = State::Flowed;
                Poll::Ready(Some(Action::Flow(
                    (0..self.element().outgoings().len()).collect(),
                )))
            }
            State::Flowed => {
                self.waker.replace(cx.waker().clone());
                Poll::Pending
            }
            State::Done => {
                self.state = State::Initialized;
                self.waker.replace(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bpmn::schema::*;
    use crate::event::ProcessEvent;
    use crate::model;
    use crate::process::Log;
    use crate::test::*;
    use std::time::Duration;
    use tokio::time::timeout;

    #[tokio::test]
    async fn event() {
        let mut definitions = Definitions {
            root_elements: vec![
                Process {
                    id: Some("proc1".into()),
                    flow_elements: vec![
                        StartEvent {
                            id: Some("start".into()),
                            ..Default::default()
                        }
                        .into(),
                        EventBasedGateway {
                            id: Some("ev".into()),
                            ..Default::default()
                        }
                        .into(),
                        IntermediateCatchEvent {
                            id: Some("f1".into()),
                            event_definitions: vec![SignalEventDefinition {
                                signal_ref: Some("f1sig".into()),
                                ..Default::default()
                            }
                            .into()],
                            ..Default::default()
                        }
                        .into(),
                        IntermediateCatchEvent {
                            id: Some("f2".into()),
                            event_definitions: vec![SignalEventDefinition {
                                signal_ref: Some("f2sig".into()),
                                ..Default::default()
                            }
                            .into()],
                            ..Default::default()
                        }
                        .into(),
                        IntermediateThrowEvent {
                            id: Some("f1t".into()),
                            event_definitions: vec![SignalEventDefinition {
                                signal_ref: Some("f1report".into()),
                                ..Default::default()
                            }
                            .into()],
                            ..Default::default()
                        }
                        .into(),
                        IntermediateThrowEvent {
                            id: Some("f2t".into()),
                            event_definitions: vec![SignalEventDefinition {
                                signal_ref: Some("f2report".into()),
                                ..Default::default()
                            }
                            .into()],
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
                .into(),
                Signal {
                    id: Some("f1sig".into()),
                    ..Default::default()
                }
                .into(),
                Signal {
                    id: Some("f2sig".into()),
                    ..Default::default()
                }
                .into(),
                Signal {
                    id: Some("f1report".into()),
                    ..Default::default()
                }
                .into(),
                Signal {
                    id: Some("f2report".into()),
                    ..Default::default()
                }
                .into(),
            ],
            ..Default::default()
        };

        definitions
            .find_by_id_mut("proc1")
            .unwrap()
            .downcast_mut::<Process>()
            .unwrap()
            .establish_sequence_flow("start", "ev", "s1", None::<FormalExpression>)
            .unwrap()
            .establish_sequence_flow("ev", "f1", "f1s", None::<FormalExpression>)
            .unwrap()
            .establish_sequence_flow("ev", "f2", "f2s", None::<FormalExpression>)
            .unwrap()
            .establish_sequence_flow("f1", "f1t", "f1s1", None::<FormalExpression>)
            .unwrap()
            .establish_sequence_flow("f2", "f2t", "f2s1", None::<FormalExpression>)
            .unwrap()
            .establish_sequence_flow("f1t", "end", "e1", None::<FormalExpression>)
            .unwrap()
            .establish_sequence_flow("f2t", "end", "e2", None::<FormalExpression>)
            .unwrap();

        let model = model::Model::new(definitions).spawn().await;

        let handle = model.processes().await.unwrap().pop().unwrap();
        let mut mailbox = Mailbox::new(handle.event_receiver());
        let mut log_mailbox = Mailbox::new(handle.log_receiver());

        assert!(handle.start().await.is_ok());

        // wait for intermediate catch events to activate

        assert!(log_mailbox
            .receive(|e| matches!(e, Log::FlowNodeIncoming { node, .. } if node.id().as_ref().unwrap() == "f1"))
            .await);

        assert!(log_mailbox
            .receive(|e| matches!(e, Log::FlowNodeIncoming { node, .. } if node.id().as_ref().unwrap() == "f2"))
            .await);

        // trigger f2sig
        let _ = handle.event_broadcast().send(ProcessEvent::SignalEvent {
            signal_ref: Some("f2sig".into()),
        });

        // should throw f2report signal
        assert!(
            mailbox
                .receive(|e| matches!(e, ProcessEvent::SignalEvent { signal_ref } if signal_ref.as_ref().unwrap() == "f2report"))
                .await
        );

        // End event should be reached
        assert!(
            log_mailbox
                .receive(|e| if let Log::FlowNodeCompleted { node } = e {
                    matches!(node.downcast_ref::<EndEvent>(),
                Some(end_event) if end_event.id().as_ref().unwrap() == "end")
                } else {
                    false
                })
                .await
        );

        // trigger f1sig
        let _ = handle.event_broadcast().send(ProcessEvent::SignalEvent {
            signal_ref: Some("f1sig".into()),
        });

        // should not throw f1report signal because the path was already chosen
        assert!(
            timeout(Duration::from_millis(100),
            mailbox
                .receive(|e| matches!(e, ProcessEvent::SignalEvent { signal_ref } if signal_ref.as_ref().unwrap() == "f1report")))
            .await.is_err()
        );
    }
}
