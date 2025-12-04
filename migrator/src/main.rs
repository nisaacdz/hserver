use app::AppSettings;
use config::{Config, Environment, File};
use infra::db;

#[tokio::main]
async fn main() {
    let app_settings = {
        dotenvy::from_filename(".env").ok();

        let app_settings: AppSettings = Config::builder()
            .add_source(File::with_name("./config/default"))
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()
            .expect("Failed to build configuration")
            .try_deserialize()
            .expect("Failed to deserialize configuration");

        app_settings
    };

    let pool = db::init_pool(&app_settings.database).expect("Failed to connect to database");

    println!("Running migrations...");

    match migrator::run_migrations(&pool).await {
        Ok(_) => println!("✓ Migrations completed successfully"),
        Err(e) => {
            eprintln!("✗ Migration error: {}", e);
            std::process::exit(1);
        }
    }
}
