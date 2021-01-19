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
        incoming_completed: SmallVec<[bool; 8]>,
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

    fn get_state(&self) -> flow_node::State {
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
    use crate::bpmn::schema::*;
    use crate::event::ProcessEvent;
    use crate::model;
    use crate::test::*;

    #[tokio::test]
    async fn fork() {
        let mut proc1: Process = Default::default();
        proc1.id = Some("proc1".into());

        let mut start_event: StartEvent = Default::default();
        start_event.id = Some("start".into());

        let mut fork: ParallelGateway = Default::default();
        fork.id = Some("fork".into());

        let mut f1: IntermediateThrowEvent = Default::default();
        f1.id = Some("f1".into());
        let mut f2: IntermediateThrowEvent = Default::default();
        f2.id = Some("f2".into());

        let mut end_event: EndEvent = Default::default();
        end_event.id = Some("end".into());

        let mut seq_flows = vec![];

        seq_flows.push(establish_sequence_flow(&mut start_event, &mut fork, "s1").unwrap());
        seq_flows.push(establish_sequence_flow(&mut fork, &mut f1, "f1s").unwrap());
        seq_flows.push(establish_sequence_flow(&mut fork, &mut f2, "f2s").unwrap());
        seq_flows.push(establish_sequence_flow(&mut f1, &mut end_event, "e1").unwrap());
        seq_flows.push(establish_sequence_flow(&mut f2, &mut end_event, "e2").unwrap());

        proc1
            .flow_elements
            .push(FlowElement::StartEvent(start_event));

        proc1.flow_elements.push(FlowElement::ParallelGateway(fork));

        proc1
            .flow_elements
            .push(FlowElement::IntermediateThrowEvent(f1));

        proc1
            .flow_elements
            .push(FlowElement::IntermediateThrowEvent(f2));

        proc1.flow_elements.push(FlowElement::EndEvent(end_event));

        proc1.flow_elements.append(
            &mut seq_flows
                .into_iter()
                .map(FlowElement::SequenceFlow)
                .collect(),
        );

        let mut definitions: Definitions = Default::default();
        definitions
            .root_elements
            .push(RootElement::Process(proc1.clone()));
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
        let mut proc1: Process = Default::default();
        proc1.id = Some("proc1".into());

        let mut start_event: StartEvent = Default::default();
        start_event.id = Some("start".into());

        let mut fork: ParallelGateway = Default::default();
        fork.id = Some("fork".into());

        let mut f1: IntermediateThrowEvent = Default::default();
        f1.id = Some("f1".into());
        let mut f2: IntermediateThrowEvent = Default::default();
        f2.id = Some("f2".into());

        let mut join: ParallelGateway = Default::default();
        join.id = Some("join".into());

        let mut f3: IntermediateThrowEvent = Default::default();
        f3.id = Some("f3".into());

        let mut end_event: EndEvent = Default::default();
        end_event.id = Some("end".into());

        let mut seq_flows = vec![];

        seq_flows.push(establish_sequence_flow(&mut start_event, &mut fork, "s1").unwrap());
        seq_flows.push(establish_sequence_flow(&mut fork, &mut f1, "f1s").unwrap());
        seq_flows.push(establish_sequence_flow(&mut fork, &mut f2, "f2s").unwrap());
        seq_flows.push(establish_sequence_flow(&mut f1, &mut join, "j1s").unwrap());
        seq_flows.push(establish_sequence_flow(&mut f2, &mut join, "j2s").unwrap());
        seq_flows.push(establish_sequence_flow(&mut join, &mut f3, "f3s").unwrap());
        seq_flows.push(establish_sequence_flow(&mut f3, &mut end_event, "e").unwrap());

        proc1
            .flow_elements
            .push(FlowElement::StartEvent(start_event));

        proc1.flow_elements.push(FlowElement::ParallelGateway(fork));
        proc1.flow_elements.push(FlowElement::ParallelGateway(join));

        proc1
            .flow_elements
            .push(FlowElement::IntermediateThrowEvent(f1));

        proc1
            .flow_elements
            .push(FlowElement::IntermediateThrowEvent(f2));

        proc1
            .flow_elements
            .push(FlowElement::IntermediateThrowEvent(f3));

        proc1.flow_elements.push(FlowElement::EndEvent(end_event));

        proc1.flow_elements.append(
            &mut seq_flows
                .into_iter()
                .map(FlowElement::SequenceFlow)
                .collect(),
        );

        let mut definitions: Definitions = Default::default();
        definitions
            .root_elements
            .push(RootElement::Process(proc1.clone()));
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
        let mut proc1: Process = Default::default();
        proc1.id = Some("proc1".into());

        let mut start_event: StartEvent = Default::default();
        start_event.id = Some("start".into());

        let mut fork: ParallelGateway = Default::default();
        fork.id = Some("fork".into());

        let mut f1: IntermediateThrowEvent = Default::default();
        f1.id = Some("f1".into());
        let mut f2: IntermediateThrowEvent = Default::default();
        f2.id = Some("f2".into());

        let mut join: ParallelGateway = Default::default();
        join.id = Some("join".into());

        let mut f3: IntermediateThrowEvent = Default::default();
        f3.id = Some("f3".into());

        let mut end_event: EndEvent = Default::default();
        end_event.id = Some("end".into());

        let mut alt_end_event: EndEvent = Default::default();
        alt_end_event.id = Some("alt_end".into());

        let mut seq_flows = vec![];

        seq_flows.push(establish_sequence_flow(&mut start_event, &mut fork, "s1").unwrap());
        seq_flows.push(establish_sequence_flow(&mut fork, &mut f1, "f1s").unwrap());
        seq_flows.push(establish_sequence_flow(&mut fork, &mut f2, "f2s").unwrap());
        seq_flows.push(establish_sequence_flow(&mut f1, &mut join, "j1s").unwrap());
        // EndEvent should not flow
        seq_flows.push(establish_sequence_flow(&mut f2, &mut alt_end_event, "j2e").unwrap());
        seq_flows.push(establish_sequence_flow(&mut alt_end_event, &mut join, "j2es").unwrap());
        seq_flows.push(establish_sequence_flow(&mut join, &mut f3, "f3s").unwrap());
        seq_flows.push(establish_sequence_flow(&mut f3, &mut end_event, "e").unwrap());

        proc1
            .flow_elements
            .push(FlowElement::StartEvent(start_event));

        proc1.flow_elements.push(FlowElement::ParallelGateway(fork));
        proc1.flow_elements.push(FlowElement::ParallelGateway(join));

        proc1
            .flow_elements
            .push(FlowElement::IntermediateThrowEvent(f1));

        proc1
            .flow_elements
            .push(FlowElement::IntermediateThrowEvent(f2));

        proc1
            .flow_elements
            .push(FlowElement::IntermediateThrowEvent(f3));

        proc1.flow_elements.push(FlowElement::EndEvent(end_event));
        proc1
            .flow_elements
            .push(FlowElement::EndEvent(alt_end_event));

        proc1.flow_elements.append(
            &mut seq_flows
                .into_iter()
                .map(FlowElement::SequenceFlow)
                .collect(),
        );

        let mut definitions: Definitions = Default::default();
        definitions
            .root_elements
            .push(RootElement::Process(proc1.clone()));
        let model = model::Model::new(definitions).spawn().await;

        let handle = model.processes().await.unwrap().pop().unwrap();
        let mut mailbox = Mailbox::new(handle.event_receiver());

        assert!(handle.start().await.is_ok());

        for _ in 0..2u8 {
            // NoneEvent should be thrown twice (forking)
            assert!(
                mailbox
                    .receive(|e| matches!(e, ProcessEvent::NoneEvent))
                    .await
            );
        }

        // f2 -> end
        assert!(mailbox.receive(|e| matches!(e, ProcessEvent::End)).await);

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
