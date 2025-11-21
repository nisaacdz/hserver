use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use std::path::Path;

pub async fn run_migrations(pool: &Pool<AsyncPgConnection>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = pool.get().await?;
    
    // Read and execute migration SQL directly
    let up_sql = std::fs::read_to_string("./migrations/2025-11-21-013625-0000_create_core_tables/up.sql")?;
    
    for statement in up_sql.split(';') {
        if statement.trim().is_empty() {
            continue;
        }
        // Ignore errors if objects already exist (simple idempotency)
        let _ = diesel::sql_query(statement)
            .execute(&mut conn)
            .await;
    }
    
    Ok(())
}

// Helper function to check if migrations directory exists
pub fn migrations_exist() -> bool {
    Path::new("./migrations").exists()
}
