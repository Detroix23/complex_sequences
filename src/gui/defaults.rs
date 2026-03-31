//! # Complex sequences.
//! src/gui/defaults.rs
//! 
//! Default launch for the `App`.

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
use crate::gui::{self, settings, debug};


const WINDOW_SIZE: [u32; 2] = [1024, 768];
const WINDOW_SIZE_FLOATS: [f32; 2] = [1024.0, 768.0];

/// Launch the default configuration for `App`.
/// 
/// It comprises different **modes**:
/// - debug,
/// - divergence,
/// - root.
pub fn launch_default() -> () {
	// Workers.
	let divergent_texture = fractals::divergence::Divergent::new(
		|z: complex_rust::Algebraic, c: complex_rust::Algebraic| z * z + c ,
		complex::Algebraic::new(0.0, 0.0),
		[400.0, 100.0], 
		[0.0, 0.0],
		1.0,
		1.0,
		50,
		2.0,
		0,
		gui::color::Rgb::new(0, 5, 15),
		gui::color::Rgb::new(255, 250, 240),
	);
	// Necessary for the closure.
	let divergent_texture_startup = divergent_texture.clone();
	let divergent_texture_update = divergent_texture.clone();

	let root_texture = fractals::root::Root::new(
		|z: complex_rust::Algebraic| z * z * z * z + complex::Algebraic::new(1.0, 0.0),
		|z: complex_rust::Algebraic| complex::Algebraic::new(4.0, 0.0) * z * z * z,
		[400.0, 100.0],
		[0.0, 0.0],
		1.0,
		1.0,
		50,
		1.0,
		0,
		gui::color::Rgb::new(0, 0, 0),
	);
	let root_texture_startup = root_texture.clone();
	let root_texture_update = root_texture.clone();

	let debug_texture = debug::DebugTexture::new();
	let debug_texture_startup = debug_texture.clone();

	let settings_state = rc::Rc::new(cell::RefCell::new(settings::Settings::default()));
	let settings_state_update = settings_state.clone();
	
	// Get `Display` size with `display.get_framebuffer_dimensions()`
	let mut _window_size: (u32, u32) = (0, 0);

	// True start.
	support::initialization::with_startup( 
		"Complex sequences. ", 
		WINDOW_SIZE,

		move |
			_context: &mut imgui::Context, 
			renderer: &mut imgui_glium_renderer::Renderer, 
			display: &glium::Display<glium::glutin::surface::WindowSurface>,
		| {
			eprintln!("\n(?) gui::defaults::launch_default() Initializing.");

			divergent_texture_startup
				.borrow_mut()
				.register_texture(
					display.get_context(), 
					renderer.textures(), 
					Option::Some(WINDOW_SIZE),
				)
				.expect("(!) gui::default::launch_default() startup: can't register `divergent` texture.");
			eprintln!("(?) gui::defaults::launch_default() `divergent` registered.");

			root_texture_startup
				.borrow_mut()
				.register_texture(
					display.get_context(), 
					renderer.textures(), 
					Option::Some(WINDOW_SIZE),
				)
				.expect("(!) gui::default::launch_default() startup: can't register `roots` texture.");
			eprintln!("(?) gui::defaults::launch_default() `roots` registered.");
		
			debug_texture_startup
				.borrow_mut()
				.register_texture(display.get_context(), renderer.textures())
				.expect("(!) gui::default::launch_default() startup: can't register `debug` texture.");
			eprintln!("(?) gui::defaults::launch_default() `debug` registered.");

			eprintln!("(?) gui::defaults::launch_default() Initializing phase completed.\n");
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
					debug_texture.clone(),
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
			ui: &mut imgui::Ui,
			renderer: &mut imgui_glium_renderer::Renderer, 
			display: &glium::Display<glium::glutin::surface::WindowSurface>,
		| {
			if display.get_framebuffer_dimensions() != _window_size {
				_window_size = display.get_framebuffer_dimensions();
				println!("(?) Window size update: x={}, y={}", _window_size.0, _window_size.1);
			}
			// Fractal controls update.
			match &settings_state_update.borrow().method_id {
				0 => debug::update(),
				1 => fractals::divergence::app::update(
					divergent_texture_update.clone(),
					ui,
					renderer, 
					display,
					_window_size.into(),
				),
				2 => fractals::root::app::update(
					root_texture_update.clone(),
					ui,
					renderer, 
					display,
					_window_size.into(),
				),
				_ => panic!("(X) `method` ({}) not implemented. ", settings_state_update.borrow().method_id),
			}
		},
	);
}
