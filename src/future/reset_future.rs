/// Reset the state of a future.
pub trait ResetFuture {
    /// Reset a future's state.
    fn reset(self: std::pin::Pin<&mut Self>);
}
