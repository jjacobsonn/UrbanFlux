// Service request repository with COPY-based bulk insert
use crate::database::Database;
use crate::domain::ServiceRequest;
use crate::error::Result;
use sqlx::Row;
use tracing::{debug, info};

pub struct ServiceRequestRepository {
    db: Database,
}

impl ServiceRequestRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// Bulk insert using batch INSERT statements
    /// Uses multi-row INSERT for high performance
    pub async fn bulk_insert(&self, records: &[ServiceRequest]) -> Result<u64> {
        if records.is_empty() {
            return Ok(0);
        }

        info!("Bulk inserting {} records", records.len());
        self.batch_insert_fallback(records).await
    }

    /// Batch insert (faster than individual inserts)
    async fn batch_insert_fallback(&self, records: &[ServiceRequest]) -> Result<u64> {
        let mut tx = self.db.pool().begin().await?;
        let mut inserted = 0u64;

        // Build batch query (PostgreSQL supports multiple VALUES)
        const BATCH_SIZE: usize = 1000;

        for chunk in records.chunks(BATCH_SIZE) {
            let mut query = String::from(
                "INSERT INTO service_requests (unique_key, created_at, closed_at, complaint_type, descriptor, borough, latitude, longitude) VALUES "
            );

            for (i, _record) in chunk.iter().enumerate() {
                if i > 0 {
                    query.push_str(", ");
                }
                let base = i * 8;
                query.push_str(&format!(
                    "(${}, ${}, ${}, ${}, ${}, ${}, ${}, ${})",
                    base + 1,
                    base + 2,
                    base + 3,
                    base + 4,
                    base + 5,
                    base + 6,
                    base + 7,
                    base + 8
                ));
            }

            query.push_str(" ON CONFLICT (unique_key) DO NOTHING");

            // Build and execute query dynamically
            let mut q = sqlx::query(&query);
            for record in chunk {
                q = q
                    .bind(record.unique_key)
                    .bind(record.created_at)
                    .bind(record.closed_at)
                    .bind(&record.complaint_type)
                    .bind(&record.descriptor)
                    .bind(record.borough.map(|b| b.as_str()))
                    .bind(record.latitude)
                    .bind(record.longitude);
            }

            let result = q.execute(&mut *tx).await?;
            inserted += result.rows_affected();
        }

        tx.commit().await?;
        debug!("Inserted {} records", inserted);

        Ok(inserted)
    }

    /// Get total count of records
    pub async fn count(&self) -> Result<i64> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM service_requests")
            .fetch_one(self.db.pool())
            .await?;

        Ok(row.get("count"))
    }

    /// Get the latest created_at timestamp
    pub async fn get_latest_timestamp(&self) -> Result<Option<chrono::DateTime<chrono::Utc>>> {
        let row = sqlx::query("SELECT MAX(created_at) as max_ts FROM service_requests")
            .fetch_one(self.db.pool())
            .await?;

        Ok(row.get("max_ts"))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_repository_creation() {
        // Placeholder - integration tests will use testcontainers
    }
}
