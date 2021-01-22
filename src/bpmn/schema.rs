//! # BPMN Schema
//!
use thiserror::Error;

/// Alias for URI
pub type URI = String;
/// Alias for ID
pub type Id = String;
/// Alias for Integer
pub type Integer = num_bigint::BigInt;
/// Alias for Int
pub type Int = i32;

use downcast_rs::{impl_downcast, Downcast};
use intertrait::*;

pub trait DocumentElementContainer: Downcast + CastFrom {
    /// Find an element by ID
    #[allow(unused_variables)]
    fn find_by_id(&self, _id: &str) -> Option<&dyn DocumentElement> {
        None
    }

    /// Find an element by ID and return a mutable reference
    #[allow(unused_variables)]
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        None
    }
}
impl_downcast!(DocumentElementContainer);

impl<T> DocumentElementContainer for Vec<T>
where
    T: DocumentElementContainer,
{
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        for e in self.iter() {
            if let Some(de) = e.find_by_id(id) {
                return Some(de);
            }
        }
        None
    }
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        for e in self.iter_mut() {
            if let Some(de) = e.find_by_id_mut(id) {
                return Some(de);
            }
        }
        None
    }
}

impl<T> DocumentElementContainer for Option<T>
where
    T: DocumentElementContainer,
{
    fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement> {
        match self {
            None => None,
            Some(e) => e.find_by_id(id),
        }
    }
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut dyn DocumentElement> {
        match self {
            None => None,
            Some(e) => e.find_by_id_mut(id),
        }
    }
}

pub trait DocumentElement: DocumentElementContainer + Send {
    fn element(&self) -> Element;
}

impl_downcast!(DocumentElement);

pub trait DocumentElementWithContent: DocumentElement {
    /// Gets document element's content
    fn content(&self) -> &Option<String>;
}

pub trait DocumentElementWithContentMut: DocumentElement {
    /// Gets a mutable reference to document element's content
    fn content_mut(&mut self) -> &mut Option<String>;
    /// Changes content
    fn set_content(&mut self, content: Option<String>);
}

mod autogenerated;
pub use autogenerated::*;

mod expr;
pub use expr::*;

#[derive(Error, Debug)]
pub enum EstablishSequenceFlowError {
    #[error("source.id must be Some")]
    NoSourceId,
    #[error("target.id must be Some")]
    NoTargetId,
    #[error("can't find source of the right type")]
    SourceNotFound,
    #[error("can't find target of the right type")]
    TargetNotFound,
}

impl Process {
    /// Establishes sequence flow between flow identified nodes
    ///
    /// Resulting sequence flow will have `id` as an ID and it will be
    /// added to the matching process.
    // TODO: add support for sub-processes
    pub fn establish_sequence_flow<E: Into<Expr>>(
        &mut self,
        source: &str,
        target: &str,
        id: &str,
        condition_expression: Option<E>,
    ) -> Result<&mut Self, EstablishSequenceFlowError> {
        use intertrait::cast::CastMut;
        // The main reason why this method is written in a somewhat convoluted fashion (not saving
        // source and target nodes when we check for their presence) has to do
        // with the need to avoid multiple mutable borrows.

        // check source element presence
        self.find_by_id_mut(source)
            .and_then(|e| e.cast::<dyn FlowNodeTypeMut>())
            .ok_or(EstablishSequenceFlowError::SourceNotFound)?;

        // check target element presence
        self.find_by_id_mut(target)
            .and_then(|e| e.cast::<dyn FlowNodeTypeMut>())
            .ok_or(EstablishSequenceFlowError::TargetNotFound)?;

        let sequence_flow = SequenceFlow {
            id: Some(id.to_string()),
            source_ref: source.to_string(),
            target_ref: target.to_string(),
            condition_expression: condition_expression.map(|e| e.into()),
            ..SequenceFlow::default()
        };

        // add sequence flow
        self.flow_elements_mut()
            .push(FlowElement::SequenceFlow(sequence_flow));

        // add outgoing
        let source_node = self
            .find_by_id_mut(source)
            .and_then(|e| e.cast::<dyn FlowNodeTypeMut>())
            .unwrap();

        source_node.outgoings_mut().push(id.into());

        // add incoming
        let target_node = self
            .find_by_id_mut(target)
            .and_then(|e| e.cast::<dyn FlowNodeTypeMut>())
            .unwrap();
        target_node.incomings_mut().push(id.into());

        Ok(self)
    }
}

