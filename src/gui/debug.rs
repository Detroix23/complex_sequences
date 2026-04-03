//! # Complex sequences.
//! src/gui/debug.rs
//! 
//! Debug window for testing.

use std::{rc, cell, error};

use imgui;
use glium;
use glium::backend::Facade;

use crate::gui::{settings, color};
use crate::support::rendering;
use crate::fractals;

#[derive(Debug, Clone)]
pub struct DebugTexture {
	texture_id: Option<imgui::TextureId>,
	size: [f32; 2],
}

impl DebugTexture {
	/// Returns a linked new `DebugTexture`
	pub fn new() -> rc::Rc<cell::RefCell<DebugTexture>> {
		rc::Rc::new(cell::RefCell::new(DebugTexture { 
			texture_id: Option::None,
			size: [150.0, 50.0]
		}))
	}

	pub fn register_texture<Facade>(
		&mut self,
		gl_context: &Facade,
        textures: &mut imgui::Textures<imgui_glium_renderer::Texture>,
	) -> Result<(), Box<dyn std::error::Error + 'static>>
	where
        Facade: glium::backend::Facade,
	{
		self.hsv_register(gl_context, textures)
	}

	pub fn hsv_register<Facade>(
        &mut self,
        gl_context: &Facade,
        textures: &mut imgui::Textures<imgui_glium_renderer::Texture>,
    ) -> Result<(), Box<dyn error::Error>>
    where
        Facade: glium::backend::Facade,
    {	
		let mut pixels: Vec<u8> = Vec::new();
		// HSV gradient.
		for degree in 0..360 {
			let rgb: color::Rgb = color::Hsv::new(degree as f64, 1.0, 1.0)
				.to_rgb();
			// eprintln!("(?) RGB: {:?}", rgb);
			pixels.push(rgb.red);
			pixels.push(rgb.green);
			pixels.push(rgb.blue);
		}

		let data: fractals::textures::Data = fractals::textures::Data {
			raw_pixels: pixels,
			iterations_total: 1,
		};
		
		self.texture_id = Option::Some(rendering::render_texture(
			self.texture_id, 
			data.raw_pixels, 
			[360, 1], 
			gl_context, 
			textures,
			rendering::ColorFormat::RGB,
		).expect("(X) gui::debug::DebugTexture::register_texture() render_texture error."));

		Ok(())
	}

	pub fn hsv_show(&self, ui: &imgui::Ui) {
		ui.window("Test: HSV. ")
			.size(self.size, imgui::Condition::FirstUseEver)
			.position([75.0, 10.0], imgui::Condition::FirstUseEver)
			.build(|| {
				ui.text("Test....");

				if let Some(texture_id) = self.texture_id {
					imgui::Image::new(texture_id, self.size)
						.build(ui);
				}
			});
	}
}

/// Draw the UI of the `DebugTexture`.
pub fn draw(
	settings: &mut settings::Settings, 
	ui: &imgui::Ui,
	debug_texture: rc::Rc<cell::RefCell<DebugTexture>>,
) -> () {
	settings::show_settings_debug(
		[400.0, 600.0], 
		[0.0, 0.0], 
		settings, 
		ui
	);

	debug_texture
		.borrow()
		.hsv_show(ui);
}

/// Update the `DebugTexture`.
pub fn update(
	debug_texture: rc::Rc<cell::RefCell<DebugTexture>>,
	_ui: &imgui::Ui,
	renderer: &mut imgui_glium_renderer::Renderer, 
	display: &glium::Display<glium::glutin::surface::WindowSurface>,
) -> () {
	debug_texture
		.borrow_mut()
		.register_texture(display.get_context(), renderer.textures())
		.expect("debug::update() Can't register textures.");
}