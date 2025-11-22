use diesel::result::ConnectionError;
use diesel_async::{
    pooled_connection::{
        deadpool::{BuildError, Pool},
        AsyncDieselConnectionManager, ManagerConfig,
    },
    AsyncPgConnection,
};
use domain::DatabaseConfig;
use rustls::{ClientConfig, RootCertStore};
use std::str::FromStr;
use tokio_postgres::{config::SslMode, NoTls};
use tokio_postgres_rustls::MakeRustlsConnect;

pub type DbPool = Pool<AsyncPgConnection>;

/// Initialize database connection pool
pub fn init_pool(db_config: &DatabaseConfig) -> Result<DbPool, BuildError> {
    let mut root_store = RootCertStore::empty();
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let rustls_config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    let tls = MakeRustlsConnect::new(rustls_config);

    let mut config = ManagerConfig::default();
    config.custom_setup = Box::new(move |url| {
        let tls = tls.clone();
        Box::pin(async move {
            let mut config = tokio_postgres::Config::from_str(url)
                .map_err(|e| ConnectionError::BadConnection(e.to_string()))?;
            // Relax channel binding requirement to avoid authentication errors with some providers (e.g. Neon)
            config.channel_binding(tokio_postgres::config::ChannelBinding::Prefer);

            let client = match config.get_ssl_mode() {
                SslMode::Require => {
                    let (client, connection) = config
                        .connect(tls)
                        .await
                        .map_err(|e| ConnectionError::BadConnection(e.to_string()))?;
                    tokio::spawn(async move {
                        if let Err(e) = connection.await {
                            eprintln!("connection error: {}", e);
                        }
                    });
                    client
                }
                _ => {
                    let (client, connection) = config
                        .connect(NoTls)
                        .await
                        .map_err(|e| ConnectionError::BadConnection(e.to_string()))?;
                    tokio::spawn(async move {
                        if let Err(e) = connection.await {
                            eprintln!("connection error: {}", e);
                        }
                    });
                    client
                }
            };

            AsyncPgConnection::try_from(client)
                .await
                .map_err(|e| ConnectionError::BadConnection(e.to_string()))
        })
    });

    let manager =
        AsyncDieselConnectionManager::<AsyncPgConnection>::new_with_config(&db_config.url, config);

    Pool::builder(manager)
        .max_size(db_config.max_connections)
        .build()
}
