use rocket::Route;

mod admin;
mod authentication;
mod generators;
mod user;

pub fn get_routes() -> Vec<Route> {
    authentication::get_routes()
        .chain(generators::get_routes())
        .chain(admin::get_routes())
        .chain(user::get_routes())
        .collect()
}
