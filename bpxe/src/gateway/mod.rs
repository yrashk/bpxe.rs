//! # Gateways
pub mod parallel;
pub use parallel::Gateway as Parallel;
pub mod exclusive;
pub use exclusive::Gateway as Exclusive;
pub mod inclusive;
pub use inclusive::Gateway as Inclusive;
pub mod event_based;
pub use event_based::Gateway as EventBased;
