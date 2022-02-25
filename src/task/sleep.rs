use std::pin::Pin;
use std::time::Duration;
use std::{future::Future, time::Instant};

use pin_project_lite::pin_project;

use async_io::Timer;
use core::task::{Context, Poll};

/// Sleeps for the specified amount of time.
pub fn sleep(dur: Duration) -> Sleep {
    Sleep {
        timer: Timer::after(dur),
        completed: false,
    }
}

/// Sleeps until the specified instant.
pub fn sleep_until(deadline: Instant) -> Sleep {
    Sleep {
        timer: Timer::at(deadline),
        completed: false,
    }
}

pin_project! {
    /// Sleeps for the specified amount of time.
    #[must_use = "Futures do nothing unless polled or .awaited"]
    pub struct Sleep {
        #[pin]
        timer: Timer,
        completed: bool,
    }
}

impl Future for Sleep {
    type Output = Instant;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        assert!(!self.completed, "Future polled after completing");
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
