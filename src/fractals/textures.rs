//! # Complex sequences.
//! src/fractals/textures.rs

use std::{f32, error, fmt, convert};

use glium; 
use complex::{self, Complex};

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

pub struct NewtonConverter {
	roots: Vec<complex::Algebraic>,
	threshold: complex::Real,
	no_root_color: color::Rgb,
	iterations_max: usize,
	color_mode: ColorMode,
	data: Vec<u8>,
	iterations_total: usize,
	loss_counter: usize,
}

/// # `NewtonConverter`: from `Vec<Vec<IsRoot>>` to `Vec<u8>`.
impl NewtonConverter {
	pub fn new(
		roots: Vec<complex::Algebraic>,
		threshold: complex::Real,
		no_root_color: color::Rgb,
		iterations_max: usize,
		color_mode: ColorMode,
	) -> NewtonConverter {
		NewtonConverter {
			roots,
			threshold,
			no_root_color,
			iterations_max,
			color_mode,
			data: Vec::new(),
			iterations_total: 0,
			loss_counter: 0,
		}
	}

	/// For one given `root`, push the correct color components and update `iterations`.
	fn match_root(self: &mut Self, root: fractals::root::IsRoot) -> () {
		match root {
			fractals::root::IsRoot::No => {
				self.data.push(self.no_root_color.red);
				self.data.push(self.no_root_color.green);
				self.data.push(self.no_root_color.blue);

				self.iterations_total += self.iterations_max;
			},
			fractals::root::IsRoot::Yes { 
				root, 
				iterations, 
			} => {
				let root_id: usize = match self.roots
					.iter()
					.position(|stored_root| {
						stored_root.distance_to_squared(root) <= self.threshold * self.threshold
					}) {
						Option::Some(index) => index,
						Option::None => {
							self.loss_counter += 1;
							0
						},
					};
				
				let weight: f64 = iterations as f64 / self.iterations_max as f64;
				let root_slider: f64 = root_id as f64 / self.roots.len() as f64;
				let color: color::Rgb = match self.color_mode {
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

				self.data.push(color.red);
				self.data.push(color.green);
				self.data.push(color.blue);

				self.iterations_total += iterations;
			},
		}
	}

	/// Convert a 2D `table`: `Vec<Vec<Root>>` into `Vec<u8>` of raw `data`. 
	pub fn convert(
		self: &mut Self, 
		table: Vec<Vec<fractals::root::IsRoot>>
	) -> Data {
		for line in table {
			for root in line {
				self.match_root(root);
			}
		}

		if self.loss_counter > 0 {
			eprintln!(
				"(!) convert_root_table_to_data() `loss_counter` = {} `roots`: {:?}", 
				self.loss_counter, self.roots
			);
		}

		Data {
			raw_pixels: self.data.clone(),
			iterations_total: self.iterations_total,
		}
	}
}

/// From a `pixel` position, on a screen of `size`, with camera `zoom` and `position`,
/// get the corresponding point in the complex plane.
pub fn position_from_pixel(
	pixel: [complex::Real; 2], 
	size: [complex::Real; 2],
	zoom: complex::Real,
	camera_position: [complex::Real; 2],
) -> [complex::Real; 2] {
	[
		(pixel[0] - size[0] / 2.0) / zoom - camera_position[0], 
		(pixel[1] - size[1] / 2.0) / zoom - camera_position[1],
	]
}

/// # `PositionConverter`: from `Vec<Vec<Polar>>` to `Vec<u8>`.
pub struct PositionConverter {
	data: Vec<u8>,
	iterations_total: usize,
	degree0: f32,
}

impl PositionConverter {
	pub fn new(degree0: f32) -> PositionConverter {
		PositionConverter { 
			data: Vec::new(), 
			iterations_total: 0,
			degree0, 
		}
	} 

	/// From a point `z`, return a `Rgb` color matching.
	/// 
	/// `angle`: Angle **d** shifted with `degree0` in degrees.
	/// We have:
	/// ```maths, ignore
	/// θ ∈ [-π; π]
	/// d = (θ + π)  * (360 / 2π)
	/// ```
	fn color_for_point(self: &Self, z: complex::Polar) -> color::Rgb {
		const PI: f64 = std::f64::consts::PI;
		let absolute: f64 = z.absolute() as f64;
		let angle: f64 = (z.argument() as f64 + PI) * 359.9 / (2.0 * PI)
			.max(0.0)
			.min(359.9);
	
		color::Hsv::new(
			angle,
			1.0,
			1.0 / (absolute + 1.0),
		).to_rgb()
	}

	pub fn convert(self: &mut Self, table: Vec<Vec<complex::Polar>>) -> Data {
		for line in table {
			for z in line {
				let color: color::Rgb = self.color_for_point(z);
				
				self.iterations_total += 1;
				self.data.push(color.red);
				self.data.push(color.green);
				self.data.push(color.blue);
			}
		}

		Data {
			iterations_total: self.iterations_total,
			raw_pixels: self.data.clone(),
		}
	}
}
