use std::io;
use std::pin::Pin;

use pin_project_lite::pin_project;

use core::task::{Context, Poll};
use futures_core::stream::Stream;

use crate::{future::ResetDeadlineFuture, utils};

pin_project! {
    /// A stream with timeout time set
    #[derive(Debug)]
    #[must_use = "streams do nothing unless polled or .awaited"]
    pub struct Timeout<S, D> {
        #[pin]
        stream: S,
        #[pin]
        deadline: D,
    }
}

impl<S, D> Timeout<S, D> {
    pub(crate) fn new(stream: S, deadline: D) -> Self {
        Self { stream, deadline }
    }
}

impl<S: Stream, D: ResetDeadlineFuture> Stream for Timeout<S, D> {
    type Item = io::Result<S::Item>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        let r = match this.stream.poll_next(cx) {
            Poll::Ready(Some(v)) => Poll::Ready(Some(Ok(v))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => match this.deadline.as_mut().poll(cx) {
                Poll::Ready(_) => Poll::Ready(Some(Err(utils::timeout_err("stream timed out")))),
                Poll::Pending => return Poll::Pending,
            },
        };

        this.deadline.as_mut().reset_deadline();

        r
    }
}
