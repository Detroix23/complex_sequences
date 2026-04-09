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

use crate::support::rendering;

pub fn listen(
	ui: &imgui::Ui, 
	window_size: (u32, u32),
	position: [f32; 2], 
	zoom: f32,
	_scale: f32,
) -> rendering::ViewportSettings {
	let scroll_strength: f32 = 1.5;

	let mouse_position: [f32; 2] = ui.io().mouse_pos;
	let mouse_scroll: f32 = ui.io().mouse_wheel;
	let mut viewport: rendering::ViewportSettings = rendering::ViewportSettings{ position, zoom };

	if ui.is_mouse_clicked(imgui::MouseButton::Middle) {
		let selection_position: [f32; 2] = [
			-(mouse_position[0] - window_size.0 as f32 / 2.0) / zoom + position[0],
			-(mouse_position[1] - window_size.1 as f32 / 2.0) / zoom + position[1],
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
