//! # Complex sequences.
//! src/gui/defaults.rs
//! 
//! Defaults settings for the `App`.

use glium;
use imgui;

use crate::gui::app;

/// Launch the default configuration for `App`. 
pub fn launch_default() -> () {	
	// &mut imgui::Context, &mut imgui_glium_renderer::Renderer, &glium::Display<glium::glutin::surface::WindowSurface>
	let startup = |
		context: &mut imgui::Context, 
		renderer: &mut imgui_glium_renderer::Renderer, 
		display: &glium::Display<glium::glutin::surface::WindowSurface>,
	| {

	};

	let run_ui = |_, ui: &mut imgui::Ui| {
		
		// Window: settings.
		ui.window("Settings.")
			.size([150.0, 250.0], imgui::Condition::FirstUseEver)
			.position([100.0, 100.0], imgui::Condition::FirstUseEver)
			.build(|| {
				ui.text_wrapped("This is the settings window.");	
			});

	};
	
	
	let my_app = app::App::new( 
		"Complex sequences. ".to_string(), 
		startup, 
		run_ui,
		[1024, 768],
	);

	my_app.start();
}

/* Texture init.
```
let my_app = std::rc::Rc::new(std::cell::RefCell::new(CustomTexturesApp::default()));
let app_clone = my_app.clone();

support::init_with_startup(
	file!(),
	move |_ctx, renderer, display| {
		app_clone
			.borrow_mut()
			.register_textures(display.get_context(), renderer.textures())
			.expect("Failed to register textures");
	},
	move |_, ui| {
		my_app.borrow_mut().show_textures(ui);
	},
);
```
*/