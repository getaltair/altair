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
    pub household_id: Option<Uuid>,
    pub initiative_id: Option<Uuid>,
    pub owner_user_id: Option<Uuid>,
    pub updated_by_user_id: Option<Uuid>,
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
    pub household_id: Option<Uuid>,
    pub initiative_id: Option<Uuid>,
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

/// Path parameters for entity-scoped relation endpoints.
/// Uses String (not EntityType enum) for Axum path extraction compatibility.
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct EntityPathParams {
    pub entity_type: String,
    pub entity_id: Uuid,
}

/// A single node in the one-hop relation graph for an entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationGraphNode {
    pub entity_type: String,
    pub entity_id: Uuid,
    pub relation_id: Uuid,
    pub relation_type: String,
    pub direction: String,
    pub source_type: String,
    pub status: String,
    pub confidence: Option<f64>,
    pub evidence_json: Option<Value>,
    pub created_at: DateTime<Utc>,
}

/// Response for the relation graph endpoint: the source entity plus all one-hop neighbors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationGraphResponse {
    pub entity_type: String,
    pub entity_id: Uuid,
    pub relations: Vec<RelationGraphNode>,
}

/// Query parameters for the suggested relations endpoint
#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
pub struct SuggestedRelationsQuery {
    pub relation_type: Option<RelationType>,
    pub entity_type: Option<EntityType>,
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
            household_id: None,
            initiative_id: None,
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

    // -- CreateRelationRequest with new optional fields --------------------------

    #[test]
    fn create_relation_with_household_and_initiative() {
        let json = r#"{
            "from_entity_type": "knowledge_note",
            "from_entity_id": "00000000-0000-0000-0000-000000000001",
            "to_entity_type": "guidance_quest",
            "to_entity_id": "00000000-0000-0000-0000-000000000002",
            "relation_type": "supports",
            "household_id": "00000000-0000-0000-0000-000000000003",
            "initiative_id": "00000000-0000-0000-0000-000000000004"
        }"#;
        let req: CreateRelationRequest = serde_json::from_str(json).unwrap();
        assert!(req.household_id.is_some());
        assert!(req.initiative_id.is_some());
    }

    #[test]
    fn create_relation_without_optional_scoping_fields() {
        let json = r#"{
            "from_entity_type": "tracking_item",
            "from_entity_id": "00000000-0000-0000-0000-000000000001",
            "to_entity_type": "knowledge_note",
            "to_entity_id": "00000000-0000-0000-0000-000000000002",
            "relation_type": "references"
        }"#;
        let req: CreateRelationRequest = serde_json::from_str(json).unwrap();
        assert!(req.household_id.is_none());
        assert!(req.initiative_id.is_none());
    }

    // -- EntityPathParams --------------------------------------------------------

    #[test]
    fn entity_path_params_deserializes_valid() {
        let json = r#"{
            "entity_type": "knowledge_note",
            "entity_id": "00000000-0000-0000-0000-000000000001"
        }"#;
        let params: EntityPathParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.entity_type, "knowledge_note");
        assert_eq!(
            params.entity_id,
            Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap()
        );
    }

    #[test]
    fn entity_path_params_accepts_any_string_entity_type() {
        // EntityPathParams uses String, not EntityType, so unknown types are accepted
        // at the deserialization level (validation happens in the service layer)
        let json = r#"{
            "entity_type": "unknown_entity",
            "entity_id": "00000000-0000-0000-0000-000000000001"
        }"#;
        let result = serde_json::from_str::<EntityPathParams>(json);
        assert!(result.is_ok());
    }

    // -- RelationGraphNode -------------------------------------------------------

    #[test]
    fn relation_graph_node_serialization_roundtrip() {
        let node = RelationGraphNode {
            entity_type: "tracking_item".to_string(),
            entity_id: Uuid::new_v4(),
            relation_id: Uuid::new_v4(),
            relation_type: "references".to_string(),
            direction: "outgoing".to_string(),
            source_type: "user".to_string(),
            status: "accepted".to_string(),
            confidence: Some(0.95),
            evidence_json: None,
            created_at: Utc::now(),
        };
        let json = serde_json::to_string(&node).unwrap();
        let back: RelationGraphNode = serde_json::from_str(&json).unwrap();
        assert_eq!(back.entity_type, node.entity_type);
        assert_eq!(back.entity_id, node.entity_id);
        assert_eq!(back.relation_id, node.relation_id);
        assert_eq!(back.direction, "outgoing");
        assert_eq!(back.confidence, Some(0.95));
    }

    #[test]
    fn relation_graph_node_with_evidence_json() {
        let evidence = serde_json::json!({"source": "ai_analysis", "score": 0.8});
        let node = RelationGraphNode {
            entity_type: "guidance_quest".to_string(),
            entity_id: Uuid::new_v4(),
            relation_id: Uuid::new_v4(),
            relation_type: "supports".to_string(),
            direction: "incoming".to_string(),
            source_type: "ai".to_string(),
            status: "suggested".to_string(),
            confidence: Some(0.8),
            evidence_json: Some(evidence.clone()),
            created_at: Utc::now(),
        };
        let json = serde_json::to_string(&node).unwrap();
        let back: RelationGraphNode = serde_json::from_str(&json).unwrap();
        assert_eq!(back.evidence_json.unwrap(), evidence);
    }

    // -- RelationGraphResponse ---------------------------------------------------

    #[test]
    fn relation_graph_response_serialization_roundtrip() {
        let response = RelationGraphResponse {
            entity_type: "knowledge_note".to_string(),
            entity_id: Uuid::new_v4(),
            relations: vec![
                RelationGraphNode {
                    entity_type: "tracking_item".to_string(),
                    entity_id: Uuid::new_v4(),
                    relation_id: Uuid::new_v4(),
                    relation_type: "references".to_string(),
                    direction: "outgoing".to_string(),
                    source_type: "user".to_string(),
                    status: "accepted".to_string(),
                    confidence: None,
                    evidence_json: None,
                    created_at: Utc::now(),
                },
            ],
        };
        let json = serde_json::to_string(&response).unwrap();
        let back: RelationGraphResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(back.entity_type, "knowledge_note");
        assert_eq!(back.relations.len(), 1);
        assert_eq!(back.relations[0].direction, "outgoing");
    }

    #[test]
    fn relation_graph_response_empty_relations() {
        let response = RelationGraphResponse {
            entity_type: "initiative".to_string(),
            entity_id: Uuid::new_v4(),
            relations: vec![],
        };
        let json = serde_json::to_string(&response).unwrap();
        let back: RelationGraphResponse = serde_json::from_str(&json).unwrap();
        assert!(back.relations.is_empty());
    }

    // -- SuggestedRelationsQuery -------------------------------------------------

    #[test]
    fn suggested_relations_query_default_has_all_none() {
        let q = SuggestedRelationsQuery::default();
        assert!(q.relation_type.is_none());
        assert!(q.entity_type.is_none());
    }
}
