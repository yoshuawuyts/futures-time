use crate::future::{Deadline, IntoFuture};

use futures_core::Stream;

use super::{Buffer, Debounce, Delay, IntoStream, Sample, Throttle, Timeout};

/// Extend `Stream` with time-based operations.
pub trait StreamExt: Stream {
    /// Yield the last item received at the end of each interval.
    ///
    /// If no items have been received during an interval, the stream will not
    /// yield any items. In addition to using a time-based interval, this method can take any
    /// stream as a source. This enables throttling based on alternative event
    /// sources, such as variable-rate timers.
    ///
    /// This is the logical inverse of [`throttle`], which yields the _first_ item
    /// received during each interval.
    ///
    /// [`throttle`]: StreamExt::throttle
    ///
    /// # Data Loss
    ///
    /// This method will discard data between intervals. Though the
    /// discarded items will have their destuctors run, __using this method
    /// incorrectly may lead to unintended data loss__. This method is best used
    /// to reduce the number of _duplicate_ items after the first has been
    /// received, such as repeated mouse clicks or key presses. This method may
    /// lead to unintended data loss when used to discard _unique_ items, such
    /// as network request.
    ///
    /// # Example
    ///
    /// ```
    /// use futures_lite::prelude::*;
    /// use futures_time::prelude::*;
    /// use futures_time::time::{Instant, Duration};
    /// use futures_time::stream;
    ///
    /// fn main() {
    ///    async_io::block_on(async {
    ///        let interval = Duration::from_millis(100);
    ///        let throttle = Duration::from_millis(200);
    ///
    ///        let take = 4;
    ///        let expected = 2;
    ///
    ///        let mut counter = 0;
    ///        stream::interval(interval)
    ///            .take(take)
    ///            .sample(throttle)
    ///            .for_each(|_| counter += 1)
    ///            .await;
    ///
    ///        assert_eq!(counter, expected);
    ///    })
    /// }
    /// ```
    fn sample<I>(self, interval: I) -> Sample<Self, I::IntoStream>
    where
        Self: Sized,
        I: IntoStream,
    {
        Sample::new(self, interval.into_stream())
    }

    /// Returns a stream which buffers items and flushes them at each interval.
    ///
    /// # Example
    ///
    /// ```
    /// use futures_lite::prelude::*;
    /// use futures_time::prelude::*;
    /// use futures_time::time::{Instant, Duration};
    /// use futures_time::stream;
    ///
    /// fn main() {
    ///     async_io::block_on(async {
    ///         let interval = Duration::from_millis(5);
    ///         let buffer = Duration::from_millis(20);
    ///
    ///         let mut counter = 0;
    ///         stream::interval(interval)
    ///             .take(10)
    ///             .buffer(buffer)
    ///             .for_each(|buf| counter += buf.len())
    ///             .await;
    ///
    ///         assert_eq!(counter, 10);
    ///     })
    /// }
    /// ```
    fn buffer<I>(self, interval: I) -> Buffer<Self, I::IntoStream>
    where
        Self: Sized,
        I: IntoStream,
    {
        Buffer::new(self, interval.into_stream())
    }

    /// Returns a stream that debounces for the given duration.
    ///
    /// # Example
    ///
    /// ```
    /// use futures_lite::prelude::*;
    /// use futures_time::prelude::*;
    /// use futures_time::time::{Instant, Duration};
    /// use futures_time::stream;
    ///
    /// fn main() {
    ///     async_io::block_on(async {
    ///         let interval = Duration::from_millis(10);
    ///         let debounce = Duration::from_millis(20);
    ///
    ///         let mut counter = 0;
    ///         stream::interval(interval)
    ///             .take(10)
    ///             .debounce(debounce)
    ///             .for_each(|_| counter += 1)
    ///             .await;
    ///
    ///         assert_eq!(counter, 1);
    ///     })
    /// }
    /// ```
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

    /// Yield an item, then ignore subsequent items for a duration.
    ///
    /// This is the logical inverse of [`sample`], which yields the _last_ item
    /// received during each interval.
    ///
    /// In addition to using a time-based interval, this method can take any
    /// stream as a source. This enables throttling based on alternative event
    /// sources, such as variable-rate timers.
    ///
    /// [`sample`]: `StreamExt::sample`
    ///
    /// # Data Loss
    ///
    /// This method will discard data between intervals. Though the
    /// discarded items will have their destuctors run, __using this method
    /// incorrectly may lead to unintended data loss__. This method is best used
    /// to reduce the number of _duplicate_ items after the first has been
    /// received, such as repeated mouse clicks or key presses. This method may
    /// lead to unintended data loss when used to discard _unique_ items, such
    /// as network request.
    ///
    /// # Examples
    ///
    /// ```
    /// use futures_lite::prelude::*;
    /// use futures_time::prelude::*;
    /// use futures_time::time::Duration;
    /// use futures_time::stream;
    ///
    /// fn main() {
    ///     async_io::block_on(async {
    ///         let mut counter = 0;
    ///         stream::interval(Duration::from_millis(100))  // Yield an item every 100ms
    ///             .take(4)                                  // Stop after 4 items
    ///             .throttle(Duration::from_millis(300))     // Only let an item through every 300ms
    ///             .for_each(|_| counter += 1)               // Increment a counter for each item
    ///             .await;
    ///
    ///         assert_eq!(counter, 2);
    ///     })
    /// }
    /// ```
    fn throttle<I>(self, interval: I) -> Throttle<Self, I::IntoStream>
    where
        Self: Sized,
        I: IntoStream,
    {
        Throttle::new(self, interval.into_stream())
    }

    /// Return an error if a stream does not yield an item within a given time
    /// span.
    ///
    /// Typically timeouts are, as the name implies, based on _time_. However
    /// this method can time out based on any future. This can be useful in
    /// combination with channels, as it allows (long-lived) streams to be
    /// cancelled based on some external event.
    ///
    /// When a timeout is returned, the stream will be dropped and destructors
    /// will be run.
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
