//! # Complex sequences.
//! src/fractals/divergence/app.rs
//! 
//! App related functions.

use std::{
	rc,
	cell,
};

use glium::{
	self,
	backend::Facade,
};
use imgui;
use complex_rust as complex;

use crate::fractals::{
	self,
	textures::Fractal,
};
use crate::gui::settings;

/// Draw settings and texture of `Divergence`.
pub fn draw<F>(
	settings_state: rc::Rc<cell::RefCell<settings::Settings>>,
	ui: &imgui::Ui,
	// Rc<RefCell<Divergent<impl Fn(Algebraic, Algebraic) -> Algebraic>>>
	divergent_texture: rc::Rc<cell::RefCell<fractals::divergence::Divergent<F>>>,
	renderer: &mut imgui_glium_renderer::Renderer, 
	display: &glium::Display<glium::glutin::surface::WindowSurface>,
) -> () 
where 
	F: Fn(complex::Algebraic, complex::Algebraic) -> complex::Algebraic + Copy + 'static,
{
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
}

/// Update settings and texture of `Divergence`.
pub fn update<F>(
	divergent_texture: rc::Rc<cell::RefCell<fractals::divergence::Divergent<F>>>,
	renderer: &mut imgui_glium_renderer::Renderer, 
	display: &glium::Display<glium::glutin::surface::WindowSurface>,
) -> () 
where
	F: Fn(complex::Algebraic, complex::Algebraic) -> complex::Algebraic + Copy + 'static,
{
	// If a setting change, draw the fractal anew.
	if divergent_texture.borrow_mut().is_state_updated() {
		divergent_texture
			.borrow_mut()
			.register_texture(display.get_context(), renderer.textures())
			.expect("(!) gui::default::launch_default() Divergent: update: can't register texture.");
	}
}