use sea_orm::{Database, DatabaseConnection};

use crate::core::enviroments::Enviroment;

pub async fn database_conf() -> DatabaseConnection {
    let db: DatabaseConnection = Database::connect(Enviroment::database_url()).await.unwrap();

    db
}

pub async fn close_conecction() -> bool {
    let db = self::database_conf().await;

    let result = db.close().await;

    result.is_ok()
}
