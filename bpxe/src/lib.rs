//! # BPXE
//! ## Business Process eXecution Engine
//!
//! [BPMN 2.0](http://omg.org/spec/BPMN/2.0) based business process execution engine. PMN stands
//! for **Business Process Model and Notation**. BPMN's goal is to help stakeholders to have a
//! shared understanding of processes.
//!
//! BPXE focuses on the execution aspect of such notation, effectively allowing the processes
//! described in BPMN to function as if they were programs. BPXE is not the only such engine, as
//! there are many commercially or community supported ones. The motivation behind the creation of
//! BPXE was to create an engine with a particular focus on type and memory safety, performance and
//! multi-tenancy capabilities (ensuring that a great deal of processes should be able to operate
//! even on a single server concurrently) and resistant to failures so that workflows can be
//! resumed with little to no consideration when a failure happen.
pub mod activity;
pub mod bpmn;
pub mod data_object;
pub mod event;
pub mod flow_node;
pub mod gateway;
pub mod language;
pub mod model;
pub mod process;

#[cfg(test)]
mod test;
