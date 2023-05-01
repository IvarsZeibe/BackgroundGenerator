use image::DynamicImage;
use rocket::{
	response::status::{Accepted, BadRequest, NotFound},
	serde::json::Json,
	Route,
};
use sea_orm::*;
use sea_orm_rocket::Connection;

use crate::{
	auth::Auth,
	models::{chains_generator_settings, generator_description},
	pools::Db,
	responders::PngImage,
	viewmodels::generator_settings::{self, Chains},
};

use super::{
	check_if_generator_limit_is_reached, delete_generator_description,
	modify_generator_description, save_generator_description,
};

pub fn get_routes() -> impl Iterator<Item = Route> {
	return routes![
		generate_chains,
		save_chains_generator_settings,
		get_chains_generator_settings,
		modify_chains_generator_settings,
		delete_chains_generator_settings
	]
	.into_iter()
	.map(|el| {
		el.map_base(|base| format!("{}{}", "/chains", base))
			.unwrap()
	});
}

fn generate(settings: generator_settings::Chains) -> Result<DynamicImage, BadRequest<()>> {
	let generator_settings::Chains {
		background_color,
		chain_count,
		color1,
		color2,
		height,
		circle_radius,
		spacing,
		seed,
		width,
	} = settings;

	let Ok(color1) = background_generator::hex_to_u8_color(color1) else { return Err(BadRequest(None)) };
	let Ok(color2) = background_generator::hex_to_u8_color(color2) else { return Err(BadRequest(None)) };
	let Ok(background_color) = background_generator::hex_to_u8_color(background_color) else { return Err(BadRequest(None)) };

	Ok(background_generator::chains_generator::generate(
		width,
		height,
		chain_count,
		circle_radius,
		spacing,
		color1,
		color2,
		background_color,
		seed as u64,
	))
}

#[post("/", data = "<settings>")]
async fn generate_chains(
	settings: Json<generator_settings::Chains>,
) -> Result<PngImage, BadRequest<()>> {
	Ok(PngImage::from(generate(settings.0)?))
}

#[post("/save", data = "<settings>")]
async fn save_chains_generator_settings(
	settings: Json<generator_settings::Settings<generator_settings::Chains>>,
	conn: Connection<'_, Db>,
	auth: Auth,
) -> Result<Accepted<String>, BadRequest<&'static str>> {
	let db = conn.into_inner();
	let generator_settings::Settings::<generator_settings::Chains> {
		name,
		description,
		generator_settings,
	} = settings.into_inner();

	let Ok(image) = generate(generator_settings.clone()) else { return Err(BadRequest(None))};

	check_if_generator_limit_is_reached(db, auth.user_id).await?;

	let generator_description =
		save_generator_description(db, auth.user_id, name, description, image, 2).await?;

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

	let settings = match settings.insert(db).await {
		Ok(settings) => settings,
		Err(error) => {
			generator_description.delete(db).await.unwrap();
			println!("{error}");
			return Err(BadRequest(None));
		}
	};
	Ok(Accepted(Some(settings.id)))
}

#[post("/<id>/save", data = "<settings>")]
async fn modify_chains_generator_settings(
	id: String,
	settings: Json<generator_settings::Settings<generator_settings::Chains>>,
	conn: Connection<'_, Db>,
	auth: Auth,
) -> Result<Accepted<()>, BadRequest<()>> {
	let db = conn.into_inner();
	let generator_settings::Settings::<generator_settings::Chains> {
		name,
		description,
		generator_settings,
	} = settings.into_inner();

	let image = generate(generator_settings.clone())?;

	modify_generator_description(db, id.clone(), auth.user_id, name, description, image).await?;

	let mut settings: chains_generator_settings::ActiveModel =
		chains_generator_settings::Entity::find_by_id(id)
			.one(db)
			.await
			.unwrap()
			.unwrap()
			.into();
	settings.width = Set(generator_settings.width);
	settings.height = Set(generator_settings.height);
	settings.chain_count = Set(generator_settings.chain_count);
	settings.circle_radius = Set(generator_settings.circle_radius);
	settings.spacing = Set(generator_settings.spacing);
	settings.color1 = Set(generator_settings.color1);
	settings.color2 = Set(generator_settings.color2);
	settings.background_color = Set(generator_settings.background_color);
	settings.seed = Set(generator_settings.seed);
	settings.save(db).await.unwrap();

	Ok(Accepted(None))
}

#[get("/<id>")]
async fn get_chains_generator_settings(
	id: String,
	auth: Auth,
	conn: Connection<'_, Db>,
) -> Result<Json<generator_settings::Settings<generator_settings::Chains>>, NotFound<()>> {
	let db = conn.into_inner();
	let generator_description = generator_description::Entity::find_by_id(id.clone())
		.one(db)
		.await
		.unwrap()
		.unwrap();
	if generator_description.user_id != auth.user_id {
		return Err(NotFound(()));
	}
	let settings = chains_generator_settings::Entity::find_by_id(id.clone())
		.one(db)
		.await
		.unwrap()
		.unwrap();
	let settings = Chains {
		color1: settings.color1,
		color2: settings.color2,
		chain_count: settings.chain_count,
		circle_radius: settings.circle_radius,
		spacing: settings.spacing,
		height: settings.height,
		background_color: settings.background_color,
		seed: settings.seed,
		width: settings.width,
	};
	return Ok(Json(generator_settings::Settings::<
		generator_settings::Chains,
	> {
		description: generator_description.description,
		name: generator_description.name,
		generator_settings: settings,
	}));
}

#[post("/<id>/delete")]
async fn delete_chains_generator_settings(
	id: String,
	conn: Connection<'_, Db>,
	auth: Auth,
) -> Result<Accepted<()>, BadRequest<()>> {
	let db = conn.into_inner();

	let settings: chains_generator_settings::ActiveModel =
		chains_generator_settings::Entity::find_by_id(id.clone())
			.one(db)
			.await
			.unwrap()
			.unwrap()
			.into();
	settings.delete(db).await.unwrap();

	delete_generator_description(db, id, auth.user_id).await?;

	Ok(Accepted(None))
}
