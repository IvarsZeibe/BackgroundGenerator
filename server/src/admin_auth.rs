use crate::models::user;
use crate::pools::Db;
use crate::sessions::Sessions;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::{Request, State};
use sea_orm::EntityTrait;
use sea_orm_rocket::Connection;

pub struct AdminAuth {
    pub user_id: i32,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminAuth {
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
        let user_id = match active_sessions.get_user(session_code) {
            Some(id) => id,
            None => return Outcome::Failure((Status::Unauthorized, "You must be logged in")),
        };

        let conn: Connection<'_, Db> = request.guard().await.unwrap();
        let db = conn.into_inner();
        let user = user::Entity::find_by_id(user_id).one(db).await.unwrap();
        if let Some(user) = user {
            if user.is_admin {
                return Outcome::Success(AdminAuth { user_id });
            } else {
                return Outcome::Failure((Status::Forbidden, "You must be an administrator"));
            }
        } else {
            return Outcome::Failure((Status::Unauthorized, "User not found"));
        }
    }
}
