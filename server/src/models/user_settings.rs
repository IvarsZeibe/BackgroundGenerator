use sea_orm::entity::prelude::*;

use super::sea_orm_active_enums::PreferredTheme;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_settings")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id: i32,
	pub preferred_theme: PreferredTheme,
	pub user_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
        belongs_to = "super::user::Entity", 
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
	User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}