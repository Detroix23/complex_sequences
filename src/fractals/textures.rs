//! # Complex sequences.
//! src/fractals/textures.rs

use std::{error, fmt, convert};

use glium; 
use complex_rust as complex;

use crate::fractals;
use crate::gui::color;

/// `ColorMode`:
/// ```
/// 0. `GRAYSCALE`,
/// 1. `HSV`.
/// ``````
#[derive(Clone, Copy)]
pub enum ColorMode {
	GRAYSCALE,
	HSV,
}

impl ColorMode {
	/// Returns a vector of all `ColorMode`s.
	pub fn list() -> Vec<ColorMode> {
		vec![
			ColorMode::GRAYSCALE,
			ColorMode::HSV,
		]
	}

	fn to_static_str(self: &Self) -> &'static str {
		match &self {
			ColorMode::GRAYSCALE => "0. Grayscale.",
			ColorMode::HSV => "1. HSV.",
		}
	}
}

impl fmt::Display for ColorMode {
	fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(formatter, "Method::{}", match &self {
			ColorMode::GRAYSCALE => "Grayscale",
			ColorMode::HSV => "HSV",
		})
	}
}

impl convert::AsRef<str> for ColorMode {
	fn as_ref(self: &Self) -> &str {
		&self.to_static_str()
	}
}

pub trait Fractal {
	fn update_size(self: &mut Self, new_size: [u32; 2]) -> ();

	/// Generate and register the fractal texture.
	/// 
	/// Put some `size` if the size need to be updated.
	/// 
	/// Source: `imgui-examples`, `custom_texture`
	fn register_texture<Facade>(
        &mut self,
        gl_context: &Facade,
        textures: &mut imgui::Textures<imgui_glium_renderer::Texture>,
		color_mode: fractals::textures::ColorMode,
    ) -> Result<(), Box<dyn error::Error>>
    where
        Facade: glium::backend::Facade;

	/// Calls `window` method on `ui`, to display the texture. 
	/// 
	/// Source: `imgui-examples`, `custom_texture`
	fn show_textures(self: &Self, ui: &imgui::Ui, position: [complex::Real; 2]) -> ();
}

/// # `Data` from holomorphic computations.
pub struct Data {
	pub raw_pixels: Vec<u8>,
	pub iterations_total: usize,
}

/// Convert a 2D `table`: `Vec<Vec<State>>` into `Vec<u8>` of raw `data`. 
pub fn convert_state_table_to_data(
	table: Vec<Vec<fractals::divergence::State>>, 
	stable: color::Rgb, 
	divergent: color::Rgb,
	iterations_max: usize,
	color_mode: ColorMode,
) -> Data {
	let mut data: Vec<u8> = Vec::new();
	let mut iterations_total: usize = 0;

	for line in table {
		for state in line {
			match state {
				fractals::divergence::State::Divergent{ iterations } => {
					let weight: f64 = iterations as f64 / iterations_max as f64;

					match color_mode {
						ColorMode::HSV => {
							let color: color::Rgb = color::Hsv::new(
								weight * 359.9, 
								1.0 - weight, 
								1.0 - weight,
							).to_rgb();
							data.push(color.red);
							data.push(color.green);
							data.push(color.blue);
						},
						ColorMode::GRAYSCALE => {
							let color: color::Rgb = color::Rgb::new(
								(divergent.red as f64 * weight) as u8,
								(divergent.green as f64 * weight) as u8,
								(divergent.blue as f64 * weight) as u8,
							);
							data.push(color.red);
							data.push(color.green);
							data.push(color.blue);
						}
					}

					iterations_total += iterations;
				},
				fractals::divergence::State::Stable => {
					data.push(stable.red);
					data.push(stable.green);
					data.push(stable.blue);

					iterations_total += iterations_max;
				},
			};
		}
	}

	Data {
		raw_pixels: data,
		iterations_total,
	}
}

/// Convert a 2D `table`: `Vec<Vec<Root>>` into `Vec<u8>` of raw `data`. 
pub fn convert_root_table_to_data(
	table: Vec<Vec<fractals::root::IsRoot>>,
	roots: Vec<complex::Algebraic>,
	threshold: complex::Real,
	no_root_color: color::Rgb,
	iterations_max: usize,
	color_mode: ColorMode,
) -> Data {
	let mut data: Vec<u8> = Vec::new();
	let mut iterations_total: usize = 0;
	let mut loss_counter: usize = 0;

	for line in table {
		for root in line {
			match root {
				fractals::root::IsRoot::No => {
					data.push(no_root_color.red);
					data.push(no_root_color.green);
					data.push(no_root_color.blue);

					iterations_total += iterations_max;
				},
				fractals::root::IsRoot::Yes { 
					root, 
					iterations, 
				} => {
					let root_id: usize = match roots
						.iter()
						.position(|stored_root| {
							stored_root.distance_to_squared(root) <= threshold * threshold
						}) {
							Option::Some(index) => index,
							Option::None => {
								loss_counter += 1;
								0
							},
						};
					
					let weight: f64 = iterations as f64 / iterations_max as f64;
					let root_slider: f64 = root_id as f64 / roots.len() as f64;
					let color: color::Rgb = match color_mode {
						ColorMode::HSV => {
							color::Hsv::new(
								 root_slider * 360.0, 
								1.0 - weight, 
								1.0 - weight,
							).to_rgb()
						},
						ColorMode::GRAYSCALE => {
							color::Grayscale::new(root_slider).to_rgb()
						}
					};

					data.push(color.red);
					data.push(color.green);
					data.push(color.blue);

					iterations_total += iterations;
				},
			}
		}
	}

	if loss_counter > 0 {
		eprintln!("(!) convert_root_table_to_data() `loss_counter` = {} `roots`: {:?}", loss_counter, roots);
	}

	Data {
		raw_pixels: data,
		iterations_total,
	}
}

/// From a `pixel` position, on a screen of `size`, with camera `zoom` and `position`,
/// get the corresponding point in the complex plane.
pub fn position_from_pixel(
	pixel: [complex::Real; 2], 
	size: [complex::Real; 2],
	zoom: complex::Real,
	camera_position: [complex::Real; 2],
) -> [complex::Real; 2] {[
	(pixel[0] - size[0] / 2.0) / zoom - camera_position[0], 
	(pixel[1] - size[1] / 2.0) / zoom - camera_position[1],
]}
