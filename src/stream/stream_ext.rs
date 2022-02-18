use std::time::{Duration, Instant};

use futures_core::Stream;

use super::{Delay, DelayUntil, Timeout, TimeoutAt};

/// Extend `Stream` with time-based operations.
pub trait StreamExt: Stream {
    /// Await a stream or times out after a duration of time.     
    fn timeout(self, dur: Duration) -> Timeout<Self>
    where
        Self: Sized,
    {
        Timeout::new(self, dur)
    }

    /// Await a stream or times out at the given deadline.
    fn timeout_at(self, deadline: Instant) -> TimeoutAt<Self>
    where
        Self: Sized,
    {
        TimeoutAt::new(self, deadline)
    }

    /// Returns a stream that delays execution for a specified time.
    fn delay(self, dur: Duration) -> Delay<Self>
    where
        Self: Sized,
    {
        Delay::new(self, dur)
    }

    /// Returns a stream that delays execution for a specified time.
    fn delay_until(self, deadline: Instant) -> DelayUntil<Self>
    where
        Self: Sized,
    {
        DelayUntil::new(self, deadline)
    }
}

impl<S> StreamExt for S where S: Stream {}
