mod errors;
mod handlers;
mod migrations;
mod models;
mod repositories;
use std::env;

use actix_web::{App, HttpServer, middleware::Logger, web::Data};

use sqlx::postgres::PgPoolOptions;

use crate::{
    handlers::ping_pong_handler::get_ping_pong,
    migrations::apply_migrations::apply_migrations,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    unsafe {
        std::env::set_var("RUST_LOG", "debug");
        std::env::set_var("BACKTRACE", "1");
    }
    env_logger::init();

    dotenv::dotenv().ok();

    // Create DB pool
    let user = env::var("DATABASE_USER").unwrap_or_else(|_| "postgres".to_string());
    let password = env::var("DATABASE_PASSWORD").unwrap_or_else(|_| "".to_string());
    let host = env::var("DATABASE_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("DATABASE_PORT").unwrap_or_else(|_| "5432".to_string());
    let name = env::var("DATABASE_NAME").unwrap_or_else(|_| "postgres".to_string());
    let database_url = format!("postgres://{}:{}@{}:{}/{}", user, password, host, port, name);

    println!("database_url: {database_url}");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    apply_migrations(&pool).await.expect("Failed to apply migrations");

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(Data::new(pool.clone()))
            .service(get_ping_pong)
            .configure(handlers::user_handler::users_routes)
    })
    .bind(("127.0.0.1", 3030))?
    .run()
    .await
}
