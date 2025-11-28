use actix_web::{App, HttpServer, web};
use api::{auth::AuthConfig, v1::configure_v1_routes};
use config::{Config, File};
use domain::AppConfig;
use infrastructure::db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = {
        dotenvy::dotenv().ok();

        let run_mode = std::env::var("RUN_MODE").unwrap_or("development".to_string());

        let config: AppConfig = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(config::Environment::with_prefix("APP").separator("__"))
            .build()
            .expect("Failed to build configuration")
            .try_deserialize()
            .expect("Failed to deserialize configuration");

        config
    };

    let pool = db::init_pool(&config.database).expect("Failed to initialize pg connection pool");
    let auth = web::Data::new(AuthConfig::new(&config.security));

    println!(
        "Starting server at {}:{}",
        config.server.host, config.server.port
    );

    let web_pool = web::Data::new(pool.clone());
    let web_auth_config = web::Data::new(auth.clone());
    let web_app_config = web::Data::new(config.clone());

    HttpServer::new(move || {
        let web_pool = web_pool.clone();
        let web_auth_config = web_auth_config.clone();
        let web_app_config = web_app_config.clone();
        App::new()
            .app_data(web_pool)
            .app_data(web_app_config)
            .app_data(web_auth_config)
            .configure(|cfg| {
                cfg.service(web::scope("/api").configure(configure_v1_routes));
            })
    })
    .bind((config.server.host.as_str(), config.server.port))?
    .run()
    .await
}
