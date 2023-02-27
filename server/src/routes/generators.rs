use background_generator::triangle_generator::TriangleGeneratorMode;
use rand::Rng;
use rocket::{serde::json::Json, Route};
use viewmodels::generator_settings;

use crate::{responders::PngImage, viewmodels};

pub fn get_routes() -> impl Iterator<Item = Route> {
	routes![generate_triangles, generate_colorful, generate_circles]
		.into_iter()
		.map(|el|
			el.map_base(|base|
				format!("{}{}", "/api/generator", base)
			).unwrap()
		)
}

#[post("/triangle", data = "<settings>")]
async fn generate_triangles(settings: Json<generator_settings::Triangle>) -> PngImage {
	let generator_settings::Triangle {width, height, edge_count, color1, color2, seed, mode} = settings.0;
	let mode = match mode {
		0 => TriangleGeneratorMode::Quad,
		_ => TriangleGeneratorMode::Diagonal
	};
	
	background_generator::triangle_generator::generate(width, height, edge_count, color1, color2, seed, mode).into()
}

#[post("/colorful", data = "<settings>")]
async fn generate_colorful(settings: Json<generator_settings::Colorful>) -> PngImage {
	let settings = settings.0;
	
	background_generator::colorful_image_generator::generate(settings.level_of_detail, 10, None).into()
}
#[post("/circles", data = "<settings>")]
async fn generate_circles(settings: Json<generator_settings::Circle>) -> PngImage {
	let generator_settings::Circle {
		width, height, circle_count, max_circle_size, color1, color2, background_color, seed 
	} = settings.0;
	
	background_generator::circles_generator::generate(width, height, circle_count, max_circle_size, color1, color2, background_color, seed).into()
}