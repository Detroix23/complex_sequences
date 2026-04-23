//! # Complex sequences.
//! src/fractals/geometry.rs

/// From a `pixel` position, on a screen of `size`, with camera `zoom` and `position`,
/// get the corresponding point in the complex plane.
pub fn position_from_pixel(
	pixel: [complex::Real; 2], 
	size: [complex::Real; 2],
	zoom: complex::Real,
	camera_position: [complex::Real; 2],
) -> [complex::Real; 2] {
	[
		(pixel[0] - size[0] / 2.0) / zoom - camera_position[0], 
		(pixel[1] - size[1] / 2.0) / zoom - camera_position[1],
	]
}
