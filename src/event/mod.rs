//! # Event subsystem
pub mod start_event;
pub use start_event::StartEvent;
pub mod end_event;
pub use end_event::EndEvent;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ProcessEvent {
    /// Process has started
    Start,
    /// Process has ended
    End,
}
