use crate::controller::routes_whatsapp::whatsapp_requests::send_whatsapp_message;
use crate::{Error, Result};
use axum::routing::post;
use axum::{Json, Router};

use serde_json::{json, Value};

use self::whatsapp_model::whatsapp_payload::WhatsappPaylaod;

pub mod whatsapp_model;
pub mod whatsapp_requests;

pub fn routes() -> Router {
    Router::new().route("/api/whatsapp/", post(send_whatsapp))
}

async fn send_whatsapp(payload: Json<WhatsappPaylaod>) -> Result<Json<Value>> {
    if payload.cellphone.is_empty()
        || payload.from.is_empty()
        || payload.message.is_empty()
        || payload.email.is_empty()
    {
        return Err(Error::WhatsappFailureMessage);
    }

    let response = send_whatsapp_message(payload.0).await;
    println!("recieve {:?}", response);

    let body = Json(json!({"result":
    {"success":!response}
    }));

    Ok(body)
}
