//! Capture domain types - Multi-modal input capture

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::enums::{CaptureSource, CaptureStatus, CaptureType};
use super::item::GeoPoint;
#[cfg(feature = "specta")]
use super::serde_helpers::ThingType;
use super::serde_helpers::{option_thing_serde, thing_serde};

/// Capture - Temporary storage for unprocessed input
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Capture {
    #[serde(with = "option_thing_serde")]
    #[cfg_attr(feature = "specta", specta(type = Option<ThingType>))]
    pub id: Option<Thing>,
    pub text_content: Option<String>,
    pub capture_type: CaptureType,
    pub source: CaptureSource,
    pub status: CaptureStatus,
    #[cfg_attr(feature = "specta", specta(type = Option<ThingType>))]
    pub processed_to: Option<Thing>, // reference to processed entity
    pub ai_suggestion: Option<String>,
    pub ai_confidence: Option<f32>,
    pub location: Option<GeoPoint>,
    #[serde(with = "thing_serde")]
    #[cfg_attr(feature = "specta", specta(type = ThingType))]
    pub owner: Thing,
    pub device_id: String,
    pub captured_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Capture {
    /// Check if capture has been processed
    pub fn is_processed(&self) -> bool {
        matches!(self.status, CaptureStatus::Processed)
    }

    /// Check if AI provided high-confidence suggestion
    pub fn has_high_confidence_suggestion(&self, threshold: f32) -> bool {
        if let Some(confidence) = self.ai_confidence {
            confidence >= threshold
        } else {
            false
        }
    }

    /// Mark as processed with destination entity
    pub fn mark_processed(&mut self, destination: Thing) {
        self.status = CaptureStatus::Processed;
        self.processed_to = Some(destination);
        self.updated_at = Utc::now();
    }

    /// Discard the capture
    pub fn discard(&mut self) {
        self.status = CaptureStatus::Discarded;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capture_processing() {
        let mut capture = Capture {
            id: None,
            text_content: Some("Test capture".to_string()),
            capture_type: CaptureType::Text,
            source: CaptureSource::Desktop,
            status: CaptureStatus::Pending,
            processed_to: None,
            ai_suggestion: Some("Create a note".to_string()),
            ai_confidence: Some(0.95),
            location: None,
            owner: Thing::from(("user".to_string(), "test".to_string())),
            device_id: "device1".to_string(),
            captured_at: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert!(!capture.is_processed());
        assert!(capture.has_high_confidence_suggestion(0.9));
        assert!(!capture.has_high_confidence_suggestion(0.99));

        let dest = Thing::from(("note".to_string(), "123".to_string()));
        capture.mark_processed(dest);
        assert!(capture.is_processed());
        assert!(capture.processed_to.is_some());
    }

    #[test]
    fn test_capture_discard() {
        let mut capture = Capture {
            id: None,
            text_content: Some("Spam".to_string()),
            capture_type: CaptureType::Text,
            source: CaptureSource::Mobile,
            status: CaptureStatus::Pending,
            processed_to: None,
            ai_suggestion: None,
            ai_confidence: None,
            location: None,
            owner: Thing::from(("user".to_string(), "test".to_string())),
            device_id: "device1".to_string(),
            captured_at: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        capture.discard();
        assert_eq!(capture.status, CaptureStatus::Discarded);
    }
}
