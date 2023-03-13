use background_generator::triangles_generator::TriangleGeneratorMode;
use image::DynamicImage;
use rocket::{
    response::status::{Accepted, BadRequest, NotFound},
    serde::json::Json,
    Route,
};
use sea_orm::*;
use sea_orm_rocket::Connection;

use crate::{
    auth::Auth,
    models::{generator_description, triangles_generator_settings},
    pools::Db,
    responders::PngImage,
    viewmodels::generator_settings::{self, Triangles},
};

use super::{
    delete_generator_description, modify_generator_description, save_generator_description,
};

pub fn get_routes() -> impl Iterator<Item = Route> {
    return routes![
        generate_triangles,
        save_triangles_generator_settings,
        get_triangles_generator_settings,
        modify_triangles_generator_settings,
        delete_triangles_generator_settings
    ]
    .into_iter()
    .map(|el| {
        el.map_base(|base| format!("{}{}", "/triangles", base))
            .unwrap()
    });
}

fn generate(settings: generator_settings::Triangles) -> Result<DynamicImage, BadRequest<()>> {
    let generator_settings::Triangles {
        width,
        height,
        edge_count,
        color1,
        color2,
        seed,
        mode,
    } = settings;

    let mode = match mode {
        0 => TriangleGeneratorMode::Quad,
        _ => TriangleGeneratorMode::Diagonal,
    };

    let Ok(color1) = background_generator::hex_to_u8_color(color1) else { return Err(BadRequest(None)) };
    let Ok(color2) = background_generator::hex_to_u8_color(color2) else { return Err(BadRequest(None)) };

    Ok(background_generator::triangles_generator::generate(
        width,
        height,
        edge_count,
        color1,
        color2,
        seed as u64,
        mode,
    ))
}

#[post("/", data = "<settings>")]
async fn generate_triangles(
    settings: Json<generator_settings::Triangles>,
) -> Result<PngImage, BadRequest<()>> {
    Ok(PngImage::from(generate(settings.0)?))
}

#[post("/save", data = "<settings>")]
async fn save_triangles_generator_settings(
    settings: Json<generator_settings::Settings<generator_settings::Triangles>>,
    conn: Connection<'_, Db>,
    auth: Auth,
) -> Result<Accepted<String>, BadRequest<()>> {
    let db = conn.into_inner();
    let generator_settings::Settings::<generator_settings::Triangles> {
        name,
        description,
        generator_settings,
    } = settings.into_inner();

    let image = generate(generator_settings.clone())?;

    let Ok(generator_description) = save_generator_description(db, auth.user_id, name, description, image, 0).await else { return Err(BadRequest(None)); };

    let settings = triangles_generator_settings::ActiveModel {
        id: Set(generator_description.id.clone()),
        width: Set(generator_settings.width),
        height: Set(generator_settings.height),
        edge_count: Set(generator_settings.edge_count),
        color1: Set(generator_settings.color1),
        color2: Set(generator_settings.color2),
        seed: Set(generator_settings.seed),
        mode: Set(generator_settings.mode),
    };

    let settings = match settings.insert(db).await {
        Ok(s) => s,
        Err(e) => {
            generator_description.delete(db).await.unwrap();
            println!("{e}");
            return Err(BadRequest(None));
        }
    };
    Ok(Accepted(Some(settings.id)))
}

#[post("/<id>/save", data = "<settings>")]
async fn modify_triangles_generator_settings(
    id: String,
    settings: Json<generator_settings::Settings<generator_settings::Triangles>>,
    conn: Connection<'_, Db>,
    auth: Auth,
) -> Result<Accepted<()>, BadRequest<()>> {
    let db = conn.into_inner();
    let generator_settings::Settings::<generator_settings::Triangles> {
        name,
        description,
        generator_settings,
    } = settings.into_inner();

    let image = generate(generator_settings.clone())?;

    modify_generator_description(db, id.clone(), auth.user_id, name, description, image).await?;

    let mut settings: triangles_generator_settings::ActiveModel =
        triangles_generator_settings::Entity::find_by_id(id)
            .one(db)
            .await
            .unwrap()
            .unwrap()
            .into();
    settings.width = Set(generator_settings.width);
    settings.height = Set(generator_settings.height);
    settings.edge_count = Set(generator_settings.edge_count);
    settings.color1 = Set(generator_settings.color1);
    settings.color2 = Set(generator_settings.color2);
    settings.seed = Set(generator_settings.seed);
    settings.mode = Set(generator_settings.mode);
    settings.save(db).await.unwrap();

    Ok(Accepted(None))
}

#[get("/<id>")]
async fn get_triangles_generator_settings(
    id: String,
    auth: Auth,
    conn: Connection<'_, Db>,
) -> Result<Json<generator_settings::Settings<generator_settings::Triangles>>, NotFound<()>> {
    let db = conn.into_inner();
    let generator_description = generator_description::Entity::find_by_id(id.clone())
        .one(db)
        .await
        .unwrap()
        .unwrap();
    if generator_description.user_id != auth.user_id {
        return Err(NotFound(()));
    }
    let settings = triangles_generator_settings::Entity::find_by_id(id.clone())
        .one(db)
        .await
        .unwrap()
        .unwrap();
    let settings = Triangles {
        color1: settings.color1,
        color2: settings.color2,
        edge_count: settings.edge_count,
        height: settings.height,
        mode: settings.mode,
        seed: settings.seed,
        width: settings.width,
    };
    return Ok(Json(generator_settings::Settings::<
        generator_settings::Triangles,
    > {
        description: generator_description.description,
        name: generator_description.name,
        generator_settings: settings,
    }));
}

#[post("/<id>/delete")]
async fn delete_triangles_generator_settings(
    id: String,
    conn: Connection<'_, Db>,
    auth: Auth,
) -> Result<Accepted<()>, BadRequest<()>> {
    let db = conn.into_inner();

    let settings: triangles_generator_settings::ActiveModel =
        triangles_generator_settings::Entity::find_by_id(id.clone())
            .one(db)
            .await
            .unwrap()
            .unwrap()
            .into();
    settings.delete(db).await.unwrap();

    delete_generator_description(db, id, auth.user_id).await?;

    Ok(Accepted(None))
}
