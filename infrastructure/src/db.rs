use diesel_async::{
    pooled_connection::{
        deadpool::{BuildError, Pool},
        AsyncDieselConnectionManager,
    },
    AsyncPgConnection,
};
use domain::DatabaseConfig;

pub type DbPool = Pool<AsyncPgConnection>;

/// Initialize database connection pool
pub fn init_pool(db_config: &DatabaseConfig) -> Result<DbPool, BuildError> {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&db_config.url);

    Pool::builder(manager)
        .max_size(db_config.max_connections)
        .build()
}
