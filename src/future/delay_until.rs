use std::future::Future;
use std::pin::Pin;

use pin_project_lite::pin_project;

use core::task::{Context, Poll};

pin_project! {
    /// A future that delays execution by the specified time.
    #[derive(Debug)]
    pub struct DelayUntil<F, D> {
        #[pin]
        future: F,
        #[pin]
        deadline: D,
    }
}

impl<F, D> DelayUntil<F, D> {
    pub(crate) fn new(future: F, deadline: D) -> DelayUntil<F, D> {
        DelayUntil { future, deadline }
    }
}

impl<F: Future, D: Future> Future for DelayUntil<F, D> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        match this.deadline.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(_) => match this.future.poll(cx) {
                Poll::Ready(v) => Poll::Ready(v),
                Poll::Pending => Poll::Pending,
            },
        }
    }
}
