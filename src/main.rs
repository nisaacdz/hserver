use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use hserver::config::Settings;
use hserver::db;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "Hotel Management System"
    }))
}

async fn get_config() -> impl Responder {
    let settings = Settings::get();
    HttpResponse::Ok().json(serde_json::json!({
        "application": &settings.application,
        "server": {
            "host": &settings.server.host,
            "port": settings.server.port
        }
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Initialize configuration (load once at startup)
    Settings::init().expect("Failed to load configuration");
    let settings = Settings::get();

    log::info!(
        "Starting {} on {}:{}",
        settings.application.name,
        settings.server.host,
        settings.server.port
    );

    // Create database pool
    let pool = db::create_pool(&settings.database.url, settings.database.max_connections);
    log::info!(
        "Database pool created with max {} connections",
        settings.database.max_connections
    );

    // Store server address before moving settings
    let server_host = settings.server.host.clone();
    let server_port = settings.server.port;

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/health", web::get().to(health_check))
            .route("/config", web::get().to(get_config))
    })
    .bind((server_host, server_port))?
    .run()
    .await
}
