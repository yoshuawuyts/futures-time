use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use async_io::Timer;
use pin_project_lite::pin_project;

use core::task::{Context, Poll};

pin_project! {
    /// A future that delays execution by the specified time.
    #[derive(Debug)]
    pub struct Delay<F> {
        #[pin]
        future: F,
        #[pin]
        delay: Timer,
    }
}

impl<F> Delay<F> {
    pub(crate) fn new(future: F, dur: Duration) -> Delay<F> {
        let delay = Timer::after(dur);

        Delay { future, delay }
    }
}

impl<F: Future> Future for Delay<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        match this.delay.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(_) => match this.future.poll(cx) {
                Poll::Ready(v) => Poll::Ready(v),
                Poll::Pending => Poll::Pending,
            },
        }
    }
}
