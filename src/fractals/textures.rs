//! # Complex sequences.
//! src/fractals/textures.rs

use std::{error, rc, cell};

use glium; 

use crate::structures::{configuration};

/// # `Fractal` texture trait: update and register.
pub trait Fractal {
	fn update_size(self: &mut Self, new_size: [u32; 2]) -> ();

	/// Generate and register the fractal texture.
	/// 
	/// Put some `size` if the size need to be updated.
	/// 
	/// Source: `imgui-examples`, `custom_texture`
	fn register_texture<Facade>(
        &mut self,
        gl_context: &Facade,
		global_settings: rc::Rc<cell::RefCell<configuration::GlobalSettings>>,
        textures: &mut imgui::Textures<imgui_glium_renderer::Texture>,
    ) -> Result<(), Box<dyn error::Error>>
    where
        Facade: glium::backend::Facade;

	/// Calls `window` method on `ui`, to display the texture. 
	/// 
	/// Source: `imgui-examples`, `custom_texture`
	fn show_textures(self: &Self, ui: &imgui::Ui, information_position: [f32; 2]) -> ();
}
