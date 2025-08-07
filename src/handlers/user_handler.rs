use actix_web::{
    delete, error::ResponseError, get, http::{header::ContentType, StatusCode}, post, put, web::{Data, Json, Path, ServiceConfig}, HttpResponse, Responder, Result
};
use serde::Deserialize;
use validator::{HasLen, Validate};
use crate::{errors::UserError, models::user_models::{CreateUser, UpdateUser, User, UserPath}, repositories::user_repository::UserRepository};
use sqlx::PgPool;


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

#[get("/users/{user_id}")]
pub async fn get_user(
    user_id: Path<String>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, UserError> {
    let user_id = user_id.parse::<i32>().map_err(|_| UserError::InvalidInput)?;
    
    let user = UserRepository::find_by_id(&pool, user_id).await?;
    
    Ok(HttpResponse::Ok().json(user))
}

#[put("/users/{user_id}")]
async fn update_user(
    pool: Data<PgPool>,
    path: Path<UserPath>,
    user_data: Json<UpdateUser>,
) -> impl Responder {
    match UserRepository::update(&pool, path.user_id, user_data.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(User {
            id: user.id,
            username: user.username,
        }),
        Err(e) => {
            log::error!("Failed to update user: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[delete("/users/{user_id}")]
async fn delete_user(
    pool: Data<PgPool>,
    path: Path<UserPath>,
) -> impl Responder {
    match UserRepository::delete(&pool, path.user_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            log::error!("Failed to delete user: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn users_routes(cfg: &mut ServiceConfig) {
    cfg.service(create_user)
       .service(get_user)
       .service(update_user)
       .service(delete_user);
}