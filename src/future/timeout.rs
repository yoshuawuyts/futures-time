use crate::utils::timeout_err;
use std::future::Future;
use std::io;
use std::pin::Pin;
use std::time::Duration;

use pin_project_lite::pin_project;

use async_io::Timer;
use core::task::{Context, Poll};

pin_project! {
    /// A future that times out after a duration of time.
    pub struct Timeout<F> {
        #[pin]
        future: F,
        #[pin]
        delay: Timer,
        completed: bool,
    }
}

impl<F> Timeout<F> {
    pub(super) fn new(future: F, dur: Duration) -> Self {
        Self {
            future,
            delay: Timer::after(dur),
            completed: false,
        }
    }
}

impl<F: Future> Future for Timeout<F> {
    type Output = io::Result<F::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        assert!(!self.completed, "Future polled after completing");
        let this = self.project();
        match this.future.poll(cx) {
            Poll::Ready(v) => {
                *this.completed = true;
                Poll::Ready(Ok(v))
            }
            Poll::Pending => match this.delay.poll(cx) {
                Poll::Ready(_) => {
                    *this.completed = true;
                    Poll::Ready(Err(timeout_err("future timed out")))
                }
                Poll::Pending => Poll::Pending,
            },
        }
    }
}
