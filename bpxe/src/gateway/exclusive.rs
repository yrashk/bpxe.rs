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
        probed: SmallVec<[(OutgoingIndex, bool); flow_node::SMALL_OUTGOING]>,
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
        outgoing: OutgoingIndex,
        sequence_flow: &SequenceFlow,
        condition_result: bool,
    ) {
        if let State::AwaitingProbing { ref mut probed } = self.state {
            probed.push((outgoing, condition_result));
            // If sequence flow has resolved to `true`, proceed with it
            // (as long as it is not a default path)
            if condition_result && self.element.default == sequence_flow.id {
                self.state = State::Done {
                    selected_outgoing: outgoing,
                };

                if let Some(waker) = self.waker.take() {
                    waker.wake();
                }

                return;
            }
            // if we've probed everything
            if probed.len() == self.element.outgoings().len() {
                // and if there was something successful then this means it was a default path
                if let Some(outgoing) =
                    probed.iter().find_map(
                        |(result_index, result)| if *result { Some(result_index) } else { None },
                    )
                {
                    // therefore, we can proceed with it
                    self.state = State::Done {
                        selected_outgoing: *outgoing,
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
    async fn choose_one() {
        let definitions = parse(include_str!("test_models/exclusive_choose_one.bpmn")).unwrap();
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
    async fn default() {
        let definitions = parse(include_str!("test_models/exclusive_default.bpmn")).unwrap();
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
            parse(include_str!("test_models/exclusive_no_default_path.bpmn")).unwrap();
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
