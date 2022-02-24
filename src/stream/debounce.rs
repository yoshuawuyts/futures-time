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
        timer: Timer,
        boundary: Duration,
        slot: Option<S::Item>,
        exhausted: bool,
        done: bool,
    }
}

impl<S: Stream> Debounce<S> {
    pub(crate) fn new(stream: S, boundary: Duration) -> Self {
        let timer = Timer::after(boundary);

        Self {
            stream,
            timer,
            boundary,
            slot: None,
            exhausted: false,
            done: false,
        }
    }
}

impl<S: Stream> Stream for Debounce<S> {
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        if *this.done {
            panic!("stream polled after completion");
        } else if *this.exhausted {
            *this.done = true;
            return Poll::Ready(None);
        }

        match this.stream.poll_next(cx) {
            Poll::Ready(Some(value)) => {
                *this.slot = Some(value);
                *this.timer.as_mut() = Timer::after(*this.boundary);
                match this.timer.as_mut().poll(cx) {
                    Poll::Ready(_) => Poll::Ready(this.slot.take()),
                    Poll::Pending => Poll::Pending,
                }
            }
            Poll::Ready(None) => match this.slot.take() {
                Some(value) => {
                    *this.exhausted = true;
                    cx.waker().wake_by_ref();
                    Poll::Ready(Some(value))
                }
                None => {
                    *this.exhausted = true;
                    *this.done = true;
                    Poll::Ready(None)
                }
            },
            Poll::Pending => match this.timer.as_mut().poll(cx) {
                Poll::Ready(_) => match this.slot.take() {
                    Some(value) => Poll::Ready(Some(value)),
                    None => Poll::Pending,
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
    fn all_values_debounce() {
        async_io::block_on(async {
            let bound = Duration::from_millis(10);
            let debounce_bound = Duration::from_millis(20);

            let mut counter = 0;
            crate::stream::interval(bound)
                .take(10)
                .debounce(debounce_bound)
                .for_each(|_| counter += 1)
                .await;

            assert_eq!(counter, 1);
        })
    }

    #[test]
    fn no_debounces_hit() {
        async_io::block_on(async {
            let bound = Duration::from_millis(40);
            let debounce_bound = Duration::from_millis(10);

            let mut counter = 0;
            crate::stream::interval(bound)
                .take(10)
                .debounce(debounce_bound)
                .for_each(|_| counter += 1)
                .await;

            assert_eq!(counter, 10);
        })
    }
}
