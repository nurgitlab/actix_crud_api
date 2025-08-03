use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("Error: {:?}", self);

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED SERVER ERROR").into_response()
    }
}