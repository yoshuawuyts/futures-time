//! # Async time operators.
//!
//! This crate provides ergonomic, async time-based operations. It serves as an
//! experimental playground to experiment with how we could potentially add
//! time-based operations to `async-std`, and subsequently the stdlib.
//!
//! The goal is to make working with time and other events feel natural. A major
//! source of inspiration for this has been RxJS, which uses events (including
//! time) to trigger operations. This crate takes that principle, inverts the
//! model to make it evaluate lazily, and wraps it in an ergnomic Rust
//! interface.
//!
//! # Examples
//!
//! __Delay a future's execution by 100ms__
//!
//! ```
//! use futures_time::prelude::*;
//! use futures_time::time::Duration;
//!
//! fn main() {
//!     async_io::block_on(async {
//!         let res = async { "meow" }
//!             .delay(Duration::from_millis(100))
//!             .await;
//!         assert_eq!(res, "meow");
//!     })
//! }
//! ```
//!
//! __Error if a future takes longer than 200ms__
//!
//! ```
//! use futures_time::prelude::*;
//! use futures_time::time::Duration;
//!
//! fn main() {
//!     async_io::block_on(async {
//!         let res = async { "meow" }
//!             .delay(Duration::from_millis(100))
//!             .timeout(Duration::from_millis(200))
//!             .await;
//!         assert_eq!(res.unwrap(), "meow");
//!     })
//! }
//! ```
//!
//! __Throttle a stream__
//!
//! This lets two items through in total: one `100ms` after the program has
//! started, and one `300ms` after the program has started.
//!
//! ```
//! use futures_lite::prelude::*;
//! use futures_time::prelude::*;
//! use futures_time::time::Duration;
//! use futures_time::stream;
//!
//! fn main() {
//!     async_io::block_on(async {
//!         let mut counter = 0;
//!         stream::interval(Duration::from_millis(100))  // Yield an item every 100ms
//!             .take(4)                                  // Stop after 4 items
//!             .throttle(Duration::from_millis(300))     // Only let an item through every 300ms
//!             .for_each(|_| counter += 1)               // Increment a counter for each item
//!             .await;
//!
//!         assert_eq!(counter, 2);
//!     })
//! }
//! ```
//!
//! # The `Deadline` trait
//!
//! The future returned by [`task::sleep`] implements the [`future::Deadline`]
//! trait. This represents a future whose deadline can be moved forward into the
//! future.
//!
//! For example, say we have a deadline of `Duration::from_secs(10)`. By calling
//! `Daedline::push_deadline` the deadline can be moved into the future relative
//! to now. This functionality is required for methods such as `debounce` and
//! `Stream::timeout`, which will regularly want to move their deadlines into
//! the future.
//!
//! Currently the only type implementing the `Deadline` trait is
//! [`task::Sleep`], which is created from a `Duration.` This is in contrast
//! with [`task::sleep_until`], which takes an `Instant`, and cannot be reset
//! relative to the present time.
//!
//! # Cancellation
//!
//! You can use [`channel::bounded`] to create a [`channel::Sender`] and [`channel::Receiver`] pair.
//! When the "sender" sends a message, all "receivers" will halt execution of the future the next time they are
//! `.await`ed. This will cause the future to stop executing, and all
//! destructors to be run.
//!
//! ```
//! use futures_time::prelude::*;
//! use futures_time::channel;
//! use futures_time::time::Duration;
//!
//! fn main() {
//!     async_io::block_on(async {
//!         let (send, recv) = channel::bounded(1); // create a new send/receive pair
//!         let mut counter = 0;
//!         let value = async { "meow" }
//!             .delay(Duration::from_millis(100))
//!             .timeout(recv) // time-out when the sender emits a message
//!             .await;
//!
//!         assert_eq!(value.unwrap(), "meow");
//!     })
//! }
//! ```
//!
//! # Futures
//!
//! - [`Future::delay`](`future::FutureExt::delay`) Delay execution for a specified time.
//! - [`Future::timeout`](`future::FutureExt::timeout`) Cancel the future if the execution takes longer than the specified time.
//! - [`future::parker`] Suspend a future for a specified amount of time.
//!
//! # Tasks
//!
//! - [`task::sleep_until`] Sleeps until the specified deadline.
//! - [`task::sleep`] Sleeps for the specified amount of time.
//!
//! # Streams
//!
//! - [`Stream::buffer`](`stream::StreamExt::buffer`) Returns a stream which buffers items and flushes them at each interval.
//! - [`Stream::debounce`](`stream::StreamExt::debounce`) Returns a stream that debounces for the given duration.
//! - [`Stream::delay`](`stream::StreamExt::delay`) Delay execution for a specified time.
//! - [`Stream::sample`](`stream::StreamExt::sample`) Yield the last value received, if any, at each interval.
//! - [`Stream::throttle`](`stream::StreamExt::throttle`) Filter out all items after the first for a specified time.
//! - [`Stream::timeout`](`stream::StreamExt::timeout`) Cancel the stream if the execution takes longer than the specified time.
//! - [`stream::interval`](`stream::interval`) Creates a new stream that yields at a set interval.
//!
//! # Re-exports
//!
//! - `channel` is a re-export of the [`async-channel`] crate, exposed for convenience
//!
//! [`async-channel`]: https://docs.rs/async-channel/latest/async_channel

#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
#![warn(missing_docs, future_incompatible, unreachable_pub)]

pub(crate) mod utils;

pub mod time;

pub mod stream;

pub mod task;

pub mod future;

#[doc(inline)]
pub use async_channel as channel;

/// The `futures-time` prelude.
pub mod prelude {
    pub use super::future::Deadline as _;
    pub use super::future::FutureExt as _;
    pub use super::future::IntoFuture as _;
    pub use super::stream::IntoStream as _;
    pub use super::stream::StreamExt as _;
}
