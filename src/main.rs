use tracing::info;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let settings = {
        let run_mode = std::env::var("RUN_MODE").unwrap_or("development".to_string());

        let settings: app::AppSettings = config::Config::builder()
            .add_source(config::File::with_name("config/default"))
            .add_source(config::File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(config::Environment::with_prefix("APP").separator("__"))
            .build()
            .expect("Failed to build configuration")
            .try_deserialize()
            .expect("Failed to deserialize configuration");

        settings
    };

    let pool = infra::db::init_pool(&settings.database).expect("Failed to initialize pg connection pool");

    info!("Starting server at http://localhost:8080");

    migrator::run_migrations(&pool).await.expect("Failed to run migrations");

    api::run(pool, settings).await
}
