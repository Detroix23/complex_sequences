//! # Complex sequences.
//! src/fractals/divergence/mod.rs
//! 
//! Create fractal with the speed of sequences.

mod maths;
pub mod texture;
pub mod app;
pub mod method;

pub use maths::State;
pub use method::LimitMethod;
pub use texture::Divergent;