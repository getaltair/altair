use crate::contracts::EntityType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Client-to-server mutation operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Operation {
    Create,
    Update,
    Delete,
}

/// A single client mutation packaged for upload to the server.
///
/// Clients include their last-known `updated_at` for an entity as `base_version`.
/// The server compares this against the row's current `updated_at` to detect conflicts.
/// `base_version` is null for create operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationEnvelope {
    /// Client-generated stable UUID for this mutation; used for idempotent dedup (invariant S-2).
    pub mutation_id: Uuid,
    /// Client device identifier.
    pub device_id: Uuid,
    /// Validated entity type; must be a member of the EntityType registry (invariant C-1).
    pub entity_type: EntityType,
    /// Target entity UUID.
    pub entity_id: Uuid,
    /// Create, update, or delete.
    pub operation: Operation,
    /// Full entity state for create/update; null for delete.
    pub payload: Option<serde_json::Value>,
    /// Client's last-known `updated_at` for this entity. Null on create.
    pub base_version: Option<DateTime<Utc>>,
    /// Client wall-clock time of the mutation; used for LWW tiebreaking.
    pub occurred_at: DateTime<Utc>,
}

/// Batch upload request from a client connector.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncUploadRequest {
    pub mutations: Vec<MutationEnvelope>,
}

/// Per-mutation outcome status.
///
/// Serialised with a flat `"status"` discriminant field so the JSON shape matches Tech.md:
/// `{ "mutation_id": "...", "status": "accepted" }`
/// `{ "mutation_id": "...", "status": "conflicted", "conflict_id": "..." }`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum MutationStatus {
    /// Mutation was validated and applied to the database.
    Accepted,
    /// Mutation was already present in `sync_mutations`; skipped without re-applying.
    Deduplicated,
    /// Conflict detected; a `sync_conflicts` row was written. Includes the conflict record ID.
    Conflicted { conflict_id: Uuid },
}

/// Outcome for a single mutation in a batch upload response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationResult {
    pub mutation_id: Uuid,
    #[serde(flatten)]
    pub status: MutationStatus,
}

/// Batch upload response: one result per submitted mutation, in submission order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncUploadResponse {
    pub results: Vec<MutationResult>,
}

/// A single pending conflict record returned by the database query.
/// `entity_type` is read back as `String` and converted to `EntityType` post-query.
#[derive(Debug, Clone, sqlx::FromRow)]
pub(crate) struct ConflictRow {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub base_version: Option<DateTime<Utc>>,
    pub current_version: Option<DateTime<Utc>>,
    pub incoming_payload: Option<serde_json::Value>,
    pub current_payload: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

/// A single pending conflict record returned to the authenticated user.
#[derive(Debug, Clone, Serialize)]
pub struct ConflictRecord {
    pub id: Uuid,
    pub entity_type: EntityType,
    pub entity_id: Uuid,
    pub base_version: Option<DateTime<Utc>>,
    pub current_version: Option<DateTime<Utc>>,
    pub incoming_payload: Option<serde_json::Value>,
    pub current_payload: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

impl ConflictRecord {
    /// Convert a `ConflictRow` to a `ConflictRecord`, parsing `entity_type`.
    /// Returns `None` if the stored string is not a known `EntityType` variant.
    pub(crate) fn from_row(row: ConflictRow) -> Option<Self> {
        let entity_type: EntityType =
            serde_json::from_value(serde_json::Value::String(row.entity_type)).ok()?;
        Some(Self {
            id: row.id,
            entity_type,
            entity_id: row.entity_id,
            base_version: row.base_version,
            current_version: row.current_version,
            incoming_payload: row.incoming_payload,
            current_payload: row.current_payload,
            created_at: row.created_at,
        })
    }
}

/// Query parameters for paginating the conflicts list.
#[derive(Debug, Clone, Deserialize)]
pub struct ConflictPageParams {
    pub cursor: Option<Uuid>,
    pub limit: Option<u32>,
}

/// Paginated response for the conflicts list endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct ConflictListResponse {
    pub conflicts: Vec<ConflictRecord>,
    pub next_cursor: Option<Uuid>,
}

/// Resolution decision for a conflict.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictResolution {
    Accepted,
    Rejected,
}

impl ConflictResolution {
    pub fn as_str(&self) -> &'static str {
        match self {
            ConflictResolution::Accepted => "accepted",
            ConflictResolution::Rejected => "rejected",
        }
    }
}

/// Request body for resolving a conflict.
#[derive(Debug, Clone, Deserialize)]
pub struct ResolveConflictRequest {
    pub resolution: ConflictResolution,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operation_serializes_to_snake_case() {
        assert_eq!(
            serde_json::to_string(&Operation::Create).unwrap(),
            r#""create""#
        );
        assert_eq!(
            serde_json::to_string(&Operation::Update).unwrap(),
            r#""update""#
        );
        assert_eq!(
            serde_json::to_string(&Operation::Delete).unwrap(),
            r#""delete""#
        );
    }

    #[test]
    fn mutation_status_accepted_serializes_flat() {
        let result = MutationResult {
            mutation_id: Uuid::nil(),
            status: MutationStatus::Accepted,
        };
        let json: serde_json::Value = serde_json::to_value(&result).unwrap();
        assert_eq!(json["status"], "accepted");
        assert!(json.get("conflict_id").is_none());
    }

    #[test]
    fn mutation_status_conflicted_includes_conflict_id() {
        let conflict_id = Uuid::nil();
        let result = MutationResult {
            mutation_id: Uuid::nil(),
            status: MutationStatus::Conflicted { conflict_id },
        };
        let json: serde_json::Value = serde_json::to_value(&result).unwrap();
        assert_eq!(json["status"], "conflicted");
        assert!(json["conflict_id"].is_string());
    }

    #[test]
    fn mutation_status_deduplicated_serializes_flat() {
        let result = MutationResult {
            mutation_id: Uuid::nil(),
            status: MutationStatus::Deduplicated,
        };
        let json: serde_json::Value = serde_json::to_value(&result).unwrap();
        assert_eq!(json["status"], "deduplicated");
        assert!(json.get("conflict_id").is_none());
    }

    #[test]
    fn mutation_envelope_round_trips_through_serde() {
        use crate::contracts::EntityType;
        let envelope = MutationEnvelope {
            mutation_id: Uuid::nil(),
            device_id: Uuid::nil(),
            entity_type: EntityType::KnowledgeNote,
            entity_id: Uuid::nil(),
            operation: Operation::Update,
            payload: Some(serde_json::json!({ "title": "test" })),
            base_version: None,
            occurred_at: DateTime::from_timestamp(0, 0).unwrap(),
        };
        let json = serde_json::to_string(&envelope).unwrap();
        let parsed: MutationEnvelope = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.mutation_id, envelope.mutation_id);
        assert!(parsed.base_version.is_none());
    }
}
