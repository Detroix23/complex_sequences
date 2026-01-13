//! # Complex sequences.
//! src/fractals/holomorphism.rs  

use complex_rust as complex;

use crate::fractals;

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
	threshold: complex::Real, 
	iterations: usize,
) -> fractals::textures::State 
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
		fractals::textures::State::Stable
	} else {
		fractals::textures::State::Divergent { iterations: counter }
	}
}

/// Compute the limit for each point in `size` [width, height].
pub fn limit_of_each_point<F>(
	z: complex::Algebraic,
	f: F,
	threshold: complex::Real, 
	iterations: usize,
	size: [usize; 2],
	position: [complex::Real; 2],
	zoom: complex::Real,
	grid_enabled: bool,
) -> Vec<Vec<fractals::textures::State>>
where
	F: Fn(complex::Algebraic, complex::Algebraic) -> complex::Algebraic,
{
	let mut grid: Vec<Vec<fractals::textures::State>> = Vec::with_capacity(size[0] * size[1]);

	for y in 0..size[1] {
		let mut line: Vec<fractals::textures::State> = Vec::with_capacity(size[0]); 
		for x in 0..size[0] {
			if grid_enabled && (x == size[0] / 2 || y == size[1] / 2) {
				line.push(fractals::textures::State::Grid);
			} else {
				line.push(limit(
					complex::Algebraic::new(
						(x as complex::Real - size[0] as complex::Real / 2.0) / zoom - position[0], 
						(y as complex::Real - size[0] as complex::Real / 2.0) / zoom - position[1],
					),
					z,
					&f,
					threshold,
					iterations,
				))
			}
		}

		grid.push(line);
	}



	grid
}
