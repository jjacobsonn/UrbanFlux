// Extract phase - CSV streaming and parsing
use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDateTime, Utc};
use csv_async::AsyncReaderBuilder;
use serde::Deserialize;
use tokio::fs::File;
use tokio_stream::StreamExt;
use tracing::{debug, info, warn};

use crate::db::schema::ServiceRequest;

#[derive(Debug, Deserialize)]
struct CsvRecord {
    unique_key: String,
    created_date: String,
    closed_date: Option<String>,
    complaint_type: String,
    descriptor: Option<String>,
    borough: Option<String>,
    latitude: Option<String>,
    longitude: Option<String>,
}

impl CsvRecord {
    fn to_service_request(&self) -> Result<ServiceRequest> {
        let unique_key = self
            .unique_key
            .trim()
            .parse::<i64>()
            .context("Invalid unique_key")?;

        let created_at = parse_datetime(&self.created_date)?;
        
        let closed_at = if let Some(ref closed) = self.closed_date {
            if !closed.trim().is_empty() {
                Some(parse_datetime(closed)?)
            } else {
                None
            }
        } else {
            None
        };

        let latitude = if let Some(ref lat_str) = self.latitude {
            lat_str.trim().parse::<f64>().ok()
        } else {
            None
        };

        let longitude = if let Some(ref lon_str) = self.longitude {
            lon_str.trim().parse::<f64>().ok()
        } else {
            None
        };

        Ok(ServiceRequest {
            unique_key,
            created_at,
            closed_at,
            complaint_type: self.complaint_type.trim().to_string(),
            descriptor: self.descriptor.as_ref().map(|s| s.trim().to_string()),
            borough: self.borough.as_ref().map(|s| s.trim().to_uppercase()),
            latitude,
            longitude,
        })
    }
}

fn parse_datetime(date_str: &str) -> Result<DateTime<Utc>> {
    // Try parsing common formats
    let formats = vec![
        "%Y-%m-%d %H:%M:%S",
        "%m/%d/%Y %H:%M:%S %p",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%d",
    ];

    for format in formats {
        if let Ok(naive) = NaiveDateTime::parse_from_str(date_str.trim(), format) {
            return Ok(naive.and_utc());
        }
    }

    Err(anyhow::anyhow!("Unable to parse date: {}", date_str))
}

pub struct Extractor {
    chunk_size: usize,
}

impl Extractor {
    pub fn new(chunk_size: usize) -> Self {
        Self { chunk_size }
    }

    pub async fn extract(&self, input_path: &str) -> Result<Vec<Vec<ServiceRequest>>> {
        info!("Starting CSV extraction from: {}", input_path);

        let file = File::open(input_path)
            .await
            .context(format!("Failed to open file: {}", input_path))?;

        let mut reader = AsyncReaderBuilder::new()
            .has_headers(true)
            .create_deserializer(file);

        let mut all_chunks = Vec::new();
        let mut current_chunk = Vec::new();
        let mut total_read = 0;
        let mut total_errors = 0;

        let mut records = reader.deserialize::<CsvRecord>();

        while let Some(result) = records.next().await {
            match result {
                Ok(csv_record) => {
                    match csv_record.to_service_request() {
                        Ok(service_request) => {
                            current_chunk.push(service_request);
                            total_read += 1;

                            if current_chunk.len() >= self.chunk_size {
                                debug!("Chunk complete with {} records", current_chunk.len());
                                all_chunks.push(std::mem::take(&mut current_chunk));
                            }
                        }
                        Err(e) => {
                            total_errors += 1;
                            warn!("Failed to convert CSV record: {}", e);
                        }
                    }
                }
                Err(e) => {
                    total_errors += 1;
                    warn!("Failed to parse CSV row: {}", e);
                }
            }
        }

        // Don't forget the last chunk
        if !current_chunk.is_empty() {
            all_chunks.push(current_chunk);
        }

        info!(
            "Extraction complete: {} records read, {} errors, {} chunks",
            total_read,
            total_errors,
            all_chunks.len()
        );

        Ok(all_chunks)
    }
}

impl Default for Extractor {
    fn default() -> Self {
        Self::new(100_000)
    }
}
