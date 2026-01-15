//! # Complex sequences.
//! src/fractals/textures.rs

use crate::fractals::divergence;

/// # `Data` from holomorphic computations.
pub struct Data {
	pub raw_pixels: Vec<u8>,
	pub iterations_total: usize,
}

/// Convert a 2D `table`: `Vec<Vec<State>>` into `Vec<u8>` of raw `data`. 
pub fn convert_state_table_to_data(
	table: Vec<Vec<divergence::State>>, 
	stable: [u8; 3], 
	divergent: [u8; 3],
	grid: [u8; 3],
	iterations_max: usize,
) -> Data {
	let mut data: Vec<u8> = Vec::new();
	let mut iterations_total: usize = 0;

	for line in table {
		for state in line {
			match state {
				divergence::State::Divergent{ iterations } => {
					data.push((divergent[0] as usize * iterations / iterations_max) as u8);
					data.push((divergent[1] as usize * iterations / iterations_max) as u8);
					data.push((divergent[2] as usize * iterations / iterations_max) as u8);

					iterations_total += iterations;
				},
				divergence::State::Stable => {
					data.push(stable[0]);
					data.push(stable[1]);
					data.push(stable[2]);

					iterations_total += iterations_max;
				},
				divergence::State::Grid => {
					data.push(grid[0]);
					data.push(grid[1]);
					data.push(grid[2]);			

					iterations_total += 1;
				}
			};
		}
	}

	Data {
		raw_pixels: data,
		iterations_total,
	}
}

