use regex::Regex;
use rocket::{
    response::{content, status},
    serde::json::Json,
    Route,
};
use sea_orm::*;
use sea_orm_rocket::Connection;

use crate::{admin_auth::AdminAuth, models::*, pools::Db, viewmodels};

pub fn get_routes() -> impl Iterator<Item = Route> {
    routes![get_users, update_user].into_iter()
}

#[get("/api/users")]
async fn get_users(
    conn: Connection<'_, Db>,
    _auth: AdminAuth,
) -> Json<Vec<viewmodels::ViewableUserData>> {
    let db = conn.into_inner();

    let users = user::Entity::find().all(db).await.unwrap();

    let users = users
        .into_iter()
        .map(|u| {
            let user::Model {
                id,
                email,
                password: _,
                is_admin,
            } = u;
            viewmodels::ViewableUserData {
                id,
                email,
                is_admin,
            }
        })
        .collect();
    Json(users)
}
#[post("/api/users/<old_id>", data = "<new_user_data>")]
async fn update_user(
    conn: Connection<'_, Db>,
    _auth: AdminAuth,
    old_id: i32,
    new_user_data: Json<viewmodels::UserData>,
) -> Result<status::Accepted<()>, status::BadRequest<content::RawJson<&'static str>>> {
    fn new_error(
        json: &'static str,
    ) -> Result<status::Accepted<()>, status::BadRequest<content::RawJson<&'static str>>> {
        Err(status::BadRequest(Some(content::RawJson(json))))
    }
    let db = conn.into_inner();

    let user = user::Entity::find_by_id(old_id).one(db).await.unwrap();

    let user = match user {
        None => return new_error("{\"error\": \"User not found\"}"),
        Some(u) => u,
    };

    if new_user_data.id <= 0 {
        return new_error("{\"id\": \"Must be atleast 1\"}");
    }
    if new_user_data.id != old_id
        && user::Entity::find_by_id(new_user_data.id)
            .one(db)
            .await
            .unwrap()
            .is_some()
    {
        return new_error("{\"id\": \"Must must be unique\"}");
    }

    let email_pattern = Regex::new(r"^[a-zA-Z0-9.!#$%&’*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,253}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,253}[a-zA-Z0-9])?)*$").unwrap();
    if !email_pattern.is_match(&new_user_data.email) {
        return new_error("{\"email\": \"Invalid email format\"}");
    }
    if new_user_data.email != user.email
        && user::Entity::find()
            .filter(user::Column::Email.eq(new_user_data.email.clone()))
            .one(db)
            .await
            .unwrap()
            .is_some()
    {
        return new_error("{\"email\": \"Email must be unique\"}");
    }

    if new_user_data.password.len() != 0 && new_user_data.password.len() < 8 {
        return new_error("{\"password\": \"Password must be alteast 8 characters\"}");
    }

    let mut user: user::ActiveModel = user.into();
    user.id = Set(new_user_data.id);
    user.email = Set(new_user_data.email.clone());
    if new_user_data.password.len() != 0 {
        user.password = Set(new_user_data.password.clone());
    }
    user.is_admin = Set(new_user_data.is_admin.clone());
    user.update(db).await.unwrap();
    return Ok(status::Accepted(None));
}
