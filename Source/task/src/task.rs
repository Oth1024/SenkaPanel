use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection::SqlxSqlitePoolConnection, EntityTrait, IntoActiveModel, QueryFilter};
use tools::database::client::get_db;
use common::senka_error::{self, SenkaError, SenkaErrorCode};

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    
    pub id: u32,

    pub task_name: String,

    pub command: String,

    pub args: Vec<String>,

    pub icon_url: String,

    pub keep_alive: bool,

    pub index: u32,

    pub associated_process: Option<u32>,
}

impl Task {
    pub fn from_database_model(model: &Model) -> Self {
        let args: Vec<String> = serde_json::from_str(&model.args).unwrap_or_default();
        Self {
            id: model.id.clone(),
            task_name: model.task_name.clone(),
            command: model.command.clone(),
            args,
            icon_url: model.icon_url.clone(),
            keep_alive: model.keep_alive,
            index: model.index,
            associated_process: None
        }
    }

    pub fn to_database_model(&self) -> Model {
        Model {
            id: self.id.clone(),
            task_name: self.task_name.clone(),
            command: self.command.clone(),
            args: serde_json::to_string(&self.args).unwrap_or_default(),
            icon_url: self.icon_url.clone(),
            keep_alive: self.keep_alive,
            index: self.index
        }
    }
}

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
    pub index: u32
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn new(id: u32, task_name: String, command: String, args: Vec<String>, icon_url: String, keep_alive: bool, index: u32) -> Self {
        let args_json = serde_json::to_string(&args).unwrap_or_default();
        Self {
            id,
            task_name,
            command,
            args: args_json,
            icon_url,
            keep_alive,
            index
        }
    }
}

// 增
pub async fn create_task_info(id: u32, task_name: impl Into<String>, command: impl Into<String>, args: Vec<String>, icon_url: impl Into<String>, keep_alive: bool, index: u32, associated_process: Option<u32>) {
    let model = Model::new(
        id,
        task_name.into(),
        command.into(),
        args,
        icon_url.into(),
        keep_alive,
        index
    );
    model.into_active_model().insert(get_db(Entity).await).await.unwrap();
}

pub async fn delete_task_info(id: u32) {
    // 从数据库中删除 task_info
    Entity::delete_by_id(id)
        .exec(get_db(Entity).await).await.unwrap();
}

pub async fn update_task_info(task_info: Task) {
    // 更新数据库中的 task_info
    let new_model = (&task_info).to_database_model();
    new_model.into_active_model().update(get_db(Entity).await).await.unwrap();
}

pub async fn get_all_task_infos() -> Result<Vec<Task>, SenkaError> {
    // 从数据库中获取所有 task_info
    let result = Entity::find()
        .all(get_db(Entity).await).await;
    if let Ok(models) = result {
        return Ok(Vec::from_iter(models.iter().map(|model|Task::from_database_model(model))));
    }
    Err(SenkaError::null())
}

pub async fn get_task_detail(id: u32) -> Result<Task, SenkaError> {
    let result = Entity::find_by_id(id)
        .one(get_db(Entity).await).await;
    if let Ok(Some(model)) = result {
        return Ok(Task::from_database_model(&model));
    }
    Err(SenkaError::null())
}