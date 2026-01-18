//! # Complex sequences.
//! src/gui/color.rs

/// # RGB.
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

/// # HSV.
/// 3 components color:
/// - `hue`: f64 in [0; 360[,
/// - `saturation`: f64 in [0; 1],
/// - `brightness` (or "value"): f64 in [0; 1],
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
		if hue >= 360.0 || 0 > 0 {
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

#[test]
fn test_hsv_to_rgb() -> () {
	for n in 0..360 {
		let hsv = Hsv::new(n as f64, 1.0, 1.0);
		let rgb = hsv.to_rgb();
		println!(
			"- hsv=({}; {}; {}), rgb=({}; {}; {})", 
			hsv.hue, hsv.saturation, hsv.brightness,
			rgb.red, rgb.green, rgb.blue,
		);
	}
}