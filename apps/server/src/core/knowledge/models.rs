use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Database-backed knowledge note record
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct KnowledgeNote {
    pub id: Uuid,
    pub user_id: Uuid,
    pub household_id: Option<Uuid>,
    pub initiative_id: Option<Uuid>,
    pub title: String,
    pub content: Option<String>,
    pub content_type: String,
    pub is_pinned: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database-backed knowledge note snapshot record (immutable revision history)
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct KnowledgeNoteSnapshot {
    pub id: Uuid,
    pub note_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub created_by_process: Option<String>,
}

/// Request payload for creating a new knowledge note
#[derive(Debug, Deserialize, Validate)]
pub struct CreateNoteRequest {
    #[validate(length(min = 1, max = 500))]
    pub title: String,
    pub content: Option<String>,
    pub content_type: Option<String>,
    pub household_id: Option<Uuid>,
    pub initiative_id: Option<Uuid>,
    pub is_pinned: Option<bool>,
}

/// Request payload for updating an existing knowledge note.
///
/// The `content`, `household_id`, and `initiative_id` fields use
/// `Option<Option<T>>` to distinguish three JSON states: field absent
/// (don't touch), field is `null` (set to NULL), field has a value
/// (set to that value).
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateNoteRequest {
    #[validate(length(min = 1, max = 500))]
    pub title: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::serde_util::double_option"
    )]
    pub content: Option<Option<String>>,
    pub content_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::serde_util::double_option"
    )]
    pub household_id: Option<Option<Uuid>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::serde_util::double_option"
    )]
    pub initiative_id: Option<Option<Uuid>>,
    pub is_pinned: Option<bool>,
}

/// Request payload for creating a manual snapshot
#[derive(Debug, Deserialize)]
pub struct CreateSnapshotRequest {
    pub created_by_process: Option<String>,
}

/// Query parameters for listing knowledge notes
#[derive(Debug, Deserialize)]
pub struct ListNotesQuery {
    pub household_id: Option<Uuid>,
    pub initiative_id: Option<Uuid>,
    pub is_pinned: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_request_valid_title_passes() {
        let req = CreateNoteRequest {
            title: "My note".to_string(),
            content: None,
            content_type: None,
            household_id: None,
            initiative_id: None,
            is_pinned: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn create_request_empty_title_fails() {
        let req = CreateNoteRequest {
            title: "".to_string(),
            content: None,
            content_type: None,
            household_id: None,
            initiative_id: None,
            is_pinned: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn create_request_title_over_500_chars_fails() {
        let req = CreateNoteRequest {
            title: "a".repeat(501),
            content: None,
            content_type: None,
            household_id: None,
            initiative_id: None,
            is_pinned: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn create_request_title_exactly_500_chars_passes() {
        let req = CreateNoteRequest {
            title: "a".repeat(500),
            content: None,
            content_type: None,
            household_id: None,
            initiative_id: None,
            is_pinned: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn update_request_valid_title_passes() {
        let req = UpdateNoteRequest {
            title: Some("Updated title".to_string()),
            content: None,
            content_type: None,
            household_id: None,
            initiative_id: None,
            is_pinned: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn update_request_empty_title_fails() {
        let req = UpdateNoteRequest {
            title: Some("".to_string()),
            content: None,
            content_type: None,
            household_id: None,
            initiative_id: None,
            is_pinned: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn update_request_all_none_passes() {
        let req = UpdateNoteRequest {
            title: None,
            content: None,
            content_type: None,
            household_id: None,
            initiative_id: None,
            is_pinned: None,
        };
        assert!(req.validate().is_ok());
    }

    // -- Double-option deserialization for content --------------------------------

    #[test]
    fn update_request_content_absent_is_none() {
        let json = r#"{"title": "test"}"#;
        let req: UpdateNoteRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.content, None); // field absent -> None (don't touch)
    }

    #[test]
    fn update_request_content_explicit_null_is_some_none() {
        let json = r#"{"title": "test", "content": null}"#;
        let req: UpdateNoteRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.content, Some(None)); // field null -> Some(None) (set to NULL)
    }

    #[test]
    fn update_request_content_with_value_is_some_some() {
        let json = r#"{"title": "test", "content": "hello"}"#;
        let req: UpdateNoteRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.content, Some(Some("hello".to_string())));
    }

    // -- Double-option deserialization for household_id ----------------------------

    #[test]
    fn update_request_household_id_absent_is_none() {
        let json = r#"{"title": "test"}"#;
        let req: UpdateNoteRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.household_id, None);
    }

    #[test]
    fn update_request_household_id_explicit_null_is_some_none() {
        let json = r#"{"title": "test", "household_id": null}"#;
        let req: UpdateNoteRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.household_id, Some(None));
    }
}
