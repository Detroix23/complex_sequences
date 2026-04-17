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
use complex;

use crate::structures::{configuration, color};
use crate::fractals;
use crate::fractals::textures::Fractal;
use crate::gui;

/// A `combo` selector for the general fractal family.
/// 
/// Item shared in all family. 
fn family_selector(
	ui: &imgui::Ui, 
	settings: rc::Rc<cell::RefCell<configuration::GlobalSettings>>
) -> () {
	ui.combo(
		"Method", 
		&mut settings.borrow_mut().method_id, 
		&fractals::Method::list(), 
		| method: &fractals::Method | borrow::Cow::Borrowed(method.as_ref()),
	);
} 

/// A `combo` selector for the color mode.
fn color_mode_selector(
	ui: &imgui::Ui, 
	settings: rc::Rc<cell::RefCell<configuration::GlobalSettings>>
) -> () {
	let mut color_mode_id: usize = settings.borrow().color_mode_id;
	
	ui.combo(
		"Color mode", 
		&mut color_mode_id, 
		&color::ColorMode::list(), 
		| mode: &color::ColorMode | {
			settings.borrow_mut().color_mode = mode.clone();
			borrow::Cow::Borrowed(mode.as_ref())
		},
	);

	settings.borrow_mut().color_mode_id = color_mode_id;
}

/// Show a settings window to read and modify values of the current fractal.
/// 
/// Modify a `Divergent`.
pub fn show_settings_divergent<F>(
	window_size: [f32; 2],
	window_position: [f32; 2],
	settings: rc::Rc<cell::RefCell<configuration::GlobalSettings>>,
	ui: &imgui::Ui,
	// Rc<RefCell<Divergent<impl Fn(Algebraic, Algebraic) -> Algebraic>>>
	divergent_texture: rc::Rc<cell::RefCell<fractals::divergence::Divergent<F>>>,
	renderer: &mut imgui_glium_renderer::Renderer, 
	display: &glium::Display<glium::glutin::surface::WindowSurface>,
) -> () 
where
	F: Fn(complex::Algebraic, complex::Algebraic) -> complex::Algebraic + Copy + Send + 'static,
{
	// Window: settings.
	ui.window("Settings.")
		.size(window_size, imgui::Condition::FirstUseEver)
		.position(window_position, imgui::Condition::FirstUseEver)
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

			family_selector(ui, settings.clone());
			// Limit type.
			ui.combo(
				"Limit type",
				&mut divergent_texture.borrow_mut().method_id,
				&fractals::divergence::LimitMethod::list(),
				| limit: &fractals::divergence::LimitMethod | borrow::Cow::Borrowed(limit.as_ref()),
			);
			color_mode_selector(ui, settings.clone());

			// Force update.
			if ui.button("Force update.") {
				divergent_texture
					.borrow_mut()
					.register_texture(
						display.get_context(), 
						settings.clone(),
						renderer.textures(), 
					)
					.expect("(!) gui::default::launch_default() run_ui: can't register texture.");
			}

			// Scale.
			ui.slider_config("Resolution scale", 1.0, 10.0)
				.build(&mut settings.borrow_mut().resolution_scale);

			// Zoom slider.
			ui.slider_config("Zoom", 1.0, 100000.0)
				.flags(
					imgui::SliderFlags::LOGARITHMIC
					| imgui::SliderFlags::NO_ROUND_TO_FORMAT
				).build(&mut divergent_texture.borrow_mut().zoom);

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
				.flags(imgui::SliderFlags::NO_ROUND_TO_FORMAT)
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
	window_size: [f32; 2],
	window_position: [f32; 2],
	settings: rc::Rc<cell::RefCell<configuration::GlobalSettings>>,
	ui: &imgui::Ui,
	root_texture: rc::Rc<cell::RefCell<fractals::root::Root<F, D>>>,
	renderer: &mut imgui_glium_renderer::Renderer, 
	display: &glium::Display<glium::glutin::surface::WindowSurface>,
) -> () 
where
	F: Fn(complex::Algebraic) -> complex::Algebraic,
	D: Fn(complex::Algebraic) -> complex::Algebraic,
{
	// Window: settings.
	ui.window("Settings.")
		.size(window_size, imgui::Condition::FirstUseEver)
		.position(window_position, imgui::Condition::FirstUseEver)
		.build(|| {
			let current_zoom: complex::Real = root_texture.borrow().zoom;

			ui.text("## Info");
			ui.text(format!(
				"- Camera: ({}; {})", 
				root_texture.borrow().position[0], 
				root_texture.borrow().position[1]
			));

			ui.separator();

			ui.text_wrapped("## Controls");	
			
			family_selector(ui, settings.clone());
			// Limit type.
			ui.combo(
				"Limit type",
				&mut root_texture.borrow_mut().method_id,
				&fractals::root::RootMethod::list(),
				| limit: &fractals::root::RootMethod | borrow::Cow::Borrowed(limit.as_ref()),
			);

			if root_texture.borrow().method_id == 0 {
				color_mode_selector(ui, settings.clone());
			}

			// Force update.
			if ui.button("Force update.") {
				root_texture.borrow_mut()
					.register_texture(
						display.get_context(), 
						settings.clone(),
						renderer.textures(), 
					)
					.expect("(!) gui::default::launch_default() run_ui: can't register texture.");
			}

			// Scale.
			ui.slider_config("Resolution scale", 1.0, 10.0)
				.build(&mut settings.borrow_mut().resolution_scale);

			// Zoom slider.
			ui.slider_config("Zoom", 1.0, 100000.0)
				.flags(
					imgui::SliderFlags::LOGARITHMIC
					| imgui::SliderFlags::NO_ROUND_TO_FORMAT
				).build(&mut root_texture.borrow_mut().zoom);

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

			if root_texture.borrow().method_id == 0 {
				// Iterations.
				ui.slider_config("Iteration", 1_usize, 250_usize)
					.build(&mut root_texture.borrow_mut().iterations);

				// Threshold for root acceptation.
				ui.slider_config("Threshold", 0.00000001, 2.0)
					.flags(imgui::SliderFlags::NO_ROUND_TO_FORMAT)
					.build(&mut root_texture.borrow_mut().threshold);
			} else if root_texture.borrow().method_id == 1 {
				// Degree 0.
				ui.slider_config("Degree 0", 0_f64, 360_f64)
					.build(&mut root_texture.borrow_mut().degree0);
			}
		});
}

/// Show a settings window for debug.
pub fn show_settings_debug(
	window_size: [f32; 2],
	window_position: [f32; 2],
	settings: rc::Rc<cell::RefCell<configuration::GlobalSettings>>,
	ui: &imgui::Ui,
) -> () {
	// Window: settings.
	ui.window("Settings (Debug).")
		.size(window_size, imgui::Condition::FirstUseEver)
		.position(window_position, imgui::Condition::FirstUseEver)
		.build(|| {
			ui.text("# Debug.");

			ui.separator();

			ui.text_wrapped("## Controls");	
			
			family_selector(ui, settings.clone());
		});
}