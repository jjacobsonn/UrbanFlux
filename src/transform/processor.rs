// Transform processor with validation and deduplication
use crate::domain::{EtlStats, ServiceRequest};
use crate::domain::validation::ServiceRequestValidator;
use crate::transform::Deduplicator;
use tracing::{debug, info};

pub struct TransformProcessor {
    validator: ServiceRequestValidator,
    deduplicator: Deduplicator,
}

impl TransformProcessor {
    pub fn new() -> Self {
        Self {
            validator: ServiceRequestValidator::new(),
            deduplicator: Deduplicator::new(),
        }
    }

    /// Transform a batch of records: validate and deduplicate
    /// Returns (clean_records, updated_stats)
    pub fn process(
        &mut self,
        records: Vec<ServiceRequest>,
        mut stats: EtlStats,
    ) -> (Vec<ServiceRequest>, EtlStats) {
        let initial_count = records.len();
        debug!("Transforming {} records", initial_count);

        let mut clean_records = Vec::with_capacity(records.len());

        for record in records {
            // Check for duplicates
            if self.deduplicator.is_duplicate(record.unique_key) {
                stats.rows_duplicated += 1;
                continue;
            }

            // Validate
            match self.validator.validate(&record) {
                Ok(()) => {
                    stats.rows_validated += 1;
                    clean_records.push(record);
                }
                Err(errors) => {
                    stats.validation_errors += 1;
                    stats.rows_rejected += 1;
                    debug!(
                        unique_key = record.unique_key,
                        errors = ?errors,
                        "Record failed validation"
                    );
                }
            }
        }

        info!(
            "Transform complete: {} in, {} validated, {} duplicates, {} rejected",
            initial_count,
            clean_records.len(),
            stats.rows_duplicated,
            stats.rows_rejected
        );

        (clean_records, stats)
    }

    /// Reset the deduplicator (for testing or new runs)
    pub fn reset(&mut self) {
        self.deduplicator.clear();
    }
}

impl Default for TransformProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Borough, Coordinates};
    use chrono::Utc;

    fn make_test_request(key: i64) -> ServiceRequest {
        ServiceRequest {
            unique_key: key,
            created_at: Utc::now(),
            closed_at: None,
            complaint_type: "Noise".to_string(),
            descriptor: Some("Test".to_string()),
            borough: Some(Borough::Manhattan),
            coordinates: Some(Coordinates::new(40.7580, -73.9855).unwrap()),
            latitude: Some(40.7580),
            longitude: Some(-73.9855),
        }
    }

    #[test]
    fn test_transform_deduplication() {
        let mut processor = TransformProcessor::new();
        let records = vec![
            make_test_request(1),
            make_test_request(2),
            make_test_request(1), // duplicate
        ];

        let stats = EtlStats::new();
        let (clean, stats) = processor.process(records, stats);

        assert_eq!(clean.len(), 2);
        assert_eq!(stats.rows_duplicated, 1);
        assert_eq!(stats.rows_validated, 2);
    }

    #[test]
    fn test_transform_validation() {
        let mut processor = TransformProcessor::new();
        let mut invalid_request = make_test_request(1);
        invalid_request.unique_key = 0; // invalid

        let records = vec![invalid_request, make_test_request(2)];

        let stats = EtlStats::new();
        let (clean, stats) = processor.process(records, stats);

        assert_eq!(clean.len(), 1);
        assert_eq!(stats.rows_rejected, 1);
        assert_eq!(stats.validation_errors, 1);
    }
}
