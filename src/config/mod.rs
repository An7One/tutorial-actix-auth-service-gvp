pub mod crypto;

use color_eyre::Result;
use crypto::CryptoService;
use dotenv::dotenv;
use eyre::WrapErr;
use serde::Deserialize;
use std::sync::Arc;
use sqlx::postgres::{PgPool, PgPoolOptions};
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize)]
pub struct Config{
    pub database_url: String,
    pub host: String,
    pub jwt_secret: String,
    pub port: i32,
    pub secret_key: String,
}

impl Config{

    #[instrument]
    pub fn from_env() -> Result<Config>{
        dotenv().ok();
        
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        info!("Loading configuration");

        let mut cfg = config::Config::new();

        cfg.merge(config::Environment::default())?;
        
        cfg.try_into()
            .context("loading configuration from environment")
    }

    #[instrument(skip(self))]
    pub async fn db_pool(&self) -> Result<PgPool>{
        info!("to create the database connection pool.");

        PgPoolOptions::new()
            .connect(&self.database_url)
            .await
            .context("to create the database connection pool")

        // PgPool::builder()
        //     .connection_timeout(Duration::from_secs(30))
        //     .connect(&self.database_url)
        //     .await
        //     .context("to create the database connection pool")
    }

    #[instrument(skip(self))]
    pub fn hashing(&self) -> CryptoService{
        CryptoService{
            key: Arc::new(self.secret_key.clone()),
            jwt_secret: Arc::new(self.jwt_secret.clone()),
        }
    }
}