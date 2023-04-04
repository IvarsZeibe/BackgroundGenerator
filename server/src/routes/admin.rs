use regex::Regex;
use rocket::{
	futures,
	response::{content, status},
	serde::json::Json,
	Route,
};
use sea_orm::*;
use sea_orm_rocket::Connection;

use crate::{admin_auth::AdminAuth, models::*, pools::Db, viewmodels};

pub fn get_routes() -> impl Iterator<Item = Route> {
	routes![get_users, update_user, delete_all_user_generators].into_iter()
}

#[get("/api/users")]
async fn get_users(
	conn: Connection<'_, Db>,
	_auth: AdminAuth,
) -> Json<Vec<viewmodels::user_data::DetailedUserData>> {
	let db = conn.into_inner();

	let users = user::Entity::find().all(db).await.unwrap();

	async fn get_user_info(
		db: &DatabaseConnection,
		user: user::Model,
	) -> viewmodels::user_data::DetailedUserData {
		let saved_generators_count = user
			.find_related(generator_description::Entity)
			.count(db)
			.await
			.unwrap();

		viewmodels::user_data::DetailedUserData {
			id: user.id,
			email: user.email,
			is_admin: user.is_admin,
			max_generators: user.max_generators,
			generators_saved: saved_generators_count as i32,
			date_created: user.date_created,
			last_authorized: user.last_authorized,
		}
	}

	let users = futures::future::join_all(users.into_iter().map(|u| get_user_info(db, u))).await;
	Json(users)
}

#[post("/api/users/<old_id>", data = "<new_user_data>")]
async fn update_user(
	conn: Connection<'_, Db>,
	_auth: AdminAuth,
	old_id: i32,
	new_user_data: Json<viewmodels::user_data::UpdateUser>,
) -> Result<status::Accepted<()>, status::BadRequest<content::RawJson<&'static str>>> {
	fn new_error(
		json: &'static str,
	) -> Result<status::Accepted<()>, status::BadRequest<content::RawJson<&'static str>>> {
		Err(status::BadRequest(Some(content::RawJson(json))))
	}
	let db = conn.into_inner();

	let user = user::Entity::find_by_id(old_id).one(db).await.unwrap();

	let user = match user {
		None => return new_error("{\"error\": \"User not found\"}"),
		Some(u) => u,
	};

	if new_user_data.id <= 0 {
		return new_error("{\"id\": \"Must be atleast 1\"}");
	}
	if new_user_data.max_generators < 0 {
		return new_error("{\"id\": \"Must be atleast 0\"}");
	}
	if new_user_data.id != old_id
		&& user::Entity::find_by_id(new_user_data.id)
			.one(db)
			.await
			.unwrap()
			.is_some()
	{
		return new_error("{\"id\": \"Must must be unique\"}");
	}

	let email_pattern = Regex::new(r"^[a-zA-Z0-9.!#$%&’*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,253}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,253}[a-zA-Z0-9])?)*$").unwrap();
	if !email_pattern.is_match(&new_user_data.email) {
		return new_error("{\"email\": \"Invalid email format\"}");
	}
	if new_user_data.email != user.email
		&& user::Entity::find()
			.filter(user::Column::Email.eq(new_user_data.email.clone()))
			.one(db)
			.await
			.unwrap()
			.is_some()
	{
		return new_error("{\"email\": \"Email must be unique\"}");
	}

	if new_user_data.password.len() != 0 && new_user_data.password.len() < 8 {
		return new_error("{\"password\": \"Password must be alteast 8 characters\"}");
	}

	let mut user: user::ActiveModel = user.into();
	user.id = Set(new_user_data.id);
	user.email = Set(new_user_data.email.clone());
	if new_user_data.password.len() != 0 {
		user.password = Set(new_user_data.password.clone());
	}
	user.is_admin = Set(new_user_data.is_admin);
	user.max_generators = Set(new_user_data.max_generators);
	user.update(db).await.unwrap();
	return Ok(status::Accepted(None));
}

#[post("/api/deleteAllUserGenerators/<user_id>")]
async fn delete_all_user_generators(
	conn: Connection<'_, Db>,
	_auth: AdminAuth,
	user_id: i32,
) -> Result<status::Accepted<()>, status::BadRequest<content::RawJson<&'static str>>> {
	let db = conn.into_inner();

	let user = user::Entity::find_by_id(user_id).one(db).await.unwrap();

	if user.is_none() {
		return Err(status::BadRequest(Some(content::RawJson(
			"{\"error\": \"User not found\"}",
		))));
	}

	let generators = generator_description::Entity::find()
		.filter(generator_description::Column::UserId.eq(user_id))
		.all(db)
		.await
		.unwrap();

	for generator in generators {
		match generator.generator_type {
			0 => {
				let triangles_generator_settings =
					triangles_generator_settings::Entity::find_by_id(generator.id.clone())
						.one(db)
						.await
						.unwrap()
						.unwrap();
				let triangles_generator_settings: triangles_generator_settings::ActiveModel =
					triangles_generator_settings.into();
				triangles_generator_settings.delete(db).await.unwrap();
			}
			1 => {
				let circles_generator_settings =
					circles_generator_settings::Entity::find_by_id(generator.id.clone())
						.one(db)
						.await
						.unwrap()
						.unwrap();
				let circles_generator_settings: circles_generator_settings::ActiveModel =
					circles_generator_settings.into();
				circles_generator_settings.delete(db).await.unwrap();
			}
			2 => {
				let chains_generator_settings =
					chains_generator_settings::Entity::find_by_id(generator.id.clone())
						.one(db)
						.await
						.unwrap()
						.unwrap();
				let chains_generator_settings: chains_generator_settings::ActiveModel =
					chains_generator_settings.into();
				chains_generator_settings.delete(db).await.unwrap();
			}
			_ => panic!("Unknown generator type"),
		};
		let generator: generator_description::ActiveModel = generator.into();
		generator.delete(db).await.unwrap();
	}

	Ok(status::Accepted(None))
}
