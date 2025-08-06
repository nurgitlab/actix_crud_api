mod migrations;
mod handlers;
mod models;
mod repositories;

use std::env;

use actix_web::{
    HttpServer, App, web::Data, middleware::Logger
};
use log::logger;

use sqlx::postgres::PgPoolOptions;

use crate::handlers::{ping_pong_handler::get_ping_pong, user_handler::create_user};

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    unsafe {
        std::env::set_var("RUST_LOG", "debug");
        std::env::set_var("BACKTRACE", "1");
    }
    env_logger::init();

    dotenv::dotenv().ok();
    
    // Создаем пул соединений с БД
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("database_url: {}", database_url);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    migrations::apply_migrations(&pool)
        .await
        .expect("Failed to apply migrations");

    HttpServer::new(move || {
        let logger =  Logger::default();
        App::new()
        .wrap(logger)
        .app_data(Data::new(pool.clone()))
        .service(get_ping_pong)
        .service(create_user)
    })
    .bind(("127.0.0.1", 3030))?
    .run()
    .await
}
