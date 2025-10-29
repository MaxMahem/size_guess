# size_guess

[![CI](https://github.com/MaxMahem/size_guess/workflows/CI/badge.svg)](https://github.com/MaxMahem/size_guess/actions)
![GitHub License](https://img.shields.io/github/license/maxmahem/size_guess)
[![dependency status](https://deps.rs/repo/github/maxmahem/size_guess/status.svg)](https://deps.rs/repo/github/maxmahem/size_guess)

Provides a way to estimate the size of an iterator. 

The guess is based on the upper bound hint of the iterator, if present; otherwise, the lower bound.

## Usage

```rust
use size_guess::SizeGuess;
use std::iter;

// exact size iterators will return an accurate guess
let mut iter = vec![1, 2, 3].into_iter();
assert_eq!(iter.size_guess(), 3);

// iterators with no upper bound return the lower bound, which may be very large
let mut iter = iter::repeat(());
assert_eq!(iter.size_guess(), usize::MAX);
```
