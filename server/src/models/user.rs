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
	pub is_admin: bool
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_one = "super::settings::Entity")]
	Settings
}

impl Related<super::settings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Settings.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}