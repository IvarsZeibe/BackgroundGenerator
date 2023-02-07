#[macro_use] extern crate rocket;
use std::{io, path::{PathBuf, Path}};

use rocket::{serde::json::Json, http::{CookieJar, Cookie}, State, fs::NamedFile, response::status::{self}};
use rocket::fs::relative;
use sea_orm::*;
use sea_orm_rocket::{Connection, Database};

mod auth;
use auth::Auth;

mod models;
use models::*;

mod viewmodels;

mod pools;
use pools::Db;

mod sessions;
use sessions::*;

mod responders;

mod generator_routes;

// use background_generator::{TriangleGenerator, Generator};

const DIST: &str = relative!("..\\client\\dist");

#[get("/<_..>", rank = 2)]
async fn index() -> io::Result<NamedFile> {
    NamedFile::open(Path::new(DIST).join("index.html")).await
}
#[get("/assets/<file..>", rank = 1)]
async fn static_file(file: PathBuf) -> io::Result<NamedFile> {
    NamedFile::open(Path::new(DIST).join("assets").join(file)).await
}

#[post("/api/register", data = "<user>")]
async fn register(conn: Connection<'_, Db>, user: Json<viewmodels::RegisterData>, cookies: &CookieJar<'_>, sessions: &State<Sessions>) {
    let db = conn.into_inner();

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
        cookies.add(Cookie::new("session", session_code));
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
    cookies.remove(Cookie::named("session"));
}

#[get("/api/users")]
async fn get_users(conn: Connection<'_, Db>, _auth: Auth) -> Json<Vec<viewmodels::UserData>> {
    let db = conn.into_inner();

    let users = user::Entity::find().all(db).await.expect("error");

    let users = users.into_iter()
        .map(|u| {
            let user::Model { id, email, password } = u;
            viewmodels::UserData { id, email, password }
        })
        .collect();
    Json(users)
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .manage(Sessions::new())
        .mount("/api/generator", generator_routes::get_routes())
        .mount("/", routes![index, get_users, register, login, logout, static_file])
}
