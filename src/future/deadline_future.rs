use std::{future::Future, pin::Pin};

/// A future which can be used as a deadline.
///
/// Compared to regular futures, a reset-deadline future must be able to shift
/// its deadline to some point in the future. This is needed for operations such
/// as `debounce` which shift the deadline into the future on each operation.
pub trait ResetDeadlineFuture: Future {
    /// Reset a future's state.
    fn reset_deadline(self: Pin<&mut Self>);
}
