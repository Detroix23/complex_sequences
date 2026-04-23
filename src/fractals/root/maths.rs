//! # Complex sequences.
//! src/fractals/root/maths.rs
//! 
//! Try to find roots of a complex function.

use complex;
use complex::{Complex, ToComplex};

use crate::structures::computations;
use crate::fractals::{geometry};

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
		let mut reference: complex::Algebraic = root;

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
	fn newton_method(self: &mut Self, z0: complex::Algebraic) -> computations::IsRoot {
		let mut z: complex::Algebraic = z0;
		let mut count: usize = 0;
		let mut current_fz: complex::Algebraic = (self.function)(z);
		let mut current_dz: complex::Algebraic;

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
			let reference: complex::Algebraic = self.append_root(z);
			computations::IsRoot::Yes{ root: reference, iterations: count }
		} else {
			computations::IsRoot::No
		}
	}

	/// # Path of each in point of screen.
	/// *Single threaded*.
	/// 
	/// Compute the limit for each point in `size` [width, height].
	/// 
	/// It is Newton's like, which is:
	/// - `z0` is `pixel.x + i*pixel.y`,
	/// - returns a 2D table of "arrivals" `Vec<Vec<IsRoot>>`, coordinates of the root reached.
	pub fn limit_on_screen_newton(self: &mut Self) -> Vec<Vec<computations::IsRoot>> {
		let mut grid: Vec<Vec<computations::IsRoot>> = Vec::with_capacity(self.size[0] * self.size[1]);

		for y in 0..self.size[1] {
			let mut line: Vec<computations::IsRoot> = Vec::with_capacity(self.size[0]); 
			
			for x in 0..self.size[0] {
				let complex_position: [complex::Real; 2] = geometry::position_from_pixel(
					[x as complex::Real, y as complex::Real], 
					[self.size[0] as complex::Real, self.size[1] as complex::Real], 
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

	/// # Color the result of each point through `function`.
	/// *Single threaded.*
	/// 
	/// Use `Polar` complex representation:
	/// - distance to origin is brightness (full white is 0);
	/// - angle (theta, argument) is the color on the HSV wheel.
	pub fn limit_on_screen_position(self: &Self) -> Vec<Vec<complex::Polar>> {
		let mut grid: Vec<Vec<complex::Polar>> = Vec::with_capacity(self.size[0] * self.size[1]);

		for y in 0..self.size[1] {
			let mut line: Vec<complex::Polar> = Vec::with_capacity(self.size[0]); 
			
			for x in 0..self.size[0] {
				let complex_position: [complex::Real; 2] = geometry::position_from_pixel(
					[x as complex::Real, y as complex::Real], 
					[self.size[0] as complex::Real, self.size[1] as complex::Real], 
					self.zoom, 
					self.position
				);
				let z = complex::Algebraic::new(
					complex_position[0], 
					complex_position[1]
				);

				line.push((self.function)(z).to_polar())
			}

			grid.push(line);
		}

		grid
	}
}