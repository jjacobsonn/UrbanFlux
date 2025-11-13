// Database connection management
use crate::config::DatabaseConfig;
use crate::error::Result;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use tracing::info;

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Create a new database connection pool
    pub async fn connect(config: &DatabaseConfig) -> Result<Self> {
        let url = format!(
            "postgresql://{}:{}@{}:{}/{}",
            config.user, config.password, config.host, config.port, config.database
        );

        info!("Connecting to database at {}:{}", config.host, config.port);

        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .acquire_timeout(Duration::from_secs(30))
            .connect(&url)
            .await?;

        info!("Database connection pool established");

        Ok(Self { pool })
    }

    /// Get a reference to the connection pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Run database migrations
    pub async fn migrate(&self) -> Result<()> {
        info!("Running database migrations");
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        info!("Migrations completed");
        Ok(())
    }

    /// Refresh materialized views
    pub async fn refresh_materialized_views(&self, concurrently: bool) -> Result<()> {
        let concurrent_keyword = if concurrently { "CONCURRENTLY" } else { "" };
        
        info!("Refreshing materialized views {}", if concurrently { "concurrently" } else { "" });

        sqlx::query(&format!(
            "REFRESH MATERIALIZED VIEW {} mv_complaints_by_day_borough",
            concurrent_keyword
        ))
        .execute(&self.pool)
        .await?;

        sqlx::query(&format!(
            "REFRESH MATERIALIZED VIEW {} mv_complaints_by_type_month",
            concurrent_keyword
        ))
        .execute(&self.pool)
        .await?;

        info!("Materialized views refreshed");
        Ok(())
    }

    /// Check if connection is healthy
    pub async fn health_check(&self) -> Result<()> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }
}
