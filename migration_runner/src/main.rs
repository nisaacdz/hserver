use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use std::env;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

fn main() {
    dotenv().ok();
    println!("Running migrations...");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let mut conn = PgConnection::establish(&database_url)
        .expect("Error connecting to database");

    match conn.run_pending_migrations(MIGRATIONS) {
        Ok(_) => println!("Migrations run successfully!"),
        Err(e) => {
            eprintln!("Error running migrations: {}", e);
            std::process::exit(1);
        }
    }
}
