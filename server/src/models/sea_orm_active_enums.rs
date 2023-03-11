use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum PreferredTheme {
	Light = 0,
	Dark = 1,
	UseDeviceTheme = 2
}