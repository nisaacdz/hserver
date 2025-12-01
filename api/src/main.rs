use actix_web::{App, HttpServer, web};
use api::{auth::TokenEngine, v1::configure_v1_routes};
use config::{Config, File};
use domain::AppConfig;
use infrastructure::db;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::fmt::format::FmtSpan;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = {
        dotenvy::dotenv().ok();
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_span_events(FmtSpan::CLOSE)
            .init();

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
    let token_engine = TokenEngine::new(&config.security);

    println!(
        "Starting server at {}:{}",
        config.server.host, config.server.port
    );

    let web_pool = web::Data::new(pool.clone());
    let web_token_engine = web::Data::new(token_engine.clone());
    let web_app_config = web::Data::new(config.clone());

    HttpServer::new(move || {
        let web_pool = web_pool.clone();
        let web_token_engine = web_token_engine.clone();
        let web_app_config = web_app_config.clone();
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web_pool)
            .app_data(web_app_config)
            .app_data(web_token_engine)
            .configure(|cfg| {
                cfg.service(web::scope("/api").configure(configure_v1_routes));
            })
    })
    .bind((config.server.host.as_str(), config.server.port))?
    .run()
    .await
}
