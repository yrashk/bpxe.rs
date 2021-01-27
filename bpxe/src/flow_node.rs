//! # Flow Node
use crate::activity;
use crate::bpmn::schema::{
    ActivityType, DocumentElement, Element, EndEvent, EventBasedGateway, ExclusiveGateway,
    FlowNodeType, InclusiveGateway, IntermediateCatchEvent, IntermediateThrowEvent,
    ParallelGateway, ScriptTask, SequenceFlow, StartEvent,
};
use crate::event::{end_event, intermediate_catch_event, intermediate_throw_event, start_event};
use crate::gateway;
use crate::process::{self};
use factory::ParameterizedFactory;
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::marker::PhantomData;

use thiserror::Error;

/// Flow node state
///
/// ## Notes
///
/// All flow nodes' state is combined into one "big" enum so that [`FlowNode`] doesn't need to have
/// any associated types (which makes the final type be sized differently, and this makes it
/// problematic for runtime dispatching.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum State {
    StartEvent(start_event::State),
    EndEvent(end_event::State),
    IntermediateThrowEvent(intermediate_throw_event::State),
    IntermediateCatchEvent(intermediate_catch_event::State),
    ParallelGateway(gateway::parallel::State),
    ExclusiveGateway(gateway::exclusive::State),
    InclusiveGateway(gateway::inclusive::State),
    EventBasedGateway(gateway::event_based::State),
    ScriptTask(activity::script_task::State),
    ActivityState(activity::State),
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

/// Hard-coded size limit for [`smallvec::SmallVec`]`<[IncomingIndex; _]>`
///
/// This is a default expectation for maintaining small arrays of incomings
/// that can grow into a heap allocation if it gets over it.
///
/// It's chosen as a somewhat arbitrary guess for what can constitute a "normal" flow count.
pub const SMALL_INCOMING: usize = 8;

/// Hard-coded size limit for [`smallvec::SmallVec`]`<[OutgoingIndex; _]>`
///
/// This is a default expectation for maintaining small arrays of outgoings
/// that can grow into a heap allocation if it gets over it.
///
/// It's chosen as a somewhat arbitrary guess for what can constitute a "normal" flow count.
pub const SMALL_OUTGOING: usize = 8;

/// Determination of next action by flow nodes
#[derive(Debug)]
pub enum Action {
    /// Check whether given outgoings will flow
    ///
    /// This is useful if the flow node needs to know whether certain outgoings
    /// *will flow.
    ProbeOutgoingSequenceFlows(SmallVec<[OutgoingIndex; SMALL_OUTGOING]>),
    /// Enact flow through given outgoings
    ///
    /// This action will still check whether given outgoings *can* flow.
    Flow(SmallVec<[OutgoingIndex; SMALL_OUTGOING]>),
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
    ///
    /// The reason why it's mutable is that in some cases some activites might want to change their
    /// data or simply get mutable access to it during state retrieval
    fn get_state(&mut self) -> State;

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
        outgoing: OutgoingIndex,
        sequence_flow: &SequenceFlow,
        condition_result: bool,
    ) {
    }

    /// Maps outgoing node's action to a new action (or inaction)
    ///
    /// This is useful for nodes with more complex processing (for example, event-based gateway)
    /// that need to handle the result action of the outgoing node.
    ///
    /// Returning `None` will mean that the action has to be dropped, returning `Some(action)` will
    /// replace the original action with the returned one in the flow.
    ///
    /// Default implementation does nothing (returns the same action)
    #[allow(unused_variables)]
    fn handle_outgoing_action(
        &mut self,
        index: OutgoingIndex,
        action: Option<Action>,
    ) -> Option<Option<Action>> {
        Some(action)
    }

    /// Reports incoming sequence flow
    ///
    /// Default implementation does nothing.
    #[allow(unused_variables)]
    fn incoming(&mut self, index: IncomingIndex) {}

    /// Reports token count at ingress.
    ///
    /// Useful for complex flow node behaviours where it needs to know how many outstanding tokens
    /// there are.
    ///
    /// Default implementation does nothing.
    #[allow(unused_variables)]
    fn tokens(&mut self, count: usize) {}

    /// Returns a flow element
    fn element(&self) -> Box<dyn FlowNodeType>;
}

pub(crate) fn new(element: Box<dyn DocumentElement>) -> Option<Box<dyn FlowNode>> {
    let e = element.element();
    match e {
        Element::StartEvent => make::<StartEvent, start_event::StartEvent>(element),
        Element::EndEvent => make::<EndEvent, end_event::EndEvent>(element),
        Element::IntermediateThrowEvent => make::<
            IntermediateThrowEvent,
            intermediate_throw_event::IntermediateThrowEvent,
        >(element),
        Element::IntermediateCatchEvent => make::<
            IntermediateCatchEvent,
            intermediate_catch_event::IntermediateCatchEvent,
        >(element),
        Element::ParallelGateway => make::<ParallelGateway, gateway::parallel::Gateway>(element),
        Element::ExclusiveGateway => make::<ExclusiveGateway, gateway::exclusive::Gateway>(element),
        Element::InclusiveGateway => make::<InclusiveGateway, gateway::inclusive::Gateway>(element),
        Element::EventBasedGateway => {
            make::<EventBasedGateway, gateway::event_based::Gateway>(element)
        }
        Element::ScriptTask => make_activity::<ScriptTask, activity::script_task::Task>(element),
        _ => None,
    }
}

fn make<E, F>(element: Box<dyn DocumentElement>) -> Option<Box<dyn FlowNode>>
where
    E: DocumentElement + FlowNodeType + Clone + Default,
    F: 'static + From<E> + FlowNode,
{
    element
        .downcast::<E>()
        .ok()
        .map(|e| Box::new(F::from((*e).clone())) as Box<dyn FlowNode>)
}

struct ActivityFactory<F, E>(PhantomData<(F, E)>)
where
    E: DocumentElement + ActivityType + Clone + Default + Unpin,
    F: 'static + From<E> + activity::Activity;

impl<F, E> Clone for ActivityFactory<F, E>
where
    E: DocumentElement + ActivityType + Clone + Default + Unpin,
    F: 'static + From<E> + activity::Activity,
{
    fn clone(&self) -> Self {
        ActivityFactory::<F, E>(PhantomData)
    }
}

impl<F, E> ParameterizedFactory for ActivityFactory<F, E>
where
    E: DocumentElement + ActivityType + Clone + Default + Unpin,
    F: 'static + From<E> + activity::Activity,
{
    type Item = F;
    type Parameter = E;
    fn create(&self, param: Self::Parameter) -> Self::Item {
        F::from(param)
    }
}

fn make_activity<E, F>(element: Box<dyn DocumentElement>) -> Option<Box<dyn FlowNode>>
where
    E: DocumentElement + ActivityType + Clone + Default + Unpin,
    F: 'static + From<E> + activity::Activity,
{
    if let Ok(e) = element.downcast::<E>() {
        Some(Box::new(activity::ActivityContainer::new(
            (*e).clone(),
            ActivityFactory::<F, E>(PhantomData),
        )) as Box<dyn FlowNode>)
    } else {
        None
    }
}
