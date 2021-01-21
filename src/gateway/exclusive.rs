//! # Exclusive Gateway
use crate::bpmn::schema::{ExclusiveGateway as Element, FlowNodeType, SequenceFlow};
use crate::flow_node::{self, Action, FlowNode, IncomingIndex, OutgoingIndex};
use crate::process;
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use smallvec::{smallvec, SmallVec};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};

/// Exclusive Gateway flow node
pub struct Gateway {
    element: Arc<Element>,
    state: State,
    waker: Option<Waker>,
    process: Option<process::Handle>,
}

impl Gateway {
    /// Creates new Exclusive Gateway flow node
    pub fn new(element: Element) -> Self {
        let element = Arc::new(element);
        Self {
            element,
            state: State::Ready,
            waker: None,
            process: None,
        }
    }
}

/// Node state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum State {
    Ready,
    Probing,
    AwaitingProbing {
        probed: SmallVec<[(OutgoingIndex, bool); 8]>,
    },
    Done {
        selected_outgoing: OutgoingIndex,
    },
}

impl FlowNode for Gateway {
    fn set_state(&mut self, state: flow_node::State) -> Result<(), flow_node::StateError> {
        match state {
            flow_node::State::ExclusiveGateway(state) => {
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
        flow_node::State::ExclusiveGateway(self.state.clone())
    }

    fn element(&self) -> Box<dyn FlowNodeType> {
        Box::new(self.element.as_ref().clone())
    }

    fn incoming(&mut self, _index: IncomingIndex) {
        if let State::Ready = self.state {
            self.state = State::Probing;
            if let Some(waker) = self.waker.take() {
                waker.wake();
            }
        }
    }

    fn sequence_flow(
        &mut self,
        output: OutgoingIndex,
        sequence_flow: &SequenceFlow,
        condition_result: bool,
    ) {
        if let State::AwaitingProbing { ref mut probed } = self.state {
            probed.push((output, condition_result));
            // If this sequence flow has an expression (not a default one)
            if sequence_flow.condition_expression.is_some() {
                // and it has resolved to `true`, proceed with it
                if condition_result {
                    self.state = State::Done {
                        selected_outgoing: output,
                    };

                    if let Some(waker) = self.waker.take() {
                        waker.wake();
                    }

                    return;
                }
            }
            // if we've probed everything
            if probed.len() == self.element.outgoings().len() {
                // and if there was something successful
                if let Some(output) =
                    probed.iter().find_map(
                        |(result_index, result)| if *result { Some(result_index) } else { None },
                    )
                {
                    self.state = State::Done {
                        selected_outgoing: *output,
                    };

                    if let Some(waker) = self.waker.take() {
                        waker.wake();
                    }
                    return;
                }
                // if we've probed everything and nothing worked (there's no default)
                // ..according to the specification:
                // "If a default path is not specified and the Process is executed such that none of the conditional Expressions
                // evaluates to true, a runtime exception occurs"
                let exception = process::Log::NoDefaultPath {
                    node: Box::new(self.element.as_ref().clone()),
                };
                let _ = self
                    .process
                    .clone()
                    .ok_or_else(|| tokio::sync::broadcast::error::SendError(exception.clone()))
                    .and_then(|process| process.log_broadcast().send(exception));
            }
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
            State::Ready | State::AwaitingProbing { .. } => {
                self.waker.replace(cx.waker().clone());
                Poll::Pending
            }
            State::Probing => {
                self.state = State::AwaitingProbing {
                    probed: smallvec![],
                };
                Poll::Ready(Some(Action::ProbeOutgoingSequenceFlows(
                    (0..self.element().outgoings().len()).collect(),
                )))
            }
            State::Done {
                selected_outgoing: index,
            } => {
                self.state = State::Ready;
                Poll::Ready(Some(Action::Flow(smallvec![index])))
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
    #[cfg(any(feature = "rhai"))]
    async fn choose_one() {
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
                        ExclusiveGateway {
                            id: Some("excl".into()),
                            ..Default::default()
                        }
                        .into(),
                        IntermediateThrowEvent {
                            id: Some("f1".into()),
                            event_definitions: vec![SignalEventDefinition {
                                signal_ref: Some("f1sig".into()),
                                ..Default::default()
                            }
                            .into()],
                            ..Default::default()
                        }
                        .into(),
                        IntermediateThrowEvent {
                            id: Some("f2".into()),
                            event_definitions: vec![SignalEventDefinition {
                                signal_ref: Some("f2sig".into()),
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
            ],
            ..Default::default()
        };

        definitions
            .find_by_id_mut("proc1")
            .unwrap()
            .downcast_mut::<Process>()
            .unwrap()
            .establish_sequence_flow("start", "excl", "s1", None)
            .unwrap()
            .establish_sequence_flow(
                "excl",
                "f1",
                "f1s",
                Some(Expression {
                    #[cfg(feature = "rhai")]
                    content: Some("false".into()),
                    xsi_type: Some("tFormalExpression".into()),
                    ..Default::default()
                }),
            )
            .unwrap()
            .establish_sequence_flow(
                "excl",
                "f2",
                "f2s",
                Some(Expression {
                    #[cfg(feature = "rhai")]
                    content: Some("true".into()),
                    xsi_type: Some("tFormalExpression".into()),
                    ..Default::default()
                }),
            )
            .unwrap()
            .establish_sequence_flow("f1", "end", "e1", None)
            .unwrap()
            .establish_sequence_flow("f2", "end", "e2", None)
            .unwrap();

        let model = model::Model::new(definitions).spawn().await;

        let handle = model.processes().await.unwrap().pop().unwrap();
        let mut mailbox = Mailbox::new(handle.event_receiver());
        let mut log_mailbox = Mailbox::new(handle.log_receiver());

        assert!(handle.start().await.is_ok());

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

        // f2's signal should be thrown
        assert!(
            mailbox
            .receive(|e| matches!(e, ProcessEvent::SignalEvent { signal_ref } if signal_ref.as_ref().unwrap() == "f2sig"))
            .await
        );

        // but f1's should not
        assert!(timeout(
                Duration::from_millis(100),
                mailbox.receive(
                    |e| matches!(e, ProcessEvent::SignalEvent { signal_ref } if signal_ref.as_ref().unwrap() == "f1sig")
                )
        )
            .await
            .is_err());
    }

    #[tokio::test]
    #[cfg(any(feature = "rhai"))]
    async fn default_if_cant_choose_condition() {
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
                        ExclusiveGateway {
                            id: Some("excl".into()),
                            ..Default::default()
                        }
                        .into(),
                        IntermediateThrowEvent {
                            id: Some("f0".into()),
                            event_definitions: vec![SignalEventDefinition {
                                signal_ref: Some("f0sig".into()),
                                ..Default::default()
                            }
                            .into()],
                            ..Default::default()
                        }
                        .into(),
                        IntermediateThrowEvent {
                            id: Some("f1".into()),
                            event_definitions: vec![SignalEventDefinition {
                                signal_ref: Some("f1sig".into()),
                                ..Default::default()
                            }
                            .into()],
                            ..Default::default()
                        }
                        .into(),
                        IntermediateThrowEvent {
                            id: Some("f2".into()),
                            event_definitions: vec![SignalEventDefinition {
                                signal_ref: Some("f2sig".into()),
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
                    id: Some("f0sig".into()),
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
            ],
            ..Default::default()
        };

        definitions
            .find_by_id_mut("proc1")
            .unwrap()
            .downcast_mut::<Process>()
            .unwrap()
            .establish_sequence_flow("start", "excl", "s1", None)
            .unwrap()
            .establish_sequence_flow("excl", "f0", "f0s", None)
            .unwrap()
            .establish_sequence_flow(
                "excl",
                "f1",
                "f1s",
                Some(Expression {
                    #[cfg(feature = "rhai")]
                    content: Some("false".into()),
                    xsi_type: Some("tFormalExpression".into()),
                    ..Default::default()
                }),
            )
            .unwrap()
            .establish_sequence_flow(
                "excl",
                "f2",
                "f2s",
                Some(Expression {
                    #[cfg(feature = "rhai")]
                    content: Some("flow".into()),
                    xsi_type: Some("tFormalExpression".into()),
                    ..Default::default()
                }),
            )
            .unwrap()
            .establish_sequence_flow("f0", "end", "e0", None)
            .unwrap()
            .establish_sequence_flow("f1", "end", "e1", None)
            .unwrap()
            .establish_sequence_flow("f2", "end", "e2", None)
            .unwrap();

        let model = model::Model::new(definitions).spawn().await;

        let handle = model.processes().await.unwrap().pop().unwrap();
        let mut mailbox = Mailbox::new(handle.event_receiver());
        let mut log_mailbox = Mailbox::new(handle.log_receiver());

        assert!(handle.start().await.is_ok());

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

        // f0's signal should be thrown
        assert!(
            mailbox
            .receive(|e| matches!(e, ProcessEvent::SignalEvent { signal_ref } if signal_ref.as_ref().unwrap() == "f0sig"))
            .await
        );

        // but f1's or f2's should not
        assert!(timeout(
                Duration::from_millis(100),
                mailbox.receive(
                    |e| matches!(e, ProcessEvent::SignalEvent { signal_ref } if signal_ref.as_ref().unwrap() == "f1sig" || signal_ref.as_ref().unwrap() == "f2sig")
                )
        )
            .await
            .is_err());
    }

    #[tokio::test]
    #[cfg(any(feature = "rhai"))]
    async fn no_default_path() {
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
                        ExclusiveGateway {
                            id: Some("excl".into()),
                            ..Default::default()
                        }
                        .into(),
                        IntermediateThrowEvent {
                            id: Some("f1".into()),
                            event_definitions: vec![SignalEventDefinition {
                                signal_ref: Some("f1sig".into()),
                                ..Default::default()
                            }
                            .into()],
                            ..Default::default()
                        }
                        .into(),
                        IntermediateThrowEvent {
                            id: Some("f2".into()),
                            event_definitions: vec![SignalEventDefinition {
                                signal_ref: Some("f2sig".into()),
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
            ],
            ..Default::default()
        };

        definitions
            .find_by_id_mut("proc1")
            .unwrap()
            .downcast_mut::<Process>()
            .unwrap()
            .establish_sequence_flow("start", "excl", "s1", None)
            .unwrap()
            .establish_sequence_flow(
                "excl",
                "f1",
                "f1s",
                Some(Expression {
                    #[cfg(feature = "rhai")]
                    content: Some("false".into()),
                    xsi_type: Some("tFormalExpression".into()),
                    ..Default::default()
                }),
            )
            .unwrap()
            .establish_sequence_flow(
                "excl",
                "f2",
                "f2s",
                Some(Expression {
                    #[cfg(feature = "rhai")]
                    content: Some("flow".into()),
                    xsi_type: Some("tFormalExpression".into()),
                    ..Default::default()
                }),
            )
            .unwrap()
            .establish_sequence_flow("f1", "end", "e1", None)
            .unwrap()
            .establish_sequence_flow("f2", "end", "e2", None)
            .unwrap();

        let model = model::Model::new(definitions).spawn().await;

        let handle = model.processes().await.unwrap().pop().unwrap();
        let mut log_mailbox = Mailbox::new(handle.log_receiver());

        assert!(handle.start().await.is_ok());

        // we should see an exception
        assert!(
            log_mailbox
                .receive(|e| matches!(e, Log::NoDefaultPath { .. }))
                .await
        );
    }
}
