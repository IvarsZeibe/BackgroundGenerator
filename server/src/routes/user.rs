use rocket::{serde::json::Json, response::status::{self}, Route};
use sea_orm::*;
use sea_orm_rocket::{Connection};

use crate::{models::{*, settings::PreferredTheme}, pools::Db, viewmodels, auth::Auth};

pub fn get_routes() -> impl Iterator<Item = Route> {
	routes![get_profile, get_preferred_theme, set_preferred_theme].into_iter()
}

#[get("/api/profile")]
async fn get_profile(conn: Connection<'_, Db>, auth: Auth) -> Result<Json<viewmodels::ViewableUserData>, status::NotFound<()>> {
	let db = conn.into_inner();

	let user = user::Entity::find_by_id(auth.user_id).one(db).await.unwrap();

	if let Some(user::Model { id, email, password: _, is_admin }) = user {
		Ok(Json(viewmodels::ViewableUserData { id, email, is_admin }))
	} else {
		Err(status::NotFound(()))
	}
}

#[get("/api/profile/theme")]
async fn get_preferred_theme(conn: Connection<'_, Db>, auth: Auth) -> Result<String, status::NotFound<()>> {
	let db = conn.into_inner();

	let user = user::Entity::find_by_id(auth.user_id).one(db).await.unwrap();
	let user = user.unwrap();

	let settings = user.find_related(settings::Entity).one(db).await.unwrap();

	if let Some(settings) = settings {
		return Ok(settings.preferred_theme.to_string());
	} else {
		Err(status::NotFound(()))
	}
}

#[post("/api/profile/theme", data = "<preferred_theme>")]
async fn set_preferred_theme(preferred_theme: String, conn: Connection<'_, Db>, auth: Auth) {
	let db = conn.into_inner();

	let user = user::Entity::find_by_id(auth.user_id).one(db).await.unwrap();
	let user = user.unwrap();
	let settings = user.find_related(settings::Entity).one(db).await.unwrap();

	let mut settings: settings::ActiveModel = match settings {
		Some(s) => s.into(),
		None => {
			let settings = settings::ActiveModel {
				user_id: Set(user.id),
				// id: Set(user.id),
				preferred_theme: Set(PreferredTheme::Light),
				..Default::default()
			};
			settings.insert(db).await.unwrap().into()
		}
	};
	if preferred_theme == "0" {
		settings.preferred_theme = Set(PreferredTheme::Light);
	} else if preferred_theme == "1" {
		settings.preferred_theme = Set(PreferredTheme::Dark);
	} else {
		settings.preferred_theme = Set(PreferredTheme::UseDeviceTheme);
	}
	settings.update(db).await.unwrap();
}