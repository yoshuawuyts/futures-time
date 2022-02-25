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
//! - [`task::sleep`] Sleeps for the specified amount of time.
//! - [`Future::delay`](`future::FutureExt::delay`) Delay execution for a specified time.
//! - [`Future::delay_until`](`future::FutureExt::delay_until`) Delay execution until the specified instant.
//! - [`Future::timeout`](`future::FutureExt::timeout`) Cancel the future if the execution takes longer than the specified time.
//! - [`Future::timeout_at`](`future::FutureExt::timeout_at`) Cancel the future if the execution goes beyond the specified instant.
//!
//! # Streams
//!
//! - [`stream::interval`](`stream::interval`) Creates a new stream that yields at a set interval.
//! - `Stream::audit`
//! - `Stream::buffer`
//! - [`Stream::debounce`](`stream::StreamExt::debounce`)
//! - [`Stream::delay`](`stream::StreamExt::delay`) Delay execution for a specified time.
//! - [`Stream::delay_until`](`stream::StreamExt::delay_until`) Delay execution until the specified instant.
//! - `Stream::sample`
//! - [`Stream::throttle`](`stream::StreamExt::throttle`) Filter out all items after the first for a specified time.
//! - [`Stream::timeout`](`stream::StreamExt::timeout`) Cancel the stream if the execution takes longer than the specified time.
//! - [`Stream::timeout_at`](`stream::StreamExt::timeout_at`) Cancel the stream if the execution goes beyond the specified instant.

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

/// Reset the state of a future.
pub trait ResetFuture {
    /// Reset a future's state.
    fn reset(self: std::pin::Pin<&mut Self>);
}
