use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::Local;
use rand::rngs::OsRng;
use rocket::{
	http::{Cookie, CookieJar},
	response::status,
	serde::json::Json,
	Route, State,
};
use sea_orm::*;
use sea_orm_rocket::Connection;

use crate::{
	models::{sea_orm_active_enums::PreferredTheme, *},
	pools::Db,
	sessions::*,
	viewmodels, password_helper, email_validator,
};

pub fn get_routes() -> impl Iterator<Item = Route> {
	routes![register, login, logout].into_iter()
}

#[post("/api/register", data = "<user>")]
async fn register(
	conn: Connection<'_, Db>,
	user: Json<viewmodels::user_data::Register>,
	cookies: &CookieJar<'_>,
	sessions: &State<Sessions>,
) -> Result<status::Accepted<()>, status::BadRequest<&'static str>> {
	let db = conn.into_inner();

	//is email unqiue
	email_validator::validate_email(&user.email, &db).await?;

	let user = user.into_inner();
	password_helper::is_valid(&user.password)?;
	let hashed_password = password_helper::hash_password(user.password)?;
	let user = user::ActiveModel {
		email: Set(user.email),
		password: Set(hashed_password),
		date_created: Set(Local::now().naive_local()),
		last_authorized: Set(Local::now().naive_local()),
		max_generators: Set(15),
		..Default::default()
	};
	let user = user.insert(db).await.expect("failed to register");

	let user_settings = user_settings::ActiveModel {
		user_id: Set(user.id),
		preferred_theme: Set(PreferredTheme::UseDeviceTheme),
		..Default::default()
	};
	user_settings.insert(db).await.unwrap();

	let sessions = sessions.inner();
	let session_code = sessions.add(user.id);
	cookies.add_private(Cookie::new("session", session_code));
	return Ok(status::Accepted(None));
}

#[post("/api/login", data = "<user_login_data>")]
async fn login(
	conn: Connection<'_, Db>,
	user_login_data: Json<viewmodels::user_data::Login>,
	cookies: &CookieJar<'_>,
	sessions: &State<Sessions>,
) -> Result<status::Accepted<()>, status::BadRequest<&'static str>> {
	let db = conn.into_inner();
	let user_login_data = user_login_data.into_inner();
	let user = match user::Entity::find()
		.filter(user::Column::Email.eq(user_login_data.email))
		.one(db)
		.await
	{
		Ok(result) => result,
		Err(_) => return Err(status::BadRequest(Some("Login failed"))),
	};
	if let Some(user) = user {
		if password_helper::is_password_correct(&user.password, &user_login_data.password)
		{
			let mut user: user::ActiveModel = user.into();
			user.last_authorized = Set(Local::now().naive_local());
			let mut user = user.save(db).await.unwrap();
			let sessions = sessions.inner();
			let session_code = sessions.add(user.id.take().unwrap());
			cookies.add_private(Cookie::new("session", session_code));
			return Ok(status::Accepted(None));
		}
	}
	Err(status::BadRequest(Some("Incorrect email or password")))
}

#[post("/api/logout")]
async fn logout(cookies: &CookieJar<'_>, sessions: &State<Sessions>) {
	if let Some(cookie) = cookies.get("session") {
		sessions.remove(cookie.value());
	}
	cookies.remove_private(Cookie::named("session"));
}
