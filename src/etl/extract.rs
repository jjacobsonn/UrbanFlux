// Extract phase - CSV streaming and parsing
use anyhow::Result;
use tracing::info;

pub struct Extractor;

impl Extractor {
    pub fn new() -> Self {
        Self
    }

    pub async fn extract(&self, _input_path: &str, _chunk_size: usize) -> Result<()> {
        info!("Extract phase - to be implemented");
        // CSV streaming logic will be implemented in next phase
        Ok(())
    }
}

impl Default for Extractor {
    fn default() -> Self {
        Self::new()
    }
}
