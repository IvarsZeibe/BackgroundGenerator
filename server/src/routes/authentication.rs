use rocket::{serde::json::Json, http::{CookieJar, Cookie}, State, response::status::{self}, Route};
use sea_orm::*;
use sea_orm_rocket::{Connection};

use crate::{models::*, pools::Db, sessions::*, viewmodels};

pub fn get_routes() -> impl Iterator<Item = Route> {
	routes![register, login, logout].into_iter()
}

#[post("/api/register", data = "<user>")]
async fn register(conn: Connection<'_, Db>, user: Json<viewmodels::RegisterData>, cookies: &CookieJar<'_>, sessions: &State<Sessions>)
	-> Result<status::Accepted<()>, status::BadRequest<&'static str>>
{
	let db = conn.into_inner();
	
	//is email unqiue
	if user::Entity::find().filter(user::Column::Email.eq(user.email.clone())).one(db).await.unwrap().is_some() {
		return Err(status::BadRequest(Some("Email already used")));
	}

	let user = user.into_inner();
	let user = user::ActiveModel {
		email: Set(user.email),
		password: Set(user.password),
		..Default::default()
	};
	let user = user.insert(db).await.expect("failed to register");
	let sessions = sessions.inner();
	let session_code = sessions.add(user.id);
	cookies.add_private(Cookie::new("session", session_code));
	return Ok(status::Accepted(None));
}

#[post("/api/login", data = "<user>")]
async fn login(
	conn: Connection<'_, Db>, 
	user: Json<viewmodels::RegisterData>, 
	cookies: &CookieJar<'_>, 
	sessions: &State<Sessions>
) -> Result<status::Accepted<()>, status::BadRequest<&'static str>> {
	let db = conn.into_inner();
	let user = user.into_inner();
	let user = match user::Entity::find()
		.filter(user::Column::Password.eq(user.password))
		.filter(user::Column::Email.eq(user.email))
		.one(db).await {
			Ok(result) => result,
			Err(_) => return Err(status::BadRequest(Some("Login failed")))
		};
	if let Some(user) = user {
		let sessions = sessions.inner();
		let session_code = sessions.add(user.id);
		cookies.add_private(Cookie::new("session", session_code));
		Ok(status::Accepted(None))
	} else {
		Err(status::BadRequest(Some("Incorrect email or password")))
	}
}

#[post("/api/logout")]
async fn logout(cookies: &CookieJar<'_>, sessions: &State<Sessions>) {
	if let Some(cookie) = cookies.get("session") {
		sessions.remove(cookie.value());
	}
	cookies.remove_private(Cookie::named("session"));
}