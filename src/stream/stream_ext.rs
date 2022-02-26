use crate::future::{IntoFuture, ResetDeadlineFuture};

use futures_core::Stream;

use super::{Buffer, Debounce, Delay, IntoStream, Throttle, Timeout};

/// Extend `Stream` with time-based operations.
pub trait StreamExt: Stream {
    /// Returns a stream which buffers items and flushes them at each interval.
    fn buffer<I>(self, interval: I) -> Buffer<Self, I::IntoStream>
    where
        Self: Sized,
        I: IntoStream,
    {
        Buffer::new(self, interval.into_stream())
    }

    /// Returns a stream that debounces for the given duration.
    fn debounce<D>(self, deadline: D) -> Debounce<Self, D::IntoFuture>
    where
        Self: Sized,
        D: IntoFuture,
        D::IntoFuture: ResetDeadlineFuture,
    {
        Debounce::new(self, deadline.into_future())
    }

    // /// Returns a stream that delays execution for a specified duration.
    // fn delay<D>(self, deadline: D) -> Delay<Self, D::IntoFuture>
    // where
    //     Self: Sized,
    //     D: IntoFuture,
    // {
    //     Delay::new(self, deadline.into())
    // }

    /// Throtlle a stream.
    fn throttle<I>(self, interval: I) -> Throttle<Self, I::IntoStream>
    where
        Self: Sized,
        I: IntoStream,
    {
        Throttle::new(self, interval.into_stream())
    }

    /// Await a stream or times out after a duration of time.     
    fn timeout<D>(self, deadline: D) -> Timeout<Self, D::IntoFuture>
    where
        Self: Sized,
        D: IntoFuture,
        D::IntoFuture: ResetDeadlineFuture,
    {
        Timeout::new(self, deadline.into_future())
    }
}

impl<S> StreamExt for S where S: Stream {}
