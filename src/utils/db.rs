use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use dotenvy::dotenv;

pub type DbPool = Pool<AsyncPgConnection>;

pub fn establish_connection(database_url: &str) -> DbPool {
    dotenv().ok();
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    Pool::builder(config)
        .max_size(10)
        .build()
        .expect("Failed to create pool")
}