#[cfg(test)]
#[test]
fn establishing_sequence_flow_in_process() {
    use intertrait::cast::CastRef;
    let mut process = Process {
        id: Some("proc1".to_string()),
        flow_elements: vec![
            FlowElement::StartEvent(StartEvent {
                id: Some("start".to_string()),
                ..Default::default()
            }),
            FlowElement::EndEvent(EndEvent {
                id: Some("end".to_string()),
                ..Default::default()
            }),
        ],
        ..Default::default()
    };

    process
        .establish_sequence_flow(
            "start",
            "end",
            "test",
            Some(FormalExpression {
                content: Some("condition".into()),
                ..Default::default()
            }),
        )
        .unwrap();

    let seq_flow = process
        .find_by_id("test")
        .unwrap()
        .downcast_ref::<SequenceFlow>()
        .unwrap();
    assert_eq!(seq_flow.id(), &Some("test".to_string()));
    assert_eq!(seq_flow.source_ref(), "start");
    assert_eq!(seq_flow.target_ref(), "end");

    let expr = seq_flow.condition_expression().as_ref().unwrap();
    assert!(
        matches!(expr, Expr::FormalExpression(FormalExpression { content, ..}) if content.as_ref().unwrap() == "condition")
    );

    let start = process
        .find_by_id("start")
        .unwrap()
        .cast::<dyn FlowNodeType>()
        .unwrap();
    assert_eq!(start.outgoings(), &vec!["test".to_string()]);

    let end = process
        .find_by_id("end")
        .unwrap()
        .cast::<dyn FlowNodeType>()
        .unwrap();
    assert_eq!(end.incomings(), &vec!["test".to_string()]);
}

#[cfg(test)]
#[test]
fn failing_to_establish_sequence_flow_in_process() {
    let mut process = Process {
        id: Some("proc1".to_string()),
        flow_elements: vec![
            FlowElement::StartEvent(StartEvent {
                id: Some("start".to_string()),
                ..Default::default()
            }),
            FlowElement::EndEvent(EndEvent {
                id: Some("end".to_string()),
                ..Default::default()
            }),
        ],
        ..Default::default()
    };

    assert!(matches!(
        process
            .establish_sequence_flow("no_start", "end", "test", None::<FormalExpression>)
            .unwrap_err(),
        EstablishSequenceFlowError::SourceNotFound
    ));

    assert!(matches!(
        process
            .establish_sequence_flow("start", "no_end", "test", None::<FormalExpression>)
            .unwrap_err(),
        EstablishSequenceFlowError::TargetNotFound
    ));
}

/// Establishes and returns a sequence flow between nodes
pub fn establish_sequence_flow<F, T, S, E>(
    source: &mut F,
    target: &mut T,
    id: S,
    condition_expression: Option<E>,
) -> Result<SequenceFlow, EstablishSequenceFlowError>
where
    F: FlowNodeTypeMut,
    T: FlowNodeTypeMut,
    S: Into<String>,
    E: Into<Expr>,
{
    let mut sequence_flow = SequenceFlow::default();
    let id_s: Option<String> = Some(id.into());
    sequence_flow.id = id_s.clone();

    if source.id().is_none() {
        return Err(EstablishSequenceFlowError::NoSourceId);
    }

    if target.id().is_none() {
        return Err(EstablishSequenceFlowError::NoTargetId);
    }

    // it is safe now to unwrap ids
    sequence_flow.set_source_ref(source.id().as_ref().unwrap().into());
    sequence_flow.set_target_ref(target.id().as_ref().unwrap().into());
    sequence_flow.set_condition_expression(condition_expression.map(|e| e.into()));

    let id = id_s.unwrap();
    source.outgoings_mut().push(id.clone());
    target.incomings_mut().push(id);

    Ok(sequence_flow)
}

