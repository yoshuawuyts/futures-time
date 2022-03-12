use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use async_channel::{bounded, Receiver, Sender};
use futures_core::stream::Stream;

/// Send a one-off signal.
///
/// When a sender is dropped, all of the receiver futures will complete,
/// allowing them to be completed. This can be used in combination with for
/// example [`timeout`] to perform cancellations.
///
/// [`timeout`]: crate::future::FutureExt::timeout
pub fn signal() -> (SignalSender, SignalReceiver) {
    let (sender, receiver) = bounded::<Infallible>(1);
    let sender = SignalSender { _chan: sender };
    let receiver = SignalReceiver { chan: receiver };
    (sender, receiver)
}

/// The sending side of a [`signal`].
///
/// This type is created by calling [`signal`], and triggers all of the receivers
/// when dropped.
#[derive(Debug)]
pub struct SignalSender {
    /// Solely for `Drop`.
    _chan: Sender<Infallible>,
}

impl SignalSender {
    /// Cancel all listeners by consuming the sender.
    pub fn cancel(self) {
        drop(self)
    }
}

/// The sender stops execution of all its associated `CancelReceivers` when
/// dropped.
impl Drop for SignalSender {
    fn drop(&mut self) {}
}

/// The receiving side of a [`signal`].
///
/// This type is created by calling [`signal`], can be cloned, and
/// will resolve when its associated sender is dropped. Pass this to
/// [`Future::timeout`] or [`Stream::timeout`] to perform a cancellation.
///
/// [`Future::timeout`]: crate::future::FutureExt::timeout
/// [`Stream::timeout`]: crate::stream::StreamExt::timeout
#[derive(Debug, Clone)]
pub struct SignalReceiver {
    chan: Receiver<Infallible>,
}

impl Future for SignalReceiver {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let chan = Pin::new(&mut self.chan);
        match Stream::poll_next(chan, cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Some(never)) => match never {},
            Poll::Ready(None) => Poll::Ready(()),
        }
    }
}
