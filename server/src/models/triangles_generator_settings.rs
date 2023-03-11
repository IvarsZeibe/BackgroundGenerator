use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "triangles_generator_settings")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub edge_count: u32,
    pub color1: String,
    pub color2: String,
    pub seed: u32,
    pub mode: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::generator_description::Entity",
        from = "Column::Id",
        to = "super::generator_description::Column::Id"
    )]
    GeneratorDescription,
}

impl Related<super::generator_description::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GeneratorDescription.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
