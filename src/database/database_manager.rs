use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, Set};

pub async fn database_conf() -> DatabaseConnection {
    let db: DatabaseConnection = Database::connect("postgresql://localhost:5432/db_steven")
        .await
        .unwrap();

    db
}

pub async fn close_conecction() -> bool {
    let db = self::database_conf().await;

    let result = db.close().await;

    result.is_ok()
}
