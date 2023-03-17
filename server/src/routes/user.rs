use rocket::{futures, response::status, serde::json::Json, Route};
use sea_orm::*;
use sea_orm_rocket::Connection;
use std::fs;

use crate::{
	auth::Auth,
	models::{self, sea_orm_active_enums::PreferredTheme, *},
	pools::Db,
	viewmodels::{self, generator_settings::MyGenerator},
};

pub fn get_routes() -> impl Iterator<Item = Route> {
	routes![
		get_profile,
		get_preferred_theme,
		set_preferred_theme,
		get_my_generators
	]
	.into_iter()
}

#[get("/api/profile")]
async fn get_profile(
	conn: Connection<'_, Db>,
	auth: Auth,
) -> Result<Json<viewmodels::user_data::BasicUserData>, status::NotFound<()>> {
	let db = conn.into_inner();

	let user = user::Entity::find_by_id(auth.user_id)
		.one(db)
		.await
		.unwrap();

	if let Some(user::Model {
		id,
		email,
		password: _,
		is_admin,
		max_generators: _,
	}) = user
	{
		Ok(Json(viewmodels::user_data::BasicUserData {
			id,
			email,
			is_admin,
		}))
	} else {
		Err(status::NotFound(()))
	}
}

#[get("/api/profile/theme")]
async fn get_preferred_theme(
	conn: Connection<'_, Db>,
	auth: Auth,
) -> Result<String, status::NotFound<()>> {
	let db = conn.into_inner();

	let user = user::Entity::find_by_id(auth.user_id)
		.one(db)
		.await
		.unwrap();
	let user = user.unwrap();

	let settings = user
		.find_related(user_settings::Entity)
		.one(db)
		.await
		.unwrap();

	if let Some(settings) = settings {
		return Ok(settings.preferred_theme.to_string());
	} else {
		Err(status::NotFound(()))
	}
}

#[post("/api/profile/theme", data = "<preferred_theme>")]
async fn set_preferred_theme(preferred_theme: String, conn: Connection<'_, Db>, auth: Auth) {
	let db = conn.into_inner();

	let user = user::Entity::find_by_id(auth.user_id)
		.one(db)
		.await
		.unwrap();
	let user = user.unwrap();
	let mut settings: user_settings::ActiveModel = user
		.find_related(user_settings::Entity)
		.one(db)
		.await
		.unwrap()
		.unwrap()
		.into();

	if preferred_theme == "0" {
		settings.preferred_theme = Set(PreferredTheme::Light);
	} else if preferred_theme == "1" {
		settings.preferred_theme = Set(PreferredTheme::Dark);
	} else {
		settings.preferred_theme = Set(PreferredTheme::UseDeviceTheme);
	}
	settings.update(db).await.unwrap();
}

#[get("/api/myGenerators")]
async fn get_my_generators(conn: Connection<'_, Db>, auth: Auth) -> Json<Vec<MyGenerator>> {
	let db = conn.into_inner();
	let user = models::user::Entity::find_by_id(auth.user_id)
		.one(db)
		.await
		.unwrap()
		.unwrap();
	let generators = user
		.find_related(models::generator_description::Entity)
		.all(db)
		.await
		.unwrap();

	async fn to_my_generator(
		g: generator_description::Model,
		db: &DatabaseConnection,
	) -> MyGenerator {
		let generator_type = models::generator_type::Entity::find_by_id(g.generator_type)
			.one(db)
			.await
			.unwrap()
			.unwrap();

		let image = fs::read(format!("data/{}.jpg", g.id.clone())).unwrap_or(vec![]);
		MyGenerator {
			id: g.id,
			name: g.name,
			description: g.description,
			date_created: g.date_created,
			generator_type: generator_type.name,
			generator_code: generator_type.code,
			image: image,
		}
	}

	Json(futures::future::join_all(generators.into_iter().map(|g| to_my_generator(g, db))).await)
}
