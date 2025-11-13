// Configuration management
use crate::domain::EtlMode;
use crate::error::{EtlError, Result};
use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub etl: EtlConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EtlConfig {
    pub chunk_size: usize,
    pub mode: EtlMode,
    pub input_path: Option<String>,
    pub bad_rows_dir: String,
    pub runs_dir: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: LogFormat,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Json,
    Pretty,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok(); // Load .env file if present

        let database = DatabaseConfig {
            host: env::var("PGHOST").unwrap_or_else(|_| "localhost".to_string()),
            port: env::var("PGPORT")
                .unwrap_or_else(|_| "5432".to_string())
                .parse()
                .map_err(|_| EtlError::Config("Invalid PGPORT".to_string()))?,
            user: env::var("PGUSER").unwrap_or_else(|_| "urbanflux_user".to_string()),
            password: env::var("PGPASSWORD")
                .map_err(|_| EtlError::Config("PGPASSWORD must be set".to_string()))?,
            database: env::var("PGDATABASE").unwrap_or_else(|_| "urbanflux".to_string()),
            max_connections: env::var("DB_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .unwrap_or(5),
        };

        let etl = EtlConfig {
            chunk_size: env::var("ETL_CHUNK_SIZE")
                .unwrap_or_else(|_| "100000".to_string())
                .parse()
                .unwrap_or(100_000),
            mode: env::var("ETL_MODE")
                .unwrap_or_else(|_| "full".to_string())
                .parse()
                .map_err(|e| EtlError::Config(e))?,
            input_path: env::var("ETL_INPUT_PATH").ok(),
            bad_rows_dir: env::var("ETL_BAD_ROWS_DIR")
                .unwrap_or_else(|_| "/app/bad_rows".to_string()),
            runs_dir: env::var("ETL_RUNS_DIR").unwrap_or_else(|_| "/app/runs".to_string()),
        };

        let logging = LoggingConfig {
            level: env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
            format: if env::var("LOG_FORMAT").unwrap_or_default() == "json" {
                LogFormat::Json
            } else {
                LogFormat::Pretty
            },
        };

        Ok(Self {
            database,
            etl,
            logging,
        })
    }

    /// Get database connection URL
    pub fn database_url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.database.user,
            self.database.password,
            self.database.host,
            self.database.port,
            self.database.database
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_url() {
        let config = Config {
            database: DatabaseConfig {
                host: "localhost".to_string(),
                port: 5432,
                user: "testuser".to_string(),
                password: "testpass".to_string(),
                database: "testdb".to_string(),
                max_connections: 5,
            },
            etl: EtlConfig {
                chunk_size: 100_000,
                mode: EtlMode::Full,
                input_path: None,
                bad_rows_dir: "/bad_rows".to_string(),
                runs_dir: "/runs".to_string(),
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: LogFormat::Pretty,
            },
        };

        assert_eq!(
            config.database_url(),
            "postgresql://testuser:testpass@localhost:5432/testdb"
        );
    }
}
