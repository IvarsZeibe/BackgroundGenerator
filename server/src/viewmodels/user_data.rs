mod basic_user_data;
mod detailed_user_data;
mod email_change;
mod login;
pub mod my_generators;
mod password_change;
mod register;
mod update_user;

pub use basic_user_data::BasicUserData;
pub use detailed_user_data::DetailedUserData;
pub use email_change::EmailChange;
pub use login::Login;
pub use password_change::PasswordChange;
pub use register::Register;
pub use update_user::UpdateUser;
