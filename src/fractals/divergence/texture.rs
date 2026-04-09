//! # Complex sequences.
//! src/fractals/divergence/texture.rs
//! 
//! Draw fractals according the limit of sequences.
//! - Mandelbrot,
//! - Julia.

use std::{cell, error, rc, time};

use glium::{self, backend};
use imgui;
use imgui_glium_renderer;
use complex_rust as complex;

use crate::{fractals, gui};
use crate::support::rendering;


/// # `Divergent`, drawing board for `imgui`.
pub struct Divergent<F> 
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
	pub size: [u32; 2],
	pub information_size: [complex::Real; 2],
	#[allow(dead_code)]
	pub scale: f32,
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
	scale_last: f32,

	/// Graphics.
	color_stable: gui::color::Rgb,
	color_divergent: gui::color::Rgb,
}

impl<F> Divergent<F> 
where
	F: Fn(complex::Algebraic, complex::Algebraic) -> complex::Algebraic + Copy
{
	/// Instantiate and returns a link to a new `Divergent`.
	pub fn new(
		function: F,
		constant: complex::Algebraic,
		information_size: [complex::Real; 2],
		position: [complex::Real; 2], 
		scale: f32,
		zoom: complex::Real,
		iterations: usize,
		threshold: complex::Real,
		method_id: usize,
		color_stable: gui::color::Rgb,
		color_divergent: gui::color::Rgb,
	) -> rc::Rc<cell::RefCell<Divergent<F>>> {
		rc::Rc::new(cell::RefCell::new(Divergent {
			function,
			iterations_total: 0usize,
			texture_id: Option::None,
			constant,
			size: [0, 0], 
			information_size,
			scale,
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
			scale_last: 1.0,

			color_stable,
			color_divergent,
		}))
	}

	/// Check the fields of the `Divergent` versus their `last` counterpart.
	/// 
	/// Returns `true` if any of them is different.
	pub fn is_state_updated(self: &mut Self) -> bool {
		let mut updated: bool = true;

		if self.zoom_last != self.zoom {
			self.zoom_last = self.zoom;
		} else if self.position_last != self.position {
			self.position_last = self.position;
		} else if self.iterations_last != self.iterations {
			self.iterations_last = self.iterations;
		} else if self.threshold_last != self.threshold {
			self.threshold_last = self.threshold;
		} else if self.constant_last != self.constant {
			self.constant_last = self.constant;
		} else if self.method_id_last != self.method_id {
			self.method_id_last = self.method_id;
		} else if self.scale_last != self.scale {
			self.scale_last = self.scale
		} else {	
			updated = false;
		}

		updated
	}
}

impl<F> fractals::textures::Fractal for Divergent<F> 
where
	F: Fn(complex::Algebraic, complex::Algebraic) -> complex::Algebraic + Clone,
{
	fn update_size(self: &mut Self, new_size: [u32; 2]) -> () {
		self.size = new_size
	}

	fn register_texture<Facade>(
        &mut self,
        gl_context: &Facade,
        textures: &mut imgui::Textures<imgui_glium_renderer::Texture>,
		color_mode: fractals::textures::ColorMode,
    ) -> Result<(), Box<dyn error::Error>>
    where
        Facade: backend::Facade,
    {	
		let scaled_size: [usize; 2] = [
			(self.size[0] as f32 / self.scale) as usize, 
			(self.size[1] as f32 / self.scale) as usize
		];
			
		// Texture generation.
		let generation_start: time::Instant = time::Instant::now();

		let table: Vec<Vec<fractals::divergence::State>> = match self.method_id {
			0 => fractals::divergence::maths::limit_on_screen_mandelbrot(
				self.constant, 
				self.function.clone(),
				self.threshold, 
				self.iterations, 
				scaled_size,
				self.position,
				self.zoom / self.scale,
			),
			1 => fractals::divergence::maths::limit_on_screen_julia(
				self.constant, 
				self.function.clone(),
				self.threshold, 
				self.iterations, 
				scaled_size,
				self.position,
				self.zoom / self.scale,
			),
			_ => panic!("(X) fractals::divergence_texture::Divergent::register_texture() `method_id` unknown ({}).", self.method_id),
		};

		let data: fractals::textures::Data = fractals::textures::convert_state_table_to_data(
			table, 
			self.color_stable,
			self.color_divergent,
			self.iterations,
			color_mode,
		);

		self.iterations_total = data.iterations_total;
		self.generation_time = Option::Some(generation_start.elapsed());
		
		// eprintln!(
		// 	"\r(?) Divergent {} Rendering: zoom={} pos=({}; {})", 
		// 	self.method_id,
		// 	self.zoom,
		// 	self.position[0],
		// 	self.position[1],
		// );

		self.texture_id = Option::Some(rendering::render_texture(
			self.texture_id, 
			data.raw_pixels, 
			scaled_size, 
			gl_context, 
			textures,
			rendering::ColorFormat::RGB,
		).expect("(X) fractals::divergence::texture::Divergent::register_texture() render_texture error."));

		eprintln!(
			"\r* Divergent {}: t={} zoom={} pos=({}; {})", 
			self.method_id,
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

	/// Display the divergent fractal render and rendering information.
	fn show_textures(&self, ui: &imgui::Ui, position: [complex::Real; 2]) {
        let draw_list_background: imgui::DrawListMut<'_> = ui.get_background_draw_list();

		// Render `Image` in the draw list.
		if let Some(texture_id) = self.texture_id {
			draw_list_background
				.add_image(texture_id, [0.0, 0.0], [self.size[0] as f32, self.size[1] as f32])
				.build();
		}

		ui.window(format!("Rendering: fractal 'Divergent' (method {}). ", self.method_id))
            .size(self.information_size, imgui::Condition::FirstUseEver)
			.position(position, imgui::Condition::FirstUseEver)
            .build(|| {
				if let Some(generation_time) = self.generation_time 
					&& generation_time.as_millis() != 0
				{
					ui.text(format!("Size: ({}; {});
Pixels = {:.0}; 
Iterations = {};
Time = {}ms;
Speed = {} iterations/ ms;", 
						self.size[0],
						self.size[1],
						(self.size[0] * self.size[1]) as f32 / self.scale,
						self.iterations_total,
						generation_time.as_millis(),
						self.iterations_total as u128 / generation_time.as_millis(),
					));
				} else {
					ui.text(format!("(!) Error: no data."));
				}

				// Classic `Image` rendering method.
				// imgui::Image::new(texture_id, self.size).build(ui);
			
			});
	}
}
