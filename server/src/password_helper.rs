use argon2::{password_hash::SaltString, Argon2, PasswordHasher, PasswordHash, PasswordVerifier};
use rand::rngs::OsRng;
use rocket::response::status::BadRequest;

pub fn is_valid(password: &String) -> Result<(), BadRequest<&'static str>> {
	if password.len() < 8 {
		return Err(BadRequest(Some("Must be atleast 8 characters")));
	}
	Ok(())
}

pub fn hash_password(password: String) -> Result<String, BadRequest<&'static str>> {
	let salt = SaltString::generate(&mut OsRng);
	match Argon2::default().hash_password(password.as_bytes(), &salt) {
		Ok(p) => Ok(p.to_string()),
		Err(_) => Err(BadRequest(None)),
	}
}

pub fn is_password_correct(expected: &String, actual: &String) -> bool {
	let parsed_hash = PasswordHash::new(expected).unwrap();
	Argon2::default().verify_password(actual.as_bytes(), &parsed_hash).is_ok()
}
