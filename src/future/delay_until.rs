use std::pin::Pin;
use std::{future::Future, time::Instant};

use async_io::Timer;
use pin_project_lite::pin_project;

use core::task::{Context, Poll};

pin_project! {
    /// A future that delays execution by the specified time.
    #[derive(Debug)]
    pub struct DelayUntil<F> {
        #[pin]
        future: F,
        #[pin]
        delay: Timer,
    }
}

impl<F> DelayUntil<F> {
    pub(crate) fn new(future: F, deadline: Instant) -> DelayUntil<F> {
        let delay = Timer::at(deadline);

        DelayUntil { future, delay }
    }
}

impl<F: Future> Future for DelayUntil<F> {
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
