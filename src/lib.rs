//! async time combinators
//!
//! # Examples
//!
//! ```
//! // tbi
//! ```

#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
#![warn(missing_docs, future_incompatible, unreachable_pub)]

mod future_ext;
mod future_timeout;
mod interval;
mod sleep;

pub use future_ext::FutureExt;
pub use interval::{interval, Interval};
pub use sleep::{sleep, Sleep};

/// Future extensions.
pub mod future {
    pub use super::future_timeout::{timeout, Timeout, TimeoutError};
}
