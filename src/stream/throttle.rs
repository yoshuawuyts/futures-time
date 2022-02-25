use pin_project_lite::pin_project;

use futures_core::stream::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

pin_project! {
    /// Filter out all items after the first for a specified time.
    #[derive(Debug)]
    #[must_use = "streams do nothing unless polled or .awaited"]
    pub struct Throttle<S: Stream, I> {
        #[pin]
        stream: S,
        #[pin]
        interval: I,
        state: State,
        slot: Option<S::Item>,
    }
}

impl<S: Stream, I> Throttle<S, I> {
    pub(crate) fn new(stream: S, interval: I) -> Self {
        Self {
            state: State::Streaming,
            stream,
            interval,
            slot: None,
        }
    }
}

#[derive(Debug)]
enum State {
    /// The underlying stream is yielding items.
    Streaming,
    /// All timers have completed and all data has been yielded.
    StreamDone,
    /// The closing `Ready(None)` has been yielded.
    AllDone,
}

impl<S: Stream, I: Stream> Stream for Throttle<S, I> {
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        match this.state {
            // The underlying stream is yielding items.
            State::Streaming => {
                // Poll the underlying stream until we get to `Poll::Pending`.
                loop {
                    match this.stream.as_mut().poll_next(cx) {
                        Poll::Ready(Some(value)) => {
                            let _ = this.slot.insert(value);
                        }
                        Poll::Ready(None) => {
                            *this.state = State::StreamDone;
                            break;
                        }
                        Poll::Pending => break,
                    }
                }

                // After the stream, always poll the interval timer.
                this.interval.as_mut().poll_next(cx).map(move |_| {
                    if let State::StreamDone = this.state {
                        cx.waker().wake_by_ref();
                    }
                    this.slot.take()
                })
            }

            // All streams have completed and all data has been yielded.
            State::StreamDone => {
                *this.state = State::AllDone;
                Poll::Ready(None)
            }

            // The closing `Ready(None)` has been yielded.
            State::AllDone => panic!("stream polled after completion"),
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
            let interval = Duration::from_millis(100);
            let throttle = Duration::from_millis(200);

            let take = 4;
            let expected = 2;

            let mut counter = 0;
            crate::stream::interval(interval)
                .take(take)
                .throttle(throttle)
                .for_each(|_| counter += 1)
                .await;

            assert_eq!(counter, expected);
        })
    }
}
