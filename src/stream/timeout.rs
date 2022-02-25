use std::future::Future;
use std::io;
use std::pin::Pin;
use std::time::Duration;

use pin_project_lite::pin_project;

use async_io::Timer;
use core::task::{Context, Poll};
use futures_core::stream::Stream;

use crate::utils;

pin_project! {
    /// A stream with timeout time set
    #[derive(Debug)]
    #[must_use = "streams do nothing unless polled or .awaited"]
    pub struct Timeout<S: Stream> {
        #[pin]
        stream: S,
        #[pin]
        delay: Timer,
        duration: Duration,
    }
}

impl<S: Stream> Timeout<S> {
    pub(crate) fn new(stream: S, dur: Duration) -> Self {
        let delay = Timer::after(dur);

        Self {
            stream,
            delay,
            duration: dur,
        }
    }
}

impl<S: Stream> Stream for Timeout<S> {
    type Item = io::Result<S::Item>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        let r = match this.stream.poll_next(cx) {
            Poll::Ready(Some(v)) => Poll::Ready(Some(Ok(v))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => match this.delay.as_mut().poll(cx) {
                Poll::Ready(_) => Poll::Ready(Some(Err(utils::timeout_err("stream timed out")))),
                Poll::Pending => return Poll::Pending,
            },
        };

        *this.delay.as_mut() = Timer::after(*this.duration);

        r
    }
}
