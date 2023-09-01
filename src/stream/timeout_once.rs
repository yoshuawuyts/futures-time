use core::future::Future;
use std::io;
use std::pin::Pin;

use pin_project_lite::pin_project;

use core::task::{Context, Poll};
use futures_core::stream::Stream;

use crate::utils;

pin_project! {
    /// A stream with timeout time set
    ///
    /// This `struct` is created by the [`timeout_once`] method on [`StreamExt`]. See its
    /// documentation for more.
    ///
    /// [`timeout_once`]: crate::stream::StreamExt::timeout_once
    /// [`StreamExt`]: crate::stream::StreamExt
    #[derive(Debug)]
    #[must_use = "streams do nothing unless polled or .awaited"]
    pub struct TimeoutOnce<S, D> {
        #[pin]
        stream: S,
        #[pin]
        deadline: D,
    }
}

impl<S, D> TimeoutOnce<S, D> {
    pub(crate) fn new(stream: S, deadline: D) -> Self {
        Self { stream, deadline }
    }
}

impl<S: Stream, D: Future> Stream for TimeoutOnce<S, D> {
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        match this.deadline.as_mut().poll(cx) {
            Poll::Ready(_) => Poll::Ready(None),
            Poll::Pending => this.stream.poll_next(cx),
        }
    }
}
