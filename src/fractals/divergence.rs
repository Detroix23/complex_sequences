//! # Complex sequences.
//! src/fractals/holomorphism.rs  

use std::fmt;
use complex_rust as complex;

use complex::Shared;

/// # `LimitMethod` for any point of R².
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum LimitMethod {
	Mandelbrot,
	Julia,
}

impl fmt::Display for LimitMethod {
	fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(formatter, "{}", match &self {
			LimitMethod::Julia => "Julia",
			LimitMethod::Mandelbrot => "Mandelbrot",
		})
	}
}

/// # Divergence `State`.
/// Tell if a function explode toward infinity or remains bounded.
pub enum State {
	/// Divergent: in how many `iterations` does it diverged. 
	Divergent{ iterations: usize },
	Stable,
	Grid,
}

/// # `Limit` of `f`. 
/// Compute a recursive sequence `iteration` times, with z0 = `z`:
/// ```math
/// p(0) = z
/// p(n + 1) = f(p(n), c),
/// ```
/// 
/// To find if |z| > `threshold` or remains stable.
/// 
/// `c` remains constant, and `z0` defines the first value taken by the sequence.
///
/// ## Example
/// Mandelbrot with f(z) = z² + c 
fn limit<F>(
	c: complex::Algebraic, 
	z0: complex::Algebraic, 
	f: F, 
	threshold: complex::Real, 
	iterations: usize,
) -> State 
where
	F: Fn(complex::Algebraic, complex::Algebraic) -> complex::Algebraic,
{
	let mut current: complex::Algebraic = z0;
	let mut counter: usize = 0;

	while counter < iterations && current.absolute() <= threshold {
		current = f(current, c);
		counter += 1;
	}

	if current.absolute() <= threshold {
		State::Stable
	} else {
		State::Divergent { iterations: counter }
	}
}

/// # Limit for each in point of screen.
/// Compute the limit for each point in `size` [width, height].
/// 
/// It is Mandelbrot-like, which is:
/// - `z0`: Complex
/// 	- first member of the sequence,
/// 	- remains fixed for any point of R².
/// - `c`: Complex
/// 	- is any point on R² (x + iy).
pub fn limit_on_screen_mandelbrot<F>(
	z0: complex::Algebraic,
	f: F,
	threshold: complex::Real, 
	iterations: usize,
	size: [usize; 2],
	position: [complex::Real; 2],
	zoom: complex::Real,
	grid_enabled: bool,
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
					(x as complex::Real - size[0] as complex::Real / 2.0) / zoom - position[0], 
					(y as complex::Real - size[0] as complex::Real / 2.0) / zoom - position[1],
				),
				z0,
				&f,
				threshold,
				iterations,
			))
		}

		grid.push(line);
	}



	grid
}

/// # Limit for each in point of screen.
/// Compute the limit for each point in `size` [width, height].
/// 
/// It is Julia-like, which is:
/// - `z`: Complex
/// 	- first member of the sequence,
/// 	- is any point of R².
/// - `c`: Complex
/// 	- constant
pub fn limit_on_screen_julia<F>(
	c: complex::Algebraic,
	f: F,
	threshold: complex::Real, 
	iterations: usize,
	size: [usize; 2],
	position: [complex::Real; 2],
	zoom: complex::Real,
	grid_enabled: bool,
) -> Vec<Vec<State>>
where
	F: Fn(complex::Algebraic, complex::Algebraic) -> complex::Algebraic,
{
	let mut grid: Vec<Vec<State>> = Vec::with_capacity(size[0] * size[1]);

	for y in 0..size[1] {
		let mut line: Vec<State> = Vec::with_capacity(size[0]); 
		for x in 0..size[0] {
			line.push(limit(
				c,
				complex::Algebraic::new(
					(x as complex::Real - size[0] as complex::Real / 2.0) / zoom - position[0], 
					(y as complex::Real - size[0] as complex::Real / 2.0) / zoom - position[1],
				),
				&f,
				threshold,
				iterations,
			))
		}

		grid.push(line);
	}



	grid
}
