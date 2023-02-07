use rocket::request::FromRequest;
use rocket::{Request, State};
use rocket::request::Outcome;
use crate::sessions::Sessions;

pub struct Auth {
    pub user_id: i32
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = &'static str;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookie = match request.cookies().get("session") {
            Some(c) => c,
            None => return Outcome::Forward(())
        };

        let session_code = cookie.value();

        let active_sessions = request.guard::<&State<Sessions>>().await;
        let active_sessions = match active_sessions {
            Outcome::Success(s) => s.inner(),
            _ => return Outcome::Forward(())
        };
        match active_sessions.get_user(session_code) {
            Some(id) => Outcome::Success(Auth { user_id: id }),
            None => Outcome::Forward(())
        }
    }
}