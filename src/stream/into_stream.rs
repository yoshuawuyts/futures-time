use futures_core::Stream;

/// Conversion into a `Stream`.
///
/// By implementing `IntoIterator` for a type, you define how it will be
/// converted to an iterator. This is common for types which describe a
/// collection of some kind.
pub trait IntoStream {
    /// The type of the elements being iterated over.
    type Item;

    /// Which kind of stream are we turning this into?
    type IntoStream: Stream<Item = Self::Item>;

    /// Creates a stream from a value.
    fn into_stream(self) -> Self::IntoStream;
}

impl<I: Stream> IntoStream for I {
    type Item = I::Item;
    type IntoStream = I;

    #[inline]
    fn into_stream(self) -> I {
        self
    }
}
