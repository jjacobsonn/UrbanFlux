// Load phase - Bulk insert to PostgreSQL
use anyhow::Result;
use tracing::info;

pub struct Loader;

impl Loader {
    pub fn new() -> Self {
        Self
    }

    pub async fn load(&self) -> Result<()> {
        info!("Load phase - to be implemented");
        // Loading logic will be implemented in next phase
        Ok(())
    }
}

impl Default for Loader {
    fn default() -> Self {
        Self::new()
    }
}
