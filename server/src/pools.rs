use sea_orm::{ConnectOptions, DatabaseConnection, ConnectionTrait, Schema, Set, ActiveModelTrait};
use sea_orm_rocket::{rocket::figment::Figment, Config, Database};
use std::time::Duration;

use crate::models::user;

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
	if let Err(e) = conn.execute(builder.build(&schema.create_table_from_entity(crate::models::user::Entity))).await {
		println!("Users table already exists, keeping old structure {e}");
	}
	// add profile settings table
	if let Err(e) = conn.execute(builder.build(&schema.create_table_from_entity(crate::models::settings::Entity))).await {
		println!("Settings table already exists, keeping old structure {e}");
	}
}

async fn add_default_admin(db: &DatabaseConnection) {
	let user = user::ActiveModel {
		email: Set(String::from("admin@admin.admin")),
		password: Set(String::from("admin123")),
		is_admin: Set(true),
		..Default::default()
	};
	if let Err(error) = user.insert(db).await {
		println!("Failed to add default admin: {}", error);
	};
}