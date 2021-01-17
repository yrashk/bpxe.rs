//! # BPMN Document
//!

pub mod schema;
pub use schema::*;

mod parser;
pub use parser::{parse, NormalizationError, ParseError};
