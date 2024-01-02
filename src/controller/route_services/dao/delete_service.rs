use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::database::database_manager::{close_conecction, database_conf};

pub async fn delete_services(uuid: Uuid) {
    let db = database_conf().await;

    let user = entity::services::Entity::find()
        .filter(entity::services::Column::Uuid.eq(uuid))
        .one(&db)
        .await
        .unwrap()
        .unwrap();
    entity::services::Entity::delete_by_id(user.id)
        .exec(&db)
        .await
        .unwrap();

    close_conecction().await;
}
