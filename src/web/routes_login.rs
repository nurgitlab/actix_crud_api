use crate::{ Error, Result};
use axum::{ routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value}; 

pub fn routes() -> Router {
    return Router::new().route("/api/login", post(api_login));
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("Login attempt with payload: {:?}", payload);

    if payload.username != "admin" || payload.password != "password" {
        return Err(Error::LoginFail);
    }

    //Todo: Set cookies

    //Create the success body
    let body = Json(json!({
        "result": {
            "success": true,
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}