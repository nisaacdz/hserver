use diesel_async::AsyncMigrationHarness;
use diesel_async::{AsyncPgConnection, pooled_connection::deadpool::Pool};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

pub async fn run_migrations(
    pool: &Pool<AsyncPgConnection>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let connection = pool.get().await?;
    let mut harness = AsyncMigrationHarness::new(connection);
    harness.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}
