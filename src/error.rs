use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]

pub enum Error {
    WhatsappFailureMessage,
    InsertFailure,
    DeleteFailure,
    UpdateFailure,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let error_description = "UNHANDLED_SERVER_ERROR";
        (StatusCode::INTERNAL_SERVER_ERROR, error_description).into_response()
    }
}
