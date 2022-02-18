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

mod future_ext;
mod future_timeout;
mod interval;
mod sleep;
pub(crate) mod utils;

/// `std::stream` extensions.
pub mod stream {
    pub use crate::interval::{interval, Interval};
}

/// `std::task` extensions.
pub mod task {
    pub use super::sleep::{sleep, Sleep};
}

/// `std::future` extensions.
pub mod future {
    pub use super::future_timeout::Timeout;
    pub use crate::future_ext::FutureExt;
}

/// The `futures-time` prelude.
pub mod prelude {
    pub use crate::future_ext::FutureExt as _;
}
