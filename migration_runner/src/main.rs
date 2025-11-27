use config::{Config, Environment, File};
use domain::AppConfig;
use infrastructure::db;

#[tokio::main]
async fn main() {
    let app_config = {
        dotenvy::from_filename(".env.test").ok();

        let app_config: AppConfig = Config::builder()
            .add_source(File::with_name("./config/default"))
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()
            .expect("Failed to build configuration")
            .try_deserialize()
            .expect("Failed to deserialize configuration");

        app_config
    };

    let conn = db::init_conn(&app_config.database)
        .await
        .expect("Failed to connect to database");

    println!("Running migrations...");

    match migration_runner::run_migrations(conn) {
        Ok(_) => println!("✓ Migrations completed successfully"),
        Err(e) => {
            eprintln!("✗ Migration error: {}", e);
            std::process::exit(1);
        }
    }
}
