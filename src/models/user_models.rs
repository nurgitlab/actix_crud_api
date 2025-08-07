use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub username: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UserPath {
    #[validate(range(min = 1, max = 10000, message = "User ID must be between 1 and 10000"))]
    pub user_id: i32,
}