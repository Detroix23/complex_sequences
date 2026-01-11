//! # Complex sequences.
//! src/gui/defaults.rs
//! 
//! Defaults settings for the `App`.

use glium;
use glium::backend::Facade;
use imgui;

use crate::support;
use crate::fractals;


/// Launch the default configuration for `App`. 
pub fn launch_default() -> () {	
	// Workers.
	let fractal_texture = fractals::textures::FractalTexture::new([700.0, 500.0], 1);
	let fractal_texture_clone = fractal_texture.clone();

	// True start.
	support::init_with_startup( 
		"Complex sequences. ", 

		move |
			context: &mut imgui::Context, 
			renderer: &mut imgui_glium_renderer::Renderer, 
			display: &glium::Display<glium::glutin::surface::WindowSurface>,
		| {
			println!("(?) gui::defaults::launch_default() Initialized.");

			fractal_texture_clone
				.borrow_mut()
				.register_texture(display.get_context(), renderer.textures())
				.expect("(!) gui::default::launch_default() startup: can't register texture.");
		}, 

		move|
			_, 
			ui: &mut imgui::Ui
		| {
		// Window: settings.
			ui.window("Settings.")
				.size([150.0, 250.0], imgui::Condition::FirstUseEver)
				.position([100.0, 100.0], imgui::Condition::FirstUseEver)
				.build(|| {
					ui.text_wrapped("This is the settings window.");	
				});
			
			fractal_texture
				.borrow_mut()
				.show_textures(ui);
		},

		[1024, 768],
	);
}
