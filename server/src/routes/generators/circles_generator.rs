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
	models::{circles_generator_settings, generator_description},
	pools::Db,
	responders::PngImage,
	viewmodels::generator_settings::{self, Circles},
};

use super::{
	check_if_generator_limit_is_reached, delete_generator_description,
	modify_generator_description, save_generator_description,
};

pub fn get_routes() -> impl Iterator<Item = Route> {
	return routes![
		generate_circles,
		save_circles_generator_settings,
		get_circles_generator_settings,
		modify_circles_generator_settings,
		delete_circles_generator_settings
	]
	.into_iter()
	.map(|el| {
		el.map_base(|base| format!("{}{}", "/circles", base))
			.unwrap()
	});
}

fn generate(settings: generator_settings::Circles) -> Result<DynamicImage, BadRequest<()>> {
	let generator_settings::Circles {
		circle_count,
		color1,
		color2,
		height,
		max_circle_size,
		seed,
		width,
	} = settings;

	let Ok(color1) = background_generator::hex_to_u8_color(color1) else { return Err(BadRequest(None)) };
	let Ok(color2) = background_generator::hex_to_u8_color(color2) else { return Err(BadRequest(None)) };

	Ok(background_generator::circles_generator::generate(
		width,
		height,
		circle_count,
		max_circle_size,
		color1,
		color2,
		seed as u64,
	))
}

#[post("/", data = "<settings>")]
async fn generate_circles(
	settings: Json<generator_settings::Circles>,
) -> Result<PngImage, BadRequest<()>> {
	Ok(PngImage::from(generate(settings.0)?))
}

#[post("/save", data = "<settings>")]
async fn save_circles_generator_settings(
	settings: Json<generator_settings::Settings<generator_settings::Circles>>,
	conn: Connection<'_, Db>,
	auth: Auth,
) -> Result<Accepted<()>, BadRequest<&'static str>> {
	let db = conn.into_inner();
	let generator_settings::Settings::<generator_settings::Circles> {
		name,
		description,
		generator_settings,
	} = settings.into_inner();

	let Ok(image) = generate(generator_settings.clone()) else { return Err(BadRequest(None))};

	check_if_generator_limit_is_reached(db, auth.user_id).await?;

	let generator_description =
		save_generator_description(db, auth.user_id, name, description, image, 1).await?;

	let settings = circles_generator_settings::ActiveModel {
		id: Set(generator_description.id.clone()),
		width: Set(generator_settings.width),
		height: Set(generator_settings.height),
		circle_count: Set(generator_settings.circle_count),
		max_circle_size: Set(generator_settings.max_circle_size),
		color1: Set(generator_settings.color1),
		color2: Set(generator_settings.color2),
		seed: Set(generator_settings.seed),
	};

	if let Err(error) = settings.insert(db).await {
		generator_description.delete(db).await.unwrap();
		println!("{error}");
		return Err(BadRequest(None));
	};
	Ok(Accepted(None))
}

#[post("/<id>/save", data = "<settings>")]
async fn modify_circles_generator_settings(
	id: String,
	settings: Json<generator_settings::Settings<generator_settings::Circles>>,
	conn: Connection<'_, Db>,
	auth: Auth,
) -> Result<Accepted<()>, BadRequest<()>> {
	let db = conn.into_inner();
	let generator_settings::Settings::<generator_settings::Circles> {
		name,
		description,
		generator_settings,
	} = settings.into_inner();

	let image = generate(generator_settings.clone())?;

	modify_generator_description(db, id.clone(), auth.user_id, name, description, image).await?;

	let mut settings: circles_generator_settings::ActiveModel =
		circles_generator_settings::Entity::find_by_id(id)
			.one(db)
			.await
			.unwrap()
			.unwrap()
			.into();
	settings.width = Set(generator_settings.width);
	settings.height = Set(generator_settings.height);
	settings.circle_count = Set(generator_settings.circle_count);
	settings.max_circle_size = Set(generator_settings.circle_count);
	settings.color1 = Set(generator_settings.color1);
	settings.color2 = Set(generator_settings.color2);
	settings.seed = Set(generator_settings.seed);
	settings.save(db).await.unwrap();

	Ok(Accepted(None))
}

#[get("/<id>")]
async fn get_circles_generator_settings(
	id: String,
	auth: Auth,
	conn: Connection<'_, Db>,
) -> Result<Json<generator_settings::Settings<generator_settings::Circles>>, NotFound<()>> {
	let db = conn.into_inner();
	let generator_description = generator_description::Entity::find_by_id(id.clone())
		.one(db)
		.await
		.unwrap()
		.unwrap();
	if generator_description.user_id != auth.user_id {
		return Err(NotFound(()));
	}
	let settings = circles_generator_settings::Entity::find_by_id(id.clone())
		.one(db)
		.await
		.unwrap()
		.unwrap();
	let settings = Circles {
		color1: settings.color1,
		color2: settings.color2,
		circle_count: settings.circle_count,
		max_circle_size: settings.max_circle_size,
		height: settings.height,
		seed: settings.seed,
		width: settings.width,
	};
	return Ok(Json(generator_settings::Settings::<
		generator_settings::Circles,
	> {
		description: generator_description.description,
		name: generator_description.name,
		generator_settings: settings,
	}));
}

#[post("/<id>/delete")]
async fn delete_circles_generator_settings(
	id: String,
	conn: Connection<'_, Db>,
	auth: Auth,
) -> Result<Accepted<()>, BadRequest<()>> {
	let db = conn.into_inner();

	let settings: circles_generator_settings::ActiveModel =
		circles_generator_settings::Entity::find_by_id(id.clone())
			.one(db)
			.await
			.unwrap()
			.unwrap()
			.into();
	settings.delete(db).await.unwrap();

	delete_generator_description(db, id, auth.user_id).await?;

	Ok(Accepted(None))
}
