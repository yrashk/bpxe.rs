//! # BPMN Document
//!

pub mod schema {
    /// Alias for URI
    pub type URI = String;
    /// Alias for ID
    pub type Id = String;
    /// Alias for Integer
    pub type Integer = num_bigint::BigInt;
    /// Alias for Int
    pub type Int = i32;

    use downcast_rs::{impl_downcast, Downcast};

    pub(crate) trait DocumentElementContainer: Downcast {
        fn find_by_id(&self, id: &str) -> Option<&dyn DocumentElement>;
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
    }

    pub(crate) trait DocumentElement: DocumentElementContainer {
        fn element(&self) -> Element;
    }
    impl_downcast!(DocumentElement);

    include!("schema.rs");

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
}

pub use schema::*;

mod parser;
pub use parser::{parse, NormalizationError, ParseError};
