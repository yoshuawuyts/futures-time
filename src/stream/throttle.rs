use std::pin::Pin;
use std::time::Duration;
use std::{future::Future, time::Instant};

use pin_project_lite::pin_project;

use async_io::Timer;
use core::task::{Context, Poll};
use futures_core::stream::Stream;

pin_project! {
    /// Throttle the stream.
    #[derive(Debug)]
    pub struct Throttle<S: Stream> {
        #[pin]
        stream: S,
        #[pin]
        delay: Timer,
        boundary: Duration,
        deadline: Option<Instant>,
        slot: Option<S::Item>,
    }
}

impl<S: Stream> Throttle<S> {
    pub(crate) fn new(stream: S, boundary: Duration) -> Self {
        let delay = Timer::after(boundary);

        Self {
            stream,
            delay,
            boundary,
            deadline: None,
            slot: None,
        }
    }
}

impl<S: Stream> Stream for Throttle<S> {
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        match this.stream.poll_next(cx) {
            Poll::Ready(Some(value)) => match this.deadline {
                Some(deadline) => {
                    *this.slot = Some(value);
                    if &Instant::now() >= deadline {
                        let deadline = Instant::now() + *this.boundary;
                        *this.delay.as_mut() = Timer::at(deadline);
                        *this.deadline = Some(deadline);
                        Poll::Ready(this.slot.take())
                    } else {
                        Poll::Pending
                    }
                }
                None => {
                    *this.slot = None;
                    let deadline = Instant::now() + *this.boundary;
                    *this.delay.as_mut() = Timer::at(deadline);
                    *this.deadline = Some(deadline);
                    Poll::Ready(Some(value))
                }
            },
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => match this.delay.as_mut().poll(cx) {
                Poll::Ready(_) => match this.slot.take() {
                    Some(item) => {
                        let deadline = Instant::now() + *this.boundary;
                        *this.delay.as_mut() = Timer::at(deadline);
                        *this.deadline = Some(deadline);
                        Poll::Ready(Some(item))
                    }
                    None => {
                        *this.deadline = None;
                        Poll::Pending
                    }
                },
                Poll::Pending => return Poll::Pending,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use futures_lite::prelude::*;
    use std::time::Duration;

    #[test]
    fn smoke() {
        async_io::block_on(async {
            let bound = Duration::from_millis(10);
            let throttle_bound = Duration::from_millis(20);

            let mut counter = 0;
            crate::stream::interval(bound)
                .take(10)
                .throttle(throttle_bound)
                .for_each(|_| counter += 1)
                .await;

            assert_eq!(counter, 5);
        })
    }
}
