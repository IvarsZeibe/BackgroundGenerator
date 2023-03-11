use background_generator::triangles_generator::TriangleGeneratorMode;
use chrono::Local;
use rocket::{serde::json::Json, Route};
use sea_orm_rocket::Connection;
use sea_orm::*;

mod triangles_generator;
mod circles_generator;
mod chains_generator;

use crate::{responders::PngImage, viewmodels::{generator_settings::{self, MyGenerator}, self, generator_type::GeneratorType}, pools::Db, auth::Auth, models::{generator_description, triangles_generator_settings, self, generator_type}};

pub fn get_routes() -> impl Iterator<Item = Route> {
	return triangles_generator::get_routes()
		.chain(circles_generator::get_routes())
		.chain(chains_generator::get_routes())
		.map(|el|
			el.map_base(|base|
				format!("{}{}", "/api/generator", base)
			).unwrap()
		)
		.chain(routes![get_generator_types]);
}

async fn save_generator_description(db: &DatabaseConnection, user_id: i32, name: String, description: String, generator_type: i32) -> Result<(generator_description::Model), DbErr> {
	let generator = generator_description::ActiveModel {
		id: Set(uuid::Uuid::now_v7().to_string()),
		name: Set(name),
		description: Set(description),
		user_id: Set(user_id),
		date_created: Set(Local::now().naive_local()),
		generator_type: Set(generator_type),
		..Default::default()
	};
	Ok(generator.insert(db).await?)
}

#[get("/api/generators")]
async fn get_generator_types(conn: Connection<'_, Db>) -> Json<Vec<GeneratorType>> {
	let db = conn.into_inner();
	let types = generator_type::Entity::find().all(db).await.unwrap();
	Json(types.into_iter().map(|t| {
		GeneratorType {
			code: t.code,
			name: t.name
		}
	}).collect())	
}