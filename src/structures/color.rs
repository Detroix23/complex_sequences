//! # Complex sequences.
//! src/gui/color.rs

use std::{fmt, convert};

/// # Float `Grayscale`.
/// Value:
/// - `0.0` is pitch black;
/// - `1.0` is full white.
pub struct Grayscale {
	pub value: f64
}

impl Grayscale {
	pub fn new(value: f64) -> Grayscale {
		Grayscale { value }
	}

	pub fn to_rgb(self: &Self) -> Rgb {
		Rgb::new(
			(255 as f64 * self.value) as u8,
			(255 as f64 * self.value) as u8,
			(255 as f64 * self.value) as u8,
		)
	}
}

/// # `Rgb`: red, green, blue.
#[derive(Debug, Clone, Copy)]
pub struct Rgb {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
}

impl Rgb {
	pub fn new(red: u8, green: u8, blue: u8) -> Rgb {
		Rgb { red, green, blue }
	}
}

/// # `Hsv`. 3 components color.
/// - `hue`: `f64` in [0; 360[; 0 and 360 are red.
/// - `saturation`: `f64` in [0; 1]; 0 is monochrome.
/// - `brightness` (or "value"): `f64` in [0; 1]; 1 is white.
#[derive(Debug, Clone, Copy)]
pub struct Hsv {
	pub hue: f64,
	pub saturation: f64,
	pub brightness: f64,
}

impl Hsv {
	/// Instantiate a new `Hsv` color. It has constraints, or panics:
	/// - `hue`: f64 in [0; 360[,
	/// - `saturation`: f64 in [0; 1],
	/// - `brightness`: f64 in [0; 1].
	pub fn new(hue: f64, saturation: f64, brightness: f64) -> Hsv {
		if hue > 360.0 || 0 > 0 {
			panic!("(X) gui::Hsv::new() `hue` ({}) must be in [0; 360[", hue);
		} if saturation > 1.0 || 0.0 > saturation {
			panic!("(X) gui::Hsv::new() `saturation` ({}) must be in [0; 1]", saturation);
		} if brightness > 1.0 || 0.0 > brightness {
			panic!("(X) gui::Hsv::new() `brightness` ({}) must be in [0; 1]", brightness);
		}

		Hsv {
			hue,
			saturation,
			brightness,
		}
	}

	/// Create a new `Rgb` color with the current `Hsv`. 
	/// 
	/// Formula from: [rapidtables](https://www.rapidtables.com/convert/color/hsv-to-rgb.html)
	pub fn to_rgb(self: &Self) -> Rgb {
		let c: f64 = self.brightness * self.saturation;
		let x = c * (1.0 - ((self.hue / 60.0) % 2.0 - 1.0).abs());
		let m = self.brightness - c;

		let (r, g, b) = if self.hue < 60.0 {
			(c, x, 0.0)
		} else if 60.0 <= self.hue && self.hue < 120.0 {
			(x, c, 0.0)
		} else if 120.0 <= self.hue && self.hue < 180.0 {
			(0.0, c, x)
		} else if 180.0 <= self.hue && self.hue < 240.0 {
			(0.0, x, c)
		} else if 240.0 <= self.hue && self.hue < 300.0 {
			(x, 0.0, c)
		} else {
			(c, 0.0, x)
		};

		// println!("c={}, x={}, m={}, r={}, g={}, b={}", c, x, m, r, g, b);

		Rgb { 
			red: ((r + m) * 255.0) as u8, 
			green: ((g + m) * 255.0) as u8, 
			blue: ((b + m) * 255.0) as u8, 
		}
	}
}


/// # `ColorMode`:
/// ```ignore,
/// 0. `GRAYSCALE`,
/// 1. `HSV`.
/// ``````
#[derive(Clone, Copy)]
pub enum ColorMode {
	GRAYSCALE,
	HSV,
}

impl ColorMode {
	/// Returns a vector of all `ColorMode`s.
	pub fn list() -> Vec<ColorMode> {
		vec![
			ColorMode::GRAYSCALE,
			ColorMode::HSV,
		]
	}

	fn to_static_str(self: &Self) -> &'static str {
		match &self {
			ColorMode::GRAYSCALE => "0. Grayscale.",
			ColorMode::HSV => "1. HSV.",
		}
	}
}

impl fmt::Display for ColorMode {
	fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(formatter, "Method::{}", match &self {
			ColorMode::GRAYSCALE => "Grayscale",
			ColorMode::HSV => "HSV",
		})
	}
}

impl convert::AsRef<str> for ColorMode {
	fn as_ref(self: &Self) -> &str {
		&self.to_static_str()
	}
}


#[test]
fn test_hsv_to_rgb() -> () {
	for d in 0..360 {
		let hsv = Hsv::new(d as f64, 1.0, 1.0);
		let rgb = hsv.to_rgb();
		println!(
			"- hsv=({}; {}; {}), rgb=({}; {}; {})", 
			hsv.hue, hsv.saturation, hsv.brightness,
			rgb.red, rgb.green, rgb.blue,
		);
	}
}

