//! # Complex sequences.
//! src/gui/settings.rs
//! 
//! Settings windows for `App`.

use std::{
	rc,
	cell,
	borrow,
};

use glium;
use glium::backend::Facade;
use imgui;
use complex_rust as complex;

use crate::fractals::{
	self,
	textures::Fractal,
};
use crate::gui;

/// # `Settings` state.
/// Store only persistent settings.
#[derive(Default, Clone)]
pub struct Settings {
	/// Fractal family, method id.
	/// ```rust, no_run
	/// 0. Divergence,
	/// 1. Roots,
	/// ```
	pub method_id: usize,
}


/// Show a settings window to read and modify values of the current fractal.
/// 
/// Modify a `Divergent`.
pub fn show_settings_divergent<F>(
	size: [complex::Real; 2],
	position: [complex::Real; 2],
	settings: &mut Settings,
	ui: &imgui::Ui,
	// Rc<RefCell<Divergent<impl Fn(Algebraic, Algebraic) -> Algebraic>>>
	divergent_texture: rc::Rc<cell::RefCell<fractals::divergence_texture::Divergent<F>>>,
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
			let current_zoom = divergent_texture.borrow().zoom.clone();

			ui.text("## Info");
			ui.text(format!(
				"- Camera: ({}; {})", 
				divergent_texture.borrow().position[0], 
				divergent_texture.borrow().position[1]
			));


			ui.separator();


			ui.text_wrapped("## Controls");	

			// General fractal family.
			ui.combo(
				"Method", 
				&mut settings.method_id, 
				&fractals::Method::list(), 
				| method: &fractals::Method | borrow::Cow::Borrowed(method.as_ref()),
			);
			// Limit type.
			ui.combo(
				"Limit type",
				&mut divergent_texture.borrow_mut().method_id,
				&fractals::divergence::LimitMethod::list(),
				| limit: &fractals::divergence::LimitMethod | borrow::Cow::Borrowed(limit.as_ref()),
			);


			// Force update.
			if ui.button("Force update.") {
				divergent_texture.borrow_mut()
					.register_texture(display.get_context(), renderer.textures())
					.expect("(!) gui::default::launch_default() run_ui: can't register texture.");
			}

			// Zoom slider.
			ui.slider_config("Zoom", 1.0, 100000.0)
				.flags(imgui::SliderFlags::LOGARITHMIC)
				.build(&mut divergent_texture.borrow_mut().zoom);

			// (x; y).
			ui.text("Position");
			gui::inputs::button_slider(
				ui, 
				&mut divergent_texture.borrow_mut().position[0], 
				1.0 / current_zoom, 
				"position(x)"
			);
			gui::inputs::button_slider(
				ui, 
				&mut divergent_texture.borrow_mut().position[1], 
				1.0 / current_zoom, 
				"position(y)"
			);

			ui.new_line();

			// Iterations.
			ui.slider_config("Iteration", 1_usize, 250_usize)
				.build(&mut divergent_texture.borrow_mut().iterations);

			// Threshold for divergence.
			ui.slider_config("Threshold", 0.0, 5.0)
				.build(&mut divergent_texture.borrow_mut().threshold);

			// Fixed constant point.
			gui::inputs::complex_2_sliders(
				ui, 
				"Constant point", 
				-5.0, 
				5.0, 
				&mut divergent_texture.borrow_mut().constant
			);
		});
}

/// Show a settings window to read and modify values of the current fractal.
/// 
/// Modify a `Root`.
pub fn show_settings_root<F, D>(
	size: [complex::Real; 2],
	position: [complex::Real; 2],
	settings: &mut Settings,
	ui: &imgui::Ui,
	root_texture: rc::Rc<cell::RefCell<fractals::root_texture::Root<F, D>>>,
	renderer: &mut imgui_glium_renderer::Renderer, 
	display: &glium::Display<glium::glutin::surface::WindowSurface>,
) -> () 
where
	F: Fn(complex::Algebraic) -> complex::Algebraic,
	D: Fn(complex::Algebraic) -> complex::Algebraic,
{
	// Window: settings.
	ui.window("Settings.")
		.size(size, imgui::Condition::FirstUseEver)
		.position(position, imgui::Condition::FirstUseEver)
		.build(|| {
			let current_zoom: f32 = root_texture.borrow().zoom.clone();

			ui.text("## Info");
			ui.text(format!(
				"- Camera: ({}; {})", 
				root_texture.borrow().position[0], 
				root_texture.borrow().position[1]
			));


			ui.separator();


			ui.text_wrapped("## Controls");	

			// General fractal family.
			ui.combo(
				"Method", 
				&mut settings.method_id, 
				&fractals::Method::list(), 
				| method: &fractals::Method | borrow::Cow::Borrowed(method.as_ref()),
			);
			// Limit type.
			ui.combo(
				"Limit type",
				&mut root_texture.borrow_mut().method_id,
				&fractals::root::RootMethod::list(),
				| limit: &fractals::root::RootMethod | borrow::Cow::Borrowed(limit.as_ref()),
			);


			// Force update.
			if ui.button("Force update.") {
				root_texture.borrow_mut()
					.register_texture(display.get_context(), renderer.textures())
					.expect("(!) gui::default::launch_default() run_ui: can't register texture.");
			}

			// Zoom slider.
			ui.slider_config("Zoom", 1.0, 100000.0)
				.flags(imgui::SliderFlags::LOGARITHMIC)
				.build(&mut root_texture.borrow_mut().zoom);

			// (x; y).
			ui.text("Position");
			gui::inputs::button_slider(
				ui, 
				&mut root_texture.borrow_mut().position[0], 
				1.0 / current_zoom, 
				"position(x)"
			);
			gui::inputs::button_slider(
				ui, 
				&mut root_texture.borrow_mut().position[1], 
				1.0 / current_zoom, 
				"position(y)"
			);

			ui.new_line();

			// Iterations.
			ui.slider_config("Iteration", 1_usize, 250_usize)
				.build(&mut root_texture.borrow_mut().iterations);

			// Threshold for root acceptation.
			ui.slider_config("Threshold", 0.0, 5.0)
				.build(&mut root_texture.borrow_mut().threshold);
			
		});
}