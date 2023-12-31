use chrono::Utc;
use entity::services;
use sea_orm::{ActiveModelTrait, Set};
use uuid::Uuid;

use crate::{
    controller::route_services::models::service_model::ServiceModel,
    database::database_manager::{close_conecction, database_conf},
};

pub async fn create_service(payload: ServiceModel) -> bool {
    let service_model = services::ActiveModel {
        uuid: Set(Uuid::new_v4()),
        detail: Set(payload.details.to_owned()),
        price: Set(payload.price.to_string()),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };
    let db = database_conf().await;
    let response = service_model.insert(&db).await.unwrap();
    close_conecction().await;

    true
}
