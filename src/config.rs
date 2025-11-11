// Configuration management for UrbanFlux ETL
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub etl: EtlConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtlConfig {
    pub input_path: String,
    pub chunk_size: usize,
    pub mode: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Config {
            database: DatabaseConfig {
                host: env::var("PGHOST").unwrap_or_else(|_| "localhost".to_string()),
                port: env::var("PGPORT")
                    .unwrap_or_else(|_| "5432".to_string())
                    .parse()
                    .context("Invalid PGPORT")?,
                user: env::var("PGUSER").context("PGUSER not set")?,
                password: env::var("PGPASSWORD").context("PGPASSWORD not set")?,
                database: env::var("PGDATABASE").unwrap_or_else(|_| "urbanflux".to_string()),
            },
            etl: EtlConfig {
                input_path: env::var("ETL_INPUT_PATH")
                    .unwrap_or_else(|_| "./testdata/sample.csv".to_string()),
                chunk_size: env::var("ETL_CHUNK_SIZE")
                    .unwrap_or_else(|_| "100000".to_string())
                    .parse()
                    .context("Invalid ETL_CHUNK_SIZE")?,
                mode: env::var("ETL_MODE").unwrap_or_else(|_| "full".to_string()),
            },
        })
    }

    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.database.user,
            self.database.password,
            self.database.host,
            self.database.port,
            self.database.database
        )
    }
}
