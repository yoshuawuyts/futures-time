use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use async_io::Timer;
use futures_core::stream::Stream;

/// Creates a new stream that yields at a set interval.
///
/// The stream first yields after `dur`, and continues to yield every
/// `dur` after that. The stream accounts for time elapsed between calls, and
/// will adjust accordingly to prevent time skews.
///
/// Each interval may be slightly longer than the specified duration, but never
/// less.
///
/// Note that intervals are not intended for high resolution timers, but rather
/// they will likely fire some granularity after the exact instant that they're
/// otherwise indicated to fire at.
pub fn interval(dur: Duration) -> Interval {
    Interval {
        delay: Timer::after(dur),
        interval: dur,
    }
}

/// A stream representing notifications at fixed interval
///
/// This stream is created by the [`interval`] function. See its
/// documentation for more.
///
/// [`interval`]: fn.interval.html
#[derive(Debug)]
pub struct Interval {
    delay: Timer,
    interval: Duration,
}

impl Stream for Interval {
    type Item = ();

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if Pin::new(&mut self.delay).poll(cx).is_pending() {
            return Poll::Pending;
        }
        let interval = self.interval;
        let _ = std::mem::replace(&mut self.delay, Timer::after(interval));
        Poll::Ready(Some(()))
    }
}
