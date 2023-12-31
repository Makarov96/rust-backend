use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct ServiceModel {
    pub uuid: Uuid,
    pub details: String,
    pub price: f64,
    pub created_at: NaiveDateTime,
}
