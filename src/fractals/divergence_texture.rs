//! # Complex sequences.
//! src/fractals/divergence_texture.rs

use std::{
	borrow, 
	cell, 
	error, 
	rc, 
	time,
};

use glium::{
	self, backend, texture, uniforms
};
use imgui;
use imgui_glium_renderer;
use complex_rust as complex;

use crate::fractals;


/// # `FractalTexture`, drawing board for `imgui`.
pub struct FractalTexture<F> 
where
	F: Fn(complex::Algebraic, complex::Algebraic) -> complex::Algebraic,
{
	function: F,
	texture_id: Option<imgui::TextureId>,
	
	iterations_total: usize,
	generation_time: Option<time::Duration>,

	// Parameters.
	/// Constant fixed point. E.g: `c` in `f(z) = z * z + c`.
	pub constant: complex::Algebraic,
	/// Size: [width, height].
	pub size: [complex::Real; 2],
	pub resolution: u32,
	pub position: [complex::Real; 2],
	pub zoom: complex::Real,
	pub iterations: usize,
	pub threshold: complex::Real,
	pub grid: bool,
	pub method: fractals::divergence::LimitMethod,

	// Variables to check if state is modified.
	constant_last: complex::Algebraic,
	zoom_last: complex::Real,
	position_last: [complex::Real; 2],
	iterations_last: usize,
	threshold_last: complex::Real,
	grid_last: bool,
	method_last: fractals::divergence::LimitMethod,

	/// Graphics.
	color_stable: [u8; 3],
	color_divergent: [u8; 3],
}

