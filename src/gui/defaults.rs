//! # Complex sequences.
//! src/gui/defaults.rs
//! 
//! Defaults settings for the `App`.

use std::{
	rc,
	cell,
};

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
		|z: complex_rust::Algebraic, c: complex_rust::Algebraic| z * z + c ,
		complex::Algebraic::new(0.0, 0.0),
		[600.0, 600.0], 
		[0.0, 0.0],
		1,
		1.0,
		50,
		2.0,
		0,
		[0, 5, 15],
		[255, 250, 240],
	);
	// Necessary for the closure.
	let divergent_texture_startup = divergent_texture.clone();
	let divergent_texture_update = divergent_texture.clone();

	let root_texture = fractals::root_texture::Root::new(
		|z: complex_rust::Algebraic| z * z * z * z + complex::Algebraic::new(1.0, 0.0),
		|z: complex_rust::Algebraic| complex::Algebraic::new(4.0, 0.0) * z * z * z,
		[600.0, 600.0],
		[0.0, 0.0],
		1,
		1.0,
		50,
		1.0,
		0,
		[0, 0, 0],
	);
	let root_texture_startup = root_texture.clone();
	let root_texture_update = root_texture.clone();

	let settings_state = rc::Rc::new(cell::RefCell::new(settings::Settings::default()));
	let settings_state_update = settings_state.clone();

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

			root_texture_startup
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
			let method_id_current: usize = settings_state.borrow().method_id;
			match method_id_current {
				0 => {
					// ## Divergence.
					// Settings window.
					settings::show_settings_divergent(
						[400.0, 600.0], 
						[0.0, 0.0], 
						&mut settings_state.borrow_mut(),
						ui, 
						divergent_texture.clone(), 
						renderer, 
						display,
					);
					

					// Fractal graphics.
					divergent_texture
						.borrow_mut()
						.show_textures(ui, [410.0, 0.0]);
				},
				1 => {
					// ## Root.
					settings::show_settings_root(
						[400.0, 600.0], 
						[0.0, 0.0], 
						&mut settings_state.borrow_mut(), 
						ui, 
						root_texture.clone(), 
						renderer, 
						display
					);

					root_texture
						.borrow_mut()
						.show_textures(ui, [410.0, 0.0]);
				},
				_ => panic!("(X) `method` ({}) not implemented", settings_state.borrow().method_id),
			};
		},

		move |
			renderer: &mut imgui_glium_renderer::Renderer, 
			display: &glium::Display<glium::glutin::surface::WindowSurface>,
		| {
			// Fractal controls update.
			match &settings_state_update.borrow().method_id {
				0 => {if divergent_texture_update.borrow_mut().is_state_updated() {
					divergent_texture_update
						.borrow_mut()
						.register_texture(display.get_context(), renderer.textures())
						.expect("(!) gui::default::launch_default() Divergent: update: can't register texture.");
				}},
				1 => {if root_texture_update.borrow_mut().is_state_updated() {
					root_texture_update
						.borrow_mut()
						.register_texture(display.get_context(), renderer.textures())
						.expect("(!) gui::default::launch_default() Root: update: can't register texture.");
				}},
				_ => panic!("(X) `method` ({}) not implemented. ", settings_state_update.borrow().method_id),
			}
		},
	);
}
