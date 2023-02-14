use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")] // needed because deserialize is reexported from rocket not directly from serde
pub struct RegisterData {
	pub email: String,
	pub password: String,
}