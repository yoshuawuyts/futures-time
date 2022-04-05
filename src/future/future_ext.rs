use core::future::Future;

use super::{Delay, IntoFuture, Timeout};

/// Extend `Future` with time-based operations.
pub trait FutureExt: Future {
    /// Return an error if a future does not complete within a given time span.
    ///
    /// Typically timeouts are, as the name implies, based on _time_. However
    /// this method can time out based on any future. This can be useful in
    /// combination with channels, as it allows (long-lived) futures to be
    /// cancelled based on some external event.
    ///
    /// When a timeout is returned, the future will be dropped and destructors
    /// will be run.
    ///
    /// # Example
    ///
    /// ```
    /// use futures_time::prelude::*;
    /// use futures_time::time::{Instant, Duration};
    /// use std::io;
    ///
    /// fn main() {
    ///     async_io::block_on(async {
    ///         let res = async { "meow" }
    ///             .delay(Duration::from_millis(100))  // longer delay
    ///             .timeout(Duration::from_millis(50)) // shorter timeout
    ///             .await;
    ///         assert_eq!(res.unwrap_err().kind(), io::ErrorKind::TimedOut); // error
    ///
    ///         let res = async { "meow" }
    ///             .delay(Duration::from_millis(50))    // shorter delay
    ///             .timeout(Duration::from_millis(100)) // longer timeout
    ///             .await;
    ///         assert_eq!(res.unwrap(), "meow"); // success
    ///     });
    /// }
    /// ```
    fn timeout<D>(self, deadline: D) -> Timeout<Self, D::IntoFuture>
    where
        Self: Sized,
        D: IntoFuture,
    {
        Timeout::new(self, deadline.into_future())
    }

    /// Returns a future that delays execution for a specified time.
    ///
    /// Alternatively an arbitrary future can be passed in as well, allowing
    /// `delay` to wait for an event to triger.
    ///
    /// # Example
    ///
    /// ```
    /// use futures_time::prelude::*;
    /// use futures_time::time::{Instant, Duration};
    ///
    /// fn main() {
    ///     async_io::block_on(async {
    ///         let now = Instant::now();
    ///         let delay = Duration::from_millis(100);
    ///         let _ = async { "meow" }.delay(delay).await;
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
}

impl<T> FutureExt for T where T: Future {}
