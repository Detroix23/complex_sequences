//! # Complex sequences.
//! src/gui/defaults.rs
//! 
//! Defaults settings for the `App`.

use glium;
use glium::backend::Facade;
use imgui;
use complex_rust as complex;

use crate::support;
use crate::fractals::{
	self,
	textures::Fractal,
};
use crate::gui::settings;

/// Launch the default configuration for `App`. 
pub fn launch_default() -> () {	
	// Workers.
	let divergent_texture = fractals::divergence_texture::Divergent::new(
		|z, c| { z * z + c },
		complex::Algebraic::new(0.0, 0.0),
		[600.0, 600.0], 
		[0.0, 0.0],
		1,
		0.08,
		50,
		2.0,
		0,
		[0, 5, 15],
		[255, 250, 240],
	);
	// Necessary for the closure.
	let divergent_texture_startup = divergent_texture.clone();
	let divergent_texture_update = divergent_texture.clone();

	let mut settings_state: settings::Settings = Default::default();

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

			divergent_texture_startup
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
			match &settings_state.method_id {
				0 => {
					// Settings window.
					settings::show_settings_divergent(
						[400.0, 600.0], 
						[0.0, 0.0], 
						&mut settings_state,
						ui, 
						divergent_texture.clone(), 
						renderer, 
						display,
					);
					

					// Fractal graphics.
					divergent_texture
						.borrow_mut()
						.show_textures(ui);
				}

				_ => todo!("(X) `method` ({}) not yet implemented", settings_state.method_id),
			};
		},

		move |
			renderer: &mut imgui_glium_renderer::Renderer, 
			display: &glium::Display<glium::glutin::surface::WindowSurface>,
		| {
			// Fractal controls update.
			if divergent_texture_update.borrow_mut().is_state_updated() {
				divergent_texture_update.borrow_mut()
					.register_texture(display.get_context(), renderer.textures())
					.expect("(!) gui::default::launch_default() update: can't register texture.");
			}
		},
	);
}
