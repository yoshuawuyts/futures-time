use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use pin_project_lite::pin_project;

use async_io::Timer;
use core::task::{Context, Poll};

/// Sleeps for the specified amount of time.
pub fn sleep<F, T>(dur: Duration) -> Sleep
where
    F: Future<Output = T>,
{
    Sleep::new(dur)
}

pin_project! {
    /// Sleeps for the specified amount of time.
    pub struct Sleep {
        #[pin]
        delay: Timer,
    }
}

impl Sleep {
    pub(super) fn new(dur: Duration) -> Sleep {
        Sleep {
            delay: Timer::after(dur),
        }
    }
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        match this.delay.poll(cx) {
            Poll::Ready(_) => Poll::Ready(()),
            Poll::Pending => Poll::Pending,
        }
    }
}
