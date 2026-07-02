use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uuid: String,
    pub task_name: String,
    pub command: String,
    pub icon_url: String,
    pub keep_alive: bool,
    pub index: u32,
    pub associated_process_id: Option<u32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn new(uuid: String, task_name: String, command: String, icon_url: String, keep_alive: bool, associated_process_id: Option<u32>, index: u32) -> Self {
        Self {
            uuid,
            task_name,
            command,
            icon_url,
            keep_alive,
            associated_process_id,
            index,
        }
    }
}