use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Settings<T> {
    pub name: String,
    pub description: String,
    #[serde(rename = "generatorSettings")]
    pub generator_settings: T,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct MyGenerator {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "dateCreated")]
    pub date_created: NaiveDateTime,
    #[serde(rename = "generatorType")]
    pub generator_type: String,
    #[serde(rename = "generatorTypeCode")]
    pub generator_code: String,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Triangles {
    pub width: u32,
    pub height: u32,
    #[serde(rename = "edgeCount")]
    pub edge_count: u32,
    pub color1: String,
    pub color2: String,
    pub seed: u32,
    pub mode: u32,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Colorful {
    #[serde(rename = "levelOfDetail")]
    pub level_of_detail: u32,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Circles {
    pub width: u32,
    pub height: u32,
    #[serde(rename = "circleCount")]
    pub circle_count: u32,
    #[serde(rename = "maxCircleSize")]
    pub max_circle_size: u32,
    pub color1: String,
    pub color2: String,
    #[serde(rename = "backgroundColor")]
    pub background_color: String,
    pub seed: u32,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Chains {
    pub width: u32,
    pub height: u32,
    #[serde(rename = "chainCount")]
    pub chain_count: u32,
    #[serde(rename = "circleRadius")]
    pub circle_radius: u32,
    pub spacing: f32,
    pub color1: String,
    pub color2: String,
    #[serde(rename = "backgroundColor")]
    pub background_color: String,
    pub seed: u32,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Fractal {
    pub width: u32,
    pub height: u32,
    pub x: f64,
    pub y: f64,
}

// #[derive(Deserialize)]
// #[serde(crate = "rocket::serde")]
// pub struct Chains {
// 	pub width: u32,
// 	pub height: u32,
// 	#[serde(rename = "chainCount")]
// 	pub chain_count: u32,
// 	#[serde(rename = "circleRadius")]
// 	pub circle_radius: u32,
// 	pub spacing: f32,
// 	#[serde(rename = "backgroundColor")]
// 	pub background_color: [u8; 3],
// 	pub color1: [u8; 3],
// 	pub color2: [u8; 3],
// 	pub seed: u64
// }
