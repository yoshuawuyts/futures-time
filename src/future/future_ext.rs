use core::future::Future;
use std::time::Duration;

use crate::future::Timeout;

/// Extend `Future` with time-based operations.
pub trait FutureExt: Future {
    /// Await a stream or times out after a duration of time.     
    fn timeout(self, dur: Duration) -> Timeout<Self>
    where
        Self: Sized,
    {
        Timeout::new(self, dur)
    }
    // /// Returns a Future that delays execution for a specified time.
    // fn delay(self, dur: Duration) -> Delay<Self>
    // where
    //     Self: Sized,
    // {
    //     Delay::new(self, dur)
    // }
}

impl<T> FutureExt for T where T: Future {}
