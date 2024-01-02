use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct ServiceModel {
    pub details: String,
    pub price: f64,
}

#[derive(Deserialize, Serialize)]
pub struct ServiceJsonModel {
    pub details: String,
    pub price: String,
    pub uuid: Uuid,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize)]
pub struct ServiceUpdateModel {
    pub details: String,
    pub price: f64,
    pub uuid: Uuid,
    pub created_at: NaiveDateTime,
}
