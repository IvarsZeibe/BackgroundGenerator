// use rocket::serde::Deserialize;

// #[derive(Deserialize)]
// #[serde(crate = "rocket::serde")]
// pub struct UserData {
//     pub id: Option<i32>,
//     pub email: Option<String>,
//     pub password: Option<String>,
//     #[serde(rename = "isAdmin")]
//     pub is_admin: Option<bool>,
//     pub max_generators: Option<i32>,
//     pub generators_saved: Option<i32>,
// }

mod basic_user_data;
mod detailed_user_data;
mod login;
mod register;
mod update_password;
mod update_user;

pub use basic_user_data::BasicUserData;
pub use detailed_user_data::DetailedUserData;
pub use login::Login;
pub use register::Register;
pub use update_password::UpdatePassword;
pub use update_user::UpdateUser;
