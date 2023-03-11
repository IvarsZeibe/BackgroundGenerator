use background_generator::triangles_generator::TriangleGeneratorMode;
use chrono::Local;
use rocket::{serde::json::Json, Route, response::status::{BadRequest, Accepted, NotFound}};
use sea_orm_rocket::Connection;
use sea_orm::*;

use crate::{responders::PngImage, viewmodels::generator_settings::{self, MyGenerator, Chains}, pools::Db, auth::Auth, models::{generator_description, triangles_generator_settings, self, chains_generator_settings}};

use super::save_generator_description;

pub fn get_routes() -> impl Iterator<Item = Route> {
	return routes![generate_chains, save_chains_generator_settings, get_chains_generator_settings]
		.into_iter()
		.map(|el|
			el.map_base(|base|
				format!("{}{}", "/chains", base)
			).unwrap()
		);
}

#[post("/", data = "<settings>")]
async fn generate_chains(settings: Json<generator_settings::Chains>) -> Result<PngImage, BadRequest<()>> {
	let generator_settings::Chains {
		background_color, chain_count, color1,
		color2, height, circle_radius, spacing, seed, width
	} = settings.0;

	let Ok(color1) = background_generator::hex_to_u8_color(color1) else { return Err(BadRequest(None)) };
	let Ok(color2) = background_generator::hex_to_u8_color(color2) else { return Err(BadRequest(None)) };
	let Ok(background_color) = background_generator::hex_to_u8_color(background_color) else { return Err(BadRequest(None)) };
	
	Ok(background_generator::chains_generator::generate(width, height, chain_count, circle_radius, spacing, color1, color2, background_color, seed as u64).into())
}

#[post("/save", data = "<settings>")]
async fn save_chains_generator_settings(
	settings: Json<generator_settings::Settings<generator_settings::Chains>>,
	conn: Connection<'_, Db>,
	auth: Auth) ->
	Result<Accepted<()>, BadRequest<()>>
{
	let db = conn.into_inner();
	let generator_settings::Settings::<generator_settings::Chains> {
		name, description, generator_settings
	} = settings.into_inner();

	let Ok(generator_description) = save_generator_description(db, auth.user_id, name, description, 2).await else { return Err(BadRequest(None)); };
	
	let settings = chains_generator_settings::ActiveModel {
		id: Set(generator_description.id.clone()),
		width: Set(generator_settings.width),
		height: Set(generator_settings.height),
		chain_count: Set(generator_settings.chain_count),
		circle_radius: Set(generator_settings.circle_radius),
		spacing: Set(generator_settings.spacing),
		color1: Set(generator_settings.color1),
		color2: Set(generator_settings.color2),
		background_color: Set(generator_settings.background_color),
		seed: Set(generator_settings.seed),
	};

	if let Err(error) = settings.insert(db).await {
		generator_description.delete(db).await.unwrap();
		println!("{error}");
		return Err(BadRequest(None));
	};
	Ok(Accepted(None))
}

#[get("/<id>")]
async fn get_chains_generator_settings(id: String, auth: Auth, conn: Connection<'_, Db>)
	-> Result<Json<generator_settings::Chains>, NotFound<()>>
{
	let db = conn.into_inner();
	if generator_description::Entity::find_by_id(id.clone()).one(db).await.unwrap().unwrap().user_id != auth.user_id {
		return Err(NotFound(()));
	}
	let settings = chains_generator_settings::Entity::find_by_id(id.clone()).one(db).await.unwrap().unwrap();
	let settings = Chains {
		color1: settings.color1,
		color2: settings.color2,
		chain_count: settings.chain_count,
		circle_radius: settings.circle_radius,
		spacing: settings.spacing,
		height: settings.height,
		background_color: settings.background_color,
		seed: settings.seed,
		width: settings.width
	};
	return Ok(Json(settings));
}