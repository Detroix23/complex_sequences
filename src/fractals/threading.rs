//! # Complex sequences.
//! src/fractals/threading.rs

use std::{thread, num};

/// # Named tuple `GenerationPart`.
/// Useful while work is divided between threads.
pub struct GenerationPart<T> {
	pub thread_id: usize,
	/// (`start`, `end`).
	#[allow(dead_code)]
	pub bounding_box: ([usize; 2], [usize; 2]),
	pub data: Vec<Vec<T>>,
}

impl<T> GenerationPart<T> {
	pub fn new(thread_id: usize, bounding_box: ([usize; 2], [usize; 2]), data: Vec<Vec<T>>) -> GenerationPart<T> {
		GenerationPart { 
			thread_id, 
			bounding_box, 
			data 
		}
	}
}

/// Always return an `usize`.
pub fn determine_threads() -> num::NonZero<usize> {
	match thread::available_parallelism() {
		Ok(value) => value,
		Err(error) => {
			eprintln!("(!) fractals::threading::determine_threads() Error `{}`. Falling to 1.", error);
			num::NonZero::new(1_usize).unwrap()
		}
	}
}
