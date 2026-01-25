//! # Complex sequences.
//! src/gui/debug.rs
//! 
//! Debug window for testing.

use imgui;

use crate::gui::{settings};

/// Draw the UI of the debug state.
pub fn draw(settings: &mut settings::Settings, ui: &imgui::Ui) -> () {
	settings::show_settings_debug(
		[400.0, 600.0], 
		[0.0, 0.0], 
		settings, 
		ui
	);
}

pub fn update() -> () {

}
