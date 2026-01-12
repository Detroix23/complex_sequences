//! # Complex sequences.
//! src/gui/defaults.rs
//! 
//! Defaults settings for the `App`.

use glium;
use glium::backend::Facade;
use imgui;

use crate::support;
use crate::fractals;


/// Launch the default configuration for `App`. 
pub fn launch_default() -> () {	
	// Workers.
	let fractal_texture = fractals::textures::FractalTexture::new(
		|z, c| { z * z + c },
		[600.0, 600.0], 
		1,
		0.08,
		50,
		2.0,
		[0, 5, 15],
		[255, 250, 240],
	);
	// Necessary for the closure.
	let fractal_texture_startup = fractal_texture.clone();
	let fractal_texture_update = fractal_texture.clone();


	// True start.
	support::init_with_startup( 
		"Complex sequences. ", 
		[1024, 768],

		move |
			_context: &mut imgui::Context, 
			renderer: &mut imgui_glium_renderer::Renderer, 
			display: &glium::Display<glium::glutin::surface::WindowSurface>,
		| {
			println!("(?) gui::defaults::launch_default() Initialized.");

			fractal_texture_startup
				.borrow_mut()
				.register_texture(display.get_context(), renderer.textures())
				.expect("(!) gui::default::launch_default() startup: can't register texture.");
		}, 

		move |
			_, 
			ui: &mut imgui::Ui,
			renderer: &mut imgui_glium_renderer::Renderer, 
			display: &glium::Display<glium::glutin::surface::WindowSurface>,
		| {
			// Window: settings.
			ui.window("Settings.")
				.size([150.0, 250.0], imgui::Condition::FirstUseEver)
				.position([100.0, 100.0], imgui::Condition::FirstUseEver)
				.build(|| {
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
                    ui.slider_config("Zoom", 0.00000001, 1.0)
                        .flags(imgui::SliderFlags::LOGARITHMIC)
                        .build(&mut fractal_texture.borrow_mut().zoom);

					// (x; y).
					ui.slider_config("x", -1000.0, 1000.0)
                        .build(&mut fractal_texture.borrow_mut().position[0]);
					ui.slider_config("y", -1000.0, 1000.0)
                        .build(&mut fractal_texture.borrow_mut().position[1]);

					// Iterations.
					ui.slider_config("Iteration", 1_usize, 250_usize)
						.build(&mut fractal_texture.borrow_mut().iterations);

					// Threshold for divergence.
					ui.slider_config("Threshold", 0.0, 5.0)
						.build(&mut fractal_texture.borrow_mut().threshold);

				});
			

			// Fractal graphics.
			fractal_texture
				.borrow_mut()
				.show_textures(ui);
		},

		move |
			renderer: &mut imgui_glium_renderer::Renderer, 
			display: &glium::Display<glium::glutin::surface::WindowSurface>,
		| {
			// Fractal controls update.
			if fractal_texture_update.borrow_mut().is_state_updated() {
				fractal_texture_update.borrow_mut()
					.register_texture(display.get_context(), renderer.textures())
					.expect("(!) gui::default::launch_default() update: can't register texture.");
			}
		},
	);
}
