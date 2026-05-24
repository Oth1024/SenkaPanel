#[cfg(test)]
mod test {
    use sea_orm::{
        entity::prelude::*,
        ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection,
        DbBackend, EntityTrait, ModelTrait, QueryFilter, Schema, Set,
    };
    use tools::database::client::{DATABASE, initialize_database};
    use tokio;

    #[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "test_table")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: u32,

        pub content: String,

        pub test_bool: bool
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}

    async fn ensure_db() -> &'static DatabaseConnection {
        if DATABASE.get().is_none() {
            initialize_database().await;
            let db = DATABASE.get().unwrap();
            let stmt = Schema::new(DbBackend::Sqlite)
                .create_table_from_entity(Entity)
                .if_not_exists()
                .to_owned();
            db.execute(db.get_database_backend().build(&stmt))
                .await
                .expect("Failed to create test_table");
        }
        DATABASE.get().unwrap()
    }

    #[tokio::test]
    async fn test_insert() {
        let db = ensure_db().await;

        let model = ActiveModel {
            id: Set(1),
            content: Set("insert test".to_owned()),
            test_bool: Set(true),
            ..Default::default()
        };

        let result = model.insert(db).await;
        assert!(result.is_ok());

        let saved = Entity::find_by_id(1).one(db).await.unwrap();
        assert!(saved.is_some());
        assert_eq!(saved.unwrap().content, "insert test");

        let _ = Entity::delete_by_id(1).exec(db).await;
    }

    #[tokio::test]
    async fn test_find() {
        let db = ensure_db().await;

        ActiveModel {
            id: Set(2),
            content: Set("find test".to_owned()),
            test_bool: Set(false),
            ..Default::default()
        }
        .insert(db)
        .await
        .unwrap();

        let result = Entity::find_by_id(2).one(db).await.unwrap();
        assert!(result.is_some());
        let model = result.unwrap();
        assert_eq!(model.content, "find test");
        assert!(!model.test_bool);

        let all = Entity::find().all(db).await.unwrap();
        assert!(!all.is_empty());

        let filtered = Entity::find()
            .filter(Column::Content.eq("find test"))
            .all(db)
            .await
            .unwrap();
        assert_eq!(filtered.len(), 1);

        let _ = Entity::delete_by_id(2).exec(db).await;
    }

    #[tokio::test]
    async fn test_update() {
        let db = ensure_db().await;

        ActiveModel {
            id: Set(3),
            content: Set("before update".to_owned()),
            test_bool: Set(true),
            ..Default::default()
        }
        .insert(db)
        .await
        .unwrap();

        let model = Entity::find_by_id(3).one(db).await.unwrap().unwrap();
        let mut active: ActiveModel = model.into();
        active.content = Set("after update".to_owned());
        active.test_bool = Set(false);
        active.update(db).await.unwrap();

        let updated = Entity::find_by_id(3).one(db).await.unwrap().unwrap();
        assert_eq!(updated.content, "after update");
        assert!(!updated.test_bool);

        let _ = Entity::delete_by_id(3).exec(db).await;
    }

    #[tokio::test]
    async fn test_delete() {
        let db = ensure_db().await;

        ActiveModel {
            id: Set(4),
            content: Set("to be deleted".to_owned()),
            test_bool: Set(false),
            ..Default::default()
        }
        .insert(db)
        .await
        .unwrap();

        let model = Entity::find_by_id(4).one(db).await.unwrap();
        assert!(model.is_some());

        model.unwrap().delete(db).await.unwrap();

        let result = Entity::find_by_id(4).one(db).await.unwrap();
        assert!(result.is_none());
    }
}
