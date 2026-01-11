//! # Complex sequences.
//! src/fractals/textures.rs

use std::{
	error,
	borrow,
	rc,
	cell,
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
	size: [f32; 2],
	resolution: u32,
	generation_time: Option<time::Duration>,
}

impl FractalTexture {
	/// Instantiate and returns a link to a new `FractalTexture`.
	pub fn new(size: [f32; 2], resolution: u32) -> rc::Rc<cell::RefCell<FractalTexture>> {
		rc::Rc::new(cell::RefCell::new(FractalTexture { 
			texture_id: Option::None, 
			size: size, 
			resolution: resolution,
			generation_time: Option::None,
		}))
	}

	/// Generate and register the fractal texture.
	/// 
	/// Source: `imgui-examples`, `custom_texture`
	pub fn register_texture<F>(
        &mut self,
        gl_ctx: &F,
        textures: &mut imgui::Textures<imgui_glium_renderer::Texture>,
    ) -> Result<(), Box<dyn error::Error>>
    where
        F: backend::Facade,
    {	
		let width: usize = self.size[1] as usize;
		let height: usize = self.size[0] as usize;

        if self.texture_id.is_none() {
			
            // Texture generation.
			let generation_start: time::Instant = time::Instant::now();
			
			let fractal_table_mandelbrot = divergence::limit_of_each_point(
				Default::default(), 
				|z: complex::Algebraic, c: complex::Algebraic| { z * z + c },
				2.0, 
				100, 
				[width, height],
				[width / 2, height / 2],
				0.008,
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
                width: width as u32,
                height: height as u32,
                format: texture::ClientFormat::U8U8U8,
            };

            let gl_texture = glium::Texture2d::new(gl_ctx, raw)?;
            
			let texture = imgui_glium_renderer::Texture {
                texture: rc::Rc::new(gl_texture),
                sampler: uniforms::SamplerBehavior {
                    magnify_filter: uniforms::MagnifySamplerFilter::Linear,
                    minify_filter: uniforms::MinifySamplerFilter::Linear,
                    ..Default::default()
                },
            };
            let texture_id = textures.insert(texture);

            self.texture_id = Some(texture_id);
        };

		Ok(())
	}

	/// Calls `window` method on `ui`, to display the texture. 
	/// 
	/// Source: `imgui-examples`, `custom_texture`
	pub fn show_textures(&self, ui: &imgui::Ui) {
        ui.window("Fractal. ")
            .size([400.0, 400.0], imgui::Condition::FirstUseEver)
            .build(|| {
                ui.text("Fractal texture. ");
                if let Some(my_texture_id) = self.texture_id {
                    ui.text(format!("Current fractal ({:?}): ", self.generation_time));
                    imgui::Image::new(my_texture_id, self.size).build(ui);
                }
			});
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