use background_generator::triangles_generator::TriangleGeneratorMode;
use chrono::Local;
use rocket::{serde::json::Json, Route, response::status::{BadRequest, Accepted, NotFound}};
use sea_orm_rocket::Connection;
use sea_orm::*;

use crate::{responders::PngImage, viewmodels::generator_settings::{self, MyGenerator, Triangles}, pools::Db, auth::Auth, models::{generator_description, triangles_generator_settings, self}};

use super::save_generator_description;

pub fn get_routes() -> impl Iterator<Item = Route> {
	return routes![generate_triangles, save_triangles_generator_settings, get_triangles_generator_settings]
		.into_iter()
		.map(|el|
			el.map_base(|base|
				format!("{}{}", "/triangles", base)
			).unwrap()
		);
}

#[post("/", data = "<settings>")]
async fn generate_triangles(settings: Json<generator_settings::Triangles>) -> Result<PngImage, BadRequest<()>> {
	let generator_settings::Triangles {
		width, height, edge_count,
		color1, color2, seed, mode
	} = settings.0;

	let mode = match mode {
		0 => TriangleGeneratorMode::Quad,
		_ => TriangleGeneratorMode::Diagonal
	};

	let Ok(color1) = background_generator::hex_to_u8_color(color1) else { return Err(BadRequest(None)) };
	let Ok(color2) = background_generator::hex_to_u8_color(color2) else { return Err(BadRequest(None)) };
	
	Ok(background_generator::triangles_generator::generate(width, height, edge_count, color1, color2, seed as u64, mode).into())
}

#[post("/save", data = "<settings>")]
async fn save_triangles_generator_settings(
	settings: Json<generator_settings::Settings<generator_settings::Triangles>>,
	conn: Connection<'_, Db>,
	auth: Auth) ->
	Result<Accepted<()>, BadRequest<()>>
{
	let db = conn.into_inner();
	let generator_settings::Settings::<generator_settings::Triangles> {
		name, description, generator_settings
	} = settings.into_inner();

	let Ok(generator_description) = save_generator_description(db, auth.user_id, name, description, 0).await else { return Err(BadRequest(None)); };
	
	let settings = triangles_generator_settings::ActiveModel {
		id: Set(generator_description.id.clone()),
		width: Set(generator_settings.width),
		height: Set(generator_settings.height),
		edge_count: Set(generator_settings.edge_count),
		color1: Set(generator_settings.color1),
		color2: Set(generator_settings.color2),
		seed: Set(generator_settings.seed),
		mode: Set(generator_settings.mode),
	};

	if let Err(error) = settings.insert(db).await {
		generator_description.delete(db).await.unwrap();
		println!("{error}");
		return Err(BadRequest(None));
	};
	Ok(Accepted(None))
}

#[get("/<id>")]
async fn get_triangles_generator_settings(id: String, auth: Auth, conn: Connection<'_, Db>)
	-> Result<Json<generator_settings::Triangles>, NotFound<()>>
{
	let db = conn.into_inner();
	if generator_description::Entity::find_by_id(id.clone()).one(db).await.unwrap().unwrap().user_id != auth.user_id {
		return Err(NotFound(()));
	}
	let settings = triangles_generator_settings::Entity::find_by_id(id.clone()).one(db).await.unwrap().unwrap();
	let settings = Triangles {
		color1: settings.color1,
		color2: settings.color2,
		edge_count: settings.edge_count,
		height: settings.height,
		mode: settings.mode,
		seed: settings.seed,
		width: settings.width
	};
	return Ok(Json(settings));
}