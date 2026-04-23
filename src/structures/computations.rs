//! # Complex sequences.
//! src/fractals/structures/computations.rs

/// # `Data` from fractal table computation.
pub struct Data {
	pub raw_pixels: Vec<u8>,
	pub iterations_total: usize,
}

/// # Divergence `State`.
/// Tell if a function diverges toward infinity or remains bounded.
#[derive(Debug, Clone, Copy)]
pub enum State {
	/// Divergent: in how many `iterations` does it diverged. 
	Divergent{ iterations: usize },
	Stable,
}

/// # `IsRoot`.
/// Define a possible root.
/// - `No`: no root,
/// - `Yes`: there is root, and the root is `root`: complex::Algebraic.
pub enum IsRoot {
	No,
	Yes { 
		root: complex::Algebraic,
		iterations: usize,
	},
}
