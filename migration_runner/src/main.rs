use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

#[tokio::main]
async fn main() {
    dotenvy::from_filename(".env").ok();
    
    let database_url = std::env::var("APP__DATABASE__URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .expect("DATABASE_URL must be set");

    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    let pool = Pool::builder(manager)
        .build()
        .expect("Failed to create pool");

    println!("Running migrations...");
    
    match migration_runner::run_migrations(&pool).await {
        Ok(_) => println!("✓ Migrations completed successfully"),
        Err(e) => {
            eprintln!("✗ Migration error: {}", e);
            std::process::exit(1);
        }
    }
}
