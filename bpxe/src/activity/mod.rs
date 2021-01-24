//! Activity
use crate::bpmn::schema::{ActivityType, FlowNodeType, SequenceFlow};
use crate::flow_node::{Action, FlowNode, IncomingIndex, OutgoingIndex, State, StateError};
use crate::process;
use futures::stream::{Stream, StreamExt};
use std::pin::Pin;
use std::task::{Context, Poll};

pub mod script_task;

pub trait Activity: FlowNode {}

pub struct ActivityContainer<T>
where
    T: Activity,
{
    flow_node: T,
}

impl<T> FlowNode for ActivityContainer<T>
where
    T: Activity,
{
    fn set_state(&mut self, state: State) -> Result<(), StateError> {
        self.flow_node.set_state(state)
    }

    fn get_state(&self) -> State {
        self.flow_node.get_state()
    }

    fn set_process(&mut self, process: process::Handle) {
        self.flow_node.set_process(process)
    }

    fn element(&self) -> Box<dyn FlowNodeType> {
        self.flow_node.element()
    }

    fn sequence_flow(
        &mut self,
        outgoing: OutgoingIndex,
        sequence_flow: &SequenceFlow,
        condition_result: bool,
    ) {
        self.flow_node
            .sequence_flow(outgoing, sequence_flow, condition_result)
    }

    fn handle_outgoing_action(
        &mut self,
        index: OutgoingIndex,
        action: Option<Action>,
    ) -> Option<Option<Action>> {
        self.flow_node.handle_outgoing_action(index, action)
    }

    fn incoming(&mut self, index: IncomingIndex) {
        self.flow_node.incoming(index)
    }

    fn tokens(&mut self, count: usize) {
        self.flow_node.tokens(count)
    }
}

impl<T> Stream for ActivityContainer<T>
where
    T: Activity,
{
    type Item = Action;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.flow_node.poll_next_unpin(cx)
    }
}

impl<T> ActivityContainer<T>
where
    T: Activity,
{
    pub fn new(flow_node: T) -> Self {
        use intertrait::cast::CastRef;
        let _element = flow_node.element().cast::<dyn ActivityType>();
        Self { flow_node }
    }
}
