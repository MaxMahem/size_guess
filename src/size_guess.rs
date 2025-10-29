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
    /// use size_guess::SizeGuess;
    ///
    /// // exact size iterators will return an accurate guess
    /// let iter = 1..10;
    /// let guess = iter.size_guess();
    /// assert_eq!(iter, iter.len());
    ///
    /// // unbounded iterators will return the lower bound, which may be very large
    /// let iter = std::iter::repeat(());
    /// let guess = iter.size_guess();
    /// assert_eq!(guess, usize::MAX);
    /// ```
    fn size_guess(&self) -> usize {
        let (low, high) = self.size_hint();
        high.unwrap_or(low)
    }
}

#[cfg(test)]
mod tests {
    use super::SizeGuess;

    #[test]
    fn test_size_guess() {
        let iter = 1..10;
        let guess = iter.size_guess();

        // exact size iterators will return an accurate guess
        assert_eq!(guess, iter.len());
    }

    #[test]
    fn test_size_guess_unbounded() {
        let iter = std::iter::repeat(());
        let guess = iter.size_guess();

        // unbounded iterators will return the lower bound, which may be very large
        assert_eq!(guess, usize::MAX);
    }
}
