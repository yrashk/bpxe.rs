//! # Parallel Gateway
use crate::bpmn::schema::{FlowNodeType, ParallelGateway as Element};
use crate::flow_node::{self, Action, FlowNode, IncomingIndex};
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use smallvec::{smallvec, SmallVec};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};

/// Parallel Gateway flow node
pub struct Gateway {
    element: Arc<Element>,
    state: State,
    waker: Option<Waker>,
}

impl Gateway {
    /// Creates new Parallel Gateway flow node
    pub fn new(element: Element) -> Self {
        let element = Arc::new(element);
        let number_of_incomings = element.incomings.len();
        Self {
            element,
            state: State::Ready {
                incoming_completed: smallvec![false; number_of_incomings],
            },
            waker: None,
        }
    }
}

/// Node state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum State {
    Ready {
        incoming_completed: SmallVec<[bool; flow_node::SMALL_INCOMING]>,
    },
    Complete,
}

impl FlowNode for Gateway {
    fn set_state(&mut self, state: flow_node::State) -> Result<(), flow_node::StateError> {
        match state {
            flow_node::State::ParallelGateway(state) => {
                self.state = state;
                Ok(())
            }
            _ => Err(flow_node::StateError::InvalidVariant),
        }
    }

    fn get_state(&mut self) -> flow_node::State {
        flow_node::State::ParallelGateway(self.state.clone())
    }

    fn element(&self) -> Box<dyn FlowNodeType> {
        Box::new(self.element.as_ref().clone())
    }

    fn incoming(&mut self, index: IncomingIndex) {
        if let State::Ready {
            ref mut incoming_completed,
        } = self.state
        {
            incoming_completed[index] = true;
            if let Some(waker) = self.waker.take() {
                waker.wake();
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
            State::Ready {
                ref incoming_completed,
            } => {
                if incoming_completed.iter().all(|c| *c) {
                    self.state = State::Complete;
                    Poll::Ready(Some(Action::Flow(
                        (0..self.element().outgoings().len()).collect(),
                    )))
                } else {
                    self.waker.replace(cx.waker().clone());
                    Poll::Pending
                }
            }
            State::Complete => {
                self.state = State::Ready {
                    incoming_completed: smallvec![false; self.element().incomings().len()],
                };
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
    use crate::process::Log;
    use crate::test::*;

    #[tokio::test]
    async fn fork() {
        let definitions = parse(include_str!("test_models/parallel_fork.bpmn")).unwrap();
        let model = model::Model::new(definitions).spawn().await;

        let handle = model.processes().await.unwrap().pop().unwrap();
        let mut mailbox = Mailbox::new(handle.event_receiver());

        assert!(handle.start().await.is_ok());

        for _ in 0..2u8 {
            // NoneEvent should be thrown twice
            assert!(
                mailbox
                    .receive(|e| matches!(e, ProcessEvent::NoneEvent))
                    .await
            );
        }
    }

    #[tokio::test]
    async fn join() {
        let definitions = parse(include_str!("test_models/parallel_join.bpmn")).unwrap();
        let model = model::Model::new(definitions).spawn().await;

        let handle = model.processes().await.unwrap().pop().unwrap();
        let mut mailbox = Mailbox::new(handle.event_receiver());

        assert!(handle.start().await.is_ok());

        for _ in 0..3u8 {
            // NoneEvent should be thrown three times
            assert!(
                mailbox
                    .receive(|e| matches!(e, ProcessEvent::NoneEvent))
                    .await
            );
        }
    }

    #[tokio::test]
    async fn not_enough_to_join() {
        let definitions =
            parse(include_str!("test_models/parallel_not_enough_to_join.bpmn")).unwrap();
        let model = model::Model::new(definitions).spawn().await;

        let handle = model.processes().await.unwrap().pop().unwrap();
        let mut mailbox = Mailbox::new(handle.event_receiver());
        let mut log_mailbox = Mailbox::new(handle.log_receiver());

        assert!(handle.start().await.is_ok());

        for _ in 0..2u8 {
            // NoneEvent should be thrown twice (forking)
            assert!(
                mailbox
                    .receive(|e| matches!(e, ProcessEvent::NoneEvent))
                    .await
            );
        }

        // f2 flows to a dead_end that will not join
        assert!(
            log_mailbox
                .receive(|log| matches!(log, Log::FlowNodeIncoming { node, .. }
                        if node.id().as_ref().unwrap() == "dead_end"))
                .await
        );

        // but third NoneEvent should not happen as the join didn't occur
        use std::time::Duration;
        use tokio::time::timeout;
        assert!(timeout(
            Duration::from_millis(100),
            mailbox.receive(|e| matches!(e, ProcessEvent::NoneEvent))
        )
        .await
        .is_err());
    }
}
