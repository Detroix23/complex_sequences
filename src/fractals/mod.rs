//! # Complex sequences.
//! src/fractals/mod.rs
//! 
//! Compute and generate the graphics.

use std::fmt;
use std::convert;

pub mod textures;
pub mod divergence;
pub mod root;

/// # Fractal drawing `Method`.
/// Choose which fractal "family" to draw. 
/// To `usize`:
/// ```rust, no_run
/// 0. Divergence,
/// 1. Roots,
/// ```
pub enum Method {
	Debug,
	Divergence,
	Roots,
}

impl Method {
	/// Return a `Vec` of all the methods.  
	/// ```rust, no_run
	/// 0. Divergence,
	/// 1. Roots,
	/// ```
	pub fn list() -> Vec<Method> {
		vec![
			Method::Debug,
			Method::Divergence,
			Method::Roots,
		]
	}

	/// Return a `&'static str` representation of `Method`, with its id.
	fn to_static_str(self: &Self) -> &'static str {
		match &self {
			Method::Debug => "0. Debug.",
			Method::Divergence => "1. Divergence.",
			Method::Roots => "2. Roots.",
		}
	}
}

impl fmt::Display for Method {
	fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(formatter, "Method::{}", match &self {
			Method::Debug => "Debug",
			Method::Divergence => "Divergence",
			Method::Roots => "Roots",
		})
	}
}

impl convert::AsRef<str> for Method {
	fn as_ref(self: &Self) -> &str {
		&self.to_static_str()
	}
}