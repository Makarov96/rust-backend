use crate::{
    database::database_manager::{close_conecction, database_conf},
    Error, Result,
};
use axum::{
    extract::Path,
    routing::{delete, get, post},
    Json, Router,
};

use entity::services::{self, Entity};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde_json::{json, Value};
use uuid::Uuid;

use super::{
    dao::{
        create_service::insert_service, delete_service::delete_services,
        get_all::get_all_services_from_database,
    },
    models::service_model::{ServiceModel, ServiceUpdateModel},
};

pub fn routes_services() -> Router {
    Router::new().nest("/api/", router())
}

pub fn router() -> Router {
    Router::new()
        .route("/services/", post(create_service))
        .route("/services/:uuid", delete(delete_service))
        .route("/services/", get(get_all_services))
}

async fn update_service(
    Path(uuid): Path<Uuid>,
    payload: Json<ServiceUpdateModel>,
) -> Result<Json<Value>> {
    if uuid.to_string().is_empty() || payload.details.is_empty() || payload.price == 0.0 {
        return Err(Error::UpdateFailure);
    }
    let db = database_conf().await;
    let mut service_data: entity::services::ActiveModel = entity::services::Entity::find()
        .filter(entity::services::Column::Uuid.eq(uuid))
        .one(&db)
        .await
        .unwrap()
        .unwrap()
        .into();

    service_data.detail = Set(String::from(payload.details.as_str()));
    service_data.price = Set(String::from(payload.price.to_string()));
    service_data.update(&db).await.unwrap();
    close_conecction().await;
    let body = Json(json!(  {"result": {"result":true}}));
    Ok(body)
}

async fn create_service(payload: Json<ServiceModel>) -> Result<Json<Value>> {
    if payload.details.is_empty() || payload.price == 0.0 {
        return Err(Error::InsertFailure);
    }

    let result = insert_service(payload.0).await;

    let body = Json(json!({"result":
    {"success":result}
    }));
    Ok(body)
}

async fn delete_service(Path(uuid): Path<Uuid>) -> Result<Json<Value>> {
    if uuid.is_nil() {
        return Err(Error::DeleteFailure);
    }
    delete_services(uuid).await;
    let body = Json(json!({"result":
    {"success":true}
    }));
    Ok(body)
}

async fn get_all_services() -> Result<Json<Value>> {
    let services = get_all_services_from_database().await;

    let body = Json(json!(  {"articles": services}));
    Ok(body)
}
