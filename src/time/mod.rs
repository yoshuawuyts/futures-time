//! async-aware `std::time` replacement.

mod duration;
mod instant;

pub use duration::Duration;
pub use instant::Instant;
