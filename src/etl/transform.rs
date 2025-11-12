// Transform phase - Data cleaning, validation, and deduplication
use anyhow::Result;
use std::collections::HashSet;
use tracing::{debug, info};

use crate::clean::Validator;
use crate::db::schema::ServiceRequest;

pub struct Transformer {
    validator: Validator,
}

impl Transformer {
    pub fn new() -> Self {
        Self {
            validator: Validator::new(),
        }
    }

    pub fn transform(&self, mut records: Vec<ServiceRequest>) -> Result<Vec<ServiceRequest>> {
        info!("Transforming {} records", records.len());

        let initial_count = records.len();

        // Deduplicate by unique_key
        let mut seen_keys = HashSet::new();
        records.retain(|record| seen_keys.insert(record.unique_key));

        let after_dedup = records.len();
        if after_dedup < initial_count {
            debug!("Removed {} duplicate records", initial_count - after_dedup);
        }

        // Clean and validate
        records.retain(|record| {
            // Validate unique_key
            if !self.validator.is_valid_unique_key(record.unique_key) {
                return false;
            }

            // Validate borough if present
            if let Some(ref borough) = record.borough {
                if !self.validator.validate_borough(borough) {
                    return false;
                }
            }

            // Validate coordinates if present
            if let (Some(lat), Some(lon)) = (record.latitude, record.longitude) {
                if !self.validator.validate_coordinates(lat, lon) {
                    return false;
                }
            }

            // Validate closed_at is after created_at
            if let Some(closed) = record.closed_at {
                if closed < record.created_at {
                    return false;
                }
            }

            true
        });

        let after_validation = records.len();
        if after_validation < after_dedup {
            debug!(
                "Removed {} invalid records",
                after_dedup - after_validation
            );
        }

        info!(
            "Transformation complete: {} records ({} removed)",
            after_validation,
            initial_count - after_validation
        );

        Ok(records)
    }
}

impl Default for Transformer {
    fn default() -> Self {
        Self::new()
    }
}
