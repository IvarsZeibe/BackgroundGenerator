use crate::sessions::Sessions;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::{Request, State};

pub struct Auth {
	pub user_id: i32,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
	type Error = &'static str;

	async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
		let cookie = match request.cookies().get_private("session") {
			Some(c) => c,
			None => return Outcome::Failure((Status::Unauthorized, "You must be logged in")),
		};

		let session_code = cookie.value();

		let active_sessions = request.guard::<&State<Sessions>>().await;
		let active_sessions = match active_sessions {
			Outcome::Success(s) => s.inner(),
			_ => return Outcome::Failure((Status::Unauthorized, "You must be logged in")),
		};
		match active_sessions.get_user(session_code) {
			Some(id) => Outcome::Success(Auth { user_id: id }),
			None => return Outcome::Failure((Status::Unauthorized, "You must be logged in")),
		}
	}
}
