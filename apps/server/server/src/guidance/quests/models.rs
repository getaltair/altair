use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Status of a guidance quest, stored as a varchar in the database.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum QuestStatus {
    NotStarted,
    InProgress,
    Completed,
    Deferred,
    Cancelled,
}

impl QuestStatus {
    /// Returns true if transitioning from `self` to `next` is a valid state machine move.
    pub fn can_transition_to(&self, next: &QuestStatus) -> bool {
        // Cancelled is a terminal state — no transitions out.
        if *self == QuestStatus::Cancelled {
            return false;
        }
        // Completed is a terminal state — no transitions out.
        if *self == QuestStatus::Completed {
            return false;
        }
        matches!(
            (self, next),
            (Self::NotStarted, Self::InProgress)
                | (Self::InProgress, Self::Completed)
                | (Self::InProgress, Self::Deferred)
                | (Self::Deferred, Self::NotStarted)
                | (_, Self::Cancelled)
        )
    }
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Quest {
    pub id: Uuid,
    pub user_id: Uuid,
    pub initiative_id: Option<Uuid>,
    pub epic_id: Option<Uuid>,
    pub routine_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub status: QuestStatus,
    pub priority: String,
    pub due_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateQuestRequest {
    pub initiative_id: Option<Uuid>,
    pub epic_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub status: Option<QuestStatus>,
    pub priority: Option<String>,
    pub due_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateQuestRequest {
    pub initiative_id: Option<Uuid>,
    pub epic_id: Option<Uuid>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<QuestStatus>,
    pub priority: Option<String>,
    pub due_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct QuestListParams {
    pub status: Option<QuestStatus>,
    pub priority: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub initiative_id: Option<Uuid>,
}

// ---------------------------------------------------------------------------
// Tests (S004-T) — pure logic, no DB needed
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::QuestStatus;

    #[test]
    fn not_started_to_in_progress_allowed() {
        assert!(QuestStatus::NotStarted.can_transition_to(&QuestStatus::InProgress));
    }

    #[test]
    fn in_progress_to_completed_allowed() {
        assert!(QuestStatus::InProgress.can_transition_to(&QuestStatus::Completed));
    }

    #[test]
    fn in_progress_to_deferred_allowed() {
        assert!(QuestStatus::InProgress.can_transition_to(&QuestStatus::Deferred));
    }

    #[test]
    fn deferred_to_not_started_allowed() {
        assert!(QuestStatus::Deferred.can_transition_to(&QuestStatus::NotStarted));
    }

    #[test]
    fn not_started_to_cancelled_allowed() {
        assert!(QuestStatus::NotStarted.can_transition_to(&QuestStatus::Cancelled));
    }

    #[test]
    fn in_progress_to_cancelled_allowed() {
        assert!(QuestStatus::InProgress.can_transition_to(&QuestStatus::Cancelled));
    }

    #[test]
    fn deferred_to_cancelled_allowed() {
        assert!(QuestStatus::Deferred.can_transition_to(&QuestStatus::Cancelled));
    }

    #[test]
    fn not_started_to_completed_rejected() {
        assert!(!QuestStatus::NotStarted.can_transition_to(&QuestStatus::Completed));
    }

    #[test]
    fn completed_to_not_started_rejected() {
        assert!(!QuestStatus::Completed.can_transition_to(&QuestStatus::NotStarted));
    }

    #[test]
    fn deferred_to_completed_rejected() {
        assert!(!QuestStatus::Deferred.can_transition_to(&QuestStatus::Completed));
    }

    #[test]
    fn cancelled_to_in_progress_rejected() {
        assert!(!QuestStatus::Cancelled.can_transition_to(&QuestStatus::InProgress));
    }
}
