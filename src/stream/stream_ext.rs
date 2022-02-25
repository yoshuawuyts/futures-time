use std::time::Duration;

use futures_core::Stream;

use crate::task::{sleep, Sleep};

use super::{interval, Buffer, Debounce, Delay, Interval, Throttle, Timeout};

/// Extend `Stream` with time-based operations.
pub trait StreamExt: Stream {
    /// Returns a stream which buffers items until it yields them at each
    /// `duration`.
    fn buffer(self, duration: Duration) -> Buffer<Self, Interval>
    where
        Self: Sized,
    {
        let interval = interval(duration);
        Buffer::new(self, interval)
    }

    /// Returns a stream that debounces for the given duration.
    fn debounce(self, duration: Duration) -> Debounce<Self, Sleep>
    where
        Self: Sized,
    {
        let deadline = sleep(duration);
        Debounce::new(self, deadline)
    }

    /// Returns a stream that delays execution for a specified duration.
    fn delay(self, duration: Duration) -> Delay<Self>
    where
        Self: Sized,
    {
        Delay::new(self, duration)
    }

    /// Throtlle a stream.
    fn throttle(self, duration: Duration) -> Throttle<Self, Interval>
    where
        Self: Sized,
    {
        let interval = interval(duration);
        Throttle::new(self, interval)
    }

    /// Await a stream or times out after a duration of time.     
    fn timeout(self, duration: Duration) -> Timeout<Self, Sleep>
    where
        Self: Sized,
    {
        let deadline = sleep(duration);
        Timeout::new(self, deadline)
    }
}

impl<S> StreamExt for S where S: Stream {}
