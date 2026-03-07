use config::{Config, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub email: String,
    pub server_uri: String,
    pub server_port: String,
    pub mc_version: String,
    pub rust_log: Option<String>,
}

impl AppConfig {
    pub fn new() -> anyhow::Result<AppConfig> {
        tracing::warn!("Getting App Config");
        let cfg = Config::builder()
            .add_source(config::File::with_name("config"))
            .add_source(Environment::default())
            .build()?;

        Ok(cfg.try_deserialize()?)
    }
}
