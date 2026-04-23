//! # Complex sequences.
//! src/fractals/divergence/methods.rs

use std::{fmt, convert};

use crate::structures::combos::ComboMethod;

/// # `LimitMethod` for any point of R².
/// Define how are defined the parameters `z0` and `c`.
/// To `usize`:
/// ```rust, no_run
/// 0. Mandelbrot,
/// 1. Julia,
/// ```
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum LimitMethod {
	Mandelbrot,
	Julia,
}

impl ComboMethod for LimitMethod {
	/// Return a `Vec` of all the methods.
	/// ```rust, no_run
	/// 0. Mandelbrot,
	/// 1. Julia,
	/// ```
	fn list() -> Vec<LimitMethod> {
		vec![
			LimitMethod::Mandelbrot,
			LimitMethod::Julia,
		]
	}

	fn id(self: &Self) -> u32 {
		match &self {
			LimitMethod::Mandelbrot => 1,
			LimitMethod::Julia => 2,
		}
	}

	/// Return a `&'static str` representation of `LimitMethod`, with ID.
	fn to_static_str(self: &Self) -> &'static str {
		match &self {
			LimitMethod::Mandelbrot => "1. Mandelbrot",
			LimitMethod::Julia => "2. Julia",
		}
	}
}

impl fmt::Display for LimitMethod {
	fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(formatter, "{}", self.to_static_str())
	}
}

impl convert::AsRef<str> for LimitMethod {
	fn as_ref(self: &Self) -> &str {
		&self.to_static_str()
	}
}
