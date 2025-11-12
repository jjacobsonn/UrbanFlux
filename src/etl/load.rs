// Load phase - Bulk insert to PostgreSQL
use anyhow::Result;
use tracing::info;

use crate::db::schema::{Database, ServiceRequest};

pub struct Loader {
    db: Database,
}

impl Loader {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn load(&self, records: Vec<ServiceRequest>) -> Result<u64> {
        info!("Loading {} records to database", records.len());

        let inserted = self.db.bulk_insert(&records).await?;

        info!("Successfully loaded {} records", inserted);

        Ok(inserted)
    }
}

