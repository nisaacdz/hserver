use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
mod app;
mod constants;
mod error;
mod schema;
mod utils;

mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("start hserver...");
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let settings = config::Settings::new().expect("Failed to load configuration");

    let pool = utils::db::establish_connection(&settings.database.url);

    let bind_address = format!("{}:{}", settings.server.host, settings.server.port);
    println!("Starting server at: {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(actix_web::web::Data::new(pool.clone()))
            .wrap(app::drivers::middlewares::cors::cors())
            .configure(app::drivers::routes::api)
    })
    .bind(bind_address)?
    .run()
    .await
}
