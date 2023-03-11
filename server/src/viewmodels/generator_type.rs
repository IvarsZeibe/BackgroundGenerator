use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GeneratorType {
	pub name: String,
	pub code: String
}