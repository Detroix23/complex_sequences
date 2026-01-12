//! # Complex sequences.
//! src/gui/defaults.rs
//! 
//! Defaults settings for the `App`.

use glium;
use glium::backend::Facade;
use imgui;
use complex_rust as complex;

use crate::support;
use crate::fractals;


/// Launch the default configuration for `App`. 
pub fn launch_default() -> () {	
	// Workers.
	let fractal_texture = fractals::textures::FractalTexture::new(
		[600.0, 600.0], 
		1
	);
	// Necessary for the closure.
	let fractal_texture_startup = fractal_texture.clone();
	let fractal_texture_update = fractal_texture.clone();

	// State
	let mut zoom: complex::Real = 1.0;


	// True start.
	support::init_with_startup( 
		"Complex sequences. ", 
		[1024, 768],

		move |
			_context: &mut imgui::Context, 
			renderer: &mut imgui_glium_renderer::Renderer, 
			display: &glium::Display<glium::glutin::surface::WindowSurface>,
		| {
			println!("(?) gui::defaults::launch_default() Initialized.");

			fractal_texture_startup
				.borrow_mut()
				.register_texture(display.get_context(), renderer.textures())
				.expect("(!) gui::default::launch_default() startup: can't register texture.");
		}, 

		move |
			_, 
			ui: &mut imgui::Ui,
			_renderer: &mut imgui_glium_renderer::Renderer, 
			_display: &glium::Display<glium::glutin::surface::WindowSurface>,
		| {
			// Window: settings.
			ui.window("Settings.")
				.size([150.0, 250.0], imgui::Condition::FirstUseEver)
				.position([100.0, 100.0], imgui::Condition::FirstUseEver)
				.build(|| {
					ui.text_wrapped("This is the settings window.");	

					// Zoom slider.
                    ui.slider_config("Zoom", 0.0001, 10.0)
                        .flags(imgui::SliderFlags::LOGARITHMIC)
                        .build(&mut fractal_texture.borrow_mut().zoom);

				});
			

			// Fractal graphics.
			fractal_texture
				.borrow_mut()
				.show_textures(ui);
		},

		move |
			renderer: &mut imgui_glium_renderer::Renderer, 
			display: &glium::Display<glium::glutin::surface::WindowSurface>,
		| {
			// Fractal controls update.
			fractal_texture_update
				.borrow_mut()
				.register_texture(display.get_context(), renderer.textures())
				.expect("(!) gui::default::launch_default() update: can't register texture.");
		},
	);
}
