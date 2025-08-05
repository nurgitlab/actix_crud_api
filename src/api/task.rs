use std::env;

use actix_web::{
    error::ResponseError, get, http::{header::ContentType, StatusCode}, post, put, web::{Data, Json, Path}, HttpResponse, Result
};
use dotenv::dotenv;
use serde::{Serialize, Deserialize};

use derive_more::{Display};

#[derive(Deserialize)]
struct Info {
    user_id: u32,
    friend: String,
}

/// extract path info using serde
#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn get_task(info: Path<Info>) -> Result<String> {
    Ok(format!(
        "Welcome {}, user_id {}1!",
        info.friend, info.user_id
    ))
}