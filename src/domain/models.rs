// Core domain models
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// NYC Borough enumeration with compile-time safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Borough {
    Bronx,
    Brooklyn,
    Manhattan,
    Queens,
    #[serde(rename = "STATEN ISLAND")]
    StatenIsland,
}

impl Borough {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim().to_uppercase().as_str() {
            "BRONX" => Some(Borough::Bronx),
            "BROOKLYN" => Some(Borough::Brooklyn),
            "MANHATTAN" => Some(Borough::Manhattan),
            "QUEENS" => Some(Borough::Queens),
            "STATEN ISLAND" => Some(Borough::StatenIsland),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Borough::Bronx => "BRONX",
            Borough::Brooklyn => "BROOKLYN",
            Borough::Manhattan => "MANHATTAN",
            Borough::Queens => "QUEENS",
            Borough::StatenIsland => "STATEN ISLAND",
        }
    }
}

/// Geographic coordinates with validation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

impl Coordinates {
    pub fn new(latitude: f64, longitude: f64) -> Option<Self> {
        if Self::is_valid_nyc_coords(latitude, longitude) {
            Some(Self { latitude, longitude })
        } else {
            None
        }
    }

    fn is_valid_nyc_coords(lat: f64, lon: f64) -> bool {
        lat >= 40.4 && lat <= 41.2 && lon >= -74.3 && lon <= -73.4
    }
}

/// Service Request - the core domain entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRequest {
    pub unique_key: i64,
    pub created_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub complaint_type: String,
    pub descriptor: Option<String>,
    pub borough: Option<Borough>,
    pub coordinates: Option<Coordinates>,
    // For database mapping
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

impl ServiceRequest {
    /// Create a new service request with validation
    pub fn new(
        unique_key: i64,
        created_at: DateTime<Utc>,
        closed_at: Option<DateTime<Utc>>,
        complaint_type: String,
        descriptor: Option<String>,
        borough: Option<Borough>,
        coordinates: Option<Coordinates>,
    ) -> Result<Self, String> {
        if unique_key <= 0 {
            return Err("unique_key must be positive".to_string());
        }

        if complaint_type.trim().is_empty() {
            return Err("complaint_type cannot be empty".to_string());
        }

        if let Some(closed) = closed_at {
            if closed < created_at {
                return Err("closed_at must be >= created_at".to_string());
            }
        }

        let (latitude, longitude) = coordinates
            .map(|c| (Some(c.latitude), Some(c.longitude)))
            .unwrap_or((None, None));

        Ok(Self {
            unique_key,
            created_at,
            closed_at,
            complaint_type: complaint_type.trim().to_string(),
            descriptor: descriptor.map(|s| s.trim().to_string()),
            borough,
            coordinates,
            latitude,
            longitude,
        })
    }
}

/// ETL run mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EtlMode {
    Full,
    Incremental,
}

impl std::str::FromStr for EtlMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "full" => Ok(EtlMode::Full),
            "incremental" => Ok(EtlMode::Incremental),
            _ => Err(format!("Invalid ETL mode: {}", s)),
        }
    }
}

/// ETL run statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EtlStats {
    pub rows_read: u64,
    pub rows_parsed: u64,
    pub rows_validated: u64,
    pub rows_inserted: u64,
    pub rows_duplicated: u64,
    pub rows_rejected: u64,
    pub parse_errors: u64,
    pub validation_errors: u64,
}

impl EtlStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn merge(&mut self, other: &EtlStats) {
        self.rows_read += other.rows_read;
        self.rows_parsed += other.rows_parsed;
        self.rows_validated += other.rows_validated;
        self.rows_inserted += other.rows_inserted;
        self.rows_duplicated += other.rows_duplicated;
        self.rows_rejected += other.rows_rejected;
        self.parse_errors += other.parse_errors;
        self.validation_errors += other.validation_errors;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_borough_parsing() {
        assert_eq!(Borough::from_str("MANHATTAN"), Some(Borough::Manhattan));
        assert_eq!(Borough::from_str("manhattan"), Some(Borough::Manhattan));
        assert_eq!(Borough::from_str("  BROOKLYN  "), Some(Borough::Brooklyn));
        assert_eq!(Borough::from_str("STATEN ISLAND"), Some(Borough::StatenIsland));
        assert_eq!(Borough::from_str("invalid"), None);
    }

    #[test]
    fn test_coordinates_validation() {
        // Valid NYC coordinates
        assert!(Coordinates::new(40.7580, -73.9855).is_some());
        // Outside NYC
        assert!(Coordinates::new(42.0, -73.0).is_none());
        assert!(Coordinates::new(40.5, -80.0).is_none());
    }

    #[test]
    fn test_service_request_validation() {
        let now = Utc::now();
        
        // Valid request
        let req = ServiceRequest::new(
            42,
            now,
            None,
            "Noise".to_string(),
            Some("Loud music".to_string()),
            Some(Borough::Manhattan),
            Some(Coordinates::new(40.7580, -73.9855).unwrap()),
        );
        assert!(req.is_ok());

        // Invalid unique_key
        let req = ServiceRequest::new(
            0,
            now,
            None,
            "Noise".to_string(),
            None,
            None,
            None,
        );
        assert!(req.is_err());

        // Invalid closed_at
        let req = ServiceRequest::new(
            42,
            now,
            Some(now - chrono::Duration::hours(1)),
            "Noise".to_string(),
            None,
            None,
            None,
        );
        assert!(req.is_err());
    }
}
