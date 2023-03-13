use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use rand::rngs::OsRng;
use sea_orm::{ActiveModelTrait, ConnectOptions, ConnectionTrait, DatabaseConnection, Schema, Set};
use sea_orm_rocket::{rocket::figment::Figment, Config, Database};
use std::time::Duration;

use crate::models::{sea_orm_active_enums::PreferredTheme, user, user_settings};

#[derive(Database, Debug)]
#[database("mydb")]
pub struct Db(SeaOrmPool);

#[derive(Debug, Clone)]
pub struct SeaOrmPool {
    pub conn: sea_orm::DatabaseConnection,
}

#[async_trait]
impl sea_orm_rocket::Pool for SeaOrmPool {
    type Error = sea_orm::DbErr;

    type Connection = sea_orm::DatabaseConnection;

    async fn init(figment: &Figment) -> Result<Self, Self::Error> {
        let config = figment.extract::<Config>().unwrap();
        let mut options: ConnectOptions = config.url.into();
        options
            .max_connections(config.max_connections as u32)
            .min_connections(config.min_connections.unwrap_or_default())
            .connect_timeout(Duration::from_secs(config.connect_timeout))
            .sqlx_logging(config.sqlx_logging);
        if let Some(idle_timeout) = config.idle_timeout {
            options.idle_timeout(Duration::from_secs(idle_timeout));
        }
        let conn = sea_orm::Database::connect(options).await?;
        add_tables_if_none(&conn).await;
        add_default_admin(&conn).await;

        Ok(SeaOrmPool { conn })
    }

    fn borrow(&self) -> &Self::Connection {
        &self.conn
    }
}

async fn add_tables_if_none(conn: &DatabaseConnection) {
    let builder = conn.get_database_backend();
    let schema = Schema::new(builder);

    // add user table
    if let Err(e) = conn
        .execute(builder.build(&schema.create_table_from_entity(crate::models::user::Entity)))
        .await
    {
        println!("Users table already exists, keeping old structure {e}");
    }
    // add profile settings table
    if let Err(e) = conn
        .execute(
            builder.build(&schema.create_table_from_entity(crate::models::user_settings::Entity)),
        )
        .await
    {
        println!("Settings table already exists, keeping old structure {e}");
    }
    // add generator description table
    if let Err(e) = conn
        .execute(
            builder.build(
                &schema.create_table_from_entity(crate::models::generator_description::Entity),
            ),
        )
        .await
    {
        println!("Generator description table already exists, keeping old structure {e}");
    }
    // add generator type table
    if let Err(e) = conn
        .execute(
            builder.build(&schema.create_table_from_entity(crate::models::generator_type::Entity)),
        )
        .await
    {
        println!("Generator type table already exists, keeping old structure {e}");
    }
    let triangles_type = crate::models::generator_type::ActiveModel {
        id: Set(0),
        name: Set("Triangles generator".to_string()),
        code: Set("triangles".to_string()),
        ..Default::default()
    };
    if triangles_type.insert(conn).await.is_err() {
        println!("Triangle type already exists");
    }
    let circles_type = crate::models::generator_type::ActiveModel {
        id: Set(1),
        name: Set("Circles generator".to_string()),
        code: Set("circles".to_string()),
        ..Default::default()
    };
    if circles_type.insert(conn).await.is_err() {
        println!("Circles type already exists");
    }
    let chains_type = crate::models::generator_type::ActiveModel {
        id: Set(2),
        name: Set("Chains generator".to_string()),
        code: Set("chains".to_string()),
        ..Default::default()
    };
    if chains_type.insert(conn).await.is_err() {
        println!("Chains type already exists");
    }

    if let Err(e) = conn
        .execute(builder.build(
            &schema.create_table_from_entity(crate::models::triangles_generator_settings::Entity),
        ))
        .await
    {
        println!("Triangles generator settings table already exists, keeping old structure {e}");
    }
    if let Err(e) = conn
        .execute(builder.build(
            &schema.create_table_from_entity(crate::models::circles_generator_settings::Entity),
        ))
        .await
    {
        println!("Circles generator settings table already exists, keeping old structure {e}");
    }
    if let Err(e) = conn
        .execute(builder.build(
            &schema.create_table_from_entity(crate::models::chains_generator_settings::Entity),
        ))
        .await
    {
        println!("Chains generator settings table already exists, keeping old structure {e}");
    }
}

async fn add_default_admin(db: &DatabaseConnection) {
    let password = b"admin123";
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(password, &salt)
        .unwrap()
        .to_string();
    let user = user::ActiveModel {
        email: Set(String::from("admin@admin.admin")),
        password: Set(hashed_password),
        is_admin: Set(true),
        ..Default::default()
    };
    match user.insert(db).await {
        Err(error) => println!("Failed to add default admin: {}", error),
        Ok(user) => {
            let user_settings = user_settings::ActiveModel {
                user_id: Set(user.id),
                preferred_theme: Set(PreferredTheme::UseDeviceTheme),
                ..Default::default()
            };
            user_settings.insert(db).await.unwrap();
        }
    }
}
