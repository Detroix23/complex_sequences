//! # Complex sequences.
//! src/fractals/textures.rs

use std::{
	borrow, 
	cell, 
	error, 
	rc, 
	time,
};

use glium::{
	self,
	backend,
	texture,
	uniforms,
};
use imgui;
use imgui_glium_renderer;

use complex_rust as complex;


use crate::fractals::divergence;

/// # `FractalTexture`, drawing board for `imgui`.
pub struct FractalTexture<F> 
where
	F: Fn(complex::Algebraic, complex::Algebraic) -> complex::Algebraic,
{
	function: F,
	texture_id: Option<imgui::TextureId>,
	
	/// Size: [width, height].
	generation_time: Option<time::Duration>,

	/// Parameters.
	pub size: [complex::Real; 2],
	pub resolution: u32,
	pub position: [complex::Real; 2],
	pub zoom: complex::Real,
	pub iterations: usize,
	pub threshold: complex::Real,

	// Variables to check if state is modified.
	zoom_last: complex::Real,
	position_last: [complex::Real; 2],
	iterations_last: usize,
	threshold_last: complex::Real,

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
		size: [complex::Real; 2], 
		resolution: u32,
		zoom: complex::Real,
		iterations: usize,
		threshold: complex::Real,
		color_stable: [u8; 3],
		color_divergent: [u8; 3],
	) -> rc::Rc<cell::RefCell<FractalTexture<F>>> {
		rc::Rc::new(cell::RefCell::new(FractalTexture {
			function,
			texture_id: Option::None, 
			size, 
			resolution,
			generation_time: Option::None,
			position: [size[0] / 2.0, size[1] / 2.0],
			zoom,
			iterations,
			threshold,

			zoom_last: 1.0,
			position_last: [0.0, 0.0],
			iterations_last: 0,
			threshold_last: 0.0,

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

		let fractal_table_mandelbrot = divergence::limit_of_each_point(
			Default::default(), 
			self.function,
			self.threshold, 
			self.iterations, 
			size,
			self.position,
			self.zoom,
		);

		let data = divergence::convert_state_table_to_data(
			fractal_table_mandelbrot, 
			self.color_stable,
			self.color_divergent,
			self.iterations,
		);

		self.generation_time = Option::Some(generation_start.elapsed());
		
		// Render (from `imgui-examples`, `custom_texture`).
		let raw = texture::RawImage2d {
			data: borrow::Cow::Owned(data),
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

				// eprintln!("(?) fractals::textures::FractalTexture.register_texture() First in {:?}. zoom={}", self.generation_time, self.zoom);
			},
			Option::Some(id) => {
				textures.replace(id, texture);

				// eprintln!("(?) fractals::textures::FractalTexture.register_texture() Updated in {:?}. zoom={}", self.generation_time, self.zoom);
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
                ui.text("Fractal texture. ");
                
				if let Some(my_texture_id) = self.texture_id {
                    match self.generation_time {
						Option::None => {
							ui.text(format!("Current fractal: "));
						},
						Option::Some(generation_time) => {
							ui.text(format!("Current fractal (in {:?}): ", generation_time));
						}
					}
                    
					imgui::Image::new(my_texture_id, self.size)
						.build(ui);
                }
			});
	}
	
	/// Check the fields of the `FractalTexture` versus their `last` counterpart.
	/// 
	/// Returns `true` if any of them is different.
	pub fn is_state_updated(self: &mut Self) -> bool {
		let mut updated: bool = false;

		if self.zoom != self.zoom_last {
			updated = true;
			self.zoom_last = self.zoom;
		} else if self.position != self.position_last {
			updated = true;
			self.position_last = self.position;
		} else if self.iterations != self.iterations_last {
			updated = true;
			self.iterations_last = self.iterations;
		} else if self.threshold != self.threshold_last {
			updated = true;
			self.threshold_last = self.threshold;
		}

		updated
	}

}
