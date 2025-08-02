use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3010
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3010").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}