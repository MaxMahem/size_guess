# size_guess

[![CI](https://github.com/MaxMahem/size_guess/workflows/CI/badge.svg)](https://github.com/MaxMahem/size_guess/actions)

Provides a way to get a guess of the size of an iterator. 

The guess is based on the upper size hint of the iterator if present, otherwise the lower.

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

## License

Licensed under either of

 * [Apache License, Version 2.0](LICENSE-APACHE)
 * [MIT license](LICENSE-MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
