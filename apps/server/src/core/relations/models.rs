use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// Database-backed entity relation record linking two entities with a typed relationship
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct EntityRelation {
    pub id: Uuid,
    pub from_entity_type: String,
    pub from_entity_id: Uuid,
    pub to_entity_type: String,
    pub to_entity_id: Uuid,
    pub relation_type: String,
    pub source_type: String,
    pub status: String,
    /// PostgreSQL NUMERIC mapped via CAST(confidence AS FLOAT8) in queries
    pub confidence: Option<f64>,
    pub evidence_json: Option<Value>,
    pub created_by_user_id: Option<Uuid>,
    pub created_by_process: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_confirmed_at: Option<DateTime<Utc>>,
}

/// Request payload for creating a new entity relation
#[derive(Debug, Deserialize)]
pub struct CreateRelationRequest {
    pub from_entity_type: String,
    pub from_entity_id: Uuid,
    pub to_entity_type: String,
    pub to_entity_id: Uuid,
    pub relation_type: String,
    pub source_type: Option<String>,
    pub confidence: Option<f64>,
    pub evidence_json: Option<Value>,
}

/// Request payload for updating an entity relation's status
#[derive(Debug, Deserialize)]
pub struct UpdateRelationStatusRequest {
    pub status: String,
    pub last_confirmed_at: Option<DateTime<Utc>>,
}

/// Query parameters for filtering entity relations
#[derive(Debug, Deserialize, Default)]
pub struct RelationQuery {
    pub from_entity_type: Option<String>,
    pub from_entity_id: Option<Uuid>,
    pub to_entity_type: Option<String>,
    pub to_entity_id: Option<Uuid>,
    pub relation_type: Option<String>,
    pub status: Option<String>,
}
