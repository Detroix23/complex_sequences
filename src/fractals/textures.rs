//! # Complex sequences.
//! src/fractals/textures.rs

use std::{
	error,
	borrow,
	rc,
	cell,
};

use glium::{
	self,
	backend,
	texture,
	uniforms,
};
use imgui;
use imgui_glium_renderer;

/// # `FractalTexture`, drawing board for `imgui`.
pub struct FractalTexture {
	texture_id: Option<imgui::TextureId>,
	/// Size: [width, height].
	size: [f32; 2],
	resolution: u32,
}

impl FractalTexture {
	/// Instantiate and returns a link to a new `FractalTexture`.
	pub fn new(size: [f32; 2], resolution: u32) -> rc::Rc<cell::RefCell<FractalTexture>> {
		rc::Rc::new(cell::RefCell::new(FractalTexture { 
			texture_id: Option::None, 
			size: size, 
			resolution: resolution,
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
            let data = _generate_dummy_texture(width, height);

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
                    ui.text("Current fractal: ");
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