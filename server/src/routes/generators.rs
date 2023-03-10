use chrono::Local;
use image::DynamicImage;
use rocket::{
    response::status::{Accepted, BadRequest, NotFound},
    serde::json::Json,
    Route,
};
use sea_orm::*;
use sea_orm_rocket::Connection;

mod chains_generator;
mod circles_generator;
mod triangles_generator;

use crate::{
    auth::Auth,
    models::{generator_description, generator_type},
    pools::Db,
    viewmodels::{generator_description::GeneratorDescription, generator_type::GeneratorType},
};

pub fn get_routes() -> impl Iterator<Item = Route> {
    return triangles_generator::get_routes()
        .chain(circles_generator::get_routes())
        .chain(chains_generator::get_routes())
        .map(|el| {
            el.map_base(|base| format!("{}{}", "/api/generator", base))
                .unwrap()
        })
        .chain(routes![get_generator_types, edit_generator_description]);
}

async fn save_generator_description(
    db: &DatabaseConnection,
    user_id: i32,
    name: String,
    description: String,
    image: DynamicImage,
    generator_type: i32,
) -> Result<generator_description::Model, DbErr> {
    let generator = generator_description::ActiveModel {
        id: Set(uuid::Uuid::now_v7().to_string()),
        name: Set(name),
        description: Set(description),
        user_id: Set(user_id),
        date_created: Set(Local::now().naive_local()),
        generator_type: Set(generator_type),
        ..Default::default()
    };
    let generator = generator.insert(db).await?;
    let image = image.thumbnail(640, 360);
    image.save(generator.id.clone() + ".jpg").unwrap();
    Ok(generator)
}

async fn modify_generator_description(
    db: &DatabaseConnection,
    id: String,
    user_id: i32,
    name: String,
    description: String,
    image: DynamicImage,
) -> Result<(), BadRequest<()>> {
    let generator_description = generator_description::Entity::find_by_id(id.clone())
        .one(db)
        .await
        .unwrap()
        .unwrap();
    if generator_description.user_id != user_id {
        return Err(BadRequest(None));
    }
    let mut generator_description: generator_description::ActiveModel =
        generator_description.into();
    generator_description.name = Set(name);
    generator_description.description = Set(description);
    generator_description.save(db).await.unwrap();

    let image = image.thumbnail(640, 360);
    image.save(id + ".jpg").unwrap();

    Ok(())
}

async fn delete_generator_description(
    db: &DatabaseConnection,
    id: String,
    user_id: i32,
) -> Result<(), BadRequest<()>> {
    let generator_description = generator_description::Entity::find_by_id(id)
        .one(db)
        .await
        .unwrap()
        .unwrap();
    if generator_description.user_id != user_id {
        return Err(BadRequest(None));
    }
    let generator_description: generator_description::ActiveModel = generator_description.into();
    generator_description.delete(db).await.unwrap();
    Ok(())
}

#[get("/api/generators")]
async fn get_generator_types(conn: Connection<'_, Db>) -> Json<Vec<GeneratorType>> {
    let db = conn.into_inner();
    let types = generator_type::Entity::find().all(db).await.unwrap();
    Json(
        types
            .into_iter()
            .map(|t| GeneratorType {
                code: t.code,
                name: t.name,
            })
            .collect(),
    )
}

#[post("/api/generatorDescription/<id>", data = "<generator_description>")]
async fn edit_generator_description(
    generator_description: Json<GeneratorDescription>,
    id: String,
    conn: Connection<'_, Db>,
    auth: Auth,
) -> Result<Accepted<()>, NotFound<()>> {
    let db = conn.into_inner();
    let GeneratorDescription { name, description } = generator_description.0;
    let Some(generator_description) = generator_description::Entity::find_by_id(id).one(db).await.unwrap() else {
        return Err(NotFound(()));
    };
    if generator_description.user_id != auth.user_id {
        return Err(NotFound(()));
    }
    let mut generator_description: generator_description::ActiveModel =
        generator_description.into();
    generator_description.name = Set(name);
    generator_description.description = Set(description);
    generator_description.save(db).await.unwrap();
    return Ok(Accepted(None));
}
