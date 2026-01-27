use serde::Deserialize;
use std::time::Duration;
use url::Url;

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

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct ReservationSettings {
    #[serde(with = "humantime_serde")]
    pub grace_period: Duration,
    pub discount_rate: f64,
    pub vat_rate: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ImageKitSettings {
    #[serde(with = "deserialize_url_from_str")]
    pub url: Url,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppSettings {
    pub server: ServerSettings,
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    pub security: SecuritySettings,
    pub imagekit: ImageKitSettings,
    pub reservation: ReservationSettings,
}

mod deserialize_url_from_str {
    use serde::{self, Deserialize, Deserializer};
    use url::Url;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Url, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Url::parse(&s).map_err(serde::de::Error::custom)
    }
}
