use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use pin_project_lite::pin_project;

use async_io::Timer;
use core::task::{Context, Poll};
use futures_core::stream::Stream;

pin_project! {
    /// Debounce the stream.
    #[derive(Debug)]
    pub struct Debounce<S: Stream> {
        #[pin]
        stream: S,
        #[pin]
        delay: Timer,
        boundary: Duration,
        slot: Option<S::Item>,
    }
}

impl<S: Stream> Debounce<S> {
    pub(crate) fn new(stream: S, boundary: Duration) -> Self {
        let delay = Timer::after(boundary);

        Self {
            stream,
            delay,
            boundary,
            slot: None,
        }
    }
}

impl<S: Stream> Stream for Debounce<S> {
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        match this.stream.poll_next(cx) {
            Poll::Ready(Some(v)) => {
                *this.slot = Some(v);
                *this.delay.as_mut() = Timer::after(*this.boundary);
                Poll::Pending
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => match this.delay.as_mut().poll(cx) {
                Poll::Ready(_) => Poll::Ready(this.slot.take()),
                Poll::Pending => return Poll::Pending,
            },
        }
    }
}
