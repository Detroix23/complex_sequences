//! # Complex sequences.
//! src/gui/inputs.rs
//! 
//! Helpers to register settings.

use imgui;
use complex_rust as complex;

/// Build a header and 2 sliders for a `complex::Algebraic` number.
pub fn complex_2_sliders(
	ui: &imgui::Ui, 
	label: &str, 
	min: complex::Real, 
	max: complex::Real, 
	number: &mut complex::Algebraic,
) -> () {
	ui.text_wrapped(label);
	ui.slider_config(format!("real({})", label), min, max)
		.build(&mut number.real);
	ui.slider_config(format!("imaginary({})", label), min, max)
		.build(&mut number.imaginary);
}

/// Increment a `value` on click.
pub fn button_increment(
	ui: &imgui::Ui,
	value: &mut complex::Real,
	increment: complex::Real,
	label: &str
) -> () {
	if ui.button(label) {
		*value += increment;
	}
}

/// Update a value using "-" and "+" buttons.
pub fn button_slider(
	ui: &imgui::Ui,
	value: &mut complex::Real,
	increment: complex::Real,
	label: &str,
) -> () {
	button_increment(ui, value, -increment, &format!("{} (-)", label));
	ui.slider_config(label, 10.0, -10.0)
		.build(value);
		//.build(value);
	button_increment(ui, value, increment, &format!("{} (+)", label));
}