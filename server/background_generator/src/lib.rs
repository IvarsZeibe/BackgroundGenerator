use std::num::ParseIntError;

pub mod chains_generator;
pub mod circles_generator;
pub mod triangles_generator;

// converts #FFFFFF to [255, 255, 255]
pub fn hex_to_u8_color(color: String) -> Result<[u8; 3], ParseIntError> {
	Ok([
		u8::from_str_radix(&color[1..3], 16)?,
		u8::from_str_radix(&color[3..5], 16)?,
		u8::from_str_radix(&color[5..7], 16)?,
	])
}
