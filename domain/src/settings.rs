use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApplicationConfig {
    pub name: String,
    pub environment: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SecurityConfig {
    pub session_duration: u64,
    pub key: String,
}

impl SecurityConfig {
    pub fn refresh_threshold(&self) -> u64 {
        self.session_duration / 2
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub application: ApplicationConfig,
    pub security: SecurityConfig,
}
