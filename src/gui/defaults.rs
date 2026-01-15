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
use crate::gui::settings;

/// Launch the default configuration for `App`. 
pub fn launch_default() -> () {	
	// Workers.
	let fractal_texture = fractals::divergence_texture::FractalTexture::new(
		|z, c| { z * z + c },
		complex::Algebraic::new(0.0, 0.0),
		[600.0, 600.0], 
		[0.0, 0.0],
		1,
		0.08,
		50,
		2.0,
		fractals::divergence::LimitMethod::Julia,
		[0, 5, 15],
		[255, 250, 240],
	);
	// Necessary for the closure.
	let fractal_texture_startup = fractal_texture.clone();
	let fractal_texture_update = fractal_texture.clone();


	// True start.
	support::init_with_startup( 
		"Complex sequences. ", 
		[1024, 768],

		move |
			_context: &mut imgui::Context, 
			renderer: &mut imgui_glium_renderer::Renderer, 
			display: &glium::Display<glium::glutin::surface::WindowSurface>,
		| {
			eprintln!("(?) gui::defaults::launch_default() Initialized.");

			fractal_texture_startup
				.borrow_mut()
				.register_texture(display.get_context(), renderer.textures())
				.expect("(!) gui::default::launch_default() startup: can't register texture.");
		}, 

		move |
			_, 
			ui: &mut imgui::Ui,
			renderer: &mut imgui_glium_renderer::Renderer, 
			display: &glium::Display<glium::glutin::surface::WindowSurface>,
		| {
			// Settings window.
			settings::show_settings(
				[200.0, 300.0], 
				[0.0, 0.0], 
				ui, 
				fractal_texture.clone(), 
				renderer, 
				display,
			);
			

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
			if fractal_texture_update.borrow_mut().is_state_updated() {
				fractal_texture_update.borrow_mut()
					.register_texture(display.get_context(), renderer.textures())
					.expect("(!) gui::default::launch_default() update: can't register texture.");
			}
		},
	);
}
