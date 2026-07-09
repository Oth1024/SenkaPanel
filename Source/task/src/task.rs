use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub task_name: String,
    pub command: String,
    pub args: String,
    pub icon_url: String,
    pub keep_alive: bool,
    pub index: u32,
    pub associated_process: Option<u32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn new(id: u32, task_name: String, command: String, args: Vec<String>, icon_url: String, keep_alive: bool, index: u32, associated_process: Option<u32>) -> Self {
        let args_json = serde_json::to_string(&args).unwrap_or_default();
        Self {
            id,
            task_name,
            command,
            args: args_json,
            icon_url,
            keep_alive,
            index,
            associated_process,
        }
    }
}