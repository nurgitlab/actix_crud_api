use actix_web::{
    delete, error::ResponseError, get, http::{header::ContentType, StatusCode}, post, put, web::{Data, Json, Path, ServiceConfig}, HttpResponse, Responder, Result
};
use validator::{ Validate};
use crate::{errors::UserError, models::user_models::{CreateUser, UpdateUser, User, UserPath}, repositories::user_repository::UserRepository};
use sqlx::PgPool;


#[post("/users")]
pub async fn create_user(
    user_data: Json<CreateUser>,
    pool: Data<PgPool>,
) ->  Result<HttpResponse, UserError> {
    user_data.validate().map_err(UserError::Validation)?;

    let user = UserRepository::create(&pool, user_data.into_inner()).await?;

    Ok(HttpResponse::Ok().json(user))
}


#[get("/users/{user_id}")]
pub async fn get_user(
    path: Path<UserPath>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, UserError> {
    path.validate().map_err(UserError::Validation)?;

    let user = UserRepository::find_by_id(&pool, path.user_id).await?;
    
    Ok(HttpResponse::Ok().json(user))
}
#[put("/users/{user_id}")]
pub async fn update_user(
    path: Path<UserPath>,
    user_data: Json<UpdateUser>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, UserError> {
    path.validate()?;
    user_data.validate().map_err(UserError::Validation)?;
    
    // Обновление пользователя
    let updated_user = UserRepository::update(
        &pool,
        path.user_id,
        user_data.into_inner()
    ).await?;
    
    Ok(HttpResponse::Ok().json(updated_user))
}

#[delete("/users/{user_id}")]
async fn delete_user(
    path: Path<UserPath>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, UserError> {
    path.validate().map_err(UserError::Validation)?;

    let user = UserRepository::delete(&pool, path.user_id).await?;

    Ok(HttpResponse::Ok().json(user))
}

pub fn users_routes(cfg: &mut ServiceConfig) {
    cfg.service(create_user)
       .service(get_user)
       .service(update_user)
       .service(delete_user);
}