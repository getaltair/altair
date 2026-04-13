// Source of truth: packages/contracts/entity-types.json, relation-types.json, sync-streams.json
// Do not add inline enum values here — update the JSON registries and regenerate bindings.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Canonical entity type identifiers. Every variant maps to a registry string via serde rename.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityType {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "household")]
    Household,
    #[serde(rename = "initiative")]
    Initiative,
    #[serde(rename = "tag")]
    Tag,
    #[serde(rename = "attachment")]
    Attachment,
    #[serde(rename = "guidance_epic")]
    GuidanceEpic,
    #[serde(rename = "guidance_quest")]
    GuidanceQuest,
    #[serde(rename = "guidance_routine")]
    GuidanceRoutine,
    #[serde(rename = "guidance_focus_session")]
    GuidanceFocusSession,
    #[serde(rename = "guidance_daily_checkin")]
    GuidanceDailyCheckin,
    #[serde(rename = "knowledge_note")]
    KnowledgeNote,
    #[serde(rename = "knowledge_note_snapshot")]
    KnowledgeNoteSnapshot,
    #[serde(rename = "tracking_location")]
    TrackingLocation,
    #[serde(rename = "tracking_category")]
    TrackingCategory,
    #[serde(rename = "tracking_item")]
    TrackingItem,
    #[serde(rename = "tracking_item_event")]
    TrackingItemEvent,
    #[serde(rename = "tracking_shopping_list")]
    TrackingShoppingList,
    #[serde(rename = "tracking_shopping_list_item")]
    TrackingShoppingListItem,
}

/// Canonical relation type identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationType {
    #[serde(rename = "references")]
    References,
    #[serde(rename = "supports")]
    Supports,
    #[serde(rename = "requires")]
    Requires,
    #[serde(rename = "related_to")]
    RelatedTo,
    #[serde(rename = "depends_on")]
    DependsOn,
    #[serde(rename = "duplicates")]
    Duplicates,
    #[serde(rename = "similar_to")]
    SimilarTo,
    #[serde(rename = "generated_from")]
    GeneratedFrom,
}

/// Provisional PowerSync sync stream names. Step 4 (Sync Engine) may revise these.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SyncStream {
    #[serde(rename = "user_data")]
    UserData,
    #[serde(rename = "household")]
    Household,
    #[serde(rename = "guidance")]
    Guidance,
    #[serde(rename = "knowledge")]
    Knowledge,
    #[serde(rename = "tracking")]
    Tracking,
}

/// A polymorphic reference to any entity by type and UUID.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityRef {
    pub entity_type: EntityType,
    pub entity_id: Uuid,
}

/// Mirrors the entity_relations table schema from docs/specs/05-erd.md.
/// Note: source_type and status are raw strings — typed enums live in Step 3 domain modules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationRecord {
    pub id: Uuid,
    pub from_entity_type: EntityType,
    pub from_entity_id: Uuid,
    pub to_entity_type: EntityType,
    pub to_entity_id: Uuid,
    pub relation_type: RelationType,
    pub source_type: String,
    pub status: String,
    pub confidence: Option<f64>,
    pub evidence: Option<String>,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Mirrors the attachments table schema from docs/specs/05-erd.md.
/// Note: state is a raw string — typed enum lives in Step 3 domain modules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentRecord {
    pub id: Uuid,
    pub entity_type: EntityType,
    pub entity_id: Uuid,
    pub file_name: String,
    pub content_type: String,
    pub size_bytes: Option<i64>,
    pub state: String,
    pub storage_path: Option<String>,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Parameters for subscribing to PowerSync sync streams.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncSubscriptionRequest {
    pub streams: Vec<SyncStream>,
    pub user_id: Uuid,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_type_guidance_epic_serializes_to_snake_case() {
        let val = serde_json::to_string(&EntityType::GuidanceEpic).unwrap();
        assert_eq!(val, r#""guidance_epic""#);
    }

    #[test]
    fn entity_type_user_serializes_to_user() {
        let val = serde_json::to_string(&EntityType::User).unwrap();
        assert_eq!(val, r#""user""#);
    }

    #[test]
    fn entity_type_unknown_string_fails_deserialization() {
        let result: Result<EntityType, _> = serde_json::from_str(r#""unknown_fake_type""#);
        assert!(
            result.is_err(),
            "Expected deserialization of unknown type to fail"
        );
    }

    #[test]
    fn relation_type_related_to_serializes_correctly() {
        let val = serde_json::to_string(&RelationType::RelatedTo).unwrap();
        assert_eq!(val, r#""related_to""#);
    }

    #[test]
    fn entity_ref_round_trips_through_serde() {
        let entity_ref = EntityRef {
            entity_type: EntityType::Initiative,
            entity_id: uuid::Uuid::nil(),
        };
        let json = serde_json::to_string(&entity_ref).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["entity_type"], "initiative");
        assert!(parsed["entity_id"].is_string());
    }
}
