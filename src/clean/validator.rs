// Data validation and quality rules
use anyhow::{anyhow, Result};

const VALID_BOROUGHS: &[&str] = &["BRONX", "BROOKLYN", "MANHATTAN", "QUEENS", "STATEN ISLAND"];

// NYC bounding box (approximate)
const MIN_LAT: f64 = 40.4;
const MAX_LAT: f64 = 41.2;
const MIN_LON: f64 = -74.3;
const MAX_LON: f64 = -73.4;

#[derive(Debug, Clone)]
pub struct Validator;

impl Validator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_borough(&self, borough: &str) -> bool {
        let normalized = borough.trim().to_uppercase();
        VALID_BOROUGHS.contains(&normalized.as_str())
    }

    pub fn normalize_borough(&self, borough: &str) -> Option<String> {
        let normalized = borough.trim().to_uppercase();
        if self.validate_borough(&normalized) {
            Some(normalized)
        } else {
            None
        }
    }

    pub fn validate_coordinates(&self, lat: f64, lon: f64) -> bool {
        lat >= MIN_LAT && lat <= MAX_LAT && lon >= MIN_LON && lon <= MAX_LON
    }

    pub fn validate_complaint_type(&self, complaint_type: &str) -> Result<String> {
        let trimmed = complaint_type.trim();
        if trimmed.is_empty() {
            return Err(anyhow!("Complaint type cannot be empty"));
        }
        Ok(trimmed.to_string())
    }

    pub fn clean_text(&self, text: &str) -> String {
        text.trim().to_string()
    }

    pub fn is_valid_unique_key(&self, key: i64) -> bool {
        key > 0
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_borough() {
        let validator = Validator::new();
        assert!(validator.validate_borough("MANHATTAN"));
        assert!(validator.validate_borough("manhattan"));
        assert!(validator.validate_borough("  BROOKLYN  "));
        assert!(!validator.validate_borough("INVALID"));
    }

    #[test]
    fn test_validate_coordinates() {
        let validator = Validator::new();
        // Valid NYC coordinates
        assert!(validator.validate_coordinates(40.7580, -73.9855));
        // Outside NYC
        assert!(!validator.validate_coordinates(42.0, -73.0));
    }

    #[test]
    fn test_clean_text() {
        let validator = Validator::new();
        assert_eq!(validator.clean_text("  test  "), "test");
    }
}
