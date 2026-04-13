use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct EntityRelation {
    pub id: Uuid,
    pub user_id: Uuid,
    pub from_entity_type: String,
    pub from_entity_id: Uuid,
    pub to_entity_type: String,
    pub to_entity_id: Uuid,
    pub relation_type: String,
    pub source_type: String,
    pub evidence: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRelationRequest {
    pub from_entity_type: String,
    pub from_entity_id: Uuid,
    pub to_entity_type: String,
    pub to_entity_id: Uuid,
    pub relation_type: String,
    pub source_type: String,
    pub evidence: Option<String>,
}