impl<F> FractalTexture<F> 
where
	F: Fn(complex::Algebraic, complex::Algebraic) -> complex::Algebraic + Copy
{
	/// Instantiate and returns a link to a new `FractalTexture`.
	pub fn new(
		function: F,
		constant: complex::Algebraic,
		size: [complex::Real; 2],
		position: [complex::Real; 2], 
		resolution: u32,
		zoom: complex::Real,
		iterations: usize,
		threshold: complex::Real,
		method: fractals::divergence::LimitMethod,
		color_stable: [u8; 3],
		color_divergent: [u8; 3],
	) -> rc::Rc<cell::RefCell<FractalTexture<F>>> {
		rc::Rc::new(cell::RefCell::new(FractalTexture {
			function,
			iterations_total: 0usize,
			texture_id: Option::None,
			constant,
			size, 
			resolution,
			generation_time: Option::None,
			position,
			zoom,
			iterations,
			threshold,
			grid: true,
			method,

			constant_last: Default::default(),
			zoom_last: 1.0,
			position_last: [0.0, 0.0],
			iterations_last: 0,
			threshold_last: 0.0,
			grid_last: true,
			method_last: fractals::divergence::LimitMethod::Mandelbrot,

			color_stable,
			color_divergent,
		}))
	}

	/// Generate and register the fractal texture.
	/// 
	/// Source: `imgui-examples`, `custom_texture`
	pub fn register_texture<Facade>(
        &mut self,
        gl_context: &Facade,
        textures: &mut imgui::Textures<imgui_glium_renderer::Texture>,
    ) -> Result<(), Box<dyn error::Error>>
    where
        Facade: backend::Facade,
    {	
		let size: [usize; 2] = [self.size[0] as usize, self.size[1] as usize];
			
		// Texture generation.
		let generation_start: time::Instant = time::Instant::now();

		let table = match self.method {
			fractals::divergence::LimitMethod::Julia => {
				fractals::divergence::limit_on_screen_julia(
					self.constant, 
					self.function,
					self.threshold, 
					self.iterations, 
					size,
					self.position,
					self.zoom,
					self.grid,
				)
			}
			fractals::divergence::LimitMethod::Mandelbrot => {
				fractals::divergence::limit_on_screen_mandelbrot(
					self.constant, 
					self.function,
					self.threshold, 
					self.iterations, 
					size,
					self.position,
					self.zoom,
					self.grid,
				)
			}
		};

		let data = fractals::textures::convert_state_table_to_data(
			table, 
			self.color_stable,
			self.color_divergent,
			[100, 100, 100],
			self.iterations,
		);

		self.iterations_total = data.iterations_total;
		self.generation_time = Option::Some(generation_start.elapsed());
		
		// Render (from `imgui-examples`, `custom_texture`).
		let raw = texture::RawImage2d {
			data: borrow::Cow::Owned(data.raw_pixels),
			width: size[0] as u32,
			height: size[1] as u32,
			format: texture::ClientFormat::U8U8U8,
		};

		let gl_texture = glium::Texture2d::new(gl_context, raw)?;
		
		let texture = imgui_glium_renderer::Texture {
			texture: rc::Rc::new(gl_texture),
			sampler: uniforms::SamplerBehavior {
				magnify_filter: uniforms::MagnifySamplerFilter::Linear,
				minify_filter: uniforms::MinifySamplerFilter::Linear,
				..Default::default()
			},
		};

		match self.texture_id {
			Option::None => {
				let texture_id = textures.insert(texture);
            	self.texture_id = Some(texture_id);

				eprintln!(
					"(?) fractals::textures::FractalTexture.register_texture() Updated in {}. zoom={}, position=({}; {})", 
					match self.generation_time {
						Option::None => "()",
						Option::Some(elapsed) => &format!("{:?}", elapsed),
					}, 
					self.zoom,
					self.position[0],
					self.position[1],
				);
			},
			Option::Some(id) => {
				textures.replace(id, texture);

				eprintln!(
					"(?) fractals::textures::FractalTexture.register_texture() Updated in {}. zoom={}, position=({}; {})", 
					match self.generation_time {
						Option::None => "()",
						Option::Some(elapsed) => &format!("{:?}", elapsed),
					}, 
					self.zoom,
					self.position[0],
					self.position[1],
				);
			}
		}


		Ok(())
	}

	/// Calls `window` method on `ui`, to display the texture. 
	/// 
	/// Source: `imgui-examples`, `custom_texture`
	pub fn show_textures(&self, ui: &imgui::Ui) {
        ui.window("Fractal. ")
            .size(self.size, imgui::Condition::FirstUseEver)
            .build(|| {
                ui.text(format!("Fractal texture: {}", self.method));
                
				if let Some(texture_id) = self.texture_id {
					if let Some(generation_time) = self.generation_time 
						&& generation_time.as_micros() != 0
					{
						ui.text(format!(
								"({}; {}) = {} pixels; {} iterations in {:?} => {} iterations/ ms", 
								self.size[0],
								self.size[1],
								self.size[0] * self.size[1],
								self.iterations_total,
								generation_time,
								self.iterations_total as u128 / generation_time.as_micros(),
							));
					} else {
						ui.text(format!("Current fractal (error: no data): "));
					}

					imgui::Image::new(texture_id, self.size)
						.build(ui);
                }
			});
	}
	
	/// Check the fields of the `FractalTexture` versus their `last` counterpart.
	/// 
	/// Returns `true` if any of them is different.
	pub fn is_state_updated(self: &mut Self) -> bool {
		let mut updated: bool = false;

		if self.zoom_last != self.zoom {
			updated = true;
			self.zoom_last = self.zoom;
		} else if self.position_last != self.position {
			updated = true;
			self.position_last = self.position;
		} else if self.iterations_last != self.iterations {
			updated = true;
			self.iterations_last = self.iterations;
		} else if self.threshold_last != self.threshold {
			updated = true;
			self.threshold_last = self.threshold;
		} else if self.constant_last != self.constant {
			updated = true;
			self.constant_last = self.constant;
		} else if self.method_last != self.method {
			updated = true;
			self.method_last = self.method;
		}

		updated
	}

}

/// # `State` of the divergent `FractalTexture`.
struct State {
	zoom: complex::Real,
	position: [complex::Real; 2],
	iterations: usize,
	threshold: complex::Real,
	constant: complex::Real,
}

