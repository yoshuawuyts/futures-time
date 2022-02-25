use std::pin::Pin;
use std::{future::Future, time::Instant};

use pin_project_lite::pin_project;

use async_io::Timer;
use core::task::{Context, Poll};

/// Sleeps for the specified amount of time.
pub fn sleep_until(deadline: Instant) -> SleepUntil {
    SleepUntil::new(deadline)
}

pin_project! {
    /// Sleeps for the specified amount of time.
    pub struct SleepUntil {
        #[pin]
        delay: Timer,
    }
}

impl SleepUntil {
    pub(super) fn new(deadline: Instant) -> Self {
        Self {
            delay: Timer::at(deadline),
        }
    }
}

impl Future for SleepUntil {
    type Output = Instant;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        match this.delay.poll(cx) {
            Poll::Ready(instant) => Poll::Ready(instant),
            Poll::Pending => Poll::Pending,
        }
    }
}
