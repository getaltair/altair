use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use uuid::Uuid;

/// `frequency_type` column value — stored as varchar in the DB.
/// Derived from the `FrequencyConfig` variant on insert; never stored independently.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum FrequencyType {
    Daily,
    Weekly,
    Interval,
}

/// Typed JSONB payload for routine frequency configuration.
/// Stored as a tagged JSON object: `{"type": "weekly", "days_of_week": ["monday","friday"]}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FrequencyConfig {
    Daily,
    Weekly { days_of_week: Vec<String> },
    Interval { interval_days: i32 },
}

impl FrequencyConfig {
    /// Returns the matching `FrequencyType` discriminant for the DB column.
    pub fn frequency_type(&self) -> FrequencyType {
        match self {
            FrequencyConfig::Daily => FrequencyType::Daily,
            FrequencyConfig::Weekly { .. } => FrequencyType::Weekly,
            FrequencyConfig::Interval { .. } => FrequencyType::Interval,
        }
    }

    /// Validates that the config produces at least one occurrence per period.
    /// Returns `None` if valid, or an error message string if invalid.
    pub fn validate(&self) -> Option<String> {
        match self {
            FrequencyConfig::Daily => None,
            FrequencyConfig::Weekly { days_of_week } => {
                if days_of_week.is_empty() {
                    Some("weekly frequency must specify at least one day_of_week".to_string())
                } else {
                    None
                }
            }
            FrequencyConfig::Interval { interval_days } => {
                if *interval_days <= 0 {
                    Some("interval frequency must have interval_days > 0".to_string())
                } else {
                    None
                }
            }
        }
    }
}

/// A guidance routine row as stored in the database.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Routine {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub frequency_type: FrequencyType,
    pub frequency_config: Json<FrequencyConfig>,
    /// Maps to the `status` VARCHAR column; values: `"active"`, `"paused"`.
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRoutineRequest {
    pub title: String,
    pub description: Option<String>,
    pub frequency_config: FrequencyConfig,
    /// Optional; defaults to `true` (active).
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRoutineRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub frequency_config: Option<FrequencyConfig>,
    pub is_active: Option<bool>,
}

/// Request body for the `POST /:id/spawn` handler.
#[derive(Debug, Deserialize)]
pub struct SpawnRequest {
    pub due_date: chrono::NaiveDate,
}
