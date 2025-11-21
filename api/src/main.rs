use actix_web::{web, App, HttpServer};
use infrastructure::db;
use core::AppConfig;
use config::{Config, File};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file if it exists
    dotenvy::dotenv().ok();

    // Load configuration
    let config_builder = Config::builder()
        .add_source(File::with_name("config/default"))
        .add_source(config::Environment::with_prefix("APP").separator("__"));

    let config = config_builder.build().expect("Failed to build configuration");
    let app_config: AppConfig = config.try_deserialize().expect("Failed to deserialize configuration");

    let pool = db::init_pool(&app_config.database.url);

    println!("Starting server at {}:{}", app_config.server.host, app_config.server.port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(app_config.clone()))
    })
    .bind((app_config.server.host.as_str(), app_config.server.port))?
    .run()
    .await
}
