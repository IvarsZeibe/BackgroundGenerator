use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GeneratorDescriptionUpdate {
	pub name: String,
	pub description: String,
}
