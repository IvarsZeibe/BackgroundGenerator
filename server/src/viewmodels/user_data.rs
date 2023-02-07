use rocket::serde::{Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserData {
    pub id: i32,
    pub email: String,
    pub password: String,
}