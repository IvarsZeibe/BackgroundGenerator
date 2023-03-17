use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct BasicUserData {
	pub id: i32,
	pub email: String,
	#[serde(rename = "isAdmin")]
	pub is_admin: bool,
}
