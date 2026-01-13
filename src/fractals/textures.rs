//! # Complex sequences.
//! src/fractals/textures.rs


/// # Divergence `State`.
/// Tell if a function explode toward infinity or remains bounded.
pub enum State {
	/// Divergent: in how many `iterations` does it diverged. 
	Divergent{ iterations: usize },
	Stable,
	Grid,
}

/// Convert a 2D `table`: `Vec<Vec<State>>` into `Vec<u8>` of raw `data`. 
pub fn convert_state_table_to_data(
	table: Vec<Vec<State>>, 
	stable: [u8; 3], 
	divergent: [u8; 3],
	grid: [u8; 3],
	iterations_max: usize,
) -> Vec<u8> {
	let mut data: Vec<u8> = Vec::new();

	for line in table {
		for state in line {
			match state {
				State::Divergent{ iterations} => {

					data.push((divergent[0] as usize * iterations / iterations_max) as u8);
					data.push((divergent[1] as usize * iterations / iterations_max) as u8);
					data.push((divergent[2] as usize * iterations / iterations_max) as u8);
				},
				State::Stable => {
					data.push(stable[0]);
					data.push(stable[1]);
					data.push(stable[2]);
				},
				State::Grid => {
					data.push(grid[0]);
					data.push(grid[1]);
					data.push(grid[2]);					
				}
			};
		}
	}

	data
}

