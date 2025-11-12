// Database schema definitions
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ServiceRequest {
    pub unique_key: i64,
    pub created_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub complaint_type: String,
    pub descriptor: Option<String>,
    pub borough: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug)]
pub struct Database {
    pool: PgPool,
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }
}

impl Database {
    pub async fn connect(database_url: &str) -> Result<Self> {
        info!("Connecting to database...");
        
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .context("Failed to connect to PostgreSQL")?;

        info!("Database connection established");
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn initialize_schema(&self) -> Result<()> {
        info!("Initializing database schema...");

        // Create service_requests table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS service_requests (
                unique_key BIGINT PRIMARY KEY,
                created_at TIMESTAMPTZ NOT NULL,
                closed_at TIMESTAMPTZ,
                complaint_type TEXT NOT NULL,
                descriptor TEXT,
                borough TEXT CHECK (borough IN ('BRONX', 'BROOKLYN', 'MANHATTAN', 'QUEENS', 'STATEN ISLAND')),
                latitude DOUBLE PRECISION,
                longitude DOUBLE PRECISION,
                ingested_at TIMESTAMPTZ DEFAULT now()
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .context("Failed to create service_requests table")?;

        // Create indexes
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_service_requests_created_at ON service_requests(created_at)",
        )
        .execute(&self.pool)
        .await
        .context("Failed to create created_at index")?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_service_requests_borough ON service_requests(borough)",
        )
        .execute(&self.pool)
        .await
        .context("Failed to create borough index")?;

        // Create ETL watermarks table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS etl_watermarks (
                id SERIAL PRIMARY KEY,
                run_id UUID NOT NULL DEFAULT gen_random_uuid(),
                last_created_at TIMESTAMPTZ,
                last_unique_key BIGINT,
                run_mode TEXT NOT NULL CHECK (run_mode IN ('full', 'incremental')),
                rows_processed BIGINT NOT NULL DEFAULT 0,
                rows_inserted BIGINT NOT NULL DEFAULT 0,
                rows_skipped BIGINT NOT NULL DEFAULT 0,
                started_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                completed_at TIMESTAMPTZ,
                status TEXT NOT NULL DEFAULT 'running' CHECK (status IN ('running', 'completed', 'failed'))
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .context("Failed to create etl_watermarks table")?;

        // Create materialized views
        sqlx::query(
            r#"
            CREATE MATERIALIZED VIEW IF NOT EXISTS mv_complaints_by_day_borough AS
            SELECT 
                DATE(created_at) as complaint_date,
                borough,
                COUNT(*) as complaint_count
            FROM service_requests
            WHERE borough IS NOT NULL
            GROUP BY DATE(created_at), borough
            ORDER BY complaint_date DESC, borough
            "#,
        )
        .execute(&self.pool)
        .await
        .context("Failed to create materialized view")?;

        info!("Database schema initialized successfully");
        Ok(())
    }

    pub async fn refresh_materialized_views(&self, concurrently: bool) -> Result<()> {
        let concurrently_str = if concurrently { "CONCURRENTLY" } else { "" };
        
        info!("Refreshing materialized views {}...", if concurrently { "concurrently" } else { "" });

        sqlx::query(&format!(
            "REFRESH MATERIALIZED VIEW {} mv_complaints_by_day_borough",
            concurrently_str
        ))
        .execute(&self.pool)
        .await
        .context("Failed to refresh materialized view")?;

        info!("Materialized views refreshed successfully");
        Ok(())
    }

    pub async fn bulk_insert(&self, records: &[ServiceRequest]) -> Result<u64> {
        if records.is_empty() {
            return Ok(0);
        }

        let mut inserted = 0;
        
        for record in records {
            let result = sqlx::query(
                r#"
                INSERT INTO service_requests 
                (unique_key, created_at, closed_at, complaint_type, descriptor, borough, latitude, longitude)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                ON CONFLICT (unique_key) DO NOTHING
                "#,
            )
            .bind(record.unique_key)
            .bind(record.created_at)
            .bind(record.closed_at)
            .bind(&record.complaint_type)
            .bind(&record.descriptor)
            .bind(&record.borough)
            .bind(record.latitude)
            .bind(record.longitude)
            .execute(&self.pool)
            .await;

            match result {
                Ok(result) => inserted += result.rows_affected(),
                Err(e) => warn!("Failed to insert record {}: {}", record.unique_key, e),
            }
        }

        Ok(inserted)
    }

    pub async fn get_record_count(&self) -> Result<i64> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM service_requests")
            .fetch_one(&self.pool)
            .await
            .context("Failed to get record count")?;
        
        Ok(row.0)
    }
}
