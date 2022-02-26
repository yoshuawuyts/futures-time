use crate::{future::IntoFuture, task::Sleep};

use super::Instant;

/// A Duration type to represent a span of time, typically used for system
/// timeouts.
///
/// This type wraps `std::time::Duration` so we can implement traits on it
/// without coherence issues, just like if we were implementing this in the
/// stdlib.
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone, Copy)]
pub struct Duration(pub(crate) std::time::Duration);
impl Duration {
    /// Creates a new `Duration` from the specified number of whole seconds and
    /// additional nanoseconds.
    pub fn new(secs: u64, nanos: u32) -> Duration {
        std::time::Duration::new(secs, nanos).into()
    }

    /// Creates a new `Duration` from the specified number of whole seconds.
    pub fn from_secs(secs: u64) -> Duration {
        std::time::Duration::from_secs(secs).into()
    }

    /// Creates a new `Duration` from the specified number of milliseconds.
    pub fn from_millis(millis: u64) -> Self {
        std::time::Duration::from_millis(millis).into()
    }
}

impl std::ops::Deref for Duration {
    type Target = std::time::Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Duration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<std::time::Duration> for Duration {
    fn from(inner: std::time::Duration) -> Self {
        Self(inner)
    }
}

impl Into<std::time::Duration> for Duration {
    fn into(self) -> std::time::Duration {
        self.0
    }
}

impl IntoFuture for Duration {
    type Output = Instant;

    type Future = Sleep;

    fn into_future(self) -> Self::Future {
        crate::task::sleep(self)
    }
}
