//! # Inclusive Gateway
use crate::bpmn::schema::{FlowNodeType, InclusiveGateway as Element, SequenceFlow};
use crate::flow_node::{self, Action, FlowNode, IncomingIndex, OutgoingIndex};
use crate::process;
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use smallvec::{smallvec, SmallVec};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};

/// Inclusive Gateway flow node
pub struct Gateway {
    element: Arc<Element>,
    state: State,
    waker: Option<Waker>,
    process: Option<process::Handle>,
}

impl Gateway {
    /// Creates new Inclusive Gateway flow node
    pub fn new(element: Element) -> Self {
        let element = Arc::new(element);
        Self {
            element,
            state: State {
                tokens: None,
                incoming: 0,
                case: StateCase::Ready,
            },
            waker: None,
            process: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    tokens: Option<usize>,
    incoming: usize,
    case: StateCase,
}

type ProbingResult = (OutgoingIndex, bool, bool);

/// Node state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StateCase {
    Ready,
    Probing,
    AwaitingProbing {
        probed: SmallVec<[ProbingResult; flow_node::SMALL_OUTGOING]>,
    },
    Done {
        selected_outgoing: SmallVec<[OutgoingIndex; flow_node::SMALL_OUTGOING]>,
    },
}

impl FlowNode for Gateway {
    fn set_state(&mut self, state: flow_node::State) -> Result<(), flow_node::StateError> {
        match state {
            flow_node::State::InclusiveGateway(state) => {
                self.state = state;
                Ok(())
            }
            _ => Err(flow_node::StateError::InvalidVariant),
        }
    }

    fn set_process(&mut self, process: process::Handle) {
        self.process = Some(process);
    }

    fn get_state(&mut self) -> flow_node::State {
        flow_node::State::InclusiveGateway(self.state.clone())
    }

    fn element(&self) -> Box<dyn FlowNodeType> {
        Box::new(self.element.as_ref().clone())
    }

    fn incoming(&mut self, _index: IncomingIndex) {
        self.state.incoming += 1;
        if let StateCase::Ready = self.state.case {
            self.state.case = StateCase::Probing;
        }
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }

    fn sequence_flow(
        &mut self,
        output: OutgoingIndex,
        sequence_flow: &SequenceFlow,
        condition_result: bool,
    ) {
        if let StateCase::AwaitingProbing { ref mut probed } = self.state.case {
            probed.push((
                output,
                sequence_flow.id == self.element.default,
                condition_result,
            ));
            // if we've probed everything
            if probed.len() == self.element.outgoings().len() {
                let successful: SmallVec<[ProbingResult; flow_node::SMALL_OUTGOING]> =
                    std::mem::replace(probed, smallvec![])
                        .into_iter()
                        .filter(|(_, _, result)| *result)
                        .collect();
                // and if there were successful flows
                if !successful.is_empty() {
                    let successful_non_default: SmallVec<
                        [&ProbingResult; flow_node::SMALL_OUTGOING],
                    > = successful
                        .iter()
                        .filter(|(_, default, _)| !(*default as bool))
                        .collect();
                    // and there were some that were not default
                    if !successful_non_default.is_empty() {
                        // then flow
                        self.state.case = StateCase::Done {
                            selected_outgoing: successful_non_default
                                .into_iter()
                                .map(|(index, _, _)| *index)
                                .collect(),
                        };

                        if let Some(waker) = self.waker.take() {
                            waker.wake();
                        }
                        return;
                    }
                    // otherwise, if there was a default path
                    let default_path =
                        successful.iter().find_map(
                            |(index, default, _)| {
                                if *default {
                                    Some(*index)
                                } else {
                                    None
                                }
                            },
                        );
                    if let Some(index) = default_path {
                        // then flow
                        self.state.case = StateCase::Done {
                            selected_outgoing: smallvec![index],
                        };

                        if let Some(waker) = self.waker.take() {
                            waker.wake();
                        }
                        return;
                    }
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

    fn tokens(&mut self, count: usize) {
        self.state.tokens.replace(count);
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
        match self.state.case {
            StateCase::Ready | StateCase::AwaitingProbing { .. } => {
                self.waker.replace(cx.waker().clone());
                Poll::Pending
            }
            StateCase::Probing => {
                self.state.case = StateCase::AwaitingProbing {
                    probed: smallvec![],
                };
                Poll::Ready(Some(Action::ProbeOutgoingSequenceFlows(
                    (0..self.element().outgoings().len()).collect(),
                )))
            }
            StateCase::Done {
                ref selected_outgoing,
            } => {
                if let Some(tokens) = self.state.tokens.as_ref().take() {
                    if *tokens >= self.state.incoming {
                        // if enough tokens, flow
                        let result = Poll::Ready(Some(Action::Flow(selected_outgoing.clone())));
                        self.state.case = StateCase::Ready;
                        return result;
                    }
                }
                // if no token count, or not enough tokens, keep waiting
                self.waker.replace(cx.waker().clone());
                self.state.case = StateCase::Ready;
                Poll::Pending
            }
        }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::bpmn::parse;
    use crate::bpmn::schema::*;
    use crate::event::ProcessEvent;
    use crate::model;
    use crate::process::Log;
    use crate::test::*;
    use std::time::Duration;
    use tokio::time::timeout;

    #[tokio::test]
    #[cfg(any(feature = "rhai"))]
    async fn fork() {
        let definitions = parse(include_str!("test_models/inclusive_fork.bpmn")).unwrap();
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

        // f1's signal should be thrown
        assert!(
            mailbox
            .receive(|e| matches!(e, ProcessEvent::SignalEvent { signal_ref } if signal_ref.as_ref().unwrap() == "f1sig"))
            .await
        );

        // f2's signal should be thrown
        assert!(
            mailbox
            .receive(|e| matches!(e, ProcessEvent::SignalEvent { signal_ref } if signal_ref.as_ref().unwrap() == "f2sig"))
            .await
        );

        // but f3's should not
        assert!(timeout(
                Duration::from_millis(100),
                mailbox.receive(
                    |e| matches!(e, ProcessEvent::SignalEvent { signal_ref } if signal_ref.as_ref().unwrap() == "f3sig")
                )
        )
            .await
            .is_err());
    }

    #[tokio::test]
    #[cfg(any(feature = "rhai"))]
    async fn default() {
        let definitions = parse(include_str!("test_models/inclusive_default.bpmn")).unwrap();
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
        let definitions =
            parse(include_str!("test_models/inclusive_no_default_path.bpmn")).unwrap();
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

    #[tokio::test]
    #[cfg(any(feature = "rhai"))]
    async fn join() {
        let definitions = parse(include_str!("test_models/inclusive_join.bpmn")).unwrap();
        let model = model::Model::new(definitions).spawn().await;

        let handle = model.processes().await.unwrap().pop().unwrap();
        let mut mailbox = Mailbox::new(handle.event_receiver());
        let mut log_mailbox = Mailbox::new(handle.log_receiver());

        assert!(handle.start().await.is_ok());

        // End event should be reached
        // Even though one condition is successful and the other is not,
        // inclusive gateway should join on successful ones only
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

        // but f1's should not (because it was false)
        assert!(timeout(
                Duration::from_millis(100),
                mailbox.receive(
                    |e| matches!(e, ProcessEvent::SignalEvent { signal_ref } if signal_ref.as_ref().unwrap() == "f1sig")
                )
        )
            .await
            .is_err());
    }
}
