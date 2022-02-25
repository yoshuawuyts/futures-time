use core::future::Future;
use std::time::{Duration, Instant};

use crate::task::Sleep;

use super::{Delay, Timeout};

/// Extend `Future` with time-based operations.
pub trait FutureExt: Future {
    /// Await a future or times out after a duration of time.     
    fn timeout(self, dur: Duration) -> Timeout<Self, Sleep>
    where
        Self: Sized,
    {
        let deadline = crate::task::sleep(dur);
        Timeout::new(self, deadline)
    }

    /// Returns a future that delays execution for a specified time.
    fn delay<D: Future>(self, deadline: Instant) -> Delay<Self, Sleep>
    where
        Self: Sized,
    {
        let deadline = crate::task::sleep_until(deadline);
        Delay::new(self, deadline)
    }
}

impl<T> FutureExt for T where T: Future {}
