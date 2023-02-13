#[macro_use] extern crate rocket;

use rocket::{serde::json::Json, http::{CookieJar, Cookie}, State, fs::{FileServer, Options}, response::status::{self}};
use rocket::fs::relative;
use sea_orm::*;
use sea_orm_rocket::{Connection, Database};

mod auth;
use auth::Auth;
mod admin_auth;

mod models;
use models::*;

mod viewmodels;

mod pools;
use pools::Db;

mod sessions;
use sessions::*;

mod responders;

mod generator_routes;

const BUILD_PATH: &str = relative!("../client/build");

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
    cookies.add(Cookie::new("session", session_code));
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

#[get("/api/profile")]
async fn get_profile(conn: Connection<'_, Db>, auth: Auth) -> Result<Json<viewmodels::UserData>, status::NotFound<()>> {
    let db = conn.into_inner();

    let user = user::Entity::find_by_id(auth.user_id).one(db).await.unwrap();

    if let Some(user::Model { id, email, password, is_admin }) = user {
        Ok(Json(viewmodels::UserData { id, email, password, is_admin }))
    } else {
        Err(status::NotFound(()))
    }
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .manage(Sessions::new())
        .mount("/", FileServer::new(BUILD_PATH, Options::Index | Options::NormalizeDirs))
        .mount("/api/generator", generator_routes::get_routes())
        .mount("/", routes![get_users, get_profile, register, login, logout])
}
