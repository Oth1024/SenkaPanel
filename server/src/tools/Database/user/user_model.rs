use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "User")]
pub struct Model {
    #[sea_orm(primary_key, auto_incement = true)]
    pub id: u32,

    #[sea_orm(unique_key = "item")]
    pub mail: String,
    
    pub user_name: String,
    
    pub password: String,
}

impl ActiveModelBehavior for ActiveModel { }