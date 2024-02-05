# Float multiplication to integer part

[![Crate informations](https://img.shields.io/crates/v/fmul-to-int.svg?style=flat-square)](https://crates.io/crates/fmul-to-int)
[![License](https://img.shields.io/crates/l/fmul-to-int.svg?style=flat-square)](https://github.com/timothee-haudebourg/fmul-to-int#license)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/fmul-to-int)

<!-- cargo-rdme start -->

This library provides the `mul_to_int` function for `f32` and `f64` that
allows one to multiply two float numbers and keep the integer part of the
result without loss of precision. The fractional part is truncated.

## Usage

Just import the `FloatMulToInt` trait to have access to the `mul_to_int`
method.

```rust
use fmul_to_int::FloatMulToInt;

assert_eq!(11.0f64.mul_to_int(1_000_000_000.0).unwrap(), 11_000_000_000i128);
```

<!-- cargo-rdme end -->
