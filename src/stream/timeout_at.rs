use std::io;
use std::pin::Pin;
use std::{future::Future, time::Instant};

use pin_project_lite::pin_project;

use async_io::Timer;
use core::task::{Context, Poll};
use futures_core::stream::Stream;

use crate::utils;

pin_project! {
    /// A stream with timeout time set
    #[derive(Debug)]
    pub struct TimeoutAt<S: Stream> {
        #[pin]
        stream: S,
        #[pin]
        delay: Timer,
        deadline: Instant,
    }
}

impl<S: Stream> TimeoutAt<S> {
    pub(crate) fn new(stream: S, deadline: Instant) -> Self {
        let delay = Timer::at(deadline);

        Self {
            stream,
            delay,
            deadline,
        }
    }
}

impl<S: Stream> Stream for TimeoutAt<S> {
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

        *this.delay.as_mut() = Timer::at(*this.deadline);

        r
    }
}
