use crate::contracts::EntityType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The operation-specific fields of a client mutation.
///
/// Using a tagged enum makes invalid cross-field states unrepresentable at the serde boundary
/// (e.g. `base_version` cannot be sent on a Create, and `payload` is required for Create/Update).
///
/// The `"operation"` field in the JSON wire format acts as the discriminant tag, so the
/// shape is identical to the previous flat struct:
///   `{ "operation": "update", "payload": {...}, "base_version": "..." }`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "snake_case")]
pub enum MutationPayload {
    /// Create a new entity. `payload` holds the full entity state.
    Create {
        /// Full entity state. Required for creates.
        payload: Option<serde_json::Value>,
    },
    /// Update an existing entity. `base_version` is the client's last-known `updated_at`.
    Update {
        /// Updated entity fields. Required for updates.
        payload: Option<serde_json::Value>,
        /// Client's last-known `updated_at`. Null triggers LWW fallback.
        base_version: Option<DateTime<Utc>>,
    },
    /// Soft-delete an existing entity. No payload needed.
    Delete {},
}

impl MutationPayload {
    /// Returns the canonical lowercase operation string for use in SQL and logs.
    pub fn operation_str(&self) -> &'static str {
        match self {
            MutationPayload::Create { .. } => "create",
            MutationPayload::Update { .. } => "update",
            MutationPayload::Delete { .. } => "delete",
        }
    }
}

/// A single client mutation packaged for upload to the server.
///
/// The operation, payload, and base_version are encoded in the `operation` field
/// via `MutationPayload`. This makes invalid cross-field states (e.g. `base_version`
/// on a Create) unrepresentable at the serde boundary.
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
    /// Operation with its associated fields (create/update/delete).
    /// Flattened into the JSON object so the wire format is unchanged.
    #[serde(flatten)]
    pub operation: MutationPayload,
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

/// Lifecycle status of a conflict row in `sync_conflicts`.
///
/// Replaces raw `'pending'` / `'accepted'` / `'rejected'` / `'conflict_copy'`
/// string literals so typos are caught at compile time rather than silently
/// producing wrong SQL results.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictStatus {
    Pending,
    Accepted,
    Rejected,
    ConflictCopy,
}

impl ConflictStatus {
    /// Returns the canonical lowercase string used in SQL and audit trails.
    pub fn as_str(self) -> &'static str {
        match self {
            ConflictStatus::Pending => "pending",
            ConflictStatus::Accepted => "accepted",
            ConflictStatus::Rejected => "rejected",
            ConflictStatus::ConflictCopy => "conflict_copy",
        }
    }
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
    fn mutation_payload_operation_str() {
        assert_eq!(
            MutationPayload::Create { payload: None }.operation_str(),
            "create"
        );
        assert_eq!(
            MutationPayload::Update {
                payload: None,
                base_version: None
            }
            .operation_str(),
            "update"
        );
        assert_eq!(MutationPayload::Delete {}.operation_str(), "delete");
    }

    #[test]
    fn mutation_payload_serializes_with_operation_discriminant() {
        let create = MutationPayload::Create {
            payload: Some(serde_json::json!({ "title": "t" })),
        };
        let json: serde_json::Value = serde_json::to_value(&create).unwrap();
        assert_eq!(json["operation"], "create");

        let update = MutationPayload::Update {
            payload: None,
            base_version: None,
        };
        let json: serde_json::Value = serde_json::to_value(&update).unwrap();
        assert_eq!(json["operation"], "update");

        let delete = MutationPayload::Delete {};
        let json: serde_json::Value = serde_json::to_value(&delete).unwrap();
        assert_eq!(json["operation"], "delete");
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
            operation: MutationPayload::Update {
                payload: Some(serde_json::json!({ "title": "test" })),
                base_version: None,
            },
            occurred_at: DateTime::from_timestamp(0, 0).unwrap(),
        };
        let json = serde_json::to_string(&envelope).unwrap();
        let parsed: MutationEnvelope = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.mutation_id, envelope.mutation_id);
        // Verify the operation field round-tripped as "update".
        let json_val: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(json_val["operation"], "update");
    }

    #[test]
    fn create_envelope_deserializes_from_wire_json() {
        // Wire format: operation discriminant flattened into the outer object.
        let wire = serde_json::json!({
            "mutation_id": "00000000-0000-0000-0000-000000000001",
            "device_id": "00000000-0000-0000-0000-000000000002",
            "entity_type": "knowledge_note",
            "entity_id": "00000000-0000-0000-0000-000000000003",
            "operation": "create",
            "payload": { "title": "Hello" },
            "occurred_at": "1970-01-01T00:00:00Z"
        });
        let envelope: MutationEnvelope = serde_json::from_value(wire).unwrap();
        assert!(matches!(envelope.operation, MutationPayload::Create { .. }));
    }

    #[test]
    fn delete_envelope_deserializes_without_payload() {
        let wire = serde_json::json!({
            "mutation_id": "00000000-0000-0000-0000-000000000001",
            "device_id": "00000000-0000-0000-0000-000000000002",
            "entity_type": "knowledge_note",
            "entity_id": "00000000-0000-0000-0000-000000000003",
            "operation": "delete",
            "occurred_at": "1970-01-01T00:00:00Z"
        });
        let envelope: MutationEnvelope = serde_json::from_value(wire).unwrap();
        assert!(matches!(envelope.operation, MutationPayload::Delete {}));
    }
}
