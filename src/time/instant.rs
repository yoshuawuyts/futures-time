use crate::{future::IntoFuture, task::SleepUntil};

/// A measurement of a monotonically nondecreasing clock. Opaque and useful only
/// with Duration.
///
/// This type wraps `std::time::Duration` so we can implement traits on it
/// without coherence issues, just like if we were implementing this in the
/// stdlib.
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone, Copy)]
pub struct Instant(pub(crate) std::time::Instant);

impl std::ops::Deref for Instant {
    type Target = std::time::Instant;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Instant {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<std::time::Instant> for Instant {
    fn from(inner: std::time::Instant) -> Self {
        Self(inner)
    }
}

impl Into<std::time::Instant> for Instant {
    fn into(self) -> std::time::Instant {
        self.0
    }
}

impl IntoFuture for Instant {
    type Output = Instant;

    type Future = SleepUntil;

    fn into_future(self) -> Self::Future {
        crate::task::sleep_until(self)
    }
}
