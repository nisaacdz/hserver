use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub url: String,
    pub max_connections: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApplicationSettings {
    pub name: String,
    pub environment: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SecuritySettings {
    pub session_duration: u64,
    pub key: String,
}

impl SecuritySettings {
    pub fn refresh_threshold(&self) -> u64 {
        self.session_duration / 2
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppSettings {
    pub server: ServerSettings,
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    pub security: SecuritySettings,
    pub imagekit: ImageKitSettings,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ImageKitSettings {
    pub url: String,
}
