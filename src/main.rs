mod api;
mod migrations;

use std::env;

use api::task::{
    get_task
};

use actix_web::{
    HttpServer, App, web::Data, middleware::Logger
};
use log::logger;

use sqlx::postgres::PgPoolOptions;

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
        .service(get_task)
    })
    .bind(("127.0.0.1", 3030))?
    .run()
    .await
}
