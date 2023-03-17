use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateUser {
	pub id: i32,
	pub email: String,
	pub password: String,
	#[serde(rename = "isAdmin")]
	pub is_admin: bool,
	#[serde(rename = "maxGenerators")]
	pub max_generators: i32,
}
