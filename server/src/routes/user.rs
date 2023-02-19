use rocket::{serde::json::Json, response::status::{self}, Route};
use sea_orm::*;
use sea_orm_rocket::{Connection};

use crate::{models::*, pools::Db, viewmodels, auth::Auth};

pub fn get_routes() -> impl Iterator<Item = Route> {
	routes![get_profile].into_iter()
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