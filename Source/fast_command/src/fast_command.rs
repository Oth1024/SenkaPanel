use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "fast_command")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uuid: String,
    pub command_name: String,
    pub command_value: String,
    pub icon_url: String,
    pub keep_alive: bool,
    pub index: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn new(uuid: String, command_name: String, command_value: String, icon_url: String, keep_alive: bool, index: u32) -> Self {
        Self {
            uuid,
            command_name,
            command_value,
            icon_url,
            keep_alive,
            index,
        }
    }
}