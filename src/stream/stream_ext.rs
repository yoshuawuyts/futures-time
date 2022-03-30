use crate::future::{Deadline, IntoFuture};

use futures_core::Stream;

use super::{Buffer, Debounce, Delay, IntoStream, Sample, Throttle, Timeout};

/// Extend `Stream` with time-based operations.
pub trait StreamExt: Stream {
    /// Yield the last value received, if any, at each interval.
    fn sample<I>(self, interval: I) -> Sample<Self, I::IntoStream>
    where
        Self: Sized,
        I: IntoStream,
    {
        Sample::new(self, interval.into_stream())
    }

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
        D::IntoFuture: Deadline,
    {
        Debounce::new(self, deadline.into_future())
    }

    /// Returns a stream that delays execution for a specified duration.
    ///
    /// # Example
    ///
    /// ```
    /// use futures_lite::prelude::*;
    /// use futures_time::prelude::*;
    /// use futures_time::time::{Instant, Duration};
    /// use futures_lite::stream;
    ///
    /// fn main() {
    ///     async_io::block_on(async {
    ///         let now = Instant::now();
    ///         let delay = Duration::from_millis(100);
    ///         let _ = stream::once("meow").delay(delay).next().await;
    ///         assert!(now.elapsed() >= *delay);
    ///     });
    /// }
    /// ```
    fn delay<D>(self, deadline: D) -> Delay<Self, D::IntoFuture>
    where
        Self: Sized,
        D: IntoFuture,
    {
        Delay::new(self, deadline.into_future())
    }

    /// Throtlle a stream.
    fn throttle<I>(self, interval: I) -> Throttle<Self, I::IntoStream>
    where
        Self: Sized,
        I: IntoStream,
    {
        Throttle::new(self, interval.into_stream())
    }

    /// Await a stream or times out after a duration of time.     
    ///
    /// # Example
    ///
    /// ```
    /// use futures_lite::prelude::*;
    /// use futures_time::prelude::*;
    /// use futures_time::time::{Instant, Duration};
    /// use futures_lite::stream;
    /// use std::io;
    ///
    /// fn main() {
    ///     async_io::block_on(async {
    ///         let res = stream::once("meow")
    ///             .delay(Duration::from_millis(100))  // longer delay
    ///             .timeout(Duration::from_millis(50)) // shorter timeout
    ///             .next()
    ///             .await;
    ///         assert_eq!(res.unwrap().unwrap_err().kind(), io::ErrorKind::TimedOut); // error
    ///
    ///         let res = stream::once("meow")
    ///             .delay(Duration::from_millis(50))    // shorter delay
    ///             .timeout(Duration::from_millis(100)) // longer timeout
    ///             .next()
    ///             .await;
    ///         assert_eq!(res.unwrap().unwrap(), "meow"); // success
    ///     });
    /// }
    /// ```
    fn timeout<D>(self, deadline: D) -> Timeout<Self, D::IntoFuture>
    where
        Self: Sized,
        D: IntoFuture,
        D::IntoFuture: Deadline,
    {
        Timeout::new(self, deadline.into_future())
    }
}

impl<S> StreamExt for S where S: Stream {}
