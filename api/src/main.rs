use actix_web::{web, App, HttpServer};
use config::{Config, File};
use domain::AppConfig;
use infrastructure::db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = {
        dotenvy::dotenv().ok();

        let config: AppConfig = Config::builder()
            .add_source(File::with_name("./config/default"))
            .add_source(config::Environment::with_prefix("APP").separator("__"))
            .build()
            .expect("Failed to build configuration")
            .try_deserialize()
            .expect("Failed to deserialize configuration");

        config
    };

    let pool = db::init_pool(&config.database).expect("Failed to initialize pg connection pool");

    println!(
        "Starting server at {}:{}",
        config.server.host, config.server.port
    );

    let web_pool = web::Data::new(pool.clone());
    let web_config = web::Data::new(config.clone());

    HttpServer::new(move || {
        let web_pool = web_pool.clone();
        let web_config = web_config.clone();
        App::new().app_data(web_pool).app_data(web_config)
    })
    .bind((config.server.host.as_str(), config.server.port))?
    .run()
    .await
}
