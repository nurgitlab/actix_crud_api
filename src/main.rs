use axum::{
    extract::{Path, Query}, response::IntoResponse, routing::{get, get_service}, Router
};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
    .merge(routes_hello())
	.fallback_service(routes_static());

    // run our app with hyper, listening globally on port 3010
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3010").await.unwrap();
    axum::serve(listener, routes_all).await.unwrap();
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