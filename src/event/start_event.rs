//! # Start Event flow node
use crate::bpmn::schema::StartEvent as Element;
use crate::flow_node::{self, Action, FlowNode};
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

/// Start Event flow node
pub struct StartEvent {
    element: Arc<Element>,
    state: State,
}

impl StartEvent {
    /// Creates new Start Event flow node
    pub fn new(element: Element) -> Self {
        Self {
            element: Arc::new(element),
            state: State::Initialized,
        }
    }

    /// Returns `startEvent` element
    pub fn element(&self) -> &Element {
        &self.element
    }
}

/// Node state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum State {
    /// Was just created
    Initialized,
    /// Was executed
    Fired,
}

impl FlowNode for StartEvent {
    fn set_state(&mut self, state: flow_node::State) -> Result<(), flow_node::StateError> {
        match state {
            flow_node::State::StartEvent(state) => {
                self.state = state;
                Ok(())
            }
            _ => Err(flow_node::StateError::InvalidVariant),
        }
    }

    fn get_state(&self) -> flow_node::State {
        flow_node::State::StartEvent(self.state.clone())
    }
}

impl From<Element> for StartEvent {
    fn from(element: Element) -> Self {
        Self::new(element)
    }
}

impl Stream for StartEvent {
    type Item = Action;
    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let State::Initialized = self.state {
            self.state = State::Fired
        }
        Poll::Ready(Some(Action::Flow))
    }
}

#[cfg(test)]
mod tests {
    use crate::bpmn::schema::*;
    use crate::model;
    use crate::process::{Log, Process as P};
    use crate::test::Mailbox;

    #[tokio::test]
    async fn start_flows() {
        let mut proc1: Process = Default::default();
        proc1.id = Some("proc1".into());
        let mut start_event: StartEvent = Default::default();
        start_event.id = Some("start".into());
        let mut end_event: EndEvent = Default::default();
        end_event.id = Some("end".into());
        let seq_flow = establish_sequence_flow(&mut start_event, &mut end_event, "s1").unwrap();
        proc1
            .flow_elements
            .push(FlowElement::StartEvent(start_event));
        proc1.flow_elements.push(FlowElement::EndEvent(end_event));
        proc1
            .flow_elements
            .push(FlowElement::SequenceFlow(seq_flow));

        let mut definitions: Definitions = Default::default();
        definitions
            .root_elements
            .push(RootElement::Process(proc1.clone()));
        let model = model::Model::new(definitions).spawn().await;

        let handle = P::new(proc1, model).spawn().await;
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

        assert!(
            mailbox
                .receive(|e| if let Log::FlowNodeCompleted { node } = e {
                    matches!(node.downcast_ref::<EndEvent>(),
                    Some(end_event) if end_event.id().as_ref().unwrap() == "end")
                } else {
                    false
                })
                .await
        );
    }
}
