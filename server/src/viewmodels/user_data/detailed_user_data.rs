use chrono::NaiveDateTime;
use rocket::serde::Serialize;
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DetailedUserData {
	pub id: i32,
	pub email: String,
	#[serde(rename = "isAdmin")]
	pub is_admin: bool,
	#[serde(rename = "maxGenerators")]
	pub max_generators: i32,
	#[serde(rename = "generatorsSaved")]
	pub generators_saved: i32,
	#[serde(rename = "dateCreated")]
	pub date_created: NaiveDateTime,
	#[serde(rename = "lastAuthorized")]
	pub last_authorized: NaiveDateTime,
}