#[cfg(test)]
#[test]
fn establishing_sequence_flow() {
    let mut start = StartEvent::default();
    start.id = Some("start".into());
    let mut end = EndEvent::default();
    end.id = Some("end".into());
    let seq_flow = establish_sequence_flow(
        &mut start,
        &mut end,
        "test",
        Some(FormalExpression {
            content: Some("condition".into()),
            ..Default::default()
        }),
    )
    .unwrap();
    assert_eq!(seq_flow.id(), &Some("test".to_string()));
    assert_eq!(seq_flow.source_ref(), "start");
    assert_eq!(seq_flow.target_ref(), "end");
    assert_eq!(start.outgoings(), &vec!["test".to_string()]);
    assert_eq!(end.incomings(), &vec!["test".to_string()]);
    let expr = seq_flow.condition_expression().as_ref().unwrap();
    assert!(
        matches!(expr, Expr::FormalExpression(FormalExpression { content, ..}) if content.as_ref().unwrap() == "condition")
    );
}

#[cfg(test)]
#[test]
fn failing_to_establish_sequence_flow() {
    let mut start = StartEvent::default();
    let mut end = EndEvent::default();
    assert!(
        establish_sequence_flow(&mut start, &mut end, "test", None::<FormalExpression>).is_err()
    );
    start.id = Some("start".into());
    assert!(
        establish_sequence_flow(&mut start, &mut end, "test", None::<FormalExpression>).is_err()
    );
    end.id = Some("end".into());
    assert!(
        establish_sequence_flow(&mut start, &mut end, "test", None::<FormalExpression>).is_ok()
    );
}

#[cfg(test)]
#[test]
fn find_by_id() {
    let mut proc: Process = Default::default();
    proc.id = Some("proc".into());
    let mut start_event: StartEvent = Default::default();
    start_event.id = Some("start".into());

    proc.flow_elements
        .push(FlowElement::StartEvent(start_event.clone()));

    let mut definitions: Definitions = Default::default();
    definitions
        .root_elements
        .push(RootElement::Process(proc.clone()));
    let proc_ = definitions
        .find_by_id("proc")
        .expect("`proc` should have been found");
    assert_eq!(proc_.element(), Element::Process);
    assert_eq!(proc_.downcast_ref::<Process>().unwrap(), &proc);

    let start_event_ = definitions
        .find_by_id("start")
        .expect("`start` should have been found");
    assert_eq!(start_event_.element(), Element::StartEvent);
    assert_eq!(
        start_event_.downcast_ref::<StartEvent>().unwrap(),
        &start_event
    );

    assert!(definitions.find_by_id("not_to_be_found").is_none());
}

#[cfg(test)]
#[test]
fn find_by_id_mut() {
    let mut proc: Process = Default::default();
    proc.id = Some("proc".into());
    let mut start_event: StartEvent = Default::default();
    start_event.id = Some("start".into());

    proc.flow_elements
        .push(FlowElement::StartEvent(start_event.clone()));

    let mut definitions: Definitions = Default::default();
    definitions
        .root_elements
        .push(RootElement::Process(proc.clone()));
    let proc_ = definitions
        .find_by_id_mut("proc")
        .expect("`proc` should have been found");
    assert_eq!(proc_.element(), Element::Process);
    assert_eq!(proc_.downcast_ref::<Process>().unwrap(), &proc);

    let start_event_ = definitions
        .find_by_id_mut("start")
        .expect("`start` should have been found");
    assert_eq!(start_event_.element(), Element::StartEvent);
    assert_eq!(
        start_event_.downcast_ref::<StartEvent>().unwrap(),
        &start_event
    );

    assert!(definitions.find_by_id_mut("not_to_be_found").is_none());
}
