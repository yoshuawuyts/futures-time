use std::time::Duration;

use futures_core::Stream;

use super::{interval, Buffer, Debounce, Delay, Interval, Throttle, Timeout};

/// Extend `Stream` with time-based operations.
pub trait StreamExt: Stream {
    /// Returns a stream which buffers items until it yields them at each
    /// `boundary`.
    fn buffer(self, duration: Duration) -> Buffer<Self, Interval>
    where
        Self: Sized,
    {
        let interval = interval(duration);
        Buffer::new(self, interval)
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
}

impl<S> StreamExt for S where S: Stream {}
