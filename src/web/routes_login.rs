use crate::{ web::AUTH_TOKEN, Error, Result};
use axum::{ routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies}; 

pub fn routes() -> Router {
	Router::new().route("/api/login", post( api_login))
}

async fn api_login(
    cookies: Cookies,
    payload: Json<LoginPayload>
) -> Result<Json<Value>> {
    println!("Login attempt with payload: {:?}", payload);

    if payload.username != "admin" || payload.password != "password" {
        return Err(Error::LoginFail);
    }

    //Todo: Create real token later
	let cookie = Cookie::new(AUTH_TOKEN, "user-1.exp.sign");
	cookies.add(cookie);

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