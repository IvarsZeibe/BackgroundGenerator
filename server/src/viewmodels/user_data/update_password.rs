use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdatePassword {
	#[serde(rename = "oldPassword")]
	pub old_password: String,
	#[serde(rename = "newPassword")]
	pub new_password: String,
}
