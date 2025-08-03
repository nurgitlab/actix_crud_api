use axum::{
    extract::{Path, Query}, 
    middleware::{map_response}, 
    response::{IntoResponse, Response}, routing::get, Router
};
use tower_http::services::ServeDir;

mod error;
pub use self::error::{Error, Result};

mod web;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
    .merge(routes_hello())
    .merge(web::routes_login::routes())
    .layer(map_response(main_response_mapper))
	.fallback_service(routes_static());

    // run our app with hyper, listening globally on port 3010
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3010").await.unwrap();
    axum::serve(listener, routes_all).await.unwrap();
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();

    res
}

fn routes_static() -> Router {
    Router::new()
        .fallback_service(ServeDir::new("./"))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/{name}", get(handler_hello2))
}

#[derive(Debug, serde::Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let current_name = params.name.as_deref().unwrap_or("Error!");

    format!("Hello by query, {}!", current_name)
}

async fn handler_hello2 (Path(name): Path<String>) -> impl IntoResponse {
    format!("Hello by route, {}!", name)
}