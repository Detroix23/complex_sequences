//! # Complex sequences.
//! src/support/textures.rs
//! 
//! Helper functions for images and textures rendering.

use std::{borrow, rc};

use glium::{
	self, backend, texture, uniforms
};
use imgui_glium_renderer;

pub enum ColorFormat {
	RGB,
	RGBA,
}

/// Register and render a texture:
/// - from image information: `texture_id`, `data`, `size`,
/// - to registers `gl_context`, `textures`.
/// 
/// Returns the new texture id. 
pub fn render_texture<Facade>(
	texture_id: Option<imgui::TextureId>,
	data: Vec<u8>,
	size: [usize; 2],
	gl_context: &Facade,
	textures: &mut imgui::Textures<imgui_glium_renderer::Texture>,
	format: ColorFormat,
) -> Result<imgui::TextureId, Box<dyn std::error::Error>> 
where
	Facade: backend::Facade,
{
	// Render (from `imgui-examples`, `custom_texture`).
	let raw = texture::RawImage2d {
		data: borrow::Cow::Owned(data),
		width: size[0] as u32,
		height: size[1] as u32,
		format: match format {
			ColorFormat::RGB => texture::ClientFormat::U8U8U8,
			ColorFormat::RGBA => texture::ClientFormat::U8U8U8U8,
		},
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

	Result::Ok(match texture_id {
		Option::None => {
			textures.insert(texture)
		},
		Option::Some(id) => {
			textures.replace(id, texture);
			id
		}
	})
}
