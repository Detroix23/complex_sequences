//! # Complex sequences.
//! src/gui/settings.rs
//! 
//! Settings windows for `App`.

use std::{
	rc,
	cell,
};

use glium;
use glium::backend::Facade;
use imgui;
use complex_rust as complex;

use crate::fractals;
use crate::gui;

/// Show a settings window to read and modify values of the current fractal.
/// 
/// Modify its `FractalTexture`.
pub fn show_settings<F>(
	size: [complex::Real; 2],
	position: [complex::Real; 2],
	ui: &imgui::Ui,
	// Rc<RefCell<FractalTexture<impl Fn(Algebraic, Algebraic) -> Algebraic>>>
	fractal_texture: rc::Rc<cell::RefCell<fractals::divergence_texture::FractalTexture<F>>>,
	renderer: &mut imgui_glium_renderer::Renderer, 
	display: &glium::Display<glium::glutin::surface::WindowSurface>,
) -> () 
where
	F: Fn(complex::Algebraic, complex::Algebraic) -> complex::Algebraic + Copy + 'static,
{
	// Window: settings.
	ui.window("Settings.")
		.size(size, imgui::Condition::FirstUseEver)
		.position(position, imgui::Condition::FirstUseEver)
		.build(|| {
			let current_zoom = fractal_texture.borrow().zoom.clone();


			ui.text("## Info");
			ui.text(format!(
				"- Position: ({}; {})", 
				fractal_texture.borrow().position[0], 
				fractal_texture.borrow().position[1]
			));


			ui.separator();

			ui.text_wrapped("## Controls");	

			// Force update.
			if ui.button("Force update.") {
				fractal_texture.borrow_mut()
					.register_texture(display.get_context(), renderer.textures())
					.expect("(!) gui::default::launch_default() run_ui: can't register texture.");
			}

			// Zoom slider.
			ui.slider_config("Zoom", 1.0, 100000.0)
				.flags(imgui::SliderFlags::LOGARITHMIC)
				.build(&mut fractal_texture.borrow_mut().zoom);

			// (x; y).
			ui.text("Position");
			gui::inputs::button_slider(
				ui, 
				&mut fractal_texture.borrow_mut().position[0], 
				1.0 / current_zoom, 
				"position(x)"
			);
			gui::inputs::button_slider(
				ui, 
				&mut fractal_texture.borrow_mut().position[1], 
				1.0 / current_zoom, 
				"position(y)"
			);

			ui.new_line();

			// Iterations.
			ui.slider_config("Iteration", 1_usize, 250_usize)
				.build(&mut fractal_texture.borrow_mut().iterations);

			// Threshold for divergence.
			ui.slider_config("Threshold", 0.0, 5.0)
				.build(&mut fractal_texture.borrow_mut().threshold);

			// Fixed point.
			gui::inputs::complex_2_sliders(
				ui, 
				"Fixed point", 
				-5.0, 
				5.0, 
				&mut fractal_texture.borrow_mut().constant
			);
		});
}