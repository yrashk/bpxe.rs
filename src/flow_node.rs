//! # Flow Node
use crate::bpmn::schema::{
    DocumentElement, DocumentElementContainer, Element, EndEvent, FlowNodeType, SequenceFlow,
    SequenceFlowType as _, StartEvent,
};
use crate::event::{end_event, start_event};
use crate::process::{self, Log};
use futures::future::{BoxFuture, FutureExt};
use futures::stream::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::task;

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
}

/// State handling errors
#[derive(Error, Debug)]
pub enum StateError {
    /// Invalid state variant. A different variant was expected.
    #[error("invalid state variant")]
    InvalidVariant,
}

/// Determination of next action by flow nodes
#[derive(Debug)]
pub enum Action {
    /// Start or continue processing outgoing sequence flows
    Flow,
    /// Stop processing outgoing sequence flows
    Done,
}

impl Default for Action {
    fn default() -> Self {
        Self::Flow
    }
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
    fn sequence_flow(&mut self, sequence_flow: &SequenceFlow, condition_result: bool) {}
}

pub(crate) fn spawn(
    node: Box<dyn FlowNodeType>,
    mut flow_node: Box<dyn FlowNode>,
    process: process::Handle,
) -> BoxFuture<'static, Action> {
    flow_node.set_process(process.clone());
    let log_broadcast = process.log_broadcast();
    let definitions = process.model().definitions();
    let _ = log_broadcast.send(Log::FlowNodeSelected { node: node.clone() });
    async move {
        let result = flow_node.next().await;
        match result {
            Some(Action::Flow) => {
                let outgoings = node.outgoings();
                for outgoing in outgoings {
                    let process_clone = process.clone();
                    let sequence_flow = outgoing
                        .as_ref()
                        .and_then(|id| definitions.find_by_id(id))
                        .map(move |seq_flow| Box::new(&(*seq_flow)))
                        .and_then(move |seq_flow| seq_flow.downcast_ref::<SequenceFlow>());

                    match sequence_flow {
                        None => unimplemented!(),
                        Some(seq_flow) => {
                            flow_node.sequence_flow(&seq_flow, true);
                            let result = flow_node.next().await;
                            match result {
                                Some(Action::Done) | None => break,
                                Some(Action::Flow) => {
                                    definitions
                                        .find_by_id(seq_flow.target_ref())
                                        .and_then(new)
                                        .map(|(node, flow_element)| {
                                            task::spawn(async move {
                                                spawn(flow_element, node, process_clone).await
                                            })
                                        });
                                }
                            }
                        }
                    }
                }
            }
            Some(Action::Done) => {}
            None => {}
        }
        let _ = log_broadcast.send(Log::FlowNodeCompleted { node: node.clone() });
        Default::default()
    }
    .boxed()
}

pub(crate) fn new(
    element: &dyn DocumentElement,
) -> Option<(Box<dyn FlowNode>, Box<dyn FlowNodeType>)> {
    match element.element() {
        Element::StartEvent => make::<StartEvent, start_event::StartEvent>(element),
        Element::EndEvent => make::<EndEvent, end_event::EndEvent>(element),
        _ => None,
    }
}

pub(crate) fn make<E, F>(
    element: &dyn DocumentElement,
) -> Option<(Box<dyn FlowNode>, Box<dyn FlowNodeType>)>
where
    E: DocumentElement + FlowNodeType + Clone + Default,
    F: 'static + From<E> + FlowNode,
{
    element.downcast_ref::<E>().map(|e| {
        let mut node: E = Default::default();
        node.clone_from(e);
        let element: Box<dyn FlowNodeType> = Box::new(node);
        let node: Box<dyn FlowNode> = Box::new(F::from(e.clone()));
        (node, element)
    })
}
