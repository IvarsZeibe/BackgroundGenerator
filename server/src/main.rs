#[macro_use] extern crate rocket;

use rocket::{fs::{FileServer, Options}};
use rocket::fs::relative;
use sea_orm_rocket::{Database};

mod auth;
mod admin_auth;

mod models;

mod viewmodels;

mod pools;
use pools::Db;

mod sessions;
use sessions::*;

mod responders;

mod routes;
mod generator_routes;

const BUILD_PATH: &str = relative!("../client/build");

#[launch]
async fn rocket() -> _ {
	rocket::build()
		.attach(Db::init())
		.manage(Sessions::new())
		.mount("/", FileServer::new(BUILD_PATH, Options::Index | Options::NormalizeDirs))
		.mount("/", routes::get_routes())
}
