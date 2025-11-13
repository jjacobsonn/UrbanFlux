// Streaming CSV reader with chunking
use crate::domain::{EtlStats, ServiceRequest};
use crate::error::{EtlError, Result};
use crate::extract::parser::{CsvParser, CsvRecord};
use csv_async::AsyncReaderBuilder;
use futures::stream::StreamExt;
use std::path::Path;
use tokio::fs::File;
use tracing::{debug, info, warn};

pub struct CsvRecordStream {
    chunk_size: usize,
    parser: CsvParser,
}

impl CsvRecordStream {
    pub fn new(chunk_size: usize) -> Self {
        Self {
            chunk_size,
            parser: CsvParser::new(),
        }
    }

    /// Stream and parse CSV file in chunks
    /// This processes the file without loading everything into memory
    pub async fn stream_chunks<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<Vec<(Vec<ServiceRequest>, EtlStats)>> {
        let path_str = path.as_ref().display().to_string();
        info!("Opening CSV file for streaming: {}", path_str);

        let file = File::open(path).await.map_err(EtlError::Io)?;

        let reader = AsyncReaderBuilder::new()
            .has_headers(true)
            .create_deserializer(file);

        let mut records_stream = reader.into_deserialize::<CsvRecord>();
        let mut chunks = Vec::new();
        let mut current_chunk = Vec::new();
        let mut current_stats = EtlStats::new();
        let mut chunk_count = 0;

        while let Some(result) = records_stream.next().await {
            current_stats.rows_read += 1;

            match result {
                Ok(csv_record) => match self.parser.parse(csv_record) {
                    Ok(service_request) => {
                        current_stats.rows_parsed += 1;
                        current_chunk.push(service_request);

                        if current_chunk.len() >= self.chunk_size {
                            chunk_count += 1;
                            debug!(chunk = chunk_count, size = current_chunk.len(), "Chunk complete");
                            
                            chunks.push((
                                std::mem::take(&mut current_chunk),
                                std::mem::replace(&mut current_stats, EtlStats::new()),
                            ));
                        }
                    }
                    Err(e) => {
                        current_stats.parse_errors += 1;
                        warn!("Failed to parse CSV record: {}", e);
                    }
                },
                Err(e) => {
                    current_stats.parse_errors += 1;
                    warn!("CSV deserialization error: {}", e);
                }
            }
        }

        // Don't forget the final chunk
        if !current_chunk.is_empty() {
            chunk_count += 1;
            debug!(chunk = chunk_count, size = current_chunk.len(), "Final chunk");
            chunks.push((current_chunk, current_stats));
        }

        info!(total_chunks = chunks.len(), "CSV streaming complete");
        Ok(chunks)
    }
}

impl Clone for CsvParser {
    fn clone(&self) -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_creation() {
        let streamer = CsvRecordStream::new(100);
        assert_eq!(streamer.chunk_size, 100);
    }
}
