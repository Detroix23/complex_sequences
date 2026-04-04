//! # Complex sequences.
//! src/fractals/root/maths.rs
//! 
//! Try to find roots of a complex function.

use std::{
	convert, fmt, io::Cursor
};

use complex_rust::{self as complex, Shared};

use crate::fractals::textures;

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

/// # `RootFinder`.
/// Build and store the roots of a polynomial.  
pub struct RootFinder<F, D> 
where
	F: Fn(complex::Algebraic) -> complex::Algebraic,
	D: Fn(complex::Algebraic) -> complex::Algebraic,
{
	/// Function (mostly polynomial) we search the root.
	function: F,
	/// Derivate of the `function` we search the root.
	derivative: D,
	threshold: complex::Real,
	iterations: usize,
	size: [usize; 2],
	position: [complex::Real; 2],
	zoom: complex::Real,
	roots: Vec<complex::Algebraic>,

}

impl<F, D> RootFinder<F, D>
where
	F: Fn(complex::Algebraic) -> complex::Algebraic,
	D: Fn(complex::Algebraic) -> complex::Algebraic,
{
	pub fn new(
		function: F,
		derivative: D,
		threshold: complex::Real,
		iterations: usize,
		size: [usize; 2],
		position: [complex::Real; 2],
		zoom: complex::Real,
	) -> RootFinder<F, D> {
		RootFinder { 
			function,
			derivative,
			threshold, 
			iterations,
			size,
			position,
			zoom,
			roots: Vec::new(), 
		}
	}

	pub fn get_roots(self: &Self) -> Vec<complex::Algebraic> {
		self.roots.clone()
	}

	pub fn get_threshold(self: &Self) -> complex::Real {
		self.threshold
	}

	/// Add a root to `roots` if the given `root` doesn't already exist.
	/// 
	/// Return the root reference.
	pub fn append_root(self: &mut Self, root: complex::Algebraic) -> complex::Algebraic {
		let mut reference: complex_rust::Algebraic = root;

		for known in &self.roots {
			if known.distance_to_squared(root) < self.threshold * self.threshold {
				reference = *known;
				break;
			}
		}

		if reference == root {
			self.roots.push(reference);
		}

		reference
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
	fn newton_method(self: &mut Self, z0: complex::Algebraic) -> IsRoot {
		let mut z: complex::Algebraic = z0;
		let mut count: usize = 0;
		let mut current_fz: complex_rust::Algebraic = (self.function)(z);
		let mut current_dz: complex_rust::Algebraic;

		while count < self.iterations 
			&& current_fz.absolute_squared() > self.threshold * self.threshold 	
		{
			current_fz = (self.function)(z);
			current_dz = (self.derivative)(z);
			z = z - current_fz / current_dz;
			count += 1;
		} 

		current_fz = (self.function)(z);

		if current_fz.absolute_squared() <= self.threshold * self.threshold {
			let reference: complex_rust::Algebraic = self.append_root(z);
			IsRoot::Yes{ root: reference, iterations: count }
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
	pub fn limit_on_screen(self: &mut Self) -> Vec<Vec<IsRoot>> {
		let mut grid: Vec<Vec<IsRoot>> = Vec::with_capacity(self.size[0] * self.size[1]);

		for y in 0..self.size[1] {
			let mut line: Vec<IsRoot> = Vec::with_capacity(self.size[0]); 
			
			for x in 0..self.size[0] {
				let complex_position: [f32; 2] = textures::position_from_pixel(
					[x as f32, y as f32], 
					[self.size[0] as f32, self.size[1] as f32], 
					self.zoom, 
					self.position
				);
				line.push(self.newton_method(complex::Algebraic::new(
					complex_position[0], 
					complex_position[1]
				)));
			}

			grid.push(line);
		}

		grid
	}
}