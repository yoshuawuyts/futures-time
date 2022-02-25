use std::{future::Future, pin::Pin};

/// Reset the state of a future.
pub trait ResetFuture: Future {
    /// Reset a future's state.
    fn reset(self: Pin<&mut Self>);
}
