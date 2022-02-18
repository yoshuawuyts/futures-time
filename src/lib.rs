//! async time combinators
//!
//! # Examples
//!
//! ```text
//! // tbi
//! ```

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
}
