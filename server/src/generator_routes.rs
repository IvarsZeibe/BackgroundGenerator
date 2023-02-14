use background_generator::triangle_generator::TriangleGeneratorMode;
use rand::Rng;
use rocket::{serde::json::Json, Route};

use crate::{responders::PngImage, viewmodels};

pub fn get_routes() -> Vec<Route> {
	routes![generate_triangles, generate_colorful, generate_circles]
}

#[post("/triangle", data = "<settings>")]
async fn generate_triangles(settings: Json<viewmodels::TriangleGeneratorSettings>) -> PngImage {
	let viewmodels::TriangleGeneratorSettings {width, height, edge_count, color1, color2, seed, mode} = settings.0;
	let mode = match mode {
		0 => TriangleGeneratorMode::Quad,
		_ => TriangleGeneratorMode::Diagonal
	};
	
	background_generator::triangle_generator::generate(width, height, edge_count, color1, color2, seed, mode).into()
}

#[post("/colorful", data = "<settings>")]
async fn generate_colorful(settings: Json<viewmodels::ColorfulGeneratorSettings>) -> PngImage {
	let settings = settings.0;
	
	background_generator::colorful_image_generator::generate(settings.level_of_detail, 10, None).into()
}
#[post("/circles", data = "<settings>")]
async fn generate_circles(settings: Option<Json<Option<i32>>>) -> PngImage {
	// let settings = settings.0;
	
	background_generator::circles_generator::generate(512, 512, 4000, 100, 100, rand::thread_rng().gen()).into()
	// background_generator::circles_generator::generate(512, 512, 1000, 100, rand::thread_rng().gen_range(0..=255), rand::thread_rng().gen()).into()
}