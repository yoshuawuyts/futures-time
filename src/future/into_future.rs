use std::future::Future;

/// Conversion into a `Future`.
pub trait IntoFuture {
    /// The output that the future will produce on completion.
    type Output;

    /// Which kind of future are we turning this into?
    type Future: Future<Output = Self::Output>;

    /// Creates a future from a value.
    fn into_future(self) -> Self::Future;
}

impl<F: Future> IntoFuture for F {
    type Output = F::Output;
    type Future = F;

    fn into_future(self) -> Self::Future {
        self
    }
}
