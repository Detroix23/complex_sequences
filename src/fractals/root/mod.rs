//! # Complex sequences.
//! src/fractals/root/mod.rs
//! 
//! Create fractals with the destination of sequence.

mod maths;
pub mod texture;
pub mod app;

pub use maths::{
	IsRoot,
	RootMethod,
};
pub use texture::Root;