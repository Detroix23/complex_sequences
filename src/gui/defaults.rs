//! # Complex sequences.
//! src/gui/defaults.rs
//! 
//! Default launch for the `App`.

use std::{
	rc,
	cell,
};

use glium;
use imgui;
use complex;

use crate::structures::{color, configuration};
use crate::support;
use crate::fractals;
use crate::gui::{debug, grid, mouse};


const WINDOW_SIZE: [u32; 2] = [1024, 768];

/// Launch the default configuration for `App`.
/// 
/// It comprises different **modes**:
/// - debug,
/// - divergence,
/// - root.
pub fn launch_default() -> () {
	// Workers.
	let divergent_texture = fractals::divergence::Divergent::new(
		|z, c| z * z + complex::ONE,
		complex::Algebraic::new(0.0, 0.0),
		[400.0, 100.0], 
		[0.0, 0.0],
		1.0,
		1.0,
		50,
		2.0,
		0,
		color::Rgb::new(0, 5, 15),
		color::Rgb::new(255, 250, 240),
	);
	// Necessary for the closure.
	let divergent_texture_update = divergent_texture.clone();
	
	let root_texture = fractals::root::Root::new(
		|z: complex::Algebraic| z * z * z + complex::ONE,
		|z: complex::Algebraic| complex::Algebraic::new(3.0, 0.0) * z * z,
		[400.0, 100.0],
		[0.0, 0.0],
		1.0,
		1.0,
		50,
		1.0,
		0,
		color::Rgb::new(0, 0, 0),
	);
	let root_texture_update = root_texture.clone();

	let debug_texture = debug::DebugTexture::new();
	let debug_texture_update = debug_texture.clone();

	let grid_update: rc::Rc<cell::RefCell<grid::Grid>> = grid::Grid::new(
		color::Rgb::new(100, 100, 110),
		100, 
		2,
	);
	let grid_draw: rc::Rc<cell::RefCell<grid::Grid>> = grid_update.clone();

	let settings_state: rc::Rc<cell::RefCell<configuration::GlobalSettings>> = rc::Rc::new(
		cell::RefCell::new(configuration::GlobalSettings::default())
	);
	let settings_state_update: rc::Rc<cell::RefCell<configuration::GlobalSettings>> = settings_state.clone();
	
	// Get `Display` size with `display.get_framebuffer_dimensions()`
	let mut window_size: (u32, u32) = (0, 0);

	// True start.
	support::initialization::with_startup( 
		"Complex sequences. ", 
		WINDOW_SIZE,

		move |
			_context: &mut imgui::Context, 
			_renderer: &mut imgui_glium_renderer::Renderer, 
			_display: &glium::Display<glium::glutin::surface::WindowSurface>,
		| {
			eprintln!("\n(?) gui::defaults::launch_default() Initializing.");

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
					settings_state.clone(),
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

			if settings_state.borrow().enable_grid {
				grid::draw(
					settings_state.clone(), 
					ui, 
					grid_draw.clone(), 
					renderer, 
					display
				);
			}
		},

		move |
			ui: &mut imgui::Ui,
			renderer: &mut imgui_glium_renderer::Renderer, 
			display: &glium::Display<glium::glutin::surface::WindowSurface>,
		| {
			// Listen for window resizing.
			if display.get_framebuffer_dimensions() != window_size {
				window_size = display.get_framebuffer_dimensions();
				println!("(?) Window size update: x={}, y={}", window_size.0, window_size.1);
			}
			
			// Listen to inputs.
			match &settings_state_update.borrow().method_id {
				0 => {},
				1 => {
					let viewport_update: support::rendering::ViewportSettings = mouse::listen(
						&ui,
						window_size,
						divergent_texture_update.borrow().position,
						divergent_texture_update.borrow().zoom,
						divergent_texture_update.borrow().scale,
					);

					divergent_texture_update.borrow_mut().position = viewport_update.position;
					divergent_texture_update.borrow_mut().zoom = viewport_update.zoom;
				},
				2 => {
					let viewport_update = mouse::listen(
						&ui,
						window_size,
						root_texture_update.borrow().position,
						root_texture_update.borrow().zoom,
						root_texture_update.borrow().scale,
					);

					root_texture_update.borrow_mut().position = viewport_update.position;
					root_texture_update.borrow_mut().zoom = viewport_update.zoom;
				},
				_ => panic!("(X) `method` ({}) not implemented. ", settings_state_update.borrow().method_id),
			}

			// Fractal controls update.
			match &settings_state_update.borrow().method_id {
				0 => debug::update(
					debug_texture_update.clone(),
					ui,
					renderer, 
					display,
				),
				1 => fractals::divergence::app::update(
					divergent_texture_update.clone(),
					settings_state_update.clone(),
					ui,
					renderer, 
					display,
					window_size.into(),
				),
				2 => fractals::root::app::update(
					root_texture_update.clone(),
					settings_state_update.clone(),
					ui,
					renderer, 
					display,
					window_size.into(),
				),
				_ => panic!("(X) `method` ({}) not implemented. ", settings_state_update.borrow().method_id),
			}

			if settings_state_update.borrow().enable_grid {
				grid::update(
					grid_update.clone(), 
					ui, 
					renderer, 
					display, 
					window_size.into(),
				);
			}
		},
	);
}
