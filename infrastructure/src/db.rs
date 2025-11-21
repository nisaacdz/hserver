use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, ManagerConfig};
use diesel_async::AsyncPgConnection;
use diesel::{ConnectionError, ConnectionResult};
use futures_util::future::BoxFuture;
use futures_util::FutureExt;
use rustls::ClientConfig;
use rustls_platform_verifier::ConfigVerifierExt;

pub type DbPool = Pool<AsyncPgConnection>;

/// Initialize database connection pool with TLS support
///
/// This uses rustls-platform-verifier which automatically uses the system's
/// trusted root certificates, equivalent to libpq's sslmode=verify-full
pub fn init_pool(database_url: &str) -> DbPool {
    // Configure the connection manager with custom TLS setup
    let mut config = ManagerConfig::default();
    config.custom_setup = Box::new(establish_connection);

    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new_with_config(
        database_url,
        config,
    );

    Pool::builder(manager)
        .max_size(20)
        .build()
        .expect("Failed to create DB pool")
}

/// Establish a TLS-secured PostgreSQL connection
///
/// This function is called by the connection pool manager for each new connection.
/// It sets up rustls with platform certificate verification (system trust store).
fn establish_connection(config: &str) -> BoxFuture<'_, ConnectionResult<AsyncPgConnection>> {
    let fut = async {
        // Use platform verifier - works on Windows, Linux, macOS
        let rustls_config = ClientConfig::with_platform_verifier();
        let tls = tokio_postgres_rustls::MakeRustlsConnect::new(rustls_config);
        
        let (client, conn) = tokio_postgres::connect(config, tls)
            .await
            .map_err(|e| ConnectionError::BadConnection(e.to_string()))?;

        AsyncPgConnection::try_from_client_and_connection(client, conn).await
    };
    fut.boxed()
}
