use std::future::Future;
use std::pin::Pin;

use pin_project_lite::pin_project;

use core::task::{Context, Poll};

pin_project! {
    /// A future that delays execution by the specified time.
    #[must_use = "Futures do nothing unless polled or .awaited"]
    #[derive(Debug)]
    pub struct Delay<F, D> {
        #[pin]
        future: F,
        #[pin]
        deadline: D,
        completed: bool,
    }
}

impl<F, D> Delay<F, D> {
    pub(crate) fn new(future: F, deadline: D) -> Self {
        Self {
            future,
            deadline,
            completed: false,
        }
    }
}

impl<F: Future, D: Future> Future for Delay<F, D> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        assert!(!self.completed, "Future polled after completing");

        let this = self.project();

        match this.deadline.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(_) => match this.future.poll(cx) {
                Poll::Ready(v) => {
                    *this.completed = true;
                    Poll::Ready(v)
                }
                Poll::Pending => Poll::Pending,
            },
        }
    }
}
