use entity::services::Model;
use sea_orm::EntityTrait;

use crate::{
    controller::route_services::models::service_model::ServiceJsonModel,
    database::database_manager::{close_conecction, database_conf},
};

pub async fn get_all_services_from_database() -> Vec<ServiceJsonModel> {
    let db = database_conf().await;

    let services: Vec<ServiceJsonModel> = entity::services::Entity::find()
        .all(&db)
        .await
        .unwrap()
        .into_iter()
        .map(|f: Model| ServiceJsonModel {
            details: f.detail,
            price: f.price,
            uuid: f.uuid,
            created_at: f.created_at,
        })
        .collect();

    close_conecction().await;

    services
}
