//! # Complex sequences.
//! src/fractals/root/app.rs
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

/*
{
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
*/


/// Draw settings and texture of `Root`.
pub fn draw<F, D>(
	settings_state: rc::Rc<cell::RefCell<settings::Settings>>,
	ui: &imgui::Ui,
	// Rc<RefCell<Divergent<impl Fn(Algebraic, Algebraic) -> Algebraic>>>
	root_texture: rc::Rc<cell::RefCell<fractals::root::Root<F, D>>>,
	renderer: &mut imgui_glium_renderer::Renderer, 
	display: &glium::Display<glium::glutin::surface::WindowSurface>,
) -> () 
where 
	F: Fn(complex::Algebraic) -> complex::Algebraic,
	D: Fn(complex::Algebraic) -> complex::Algebraic,
{
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
}

/// Updated settings and texture of `Root`.
pub fn update<F, D>(
	root_texture: rc::Rc<cell::RefCell<fractals::root::Root<F, D>>>,
	renderer: &mut imgui_glium_renderer::Renderer, 
	display: &glium::Display<glium::glutin::surface::WindowSurface>,
) -> () 
where 
	F: Fn(complex::Algebraic) -> complex::Algebraic,
	D: Fn(complex::Algebraic) -> complex::Algebraic,
{
	// If a setting change, draw the fractal anew.
	if root_texture.borrow_mut().is_state_updated() {
		root_texture
			.borrow_mut()
			.register_texture(display.get_context(), renderer.textures())
			.expect("(!) gui::default::launch_default() Root: update: can't register texture.");
	}
}