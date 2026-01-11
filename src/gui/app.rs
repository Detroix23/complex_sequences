//! # Complex sequence.
//! src/gui/start.rs
//! 
//! Starts the whole application.

use glium;
use imgui;
use imgui_glium_renderer;

use crate::support;

/// # `App` for complex sequences.
/// Holds the state, and the graphics.
#[deprecated(note="Closures in generic types mismatch. Switched to functional.")]
pub struct App<FStart, FRun>
where
	FStart: FnMut(&mut imgui::Context, &mut imgui_glium_renderer::Renderer, &glium::Display<glium::glutin::surface::WindowSurface>) + 'static,
	FRun: FnMut(&mut bool, &mut imgui::Ui) + 'static,
{
	title: String,
	/// Defaults: "App initialized."
	startup: FStart,
	run_ui: FRun,
	window_size: [u32; 2],
}

impl<FStart, FRun> App<FStart, FRun> 
where
	FStart: FnMut(&mut imgui::Context, &mut imgui_glium_renderer::Renderer, &glium::Display<glium::glutin::surface::WindowSurface>) + 'static,
	FRun: FnMut(&mut bool, &mut imgui::Ui) + 'static,
{
	pub fn new(title: String, startup: FStart, run_ui: FRun, window_size: [u32; 2]) -> App<FStart, FRun> {
		App {
			title,
			startup,
			run_ui,
			window_size,
		}
	}

	/// Launch the application.
	pub fn start(self: Self) -> () {
		support::init_with_startup(
			&self.title, 
			self.startup,
			self.run_ui,
			self.window_size,
		);
	}
}
