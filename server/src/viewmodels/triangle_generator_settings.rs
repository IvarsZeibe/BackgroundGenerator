use background_generator::triangle_generator::TriangleGeneratorMode;
use rocket::serde::{Deserialize};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TriangleGeneratorSettings {
    pub width: u32,
    pub height: u32,
    pub edge_count: u32,
    pub color1: Option<String>,
    pub color2: Option<String>,
    pub seed: Option<u64>,
    pub mode: u32
}
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ColorfulGeneratorSettings {
    pub level_of_detail: u32,
}