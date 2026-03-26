use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use validator::Validate;

use crate::contracts::{EntityType, RelationStatus, RelationType, SourceType};

/// Database-backed entity relation record linking two entities with a typed relationship
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
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
#[derive(Debug, Deserialize, Validate)]
pub struct CreateRelationRequest {
    pub from_entity_type: EntityType,
    pub from_entity_id: Uuid,
    pub to_entity_type: EntityType,
    pub to_entity_id: Uuid,
    pub relation_type: RelationType,
    pub source_type: Option<SourceType>,
    #[validate(range(min = 0.0, max = 1.0))]
    pub confidence: Option<f64>,
    pub evidence_json: Option<Value>,
}

/// Request payload for updating an entity relation's status
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateRelationStatusRequest {
    pub status: RelationStatus,
    pub last_confirmed_at: Option<DateTime<Utc>>,
}

/// Query parameters for filtering entity relations
#[derive(Debug, Deserialize, Default)]
pub struct RelationQuery {
    pub from_entity_type: Option<EntityType>,
    pub from_entity_id: Option<Uuid>,
    pub to_entity_type: Option<EntityType>,
    pub to_entity_id: Option<Uuid>,
    pub relation_type: Option<RelationType>,
    pub status: Option<RelationStatus>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::{EntityType, RelationType};
    use validator::Validate;

    fn valid_create_request(confidence: Option<f64>) -> CreateRelationRequest {
        CreateRelationRequest {
            from_entity_type: EntityType::Initiative,
            from_entity_id: Uuid::new_v4(),
            to_entity_type: EntityType::Tag,
            to_entity_id: Uuid::new_v4(),
            relation_type: RelationType::References,
            source_type: None,
            confidence,
            evidence_json: None,
        }
    }

    // -- CreateRelationRequest confidence validation ---------------------------

    #[test]
    fn create_relation_confidence_none_is_valid() {
        let req = valid_create_request(None);
        assert!(req.validate().is_ok());
    }

    #[test]
    fn create_relation_confidence_zero_is_valid() {
        let req = valid_create_request(Some(0.0));
        assert!(req.validate().is_ok());
    }

    #[test]
    fn create_relation_confidence_one_is_valid() {
        let req = valid_create_request(Some(1.0));
        assert!(req.validate().is_ok());
    }

    #[test]
    fn create_relation_confidence_mid_range_is_valid() {
        let req = valid_create_request(Some(0.5));
        assert!(req.validate().is_ok());
    }

    #[test]
    fn create_relation_confidence_above_one_is_invalid() {
        let req = valid_create_request(Some(1.1));
        assert!(req.validate().is_err());
    }

    #[test]
    fn create_relation_confidence_below_zero_is_invalid() {
        let req = valid_create_request(Some(-0.1));
        assert!(req.validate().is_err());
    }

    // -- CreateRelationRequest serde for enum fields ---------------------------

    #[test]
    fn create_relation_deserializes_valid_json() {
        let json = r#"{
            "from_entity_type": "initiative",
            "from_entity_id": "00000000-0000-0000-0000-000000000001",
            "to_entity_type": "tag",
            "to_entity_id": "00000000-0000-0000-0000-000000000002",
            "relation_type": "references"
        }"#;
        let req: CreateRelationRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.from_entity_type, EntityType::Initiative);
        assert_eq!(req.to_entity_type, EntityType::Tag);
        assert_eq!(req.relation_type, RelationType::References);
        assert!(req.source_type.is_none());
        assert!(req.confidence.is_none());
    }

    #[test]
    fn create_relation_rejects_unknown_entity_type() {
        let json = r#"{
            "from_entity_type": "bogus_type",
            "from_entity_id": "00000000-0000-0000-0000-000000000001",
            "to_entity_type": "tag",
            "to_entity_id": "00000000-0000-0000-0000-000000000002",
            "relation_type": "references"
        }"#;
        let result = serde_json::from_str::<CreateRelationRequest>(json);
        assert!(result.is_err());
    }

    #[test]
    fn create_relation_rejects_unknown_relation_type() {
        let json = r#"{
            "from_entity_type": "initiative",
            "from_entity_id": "00000000-0000-0000-0000-000000000001",
            "to_entity_type": "tag",
            "to_entity_id": "00000000-0000-0000-0000-000000000002",
            "relation_type": "links_to"
        }"#;
        let result = serde_json::from_str::<CreateRelationRequest>(json);
        assert!(result.is_err());
    }

    // -- UpdateRelationStatusRequest -------------------------------------------

    #[test]
    fn update_status_deserializes_valid_json() {
        let json = r#"{"status": "dismissed"}"#;
        let req: UpdateRelationStatusRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.status, RelationStatus::Dismissed);
        assert!(req.last_confirmed_at.is_none());
    }

    #[test]
    fn update_status_rejects_unknown_status() {
        let json = r#"{"status": "pending"}"#;
        let result = serde_json::from_str::<UpdateRelationStatusRequest>(json);
        assert!(result.is_err());
    }

    // -- RelationQuery defaults ------------------------------------------------

    #[test]
    fn relation_query_default_has_all_none() {
        let q = RelationQuery::default();
        assert!(q.from_entity_type.is_none());
        assert!(q.from_entity_id.is_none());
        assert!(q.to_entity_type.is_none());
        assert!(q.to_entity_id.is_none());
        assert!(q.relation_type.is_none());
        assert!(q.status.is_none());
    }
}
