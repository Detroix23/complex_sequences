//! # Complex sequences.
//! src/fractals/structures/configuration.rs

use crate::structures::color;

/// # Global `Settings` state.
/// Store only persistent settings.
pub struct GlobalSettings {
	/// Fractal family, method id.
	/// ```rust, no_run
	/// 0. Debug
	/// 1. Divergence,
	/// 2. Roots,
	/// ```
	pub method_id: usize,
	pub enable_grid: bool,
	pub color_mode_id: usize,
	pub color_mode: color::ColorMode,
	pub resolution_scale: f32,
}

impl Default for GlobalSettings {
	fn default() -> Self {
		GlobalSettings { 
			// 0 is debug.
			method_id: 1usize, 
			enable_grid: true,
			color_mode_id: 0,
			color_mode: color::ColorMode::GRAYSCALE,
			resolution_scale: 1.0,
		}
	}
}
