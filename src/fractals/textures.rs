//! # Complex sequences.
//! src/fractals/textures.rs

use std::error;

use glium; 
use complex_rust as complex;

use crate::{
	fractals,
	gui,
};

pub trait Fractal {
	/// Generate and register the fractal texture.
	/// 
	/// Source: `imgui-examples`, `custom_texture`
	fn register_texture<Facade>(
        &mut self,
        gl_context: &Facade,
        textures: &mut imgui::Textures<imgui_glium_renderer::Texture>,
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
	stable: [u8; 3], 
	divergent: [u8; 3],
	iterations_max: usize,
) -> Data {
	let mut data: Vec<u8> = Vec::new();
	let mut iterations_total: usize = 0;

	for line in table {
		for state in line {
			match state {
				fractals::divergence::State::Divergent{ iterations } => {
					data.push((divergent[0] as usize * iterations / iterations_max) as u8);
					data.push((divergent[1] as usize * iterations / iterations_max) as u8);
					data.push((divergent[2] as usize * iterations / iterations_max) as u8);

					iterations_total += iterations;
				},
				fractals::divergence::State::Stable => {
					data.push(stable[0]);
					data.push(stable[1]);
					data.push(stable[2]);

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

/// Extract, from a `table`, all the unique found roots.
fn extract_unique_roots(
	table: &Vec<Vec<fractals::root::Root>>,
) -> Vec<complex::Algebraic> {
	let mut found: Vec<complex::Algebraic> = Vec::new();

	for line in table {
		for root in line {
			match root {
				fractals::root::Root::Yes { root, .. } => {
					if !found.contains(root) {
						found.push(*root);
					}
				},
				_ => (),
			}
		}
	}

	found
}

/// Convert a 2D `table`: `Vec<Vec<Root>>` into `Vec<u8>` of raw `data`. 
pub fn convert_root_table_to_data(
	table: Vec<Vec<fractals::root::Root>>,
	no_root_color: [u8; 3],
	iterations_max: usize,
) -> Data {
	let mut data: Vec<u8> = Vec::new();
	let mut iterations_total: usize = 0;
	let roots: Vec<complex_rust::Algebraic> = extract_unique_roots(&table);

	for line in table {
		for root in line {
			match root {
				fractals::root::Root::No => {
					data.push(no_root_color[0]);
					data.push(no_root_color[1]);
					data.push(no_root_color[2]);

					iterations_total += iterations_max;
				},
				fractals::root::Root::Yes { 
					root, 
					iterations, 
				} => {
					let root_id = roots
						.iter()
						.position(|stored_root| *stored_root == root)
						.expect(&format!(
							"(X) fractals::textures::convert_root_table_to_data() `root` ({}) has no counter part",
							root,
						));

					let color = gui::color::Hsv::new(
						(root_id / roots.len() * 360) as f64, 
						1.0, 
						1.0
					).to_rgb();


					data.push(color.red);
					data.push(color.green);
					data.push(color.blue);

					iterations_total += iterations;
				},
			}
		}
	}

	Data {
		raw_pixels: data,
		iterations_total,
	}
}