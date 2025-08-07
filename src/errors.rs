use actix_web::{HttpResponse, ResponseError};
use sqlx::Error as SqlxError;

#[derive(Debug)]
pub enum UserError {
    UserNotFound,
    DatabaseError(String),
    InvalidInput,
    EmptyInput,
    // Можно добавить другие типы ошибок
}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::UserNotFound => write!(f, "User not found"),
            UserError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            UserError::InvalidInput => write!(f, "Invalid input parameters"),
            UserError::EmptyInput => write!(f, "Empty input parameters"),
        }
    }
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UserError::UserNotFound => HttpResponse::NotFound().json(serde_json::json!({
                "error": "User not found"
            })),
            UserError::DatabaseError(_) => HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            })),
            UserError::InvalidInput => HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid input"
            })),
            UserError::EmptyInput => HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Empty input"
            })),
        }
    }
}