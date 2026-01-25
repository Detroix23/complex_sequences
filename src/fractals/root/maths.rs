//! # Complex sequences.
//! src/fractals/root/maths.rs
//! 
//! Try to find roots of a complex function.

use std::{
	fmt,
	convert,
};

use complex_rust as complex;

/// # `IsRoot`.
/// Define a possible root.
/// - `No`: no root,
/// - `Yes`: there is root, and the root is `root`: complex::Algebraic.
pub enum IsRoot {
	No,
	Yes { 
		root: complex::Algebraic,
		iterations: usize,
	},
}

/// # `RootMethod`.
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum RootMethod {
	Newton,
}

impl RootMethod {
	/// Return a `Vec` of all the methods.
	/// ```rust, no_run
	/// 1. Newton,
	/// ```
	pub fn list() -> Vec<RootMethod> {
		vec![
			RootMethod::Newton,
		]
	}

	/// Return a `&'static str` representation of `RootMethod`, with its id.
	fn to_static_str(self: &Self) -> &'static str {
		match &self {
			RootMethod::Newton => "1. Newton",
		}
	}
}

impl fmt::Display for RootMethod {
	fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(formatter, "{}", match &self {
			RootMethod::Newton => "1. Newton",
		})
	}
}

impl convert::AsRef<str> for RootMethod {
	fn as_ref(self: &Self) -> &str {
		&self.to_static_str()
	}
}

/// # Newton's method.
/// Try to find a root of `function`:
/// - starting from `z0` complex,
/// - holomorphic dynamics with the sequence:
/// ```math
/// u(0) = z0
/// u(n + 1) = u(n) - T(u(n))
/// 
/// T(z) = f(z) / f'(z) 
/// ```
/// cf. Desmos: https://www.desmos.com/calculator/dhirelyn0y
fn newton_method<F, D>(
	z0: complex::Algebraic,
	function: F,
	derivative: D,
	iterations: usize,
) -> IsRoot
where
	F: Fn(complex::Algebraic) -> complex::Algebraic,
	D: Fn(complex::Algebraic) -> complex::Algebraic,
{
	let mut z: complex::Algebraic = z0;
	let mut count: usize = 0;

	while count < iterations {
		z = z - function(z) / derivative(z);
		count += 1;
	} 

	if function(z) == complex::Algebraic::default() {
		IsRoot::Yes{ root: z, iterations: count }
	} else {
		IsRoot::No
	}
}

/// # Path of each in point of screen.
/// Compute the limit for each point in `size` [width, height].
/// 
/// It is Newton's like, which is:
/// - z0 is pixel.x + i*pixel.y,
/// - returns a `Vec<IsRoot>`, coordinates of the root reached.
pub fn limit_on_screen_root<F, D>(
	function: F,
	derivative: D,
	iterations: usize,
	size: [usize; 2],
	position: [complex::Real; 2],
	zoom: complex::Real,
) -> Vec<Vec<IsRoot>>
where
	F: Fn(complex::Algebraic) -> complex::Algebraic,
	D: Fn(complex::Algebraic) -> complex::Algebraic,
{
	let mut grid: Vec<Vec<IsRoot>> = Vec::with_capacity(size[0] * size[1]);

	for y in 0..size[1] {
		let mut line: Vec<IsRoot> = Vec::with_capacity(size[0]); 
		
		for x in 0..size[0] {
			line.push(newton_method(
				complex::Algebraic::new(
					(x as complex::Real - size[0] as complex::Real / 2.0) / zoom - position[0], 
					(y as complex::Real - size[0] as complex::Real / 2.0) / zoom - position[1],
				), 
				&function, 
				&derivative, 
				iterations
			));
		}

		grid.push(line);
	}

	grid
}
