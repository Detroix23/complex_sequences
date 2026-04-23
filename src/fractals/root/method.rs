//! # Complex sequences.
//! src/fractals/root/method.rs

use std::{convert, fmt};

use crate::structures::combos::ComboMethod;

/// # `RootMethod`.
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum RootMethod {
	/// Use the Newton method (`f(x)/f'(x)`)to find a root.
	Newton,
	/// Color map of the complex plane through the function.
	Position
}

impl ComboMethod for RootMethod {
	/// Return a `Vec` of all the methods.
	/// ```rust, no_run
	/// 1. Newton;
	/// 2. Position.
	/// ```
	fn list() -> Vec<RootMethod> {
		vec![
			RootMethod::Newton,
			RootMethod::Position,
		]
	}
	
	fn id(self: &Self) -> u32 {
		match &self {
			RootMethod::Newton => 1,
			RootMethod::Position => 2,
		}
	}

	/// Return a `&'static str` representation of `RootMethod`, with ID.
	fn to_static_str(self: &Self) -> &'static str {
		match &self {
			RootMethod::Newton => "1. Newton",
			RootMethod::Position => "2. Position",
		}
	}
}

impl fmt::Display for RootMethod {
	fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(formatter, "{}", self.to_static_str())
	}
}

impl convert::AsRef<str> for RootMethod {
	fn as_ref(self: &Self) -> &str {
		&self.to_static_str()
	}
}