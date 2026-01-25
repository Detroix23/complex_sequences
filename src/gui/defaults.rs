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
use crate::gui::{
	settings,
	debug,
};

/// Launch the default configuration for `App`. 
pub fn launch_default() -> () {	
	// Workers.
	let divergent_texture = fractals::divergence::Divergent::new(
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

	let root_texture = fractals::root::Root::new(
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

		// Run, draw.
		move |
			_, 
			ui: &mut imgui::Ui,
			renderer: &mut imgui_glium_renderer::Renderer, 
			display: &glium::Display<glium::glutin::surface::WindowSurface>,
		| {
			let method_id_current: usize = settings_state.borrow().method_id;
			match method_id_current {
				0 => debug::draw(
					&mut settings_state.borrow_mut(),
					ui, 
				),
				1 => fractals::divergence::app::draw(
					settings_state.clone(), 
					ui, 
					divergent_texture.clone(), 
					renderer, 
					display
				),
				2 => fractals::root::app::draw(
					settings_state.clone(), 
					ui, 
					root_texture.clone(), 
					renderer, 
					display
				),
				_ => panic!(
					"(X) gui::defaults::launch_default() `method` ({}) not implemented",
					settings_state.borrow().method_id
				),
			};
		},

		move |
			renderer: &mut imgui_glium_renderer::Renderer, 
			display: &glium::Display<glium::glutin::surface::WindowSurface>,
		| {
			// Fractal controls update.
			match &settings_state_update.borrow().method_id {
				0 => debug::update(),
				1 => fractals::divergence::app::update(
					divergent_texture_update.clone(), 
					renderer, 
					display,
				),
				2 => fractals::root::app::update(
					root_texture_update.clone(),
					renderer, 
					display,
				),
				_ => panic!("(X) `method` ({}) not implemented. ", settings_state_update.borrow().method_id),
			}
		},
	);
}
