use rocket::{serde::json::Json, Route};
use sea_orm::*;
use sea_orm_rocket::{Connection};

use crate::{models::*, pools::Db, viewmodels, auth::Auth};

pub fn get_routes() -> impl Iterator<Item = Route> {
	routes![get_users].into_iter()
}

#[get("/api/users")]
async fn get_users(conn: Connection<'_, Db>, _auth: Auth) -> Json<Vec<viewmodels::UserData>> {
	let db = conn.into_inner();

	let users = user::Entity::find().all(db).await.unwrap();

	let users = users.into_iter()
		.map(|u| {
			let user::Model { id, email, password, is_admin } = u;
			viewmodels::UserData { id, email, password, is_admin }
		})
		.collect();
	Json(users)
}