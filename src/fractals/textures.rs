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
struct FractalTexture {
	texture_id: Option<imgui::TextureId>,
	/// Size: [width, height].
	size: [f32; 2],
	resolution: u32,
}

impl FractalTexture {
	/// Instantiate and returns a link to a new `FractalTexture`.
	fn new(size: [f32; 2], resolution: u32) -> rc::Rc<cell::RefCell<FractalTexture>> {
		rc::Rc::new(cell::RefCell::new(FractalTexture { 
			texture_id: Option::None, 
			size: size, 
			resolution: resolution,
		}))
	}

	/// Generate and register the fractal texture.
	fn register_texture<F>(
        &mut self,
        gl_ctx: &F,
        textures: &mut imgui::Textures<imgui_glium_renderer::Texture>,
    ) -> Result<(), Box<dyn error::Error>>
    where
        F: backend::Facade,
    {
		let height: usize = self.size[0] as usize;
		let width: usize = self.size[1] as usize;

        if self.texture_id.is_none() {

            // Texture generation.
            let mut data = Vec::with_capacity(height * width);
            for y in 0..height {
                for x in 0..width {
                    // Insert RGB values
                    data.push((y % 255) as u8);
                    data.push(((x * x) % 255) as u8);
                    data.push(((y + x) % 255) as u8);
                }
            }

			// Render.
            let raw = texture::RawImage2d {
                data: borrow::Cow::Owned(data),
                width: height as u32,
                height: width as u32,
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
	fn show_textures(&self, ui: &imgui::Ui) {
        ui.window("Hello textures")
            .size([400.0, 400.0], imgui::Condition::FirstUseEver)
            .build(|| {
                ui.text("Hello textures!");
                if let Some(my_texture_id) = self.texture_id {
                    ui.text("Some generated texture");
                    imgui::Image::new(my_texture_id, self.size).build(ui);
                }
			});
	}
}