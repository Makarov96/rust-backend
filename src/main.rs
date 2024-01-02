use crate::{
    controller::route_services::request_service::routes_services, core::enviroments::Enviroment,
};

pub use self::error::{Error, Result};
mod controller;
mod core;
mod database;
mod error;
use axum::Router;
use controller::routes_whatsapp::routes;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    Enviroment::init();
    let app = Router::new().merge(routes_services()).merge(routes());

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
