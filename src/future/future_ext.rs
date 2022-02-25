use core::future::Future;
use std::time::{Duration, Instant};

use crate::task::SleepUntil;

use super::{Delay, DelayUntil, Timeout, TimeoutAt};

/// Extend `Future` with time-based operations.
pub trait FutureExt: Future {
    /// Await a future or times out after a duration of time.     
    fn timeout(self, dur: Duration) -> Timeout<Self>
    where
        Self: Sized,
    {
        Timeout::new(self, dur)
    }

    /// Await a future or times out at the given deadline.
    fn timeout_at(self, deadline: Instant) -> TimeoutAt<Self>
    where
        Self: Sized,
    {
        TimeoutAt::new(self, deadline)
    }

    /// Returns a future that delays execution for a specified time.
    fn delay(self, dur: Duration) -> Delay<Self>
    where
        Self: Sized,
    {
        Delay::new(self, dur)
    }

    /// Returns a future that delays execution for a specified time.
    fn delay_until<D: Future>(self, deadline: Instant) -> DelayUntil<Self, SleepUntil>
    where
        D: Future,
        Self: Sized,
    {
        let deadline = crate::task::sleep_until(deadline);
        DelayUntil::new(self, deadline)
    }
}

impl<T> FutureExt for T where T: Future {}
