use core::future::Future;
use core::pin::Pin;
use std::time::Instant;

use pin_project_lite::pin_project;

use async_io::Timer;
use core::task::{Context, Poll};
use futures_core::stream::Stream;

pin_project! {
    /// Delay execution of a stream once until the specified instant.
    pub struct DelayUntil<S> {
        #[pin]
        stream: S,
        #[pin]
        delay: Timer,
        delay_done: bool,
    }
}

impl<S> DelayUntil<S> {
    pub(super) fn new(stream: S, deadline: Instant) -> Self {
        DelayUntil {
            stream,
            delay: Timer::at(deadline),
            delay_done: false,
        }
    }
}

impl<S> Stream for DelayUntil<S>
where
    S: Stream,
{
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();

        if !*this.delay_done {
            futures_core::ready!(this.delay.poll(cx));
            *this.delay_done = true;
        }

        this.stream.poll_next(cx)
    }
}
