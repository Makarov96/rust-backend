use crate::database::database_manager::database_conf;

pub use self::error::{Error, Result};
mod controller;
mod database;
mod error;
use axum::Router;
use controller::routes_whatsapp::routes;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let db = database_conf().await;
    let app = Router::new().merge(routes());

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
