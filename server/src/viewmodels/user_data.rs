use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserData {
    pub id: i32,
    pub email: String,
    pub password: String,
    #[serde(rename = "isAdmin")]
    pub is_admin: bool,
}
