use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::sync::LazyLock;
use utoipa::ToSchema;

static SETTINGS: LazyLock<Settings> = LazyLock::new(|| {
    Settings::new().expect("Failed to load configuration")
});

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerSettings,
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(Debug, Deserialize, serde::Serialize, Clone, ToSchema)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, serde::Serialize, Clone, ToSchema)]
pub struct DatabaseSettings {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, serde::Serialize, Clone, ToSchema)]
pub struct ApplicationSettings {
    pub name: String,
    pub environment: String,
}

impl Settings {
    /// Load configuration from files and environment variables
    /// This should be called once at startup
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = std::env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let config = Config::builder()
            // Start with default configuration
            .add_source(File::with_name("config/default").required(false))
            // Layer on environment-specific configuration
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            // Layer on local configuration (for development overrides)
            .add_source(File::with_name("config/local").required(false))
            // Add in settings from environment variables (with prefix APP)
            // E.g., `APP_SERVER__PORT=8080` would set `Settings.server.port`
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()?;

        config.try_deserialize()
    }

    /// Get a reference to the global settings
    /// The settings are lazily initialized on first access
    pub fn get() -> &'static Settings {
        &SETTINGS
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            server: ServerSettings {
                host: "127.0.0.1".to_string(),
                port: 8080,
            },
            database: DatabaseSettings {
                url: "postgres://localhost/hserver".to_string(),
                max_connections: 10,
            },
            application: ApplicationSettings {
                name: "Hotel Management System".to_string(),
                environment: "development".to_string(),
            },
        }
    }
}
