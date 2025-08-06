use actix_web::{
    error::ResponseError, get, http::{header::ContentType, StatusCode}, post, put, web::{Data, Json, Path}, HttpResponse, Responder, Result
};use crate::{models::user_models::{CreateUser, UpdateUser, User, UserPath}, repositories::user_repository::UserRepository};
use sqlx::PgPool;

// async fn get_user(
//     pool: Data<PgPool>,
//     path: Path<UserPath>,
// ) -> impl Responder {
//     match UserRepository::find_by_id(&pool, path.user_id).await {
//         Ok(Some(user)) => HttpResponse::Ok().json(User {
//             id: user.id,
//             username: user.username,
//         }),
//         Ok(None) => HttpResponse::NotFound().finish(),
//         Err(e) => {
//             log::error!("Failed to fetch user: {}", e);
//             HttpResponse::InternalServerError().finish()
//         }
//     }
// }

#[post("/users")]
pub async fn create_user(
    pool: Data<PgPool>,
    user_data: Json<CreateUser>,
) -> impl Responder {
    match UserRepository::create(&pool, user_data.into_inner()).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => {
            eprintln!("Failed to create user: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}