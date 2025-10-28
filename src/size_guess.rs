pub trait SizeGuess {
    /// Provides a guess of the number of elements in the iterator.
    fn size_guess(&self) -> usize;
}

/// Default implementation for iterators.
impl<I: Iterator> SizeGuess for I {
    /// Provides a guess of the number of elements in the iterator.
    ///
    /// The guess is based on the upper size hint of the iterator if
    /// present, otherwise the lower.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::iter;
    /// use size_guess::SizeGuess;
    ///
    /// // exact size iterators will return an accurate guess
    /// let mut iter = vec![1, 2, 3].into_iter();
    /// assert_eq!(iter.size_guess(), 3);
    ///
    /// // iterators with no upper bound will return the lower bound which may be very large
    /// let mut iter = iter::repeat(());
    /// assert_eq!(iter.size_guess(), usize::MAX);
    /// ```
    fn size_guess(&self) -> usize {
        let (low, high) = self.size_hint();
        high.unwrap_or(low)
    }
}
