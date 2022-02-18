use std::error::Error;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use pin_project_lite::pin_project;

use async_io::Timer;
use core::task::{Context, Poll};

/// Awaits a future or times out after a duration of time.
pub async fn timeout<F, T>(dur: Duration, f: F) -> Result<T, TimeoutError>
where
    F: Future<Output = T>,
{
    Timeout::new(f, dur).await
}

pin_project! {
    /// A future that times out after a duration of time.
    pub struct Timeout<F> {
        #[pin]
        future: F,
        #[pin]
        delay: Timer,
    }
}

impl<F> Timeout<F> {
    #[allow(dead_code)]
    pub(super) fn new(future: F, dur: Duration) -> Self {
        Self {
            future,
            delay: Timer::after(dur),
        }
    }
}

impl<F: Future> Future for Timeout<F> {
    type Output = Result<F::Output, TimeoutError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        match this.future.poll(cx) {
            Poll::Ready(v) => Poll::Ready(Ok(v)),
            Poll::Pending => match this.delay.poll(cx) {
                Poll::Ready(_) => Poll::Ready(Err(TimeoutError { _private: () })),
                Poll::Pending => Poll::Pending,
            },
        }
    }
}

/// An error returned when a future times out.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimeoutError {
    _private: (),
}

impl Error for TimeoutError {}

impl fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "future has timed out".fmt(f)
    }
}
