//! # Complex sequences.
//! src/fractals/root/texture.rs
//!
//! Draw the texture for a Newton fractal.

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


/// # `Root`, drawing board for `imgui`.
pub struct Root<F, D> 
where
	F: Fn(complex::Algebraic) -> complex::Algebraic,
	D: Fn(complex::Algebraic) -> complex::Algebraic,
{
	function: F,
	derivative: D,
	texture_id: Option<imgui::TextureId>,
	
	iterations_total: usize,
	generation_time: Option<time::Duration>,

	// Parameters.
	/// Size: [width, height].
	pub size: [complex::Real; 2],
	#[allow(dead_code)]
	pub resolution: u32,
	pub position: [complex::Real; 2],
	pub zoom: complex::Real,
	pub iterations: usize,
	pub threshold: complex::Real,
	pub method_id: usize,

	// Variables to check if state is modified.
	constant_last: complex::Algebraic,
	zoom_last: complex::Real,
	position_last: [complex::Real; 2],
	iterations_last: usize,
	threshold_last: complex::Real,
	method_id_last: usize,

	/// Graphics.
	color_no_root: [u8; 3],

}

impl<F, D> Root<F, D> 
where
	F: Fn(complex::Algebraic) -> complex::Algebraic,
	D: Fn(complex::Algebraic) -> complex::Algebraic,
{
	/// Instantiate and returns a link to a new `Root`.
	pub fn new(
		function: F,
		derivative: D,
		size: [complex::Real; 2],
		position: [complex::Real; 2], 
		resolution: u32,
		zoom: complex::Real,
		iterations: usize,
		threshold: complex::Real,
		method_id: usize,
		color_no_root: [u8; 3],
	) -> rc::Rc<cell::RefCell<Root<F, D>>> {
		rc::Rc::new(cell::RefCell::new(Root {
			function,
			derivative,
			iterations_total: 0usize,
			texture_id: Option::None,
			size, 
			resolution,
			generation_time: Option::None,
			position,
			zoom,
			iterations,
			threshold,
			method_id,

			constant_last: Default::default(),
			zoom_last: 1.0,
			position_last: [0.0, 0.0],
			iterations_last: 0,
			threshold_last: 0.0,
			method_id_last: 0,

			color_no_root,
		}))
	}

	/// Check the fields of the `Root` versus their `last` counterpart.
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
		} else if self.method_id_last != self.method_id {
			updated = true;
			self.method_id_last = self.method_id;
		}

		updated
	}
}

impl<F, D> fractals::textures::Fractal for Root<F, D> 
where
	F: Fn(complex::Algebraic) -> complex::Algebraic,
	D: Fn(complex::Algebraic) -> complex::Algebraic,
{

	fn register_texture<Facade>(
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

		let table = match self.method_id {
			0 => {
				fractals::root::maths::limit_on_screen_root(
					&self.function, 
					&self.derivative, 
					self.iterations, 
					size, 
					self.position, 
					self.zoom
				)
			},
			_ => panic!(
				"(X) fractals::divergence_texture::Divergent::register_texture() `method_id` unknown ({}).", 
				self.method_id
			),
		};

		let data = fractals::textures::convert_root_table_to_data(
			table, 
			self.color_no_root,
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
			},
			Option::Some(id) => {
				textures.replace(id, texture);

				
			}
		}
		
		eprintln!(
			"(?) Root: t={} zoom={} pos=({}; {})", 
			match self.generation_time {
				Option::None => "()",
				Option::Some(elapsed) => &format!("{:?}", elapsed),
			}, 
			self.zoom,
			self.position[0],
			self.position[1],
		);


		Ok(())
	}

	fn show_textures(&self, ui: &imgui::Ui, position: [complex::Real; 2]) {
        ui.window("Fractal 'Root'. ")
            .size(self.size, imgui::Condition::FirstUseEver)
			.position(position, imgui::Condition::FirstUseEver)
            .build(|| {
                ui.text(format!("Fractal 'root' texture: {}", self.method_id));
                
				if let Some(texture_id) = self.texture_id {
					if let Some(generation_time) = self.generation_time 
						&& generation_time.as_millis() != 0
					{
						ui.text(format!(
							"({}; {}) = {} pixels; {} iterations in {:?} => {} iterations/ ms", 
							self.size[0],
							self.size[1],
							self.size[0] * self.size[1],
							self.iterations_total,
							generation_time,
							self.iterations_total as u128 / generation_time.as_millis(),
						));
					} else {
						ui.text(format!("Current fractal (error: no data): "));
					}

					imgui::Image::new(texture_id, self.size)
						.build(ui);
                }
			});
	}
}
