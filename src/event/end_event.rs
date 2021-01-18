//! # End Event flow node
use crate::bpmn::schema::EndEvent as Element;
use crate::flow_node::{self, Action, FlowNode};
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

/// End Event flow node
pub struct EndEvent {
    element: Arc<Element>,
    state: State,
}

impl EndEvent {
    /// Creates new End Event flow node
    pub fn new(element: Element) -> Self {
        Self {
            element: Arc::new(element),
            state: State::Initialized,
        }
    }

    /// Returns `endEvent` element
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

impl FlowNode for EndEvent {
    fn set_state(&mut self, state: flow_node::State) -> Result<(), flow_node::StateError> {
        match state {
            flow_node::State::EndEvent(state) => {
                self.state = state;
                Ok(())
            }
            _ => Err(flow_node::StateError::InvalidVariant),
        }
    }

    fn get_state(&self) -> flow_node::State {
        flow_node::State::EndEvent(self.state.clone())
    }
}

impl From<Element> for EndEvent {
    fn from(element: Element) -> Self {
        Self::new(element)
    }
}

impl Stream for EndEvent {
    type Item = Action;
    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let State::Initialized = self.state {
            self.state = State::Fired
        }
        Poll::Ready(Some(Action::Done))
    }
}
