//! Async time combinators
//!
//! # Deadlines and Boundaries
//!
//! While time is the most common type to use as an event source, other sources
//! of events also exist. In order to enable this, all functions take a generic
//! of either `Deadline` or `Boundary`.
//!
//! - `Deadline` represents a specific one-off event. Types which implement
//!   `Deadline` are `std::time::Instant` and `Future`.
//! - `Boundary` represents a range. Types which implement `Boundary` are `Duration`,
//!   and `Stream`.
//!
//! # Futures
//! - [`task::sleep`]
//! - [`Future::delay`](`future::FutureExt::delay`)
//! - [`Future::delay_until`](`future::FutureExt::delay_until`)
//! - [`Future::timeout`](`future::FutureExt::timeout`)
//! - [`Future::timeout_at`](`future::FutureExt::timeout_at`)
//!
//! # Streams
//!
//! - `stream::interval`
//! - `Stream::audit`
//! - `Stream::buffer`
//! - [`Stream::debounce`](`stream::StreamExt::debounce`)
//! - [`Stream::delay`](`stream::StreamExt::delay`)
//! - [`Stream::delay_until`](`stream::StreamExt::delay_until`)
//! - `Stream::sample`
//! - [`Stream::throttle`](`stream::StreamExt::throttle`)
//! - [`Stream::timeout`](`stream::StreamExt::timeout`)
//! - [`Stream::timeout_at`](`stream::StreamExt::timeout_at`)

#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
#![warn(missing_docs, future_incompatible, unreachable_pub)]

pub(crate) mod utils;

/// `std::stream` extensions.
pub mod stream;

/// `std::task` extensions.
pub mod task;

/// `std::future` extensions.
pub mod future;

/// The `futures-time` prelude.
pub mod prelude {
    pub use super::future::FutureExt as _;
    pub use super::stream::StreamExt as _;
}
