use rocket::serde::{Deserialize};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Triangle {
	pub width: u32,
	pub height: u32,
	#[serde(rename = "edgeCount")]
	pub edge_count: u32,
	pub color1: [u8; 3],
	pub color2: [u8; 3],
	pub seed: u64,
	pub mode: u32
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Colorful {
	#[serde(rename = "levelOfDetail")]
	pub level_of_detail: u32,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Circle {
	pub width: u32,
	pub height: u32,
	#[serde(rename = "circleCount")]
	pub circle_count: u32,
	#[serde(rename = "maxCircleSize")]
	pub max_circle_size: u32,
	pub color1: [u8; 3],
	pub color2: [u8; 3],
	#[serde(rename = "backgroundColor")]
	pub background_color: [u8; 3],
	pub seed: u64,
}