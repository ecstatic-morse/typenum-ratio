# typenum_ratio

Compile-time rational arithmetic based on [`typenum`].

## Notes

At the moment, this crate relies on a custom version of [`typenum`] with support for computing the greatest common divisor of two compile-time integers.

When support for integer generics lands on stable, this crate will be obseleted by another which implements the same functionality using only language features.

[`typenum`]: https://crates.io/crates/typenum

