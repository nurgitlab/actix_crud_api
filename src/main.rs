mod api;

use api::task::{
    get_task
};

use actix_web::{
    HttpServer, App, web::Data, middleware::Logger
};
use log::logger;


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    unsafe {
        std::env::set_var("RUST_LOG", "debug");
        std::env::set_var("BACKTRACE", "1");
    }
    env_logger::init();

    HttpServer::new(move || {
        let logger =  Logger::default();
        App::new()
        .wrap(logger)
        .service(get_task)
    })
    .bind(("127.0.0.1", 3030))?
    .run()
    .await
}
