//! # Complex sequences.
//! src/gui/mouse.rs
//! 
//! Handles mouse inputs.
//! 
//! Uses mainly `imgui`'s:
//! ```rs
//! ui.io().mouse_pos;
//! 
//! ui.is_mouse_down(MouseButton::Left);
//! ```

use imgui;
use complex;

use crate::support::rendering;

pub fn listen(
	ui: &imgui::Ui, 
	window_size: (u32, u32),
	position: [complex::Real; 2], 
	zoom: complex::Real,
	_scale: complex::Real,
) -> rendering::ViewportSettings {
	let scroll_strength: complex::Real = 1.5;

	let mouse_position: [complex::Real; 2] = [
		ui.io().mouse_pos[0] as complex::Real, 
		ui.io().mouse_pos[1] as complex::Real
	];
	let mouse_scroll: complex::Real = ui.io().mouse_wheel.into();
	let mut viewport: rendering::ViewportSettings = rendering::ViewportSettings { position, zoom };

	if ui.is_mouse_clicked(imgui::MouseButton::Middle) {
		let selection_position: [complex::Real; 2] = [
			-(mouse_position[0] - window_size.0 as complex::Real / 2.0) / zoom + position[0],
			-(mouse_position[1] - window_size.1 as complex::Real / 2.0) / zoom + position[1],
		];

		println!(
			"(?) gui::mouse::listen() Left clicked: [{:.2}, {:.2}], Selection: [{:.2}, {:.2}]",
			mouse_position[0], mouse_position[1],
			selection_position[0], selection_position[1],
		);
		
		viewport.position = selection_position;
	} if mouse_scroll != 0.0 {
		viewport.zoom = zoom * scroll_strength.powf(mouse_scroll);
		println!("(?) gui::mouse::listen() Mouse scrolled: {}", mouse_scroll);
	}

	viewport
}
