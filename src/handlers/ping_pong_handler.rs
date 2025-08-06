use actix_web::{
    get, web::{Path}, Result
};
use crate::models::ping_pong_models::PingPongUser;


#[get("/pingpong/{user_id}/{friend}")]
pub async fn get_ping_pong(info: Path<PingPongUser>) -> Result<String> {
    Ok(format!(
        "Welcome {}, user_id {}1!",
        info.friend, info.user_id
    ))
}