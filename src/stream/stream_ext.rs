use std::time::Duration;

use futures_core::Stream;

use super::{Delay, Timeout};

/// Extend `Stream` with time-based operations.
pub trait StreamExt: Stream {
    /// Await a stream or times out after a duration of time.     
    fn timeout(self, dur: Duration) -> Timeout<Self>
    where
        Self: Sized,
    {
        Timeout::new(self, dur)
    }

    /// Returns a stream that delays execution for a specified time.
    fn delay(self, dur: Duration) -> Delay<Self>
    where
        Self: Sized,
    {
        Delay::new(self, dur)
    }
}

impl<S> StreamExt for S where S: Stream {}
