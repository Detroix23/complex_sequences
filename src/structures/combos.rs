//! # Complex sequences.
//! src/gui/combos.rs

use std::{fmt, convert};

/// # Shared behavior for `ComboMethod`s, with `imgui`'s combos.
/// Implement for `enum`s.
pub trait ComboMethod: 
	Sized 
	+ fmt::Display 
	+ convert::AsRef<str> 
{
	/// Returns a list of all options of the enumeration.
	fn list() -> Vec<Self>;

	/// Returns the ID of the current value.
	#[allow(dead_code)]
	fn id(self: &Self) -> u32;

	/// Return a string of the current value, with ID.
	fn to_static_str(self: &Self) -> &'static str;
}