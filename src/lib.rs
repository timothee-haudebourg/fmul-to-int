//! This library provides the `mul_to_int` function for `f32` and `f64` that
//! allows one to multiply two float numbers and keep the integer part of the
//! result without loss of precision. The fractional part is truncated.
//!
//! # Usage
//!
//! Just import the `FloatMulToInt` trait to have access to the `mul_to_int`
//! method.
//!
//! ```
//! use fmul_to_int::FloatMulToInt;
//!
//! assert_eq!(11.0f64.mul_to_int(1_000_000_000.0).unwrap(), 11_000_000_000i128);
//! ```

/// Float type implementing the `mul_to_int` function.
pub trait FloatMulToInt {
	/// Integer output type.
	type Output;

	/// Multiplies the two input numbers `a` and `b`, and returns the
	/// integer part of the result *without approximation*.
	/// The fractional part is truncated.
	///
	/// This function returns an `Overflow` error if the integer
	/// part does not fit into the [`Self::Output`] type.
	///
	/// # Panics
	///
	/// This function panics if the input values are not finite
	/// (so if at least one of them is infinite or NaN).
	fn mul_to_int(self, other: Self) -> Result<Self::Output, Overflow>;
}

impl FloatMulToInt for f32 {
	type Output = i64;

	fn mul_to_int(self, other: f32) -> Result<i64, Overflow> {
		/// Mask for the sign bit of the `i64` integer type.
		const SIGN_MASK: u64 = 1 << 63;

		/// Decompose a `f32` value into its sign, exponent and significand.
		#[derive(Debug)]
		struct DecomposedF32 {
			/// Sign bit.
			///
			/// False is positive, true is negative.
			sign: bool,

			/// Exponent.
			exponent: i16,

			/// Significand, with the implicit largest `1`-bit omitted in the `f32`
			/// representation.
			significand: u32,
		}

		impl DecomposedF32 {
			pub fn new(value: f32) -> Self {
				if !value.is_finite() {
					panic!("input must be finite")
				}

				if value == 0.0 {
					Self {
						sign: false,
						exponent: 0,
						significand: 0,
					}
				} else {
					let raw = value.to_bits();

					Self {
						sign: (raw >> 31) == 1,
						exponent: ((raw >> 23) & 0xff) as i16 - 127,
						significand: 1 << 31 | raw << 8,
					}
				}
			}
		}

		let a = DecomposedF32::new(self);
		let b = DecomposedF32::new(other);

		let exponent = a.exponent + b.exponent;
		if exponent > 62 {
			// The absolute value is simply too big.
			Err(Overflow)
		} else if exponent < 0 {
			// Integer part is 0.
			Ok(0)
		} else {
			let significand = a.significand as u64 * b.significand as u64;
			let shift = 62 - exponent as u8;
			let unsigned = significand >> shift;

			if unsigned & SIGN_MASK != 0 {
				// No room for the sign.
				return Err(Overflow);
			} else if a.sign ^ b.sign {
				Ok(-(unsigned as i64))
			} else {
				Ok(unsigned as i64)
			}
		}
	}
}

impl FloatMulToInt for f64 {
	type Output = i128;

	fn mul_to_int(self, other: f64) -> Result<i128, Overflow> {
		/// Mask for the sign bit of the `i128` integer type.
		const SIGN_MASK: u128 = 1 << 127;

		/// Decompose a `f64` value into its sign, exponent and significand.
		#[derive(Debug)]
		struct DecomposedF64 {
			/// Sign bit.
			///
			/// False is positive, true is negative.
			sign: bool,

			/// Exponent.
			exponent: i16,

			/// Significand, with the implicit largest `1`-bit omitted in the `f64`
			/// representation.
			significand: u64,
		}

		impl DecomposedF64 {
			pub fn new(value: f64) -> Self {
				if !value.is_finite() {
					panic!("input must be finite")
				}

				if value == 0.0 {
					Self {
						sign: false,
						exponent: 0,
						significand: 0,
					}
				} else {
					let raw = value.to_bits();

					Self {
						sign: (raw >> 63) == 1,
						exponent: ((raw >> 52) & 0x7FF) as i16 - 1023,
						significand: 1 << 63 | raw << 11,
					}
				}
			}
		}

		let a = DecomposedF64::new(self);
		let b = DecomposedF64::new(other);

		let exponent = a.exponent + b.exponent;
		if exponent > 126 {
			// The absolute value is simply too big.
			Err(Overflow)
		} else if exponent < 0 {
			// Integer part is 0.
			Ok(0)
		} else {
			let significand = a.significand as u128 * b.significand as u128;
			let shift = 126 - exponent as u8;
			let unsigned = significand >> shift;

			if unsigned & SIGN_MASK != 0 {
				// No room for the sign.
				return Err(Overflow);
			} else if a.sign ^ b.sign {
				Ok(-(unsigned as i128))
			} else {
				Ok(unsigned as i128)
			}
		}
	}
}

#[derive(Debug)]
pub struct Overflow;

impl core::fmt::Display for Overflow {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "integer overflow")
	}
}

#[cfg(feature = "std")]
impl std::error::Error for Overflow {}

#[cfg(test)]
mod tests {
	use crate::FloatMulToInt;

	#[test]
	fn test_f32() {
		let vectors = [
			(0.0f32, 10000.0f32, 0i64),
			(0.0, -10000.0, 0),
			(0.0, 0.0, 0),
			(2.0, 2.0, 4),
			(1.0, 2.0, 2),
			(20.0, 20.0, 400),
			(11.0, 1_000_000_000.0, 11_000_000_000),
			(0.1234, 1_000_000_000.0, 123_400_002), // `0.1234` is not a `f32`.
			(0.2222, 22222.0, 4937),
			(-2.0, 2.0, -4),
			(1.0, -2.0, -2),
			(20.0, -20.0, -400),
			(11.0, -1_000_000_000.0, -11_000_000_000),
			(-0.1234, 1_000_000_000.0, -123_400_002), // `0.1234` is not a `f32`.
			(-0.2222, 22222.0, -4937),
			(-2.0, -2.0, 4),
		];

		for (a, b, c) in vectors {
			assert_eq!(a.mul_to_int(b).unwrap(), c);
		}
	}

	#[test]
	fn test_f64() {
		let vectors = [
			(0.0f64, 10000.0f64, 0i128),
			(0.0, -10000.0, 0),
			(0.0, 0.0, 0),
			(2.0, 2.0, 4),
			(1.0, 2.0, 2),
			(20.0, 20.0, 400),
			(11.0, 1_000_000_000.0, 11_000_000_000),
			(0.1234, 1_000_000_000.0, 123_399_999), // `0.1234` is not a `f64`.
			(0.2222, 22222.0, 4937),
			(-2.0, 2.0, -4),
			(1.0, -2.0, -2),
			(20.0, -20.0, -400),
			(11.0, -1_000_000_000.0, -11_000_000_000),
			(-0.1234, 1_000_000_000.0, -123_399_999), // `0.1234` is not a `f64`.
			(-0.2222, 22222.0, -4937),
			(-2.0, -2.0, 4),
		];

		for (a, b, c) in vectors {
			assert_eq!(a.mul_to_int(b).unwrap(), c);
		}
	}
}
