use crate::{database::database_manager::database_conf, Error, Result};
use axum::{routing::post, Json, Router};
use chrono::Utc;

use entity::services;
use sea_orm::{ActiveModelTrait, Set};
use serde_json::{json, Value};
use uuid::Uuid;

use super::models::service_model::ServiceModel;

pub fn routes() -> Router {
    Router::new().route("/api/services/", post(create_service))
}

async fn create_service(payload: Json<ServiceModel>) -> Result<Json<Value>> {
    let service_model = services::ActiveModel {
        uuid: Set(Uuid::new_v4()),
        detail: Set(payload.details.to_owned()),
        price: Set(payload.price.to_string()),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };
    let db = database_conf().await;
    service_model.insert(&db).await.unwrap();
    let body = Json(json!({"result":
    {"success":true}
    }));
    Ok(body)
}
