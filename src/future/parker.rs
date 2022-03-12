use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures_core::{ready, Future};

/// A future parking primitive.
///
/// Conceptually, each `Parker` has an associated token which is initially not present:
///
/// * The [`park`] method suspends the current future unless or until the token is available, at
///   which point it automatically consumes the token.
///
/// * The [`unpark`] method atomically makes the token available if it wasn't already. Because the
///   token is initially absent, [`unpark`] followed by [`park`] will result in the second call
///   returning immediately.
///
/// In other words, each `Parker` acts a bit like a spinlock that can be locked and unlocked using
/// [`park`] and [`unpark`].
///
/// [`park`]: Parker::park
/// [`unpark`]: Unparker::unpark
pub fn parker() -> (Parker, Unparker) {
    let (sender, receiver) = async_channel::bounded(1);
    let parker = Parker(receiver);
    let unparker = Unparker(sender);
    (parker, unparker)
}

/// Suspends a future until woken up by its associated [`Unparker`].
///
/// This type is created by [`parker`]. See its documentation for more.
#[derive(Debug)]
pub struct Parker(async_channel::Receiver<()>);

impl Parker {
    /// Suspend the current future until woken.
    ///
    /// This is also woken if no more `Unparker`s are available.
    pub fn park(&self) -> Park<'_> {
        Park {
            recv: self.0.recv(),
        }
    }
}

pin_project_lite::pin_project! {
    /// The future returned by [`Parker::park`].
    #[derive(Debug)]
    pub struct Park<'a> {
        #[pin]
        recv: async_channel::Recv<'a, ()>,
    }
}

impl<'a> Future for Park<'a> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let _ = ready!(this.recv.poll(cx));
        Poll::Ready(())
    }
}

/// Unparks a future parked by the associated [`Parker`].
///
/// This type is created by [`parker`]. See its documentation for more.
#[derive(Debug, Clone)]
pub struct Unparker(async_channel::Sender<()>);

impl Unparker {
    /// Wake up the associated future if it isn't already.
    ///
    /// If the associated `Parker` no longer exists, this function will complete
    /// as well.
    pub fn unpark(&self) -> Unpark<'_> {
        let mut completed = false;
        // Make sure we don't try and wake without listeners.
        if self.0.receiver_count() == 0 {
            completed = true;
        }
        // Make sure if a wakeup is in progress we don't queue up.
        if self.0.is_full() {
            completed = true;
        }

        // FIXME: the guard checks above are non-atomic, this means there is a
        // chance we might race / hang indefinitely.
        // Someone with a good grasp of atomics should try and reimplement this
        // without using channels so the checks actually hold.
        Unpark {
            send: self.0.send(()),
            completed,
        }
    }
}

pin_project_lite::pin_project! {
    /// The future returned by [`Unparker::unpark`].
    #[derive(Debug)]
    pub struct Unpark<'a> {
        #[pin]
        send: async_channel::Send<'a, ()>,
        completed: bool,
    }
}

impl<'a> Future for Unpark<'a> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        if *this.completed {
            return Poll::Ready(());
        }

        // Encountering errors is fine; if there is no more future to wake
        // there's no point in continuing to wait.
        let _ = ready!(this.send.poll(cx));
        Poll::Ready(())
    }
}

impl Drop for Unparker {
    /// When the last `Unparker` is dropped, the `Parker` will be woken.
    fn drop(&mut self) {
        // This behavior is part of `async-channel`. Nothing for us to implement
        // here.
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn smoke() {
        async_io::block_on(async {
            let (parker, unparker) = parker();
            drop(unparker);
            parker.park().await;
            // did not hang!
        });
    }
}
