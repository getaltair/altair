//! Enum types matching SurrealDB ASSERT constraints
//!
//! All enums use snake_case serialization to match SurrealDB storage format.

use serde::{Deserialize, Serialize};

/// Quest board column position
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestColumn {
    IdeaGreenhouse,
    QuestLog,
    ThisCycle,
    NextUp,
    InProgress,
    Harvested,
    Archived,
}

/// Energy cost to complete a quest (user effort estimation)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnergyCost {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
}

/// User's daily energy level self-assessment (1-5 scale)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct EnergyLevel(u8);

impl EnergyLevel {
    pub fn new(level: u8) -> Result<Self, String> {
        if (1..=5).contains(&level) {
            Ok(Self(level))
        } else {
            Err(format!("Energy level must be 1-5, got {}", level))
        }
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

/// General entity status (soft delete pattern)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityStatus {
    #[default]
    Active,
    Archived,
}

/// Item availability status
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemStatus {
    #[default]
    Available,
    Reserved,
    InUse,
    Depleted,
    Archived,
}

/// Reservation lifecycle status
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReservationStatus {
    #[default]
    Pending,
    InUse,
    Released,
}

/// Capture processing status
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureStatus {
    #[default]
    Pending,
    Processed,
    Discarded,
}

/// Capture input method
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureType {
    Text,
    Voice,
    Photo,
    Video,
    Mixed,
}

/// Source of capture input
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureSource {
    Desktop,
    Mobile,
    Widget,
    VoiceAssistant,
}

/// Streak tracking type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StreakType {
    DailyCheckin,
    QuestCompletion,
    FocusSession,
}

/// Attachment media type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaType {
    Photo,
    Audio,
    Video,
    Document,
}

/// User role for authorization
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    #[default]
    Owner,
    Viewer,
}

/// Focus session status
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FocusSessionStatus {
    #[default]
    Active,
    Completed,
    Abandoned,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_level_validation() {
        assert!(EnergyLevel::new(1).is_ok());
        assert!(EnergyLevel::new(5).is_ok());
        assert!(EnergyLevel::new(0).is_err());
        assert!(EnergyLevel::new(6).is_err());
    }

    #[test]
    fn test_enum_serialization() {
        let column = QuestColumn::InProgress;
        let json = serde_json::to_string(&column).unwrap();
        assert_eq!(json, "\"in_progress\"");

        let cost = EnergyCost::Medium;
        let json = serde_json::to_string(&cost).unwrap();
        assert_eq!(json, "\"medium\"");
    }

    #[test]
    fn test_default_values() {
        assert_eq!(EntityStatus::default(), EntityStatus::Active);
        assert_eq!(ItemStatus::default(), ItemStatus::Available);
        assert_eq!(UserRole::default(), UserRole::Owner);
    }
}
