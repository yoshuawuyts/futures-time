use core::future::Future;

use super::{Delay, IntoFuture, Timeout};

/// Extend `Future` with time-based operations.
pub trait FutureExt: Future {
    /// Await a future or times out after a duration of time.     
    fn timeout<D>(self, deadline: D) -> Timeout<Self, D::IntoFuture>
    where
        Self: Sized,
        D: IntoFuture,
    {
        Timeout::new(self, deadline.into_future())
    }

    /// Returns a future that delays execution for a specified time.
    fn delay<D>(self, deadline: D) -> Delay<Self, D::IntoFuture>
    where
        Self: Sized,
        D: IntoFuture,
    {
        Delay::new(self, deadline.into_future())
    }
}

impl<T> FutureExt for T where T: Future {}
