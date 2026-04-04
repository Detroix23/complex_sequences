//! # Complex sequences.
//! src/fractals/divergence/mod.rs
//! 
//! Create fractal with the speed of sequences.

mod maths;
pub mod texture;
pub mod app;

pub use maths::{
	State,
	LimitMethod,
};
pub use texture::Divergent;