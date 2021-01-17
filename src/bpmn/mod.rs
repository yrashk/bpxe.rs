//! # BPMN Document
//!

pub mod schema;

mod parser;
pub use parser::{parse, NormalizationError, ParseError};
