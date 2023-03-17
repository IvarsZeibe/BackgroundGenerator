use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id: i32,
	#[sea_orm(unique)]
	pub email: String,
	pub password: String,
	#[sea_orm(default_value = false)]
	pub is_admin: bool,
	pub max_generators: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::user_settings::Entity> for Entity {
	fn to() -> RelationDef {
		super::user_settings::Relation::User.def().rev()
	}
}

impl Related<super::generator_description::Entity> for Entity {
	fn to() -> RelationDef {
		super::generator_description::Relation::User.def().rev()
	}
}

impl ActiveModelBehavior for ActiveModel {}
