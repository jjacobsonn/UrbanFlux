// CSV parser with proper error handling
use crate::domain::{Borough, Coordinates, ServiceRequest};
use crate::error::{EtlError, Result};
use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CsvRecord {
    pub unique_key: String,
    pub created_date: String,
    pub closed_date: Option<String>,
    pub complaint_type: String,
    pub descriptor: Option<String>,
    pub borough: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
}

pub struct CsvParser;

impl CsvParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse a CSV record into a ServiceRequest
    pub fn parse(&self, record: CsvRecord) -> Result<ServiceRequest> {
        let unique_key = record
            .unique_key
            .trim()
            .parse::<i64>()
            .map_err(|e| EtlError::CsvParse(format!("Invalid unique_key: {}", e)))?;

        let created_at = Self::parse_datetime(&record.created_date)?;

        let closed_at = record
            .closed_date
            .as_ref()
            .filter(|s| !s.trim().is_empty())
            .map(|s| Self::parse_datetime(s))
            .transpose()?;

        let borough = record
            .borough
            .as_ref()
            .and_then(|s| Borough::from_str(s.trim()));

        let coordinates = match (record.latitude.as_ref(), record.longitude.as_ref()) {
            (Some(lat_str), Some(lon_str)) => {
                match (lat_str.trim().parse::<f64>(), lon_str.trim().parse::<f64>()) {
                    (Ok(lat), Ok(lon)) => Coordinates::new(lat, lon),
                    _ => None,
                }
            }
            _ => None,
        };

        ServiceRequest::new(
            unique_key,
            created_at,
            closed_at,
            record.complaint_type,
            record.descriptor,
            borough,
            coordinates,
        )
        .map_err(|e| EtlError::Validation(e))
    }

    /// Parse datetime from string supporting multiple formats
    fn parse_datetime(date_str: &str) -> Result<DateTime<Utc>> {
        let s = date_str.trim();

        // Try datetime formats
        let dt_formats = [
            "%Y-%m-%d %H:%M:%S",
            "%m/%d/%Y %I:%M:%S %p",
            "%Y-%m-%dT%H:%M:%S",
            "%Y-%m-%d %H:%M",
        ];

        for fmt in dt_formats.iter() {
            if let Ok(naive) = NaiveDateTime::parse_from_str(s, fmt) {
                return Ok(Utc.from_utc_datetime(&naive));
            }
        }

        // Try parsing a date-only value and treat as midnight UTC
        if let Ok(date) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            let naive = date
                .and_hms_opt(0, 0, 0)
                .ok_or_else(|| EtlError::DateTimeParse("Invalid date".to_string()))?;
            return Ok(Utc.from_utc_datetime(&naive));
        }

        Err(EtlError::DateTimeParse(format!(
            "Unable to parse date: {}",
            date_str
        )))
    }
}

impl Default for CsvParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Timelike};

    #[test]
    fn test_parse_datetime_formats() {
        let s1 = "2025-01-01 10:00:00";
        let dt1 = CsvParser::parse_datetime(s1).unwrap();
        assert_eq!(dt1.year(), 2025);
        assert_eq!(dt1.hour(), 10);

        let s2 = "01/02/2025 03:04:05 PM";
        let dt2 = CsvParser::parse_datetime(s2).unwrap();
        assert_eq!(dt2.year(), 2025);
        assert_eq!(dt2.month(), 1);
        assert_eq!(dt2.day(), 2);
        assert_eq!(dt2.hour(), 15);

        let s3 = "2025-01-01";
        let dt3 = CsvParser::parse_datetime(s3).unwrap();
        assert_eq!(dt3.hour(), 0);
    }

    #[test]
    fn test_parse_valid_record() {
        let parser = CsvParser::new();
        let record = CsvRecord {
            unique_key: "42".to_string(),
            created_date: "2025-01-01 10:00:00".to_string(),
            closed_date: Some("2025-01-01 12:00:00".to_string()),
            complaint_type: "Noise".to_string(),
            descriptor: Some("Loud Music".to_string()),
            borough: Some("MANHATTAN".to_string()),
            latitude: Some("40.7580".to_string()),
            longitude: Some("-73.9855".to_string()),
        };

        let sr = parser.parse(record).unwrap();
        assert_eq!(sr.unique_key, 42);
        assert_eq!(sr.complaint_type, "Noise");
        assert_eq!(sr.borough, Some(Borough::Manhattan));
    }

    #[test]
    fn test_parse_invalid_key() {
        let parser = CsvParser::new();
        let record = CsvRecord {
            unique_key: "abc".to_string(),
            created_date: "2025-01-01 10:00:00".to_string(),
            closed_date: None,
            complaint_type: "Noise".to_string(),
            descriptor: None,
            borough: None,
            latitude: None,
            longitude: None,
        };

        assert!(parser.parse(record).is_err());
    }
}
