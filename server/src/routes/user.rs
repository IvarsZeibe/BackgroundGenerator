use rocket::{
	response::status::{self, Accepted, BadRequest},
	serde::json::Json,
	Route,
};
use sea_orm::*;
use sea_orm_rocket::Connection;
use std::fs;

use crate::{
	auth::Auth,
	email_validator,
	models::{self, sea_orm_active_enums::PreferredTheme, *},
	password_helper,
	pools::Db,
	viewmodels::{
		self,
		user_data::{
			my_generators::{GeneratorDescription, MyGenerators},
			EmailChange, PasswordChange,
		},
	},
};

pub fn get_routes() -> impl Iterator<Item = Route> {
	routes![
		get_profile,
		get_preferred_theme,
		set_preferred_theme,
		get_my_generators,
		change_password,
		change_email,
		delete_profile,
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
		date_created: _,
		last_authorized: _,
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
async fn get_my_generators(conn: Connection<'_, Db>, auth: Auth) -> Json<MyGenerators> {
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

	let mut generator_descriptions = Vec::new();
	for generator in generators {
		let generator_type = models::generator_type::Entity::find_by_id(generator.generator_type)
			.one(db)
			.await
			.unwrap()
			.unwrap();
		let image = fs::read(format!("data/{}.jpg", generator.id)).unwrap_or(vec![]);
		generator_descriptions.push(GeneratorDescription {
			id: generator.id.to_string(),
			name: generator.name,
			description: generator.description,
			date_created: generator.date_created,
			date_modified: generator.date_modified,
			generator_type: generator_type.name,
			generator_code: generator_type.code,
			image,
		});
	}

	Json(MyGenerators {
		max_generators: user.max_generators,
		generator_descriptions,
	})
}

#[post("/api/profile/password", data = "<password_change>")]
async fn change_password(
	password_change: Json<PasswordChange>,
	conn: Connection<'_, Db>,
	auth: Auth,
) -> Result<Accepted<()>, BadRequest<&'static str>> {
	let db = conn.into_inner();
	let user = user::Entity::find_by_id(auth.user_id)
		.one(db)
		.await
		.unwrap()
		.unwrap();

	if !password_helper::is_password_correct(&user.password, &password_change.old_password) {
		return Err(BadRequest(None));
	}
	password_helper::is_valid(&password_change.new_password)?;

	let mut user: user::ActiveModel = user.into();
	user.password = Set(password_helper::hash_password(
		password_change.0.new_password,
	)?);
	user.save(db).await.unwrap();
	Ok(Accepted(None))
}

#[post("/api/profile/email", data = "<email_change>")]
async fn change_email(
	email_change: Json<EmailChange>,
	conn: Connection<'_, Db>,
	auth: Auth,
) -> Result<Accepted<()>, BadRequest<&'static str>> {
	let db = conn.into_inner();
	email_validator::validate_email(&email_change.email, &db).await?;

	let user = user::Entity::find_by_id(auth.user_id)
		.one(db)
		.await
		.unwrap()
		.unwrap();

	let mut user: user::ActiveModel = user.into();
	user.email = Set(email_change.0.email);
	user.save(db).await.unwrap();
	Ok(Accepted(None))
}

#[post("/api/profile/delete")]
async fn delete_profile(
	conn: Connection<'_, Db>,
	auth: Auth,
) -> Result<Accepted<()>, BadRequest<&'static str>> {
	let db = conn.into_inner();
	let user = user::Entity::find_by_id(auth.user_id)
		.one(db)
		.await
		.unwrap()
		.unwrap();
	let generator_descriptions: Vec<models::generator_description::Model> = user
		.find_related(generator_description::Entity)
		.all(db)
		.await
		.unwrap();
	for generator_description in generator_descriptions {
		match generator_description.generator_type {
			0 => {
				let settings = triangles_generator_settings::Entity::find_by_id(
					generator_description.id.clone(),
				)
				.one(db)
				.await
				.unwrap()
				.unwrap();
				let settings: triangles_generator_settings::ActiveModel = settings.into();
				settings.delete(db).await.unwrap();
			}
			1 => {
				let settings = circles_generator_settings::Entity::find_by_id(
					generator_description.id.clone(),
				)
				.one(db)
				.await
				.unwrap()
				.unwrap();
				let settings: circles_generator_settings::ActiveModel = settings.into();
				settings.delete(db).await.unwrap();
			}
			2 => {
				let settings =
					chains_generator_settings::Entity::find_by_id(generator_description.id.clone())
						.one(db)
						.await
						.unwrap()
						.unwrap();
				let settings: chains_generator_settings::ActiveModel = settings.into();
				settings.delete(db).await.unwrap();
			}
			_ => {
				panic!("Unknown generator type");
			}
		};
		std::fs::remove_file(format!("data/{}.jpg", generator_description.id)).unwrap();
		let generator_description: models::generator_description::ActiveModel =
			generator_description.into();
		generator_description.delete(db).await.unwrap();
	}

	let user_settings = user
		.find_related(user_settings::Entity)
		.one(db)
		.await
		.unwrap()
		.unwrap();
	let user_settings: user_settings::ActiveModel = user_settings.into();
	user_settings.delete(db).await.unwrap();

	let user: user::ActiveModel = user.into();
	user.delete(db).await.unwrap();
	Ok(Accepted(None))
}
