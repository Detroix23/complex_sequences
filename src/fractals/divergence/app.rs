//! # Complex sequences.
//! src/fractals/divergence/app.rs
//! 
//! App related functions.

use std::{rc, cell};

use glium;
use glium::backend::Facade;
use imgui;
use complex;

use crate::structures::{configuration};
use crate::{fractals, gui};
use crate::fractals::textures::Fractal;

/// Draw settings and texture of `Divergence`.
pub fn draw<F>(
	settings_state: rc::Rc<cell::RefCell<configuration::GlobalSettings>>,
	ui: &imgui::Ui,
	// Rc<RefCell<Divergent<impl Fn(Algebraic, Algebraic) -> Algebraic>>>
	divergent_texture: rc::Rc<cell::RefCell<fractals::divergence::Divergent<F>>>,
	renderer: &mut imgui_glium_renderer::Renderer, 
	display: &glium::Display<glium::glutin::surface::WindowSurface>,
) -> () 
where 
	F: Fn(complex::Algebraic, complex::Algebraic) -> complex::Algebraic + Copy + Send + 'static,
{
	// ## Divergence.
	
	// Fractal graphics.
	divergent_texture
		.borrow_mut()
		.show_textures(ui, [410.0, 0.0]);
	
	// Settings window.
	gui::settings::show_settings_divergent(
		[400.0, 600.0], 
		[0.0, 0.0], 
		settings_state.clone(),
		ui, 
		divergent_texture.clone(), 
		renderer, 
		display,
	);
}

/// Update settings and texture of `Divergence`.
pub fn update<F>(
	divergent_texture: rc::Rc<cell::RefCell<fractals::divergence::Divergent<F>>>,
	global_settings: rc::Rc<cell::RefCell<configuration::GlobalSettings>>,
	_ui: &imgui::Ui,
	renderer: &mut imgui_glium_renderer::Renderer, 
	display: &glium::Display<glium::glutin::surface::WindowSurface>,
	window_size: [u32; 2],
) -> () 
where
	F: Fn(complex::Algebraic, complex::Algebraic) -> complex::Algebraic + Copy + Send + 'static,
{
	divergent_texture
		.borrow_mut()
		.update_size(window_size);

	// If a setting change, draw the fractal anew.
	if divergent_texture.borrow_mut().is_state_updated() {
		divergent_texture
			.borrow_mut()
			.register_texture(
				display.get_context(), 
				global_settings.clone(),
				renderer.textures(), 
			)
			.expect("(!) gui::default::launch_default() Divergent: update: can't register texture.");
	}
}
