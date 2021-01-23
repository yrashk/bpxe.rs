//! # BPMN Document
//!

pub use bpxe_bpmn_schema as schema;

mod parser;
pub use parser::{parse, NormalizationError, ParseError};
