use rocket::response::status::BadRequest;
use sea_orm::*;

use crate::models::user;

pub async fn validate_email(
	email: &String,
	db: &DatabaseConnection,
) -> Result<(), BadRequest<&'static str>> {
	if user::Entity::find()
		.filter(user::Column::Email.eq(email.clone()))
		.one(db)
		.await
		.unwrap()
		.is_some()
	{
		return Err(BadRequest(Some("Email already used")));
	}
	Ok(())
}
