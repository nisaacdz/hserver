use diesel_async::AsyncMigrationHarness;
use diesel_async::AsyncPgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

pub fn run_migrations(
    connection: AsyncPgConnection,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut harness = AsyncMigrationHarness::new(connection);
    harness.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}
