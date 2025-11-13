// Data validation logic
use crate::domain::models::ServiceRequest;

/// Validator for service request data
#[derive(Debug, Clone)]
pub struct ServiceRequestValidator;

impl ServiceRequestValidator {
    pub fn new() -> Self {
        Self
    }

    /// Validate a service request entity
    pub fn validate(&self, request: &ServiceRequest) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if request.unique_key <= 0 {
            errors.push("unique_key must be positive".to_string());
        }

        if request.complaint_type.trim().is_empty() {
            errors.push("complaint_type cannot be empty".to_string());
        }

        if let Some(closed) = request.closed_at {
            if closed < request.created_at {
                errors.push("closed_at must be >= created_at".to_string());
            }
        }

        if let (Some(lat), Some(lon)) = (request.latitude, request.longitude) {
            if !Self::is_valid_nyc_coords(lat, lon) {
                errors.push(format!("coordinates out of NYC bounds: ({}, {})", lat, lon));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn is_valid_nyc_coords(lat: f64, lon: f64) -> bool {
        lat >= 40.4 && lat <= 41.2 && lon >= -74.3 && lon <= -73.4
    }
}

impl Default for ServiceRequestValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Borough, Coordinates};
    use chrono::Utc;

    fn make_valid_request() -> ServiceRequest {
        ServiceRequest {
            unique_key: 42,
            created_at: Utc::now(),
            closed_at: None,
            complaint_type: "Noise".to_string(),
            descriptor: Some("Loud music".to_string()),
            borough: Some(Borough::Manhattan),
            coordinates: Some(Coordinates::new(40.7580, -73.9855).unwrap()),
            latitude: Some(40.7580),
            longitude: Some(-73.9855),
        }
    }

    #[test]
    fn test_validator_accepts_valid_request() {
        let validator = ServiceRequestValidator::new();
        let req = make_valid_request();
        assert!(validator.validate(&req).is_ok());
    }

    #[test]
    fn test_validator_rejects_invalid_key() {
        let validator = ServiceRequestValidator::new();
        let mut req = make_valid_request();
        req.unique_key = 0;
        assert!(validator.validate(&req).is_err());
    }

    #[test]
    fn test_validator_rejects_invalid_dates() {
        let validator = ServiceRequestValidator::new();
        let mut req = make_valid_request();
        req.closed_at = Some(req.created_at - chrono::Duration::hours(1));
        assert!(validator.validate(&req).is_err());
    }

    #[test]
    fn test_validator_rejects_out_of_bounds_coords() {
        let validator = ServiceRequestValidator::new();
        let mut req = make_valid_request();
        req.latitude = Some(42.0);
        req.longitude = Some(-73.0);
        assert!(validator.validate(&req).is_err());
    }
}
