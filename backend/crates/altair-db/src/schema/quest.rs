//! Quest domain types - Quest-Based Agile entities

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::enums::{EnergyCost, EntityStatus, FocusSessionStatus, QuestColumn};

/// Campaign - Container for related quests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Campaign {
    pub id: Option<Thing>,
    pub title: String,
    pub description: Option<String>,
    pub status: EntityStatus,
    pub color: Option<String>,
    pub owner: Thing,
    pub device_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Quest - Individual task with energy cost
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quest {
    pub id: Option<Thing>,
    pub title: String,
    pub description: Option<String>,
    pub column: QuestColumn,
    pub energy_cost: EnergyCost,
    pub estimated_minutes: Option<i32>,
    pub actual_minutes: Option<i32>,
    pub xp_value: i32,
    pub due_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub status: EntityStatus,
    pub owner: Thing,
    pub device_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Quest {
    /// Calculate XP value based on energy cost
    pub fn calculate_xp(energy_cost: &EnergyCost) -> i32 {
        match energy_cost {
            EnergyCost::Tiny => 10,
            EnergyCost::Small => 25,
            EnergyCost::Medium => 50,
            EnergyCost::Large => 100,
            EnergyCost::Huge => 200,
        }
    }
}

/// FocusSession - Timed work session on a quest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusSession {
    pub id: Option<Thing>,
    pub started_at: DateTime<Utc>,
    pub planned_duration: i32, // minutes
    pub actual_duration: Option<i32>,
    pub completed_steps: Option<String>,
    pub status: FocusSessionStatus,
    pub notes: Option<String>,
    pub owner: Thing,
    pub device_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// EnergyCheckIn - Daily energy level self-assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyCheckIn {
    pub id: Option<Thing>,
    pub date: chrono::NaiveDate,
    pub energy_level: i32, // 1-5 scale
    pub notes: Option<String>,
    pub owner: Thing,
    pub device_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quest_xp_calculation() {
        assert_eq!(Quest::calculate_xp(&EnergyCost::Tiny), 10);
        assert_eq!(Quest::calculate_xp(&EnergyCost::Small), 25);
        assert_eq!(Quest::calculate_xp(&EnergyCost::Medium), 50);
        assert_eq!(Quest::calculate_xp(&EnergyCost::Large), 100);
        assert_eq!(Quest::calculate_xp(&EnergyCost::Huge), 200);
    }
}
