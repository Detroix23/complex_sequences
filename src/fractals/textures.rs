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
pub struct FractalTexture {
	texture_id: Option<imgui::TextureId>,
	/// Size: [width, height].
	size: [complex::Real; 2],
	resolution: u32,
	generation_time: Option<time::Duration>,
	position: [complex::Real; 2],
	pub zoom: complex::Real,
}

impl FractalTexture {
	/// Instantiate and returns a link to a new `FractalTexture`.
	pub fn new(size: [complex::Real; 2], resolution: u32) -> rc::Rc<cell::RefCell<FractalTexture>> {
		rc::Rc::new(cell::RefCell::new(FractalTexture { 
			texture_id: Option::None, 
			size: size, 
			resolution: resolution,
			generation_time: Option::None,
			position: [size[0] / 2.0, size[1] / 2.0],
			zoom: 0.08,
		}))
	}

	/// Generate and register the fractal texture.
	/// 
	/// Source: `imgui-examples`, `custom_texture`
	pub fn register_texture<F>(
        &mut self,
        gl_context: &F,
        textures: &mut imgui::Textures<imgui_glium_renderer::Texture>,
    ) -> Result<(), Box<dyn error::Error>>
    where
        F: backend::Facade,
    {	
		let size: [usize; 2] = [self.size[0] as usize, self.size[1] as usize];
		let position: [usize; 2] = [self.position[0] as usize, self.position[1] as usize];
			
		// Texture generation.
		let generation_start: time::Instant = time::Instant::now();
		
		let fractal_table_mandelbrot = divergence::limit_of_each_point(
			Default::default(), 
			|z: complex::Algebraic, c: complex::Algebraic| { z * z + c },
			2.0, 
			100, 
			size,
			position,
			self.zoom,
		);

		let data = divergence::convert_state_table_to_data(
			fractal_table_mandelbrot, 
			[0, 0, 0], 
			[255, 255, 255],
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
                    ui.text(format!("Current fractal (in {:?}): ", self.generation_time));
                    
					imgui::Image::new(my_texture_id, self.size)
						.build(ui);
                }
			});
	}

	/// Update the size. If `self.size` and `new` differ, update the texture.
	pub fn update_size<F>(
		self: &mut Self, 
		new: &[complex::Real; 2],
		gl_context: &F,
        textures: &mut imgui::Textures<imgui_glium_renderer::Texture>,
    ) -> Result<(), Box<dyn error::Error>>
    where
        F: backend::Facade,
	{
		if self.size[0] != new[0] && self.size[1] != new[1] {
			self.size = *new;
			self.register_texture(gl_context, textures)
		} else {
			Result::Ok(())
		}
	}

	/// Update the size. If `self.size` and `new` differ, update the texture.
	pub fn update_zoom<F>(
		self: &mut Self, 
		new: &complex::Real,
		gl_context: &F,
        textures: &mut imgui::Textures<imgui_glium_renderer::Texture>,
    ) -> Result<(), Box<dyn error::Error>>
    where
        F: backend::Facade,
	{
		eprintln!("(?) fractals::textures::FractalTexture.update_zoom() zoom={}, new={}",  self.zoom, new);

		if self.zoom != *new {
			self.zoom = *new;
			self.register_texture(gl_context, textures)
		} else {
			Result::Ok(())
		}
	}

}

/// Generate a blue screen with a red square 1/ 4 at the bottom right corner.
fn _generate_dummy_texture(width: usize, height: usize) -> Vec<u8> {
	let mut data: Vec<u8> = Vec::with_capacity(width * height);

	for y in 0..height {
		for x in 0..width {
			if x > (width / 2) && y > (height / 2) {
				data.push(255);
				data.push(0);
				data.push(0);
			} else {
				data.push(0);
				data.push(0);
				data.push(255);
			}
		}
	}

	data
}