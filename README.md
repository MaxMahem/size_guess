# size_guess

[![CI](https://github.com/MaxMahem/size_guess/workflows/CI/badge.svg)](https://github.com/MaxMahem/size_guess/actions)
![GitHub License](https://img.shields.io/github/license/maxmahem/size_guess)
[![dependency status](https://deps.rs/repo/github/maxmahem/size_guess/status.svg)](https://deps.rs/repo/github/maxmahem/size_guess)
[![codecov](https://codecov.io/github/MaxMahem/size_guess/graph/badge.svg?token=I0HHWBYHBO)](https://codecov.io/github/MaxMahem/size_guess)

Provides a way to estimate the size of an iterator. 

The guess is based on the iterator's upper bound hint, if present; otherwise, the lower bound.

## Usage

```rust
use size_guess::SizeGuess;

// exact size iterators will return an accurate guess
let iter = 1..10;
let guess = iter.size_guess();
assert_eq!(guess, iter.len());

// unbounded iterators will return the lower bound, which may be very large
let iter = std::iter::repeat(());
let guess = iter.size_guess();
assert_eq!(guess, usize::MAX);
```
