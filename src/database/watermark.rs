// Watermark repository for incremental ETL
use crate::database::Database;
use crate::domain::{EtlMode, EtlStats};
use crate::error::Result;
use chrono::{DateTime, Utc};
use sqlx::Row;
use uuid::Uuid;

pub struct WatermarkRepository {
    db: Database,
}

#[derive(Debug, Clone)]
pub struct Watermark {
    pub run_id: Uuid,
    pub last_created_at: Option<DateTime<Utc>>,
    pub last_unique_key: Option<i64>,
    pub run_mode: EtlMode,
}

impl WatermarkRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// Start a new ETL run and create watermark record
    pub async fn start_run(&self, mode: EtlMode) -> Result<Uuid> {
        let run_id = Uuid::new_v4();
        let mode_str = match mode {
            EtlMode::Full => "full",
            EtlMode::Incremental => "incremental",
        };

        sqlx::query(
            "INSERT INTO etl_watermarks (run_id, run_mode, status) VALUES ($1, $2, 'running')",
        )
        .bind(run_id)
        .bind(mode_str)
        .execute(self.db.pool())
        .await?;

        Ok(run_id)
    }

    /// Complete an ETL run with final statistics
    pub async fn complete_run(&self, run_id: Uuid, stats: &EtlStats) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE etl_watermarks 
            SET status = 'completed',
                completed_at = now(),
                rows_processed = $2,
                rows_inserted = $3,
                rows_duplicated = $4,
                rows_rejected = $5
            WHERE run_id = $1
            "#,
        )
        .bind(run_id)
        .bind(stats.rows_parsed as i64)
        .bind(stats.rows_inserted as i64)
        .bind(stats.rows_duplicated as i64)
        .bind(stats.rows_rejected as i64)
        .execute(self.db.pool())
        .await?;

        Ok(())
    }

    /// Mark a run as failed
    pub async fn fail_run(&self, run_id: Uuid, error: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE etl_watermarks 
            SET status = 'failed',
                completed_at = now(),
                error_message = $2
            WHERE run_id = $1
            "#,
        )
        .bind(run_id)
        .bind(error)
        .execute(self.db.pool())
        .await?;

        Ok(())
    }

    /// Get the last successful watermark for incremental loads
    pub async fn get_last_watermark(&self) -> Result<Option<Watermark>> {
        let row = sqlx::query(
            r#"
            SELECT run_id, last_created_at, last_unique_key, run_mode
            FROM etl_watermarks
            WHERE status = 'completed'
            ORDER BY completed_at DESC
            LIMIT 1
            "#,
        )
        .fetch_optional(self.db.pool())
        .await?;

        match row {
            Some(row) => {
                let mode_str: String = row.get("run_mode");
                let mode = mode_str.parse().unwrap_or(EtlMode::Full);

                Ok(Some(Watermark {
                    run_id: row.get("run_id"),
                    last_created_at: row.get("last_created_at"),
                    last_unique_key: row.get("last_unique_key"),
                    run_mode: mode,
                }))
            }
            None => Ok(None),
        }
    }
}
