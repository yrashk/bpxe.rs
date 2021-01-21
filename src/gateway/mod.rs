//! # Gateways
pub mod parallel;
pub use parallel::Gateway as Parallel;
pub mod exclusive;
pub use exclusive::Gateway as Exclusive;
