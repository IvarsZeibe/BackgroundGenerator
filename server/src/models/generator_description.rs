use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "generator_description")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: String,
	pub name: String,
	pub description: String,
	pub date_created: DateTime,
	pub date_modified: DateTime,
	pub user_id: i32,
	pub generator_type: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::user::Entity",
		from = "Column::UserId",
		to = "super::user::Column::Id"
	)]
	User,
	#[sea_orm(
		belongs_to = "super::generator_type::Entity",
		from = "Column::GeneratorType",
		to = "super::generator_type::Column::Id"
	)]
	GeneratorType,
}

impl Related<super::user::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::User.def()
	}
}

impl Related<super::generator_type::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::GeneratorType.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
