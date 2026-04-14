//! # Complex sequences.
//! src/gui/defaults.rs

use std::{cell, error, rc};

use imgui;
use glium::{self, backend};
use glium::backend::Facade;

use crate::structures::{color, configuration};
use crate::support::rendering;

/// # Draw a `Grid` atop of a fractal texture.
/// 
/// The `Grid` is centered, (0; 0) is a the center. 
pub struct Grid {
	color_main: color::Rgb,
	/// `0`: invisible, `255`: opaque.
	transparency: u8,
	thickness: u32,
	size: [u32; 2],
	center: [u32; 2],
	texture_id: Option<imgui::TextureId>,
}

impl Grid {
	pub fn new(color_main: color::Rgb, transparency: u8, thickness: u32) -> rc::Rc<cell::RefCell<Grid>> {
		rc::Rc::new(cell::RefCell::new(Grid {
			color_main,
			transparency,
			thickness,
			size: [0, 0],
			center: [0, 0],
			texture_id: Option::None,
		}))
	}

	/// Update `size` and `center`. Returns if `size` changed.
	pub fn update_size(self: &mut Self, new_size: [u32; 2]) -> bool {
		let has_changed: bool = self.size != new_size; 
		self.size = new_size;
		self.center = [new_size[0] / 2, new_size[1] / 2];

		has_changed
	}

	/// Register a RGBA grid.
	pub fn register_texture<Facade>(
        &mut self,
        gl_context: &Facade,
        textures: &mut imgui::Textures<imgui_glium_renderer::Texture>,
    ) -> Result<(), Box<dyn error::Error>>
    where
        Facade: backend::Facade,
    {	
		let mut data: Vec<u8> = Vec::with_capacity((self.size[0] * self.size[1]) as usize);
		for y in 0..self.size[1] {
			for x in 0..self.size[0] {
				if x.abs_diff(self.center[0]) < self.thickness
					|| y.abs_diff(self.center[1]) < self.thickness
				{
					data.push(self.color_main.red);
					data.push(self.color_main.green);
					data.push(self.color_main.blue);
					data.push(self.transparency);
				} else {
					data.push(0);
					data.push(0);
					data.push(0);
					data.push(0);
				}
			}
		} 

		self.texture_id = Option::Some(rendering::render_texture(
			self.texture_id, 
			data, 
			[self.size[0] as usize, self.size[1] as usize], 
			gl_context, 
			textures, 
			rendering::ColorFormat::RGBA,
		).expect("(X) Grid.register_texture() render texture error !"));

		Result::Ok(())
	}

	pub fn show_textures(&self, ui: &imgui::Ui) -> () {
		let draw_list_background: imgui::DrawListMut<'_> = ui.get_background_draw_list();

		// Render `Image` in the draw list.
		if let Some(texture_id) = self.texture_id {
			draw_list_background
				.add_image(
					texture_id, 
					[0.0, 0.0], 
					[self.size[0] as f32, self.size[1] as f32]
				).build();
		}
	}
}

/// Draw a frame for `Grid`.
pub fn draw(
	_settings_state: rc::Rc<cell::RefCell<configuration::GlobalSettings>>,
	ui: &imgui::Ui,
	// Rc<RefCell<Divergent<impl Fn(Algebraic, Algebraic) -> Algebraic>>>
	grid: rc::Rc<cell::RefCell<Grid>>,
	_renderer: &mut imgui_glium_renderer::Renderer, 
	_display: &glium::Display<glium::glutin::surface::WindowSurface>,
) -> () {
	grid.borrow()
		.show_textures(ui);
}

pub fn update(
	grid: rc::Rc<cell::RefCell<Grid>>,
	_ui: &imgui::Ui,
	renderer: &mut imgui_glium_renderer::Renderer, 
	display: &glium::Display<glium::glutin::surface::WindowSurface>,
	window_size: [u32; 2],
) -> () {
	let need_update: bool;

	need_update = grid.borrow_mut()
		.update_size(window_size);

	if need_update {
		grid.borrow_mut()
			.register_texture(display.get_context(), renderer.textures())
			.expect("(X) update() Error registering texture.");
	}
}