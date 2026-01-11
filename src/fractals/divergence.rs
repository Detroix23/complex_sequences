//! # Complex sequences.
//! src/fractals/holomorphism.rs  

use complex_rust as complex;

/// # Divergence `State`.
/// Tell if a function explode toward infinity or remains bounded.
pub enum State {
	Divergent,
	Stable
}

/// # `Limit` of `f`. 
/// Compute a recursive sequence `iteration` times, with z0 = `z`:
/// ```math
/// z -> f(z, c)
/// ```
/// 
/// To find if |z| > `threshold` or remains stable.
///
/// ## Example
/// Mandelbrot with f(z) = z^2 + c 
pub fn limit<F>(
	c: complex::Algebraic, 
	z: complex::Algebraic, 
	f: F, 
	threshold: complex::Number, 
	iterations: usize,
) -> State 
where
	F: Fn(complex::Algebraic, complex::Algebraic) -> complex::Algebraic,
{
	let mut current: complex::Algebraic = z;
	let mut counter: usize = 0;

	while counter < iterations && current.absolute() <= threshold {
		current = f(current, c);
		counter += 1;
	}

	if current.absolute() <= threshold {
		State::Stable
	} else {
		State::Divergent
	}
}

/// Compute the limit for each point in `size` [width, height].
pub fn limit_of_each_point<F>(
	z: complex::Algebraic,
	f: F,
	threshold: complex::Number, 
	iterations: usize,
	size: [usize; 2],
	position: [usize; 2],
	zoom: complex::Number,
) -> Vec<Vec<State>>
where
	F: Fn(complex::Algebraic, complex::Algebraic) -> complex::Algebraic,
{
	let mut grid: Vec<Vec<State>> = Vec::with_capacity(size[0] * size[1]);

	for y in 0..size[1] {
		let mut line: Vec<State> = Vec::with_capacity(size[0]); 
		for x in 0..size[0] {
			line.push(limit(
				complex::Algebraic::new(
					(x as complex::Number - (position[0]) as complex::Number) * zoom, 
					(y as complex::Number - (position[1]) as complex::Number) * zoom,
				),
				z,
				&f,
				threshold,
				iterations,
			))
		}

		grid.push(line);
	}



	grid
}

pub fn convert_state_table_to_data(
	table: Vec<Vec<State>>, 
	stable: [u8; 3], 
	divergent: [u8; 3],
) -> Vec<u8> {
	let mut data: Vec<u8> = Vec::new();

	for line in table {
		for state in line {
			match state {
				State::Divergent => {
					data.push(divergent[0]);
					data.push(divergent[1]);
					data.push(divergent[2]);
				},
				State::Stable => {
					data.push(stable[0]);
					data.push(stable[1]);
					data.push(stable[2]);
				},
			};
		}
	}

	data
}