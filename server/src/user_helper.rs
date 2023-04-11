use rocket::response::status::{Accepted, BadRequest};
use sea_orm::*;
use sea_orm_rocket::Connection;

use crate::{
	models::{self, *},
	pools::Db,
};

pub async fn delete_user(
	conn: Connection<'_, Db>,
	user_id: i32,
) -> Result<Accepted<()>, BadRequest<&'static str>> {
	let db = conn.into_inner();
	let Some(user) = user::Entity::find_by_id(user_id)
		.one(db)
		.await
		.unwrap() else {
			return Err(BadRequest(Some("User not found")));
		};
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
