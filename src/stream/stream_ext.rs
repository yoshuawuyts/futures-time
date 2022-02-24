use std::time::{Duration, Instant};

use futures_core::Stream;

use super::{Buffer, Debounce, Delay, DelayUntil, Throttle, Timeout, TimeoutAt};

/// Extend `Stream` with time-based operations.
pub trait StreamExt: Stream {
    /// Returns a stream which buffers items until it yields them at each
    /// `boundary`.
    fn buffer(self, boundary: Duration) -> Buffer<Self>
    where
        Self: Sized,
    {
        Buffer::new(self, boundary)
    }

    /// Returns a stream that debounces for the given boundary.
    fn debounce(self, boundary: Duration) -> Debounce<Self>
    where
        Self: Sized,
    {
        Debounce::new(self, boundary)
    }

    /// Returns a stream that delays execution for a specified duration.
    fn delay(self, boundary: Duration) -> Delay<Self>
    where
        Self: Sized,
    {
        Delay::new(self, boundary)
    }

    /// Returns a stream that delays execution until a specified deadline.
    fn delay_until(self, deadline: Instant) -> DelayUntil<Self>
    where
        Self: Sized,
    {
        DelayUntil::new(self, deadline)
    }

    /// Throtlle a stream.
    fn throttle(self, boundary: Duration) -> Throttle<Self>
    where
        Self: Sized,
    {
        Throttle::new(self, boundary)
    }

    /// Await a stream or times out after a duration of time.     
    fn timeout(self, boundary: Duration) -> Timeout<Self>
    where
        Self: Sized,
    {
        Timeout::new(self, boundary)
    }

    /// Await a stream or time out at the given deadline.
    fn timeout_at(self, deadline: Instant) -> TimeoutAt<Self>
    where
        Self: Sized,
    {
        TimeoutAt::new(self, deadline)
    }
}

impl<S> StreamExt for S where S: Stream {}
