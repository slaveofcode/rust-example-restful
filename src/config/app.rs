use dotenvy::dotenv;
use serde::Deserialize;
use tracing::{info, Level, subscriber::set_global_default};
use tracing_subscriber::FmtSubscriber;
use config::{Config as ConfigBuilder, Environment, ConfigError};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mode: String,
    pub host: String,
    pub port: u16,
    
    #[serde(rename = "database_url")]
    pub db_url: String,
    
    #[serde(rename = "database_max_conn")]
    pub db_max_conn: Option<u32>,
    
    #[serde(rename = "cors_origin_url")]
    pub cors_origin: String,
}


impl Config {
    #[tracing::instrument]
    pub fn from_env() -> Result<Config, ConfigError> {
        dotenv().ok();

        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .finish();

        set_global_default(subscriber)
            .expect("setting default subscriber failed");
        
        info!("Loading configuration...");

        let cfg = ConfigBuilder::builder()
            .add_source(Environment::default())
            .build()
            .unwrap();

        cfg.try_deserialize::<Config>()
    }
}