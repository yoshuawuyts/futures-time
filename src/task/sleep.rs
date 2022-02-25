use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use std::{future::Future, time::Instant};

use async_io::Timer;
use pin_project_lite::pin_project;

use crate::future::ResetFuture;

/// Sleeps for the specified amount of time.
pub fn sleep(dur: Duration) -> Sleep {
    Sleep {
        dur,
        timer: Timer::after(dur),
        completed: false,
    }
}

pin_project! {
    /// Sleeps for the specified amount of time.
    #[must_use = "futures do nothing unless polled or .awaited"]
    pub struct Sleep {
        #[pin]
        timer: Timer,
        completed: bool,
        dur: Duration,
    }
}

impl Future for Sleep {
    type Output = Instant;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        assert!(!self.completed, "future polled after completing");
        let this = self.project();
        match this.timer.poll(cx) {
            Poll::Ready(instant) => {
                *this.completed = true;
                Poll::Ready(instant)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

impl ResetFuture for Sleep {
    /// Resets the timer to be `Instant::now()` + `Duration` into the future.
    fn reset(self: std::pin::Pin<&mut Self>) {
        let mut this = self.project();
        this.timer.set_after(*this.dur);
        *this.completed = false;
    }
}
