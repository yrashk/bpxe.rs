//! # Flow Node
use crate::bpmn::schema::{
    DocumentElement, Element, EndEvent, FlowNodeType, IntermediateThrowEvent, ParallelGateway,
    SequenceFlow, StartEvent,
};
use crate::event::{end_event, intermediate_throw_event, start_event};
use crate::gateway;
use crate::process::{self};
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use thiserror::Error;

/// Flow node state
///
/// ## Notes
///
/// All flow nodes' state is combined into one "big" enum so that [`FlowNode`] doesn't need to have
/// any associated types (which makes the final type be sized differently, and this makes it
/// problematic for runtime dispatching.
#[derive(Serialize, Deserialize, Debug)]
pub enum State {
    StartEvent(start_event::State),
    EndEvent(end_event::State),
    ParallelGateway(gateway::parallel::State),
    IntermediateThrowEvent(intermediate_throw_event::State),
}

/// State handling errors
#[derive(Error, Debug)]
pub enum StateError {
    /// Invalid state variant. A different variant was expected.
    #[error("invalid state variant")]
    InvalidVariant,
}

pub type IncomingIndex = usize;
pub type OutgoingIndex = usize;

/// Determination of next action by flow nodes
#[derive(Debug)]
pub enum Action {
    /// Check whether given outputs will flow
    ///
    /// This is useful if the flow node needs to know whether certain outputs
    /// *will flow.
    ProbeOutgoingSequenceFlows(SmallVec<[OutgoingIndex; 8]>),
    /// Enact flow through given outputs
    ///
    /// This action will still check whether given outputs *can* flow.
    Flow(SmallVec<[OutgoingIndex; 8]>),
    /// Mark flow node as complete, no further action necessary.
    Complete,
}

/// Flow node
///
/// Flow node type should also implement [`futures::stream::Stream`] with `Item` set to [`Action`].
pub trait FlowNode: Stream<Item = Action> + Send + Unpin {
    /// Sets durable state
    ///
    /// If the state variant is incorect, [`StateError`] error should be returned.
    fn set_state(&mut self, state: State) -> Result<(), StateError>;

    /// Gets durable state
    fn get_state(&self) -> State;

    /// Sets process handle
    ///
    /// This allows flow nodes to access the process they are running in.
    ///
    /// Default implementation does nothing.
    #[allow(unused_variables)]
    fn set_process(&mut self, process: process::Handle) {}

    /// Reports outgoing sequence flow processing
    ///
    /// If condition was not present or it returned a truthful result, `condition_result`
    /// will be set to `true`, otherwise it will be zero.
    ///
    /// This allows flow nodes to make further decisions after each sequence flow processing.
    /// Flow node will be polled after this.
    ///
    /// Default implementation does nothing.
    #[allow(unused_variables)]
    fn sequence_flow(
        &mut self,
        output: OutgoingIndex,
        sequence_flow: &SequenceFlow,
        condition_result: bool,
    ) {
    }

    /// Reports incoming sequence flow
    ///
    /// Default implementation does nothing.
    #[allow(unused_variables)]
    fn incoming(&mut self, index: IncomingIndex) {}

    /// Returns a flow element
    fn element(&self) -> Box<dyn FlowNodeType>;
}

pub(crate) fn new(element: &dyn DocumentElement) -> Option<Box<dyn FlowNode>> {
    match element.element() {
        Element::StartEvent => make::<StartEvent, start_event::StartEvent>(element),
        Element::EndEvent => make::<EndEvent, end_event::EndEvent>(element),
        Element::ParallelGateway => make::<ParallelGateway, gateway::parallel::Gateway>(element),
        Element::IntermediateThrowEvent => make::<
            IntermediateThrowEvent,
            intermediate_throw_event::IntermediateThrowEvent,
        >(element),
        _ => None,
    }
}

fn make<E, F>(element: &dyn DocumentElement) -> Option<Box<dyn FlowNode>>
where
    E: DocumentElement + FlowNodeType + Clone + Default,
    F: 'static + From<E> + FlowNode,
{
    element
        .downcast_ref::<E>()
        .map(|e| Box::new(F::from(e.clone())) as Box<dyn FlowNode>)
}
