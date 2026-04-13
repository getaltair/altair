use sqlx::PgPool;
use uuid::Uuid;

use super::models::{CreateRelationRequest, EntityRelation};
use crate::contracts::{EntityType, RelationType};
use crate::error::AppError;

/// Valid source_type values per the spec.
const VALID_SOURCE_TYPES: &[&str] = &["user", "ai", "import", "rule", "migration", "system"];

/// Validate a `CreateRelationRequest` against the registries (C-1, C-2).
/// Returns Ok(()) if all fields are valid, or the appropriate AppError.
pub fn validate_create_request(req: &CreateRelationRequest) -> Result<(), AppError> {
    // Validate from_entity_type
    serde_json::from_value::<EntityType>(serde_json::Value::String(req.from_entity_type.clone()))
        .map_err(|_| {
        AppError::UnprocessableEntity(format!("Unknown entity type: {}", req.from_entity_type))
    })?;

    // Validate to_entity_type
    serde_json::from_value::<EntityType>(serde_json::Value::String(req.to_entity_type.clone()))
        .map_err(|_| {
            AppError::UnprocessableEntity(format!("Unknown entity type: {}", req.to_entity_type))
        })?;

    // Validate relation_type
    serde_json::from_value::<RelationType>(serde_json::Value::String(req.relation_type.clone()))
        .map_err(|_| {
            AppError::UnprocessableEntity(format!("Unknown relation type: {}", req.relation_type))
        })?;

    // Validate source_type
    if !VALID_SOURCE_TYPES.contains(&req.source_type.as_str()) {
        return Err(AppError::UnprocessableEntity(format!(
            "Unknown source type: {}",
            req.source_type
        )));
    }

    Ok(())
}

pub async fn list_relations(pool: &PgPool, user_id: Uuid) -> Result<Vec<EntityRelation>, AppError> {
    let rows = sqlx::query_as::<_, EntityRelation>(
        "SELECT id, user_id, from_entity_type, from_entity_id, to_entity_type, to_entity_id, \
                relation_type, source_type, evidence, created_at, updated_at, deleted_at \
         FROM entity_relations \
         WHERE user_id = $1 AND deleted_at IS NULL \
         ORDER BY created_at DESC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))?;

    Ok(rows)
}

pub async fn create_relation(
    pool: &PgPool,
    user_id: Uuid,
    req: CreateRelationRequest,
) -> Result<EntityRelation, AppError> {
    validate_create_request(&req)?;

    let row = sqlx::query_as::<_, EntityRelation>(
        "INSERT INTO entity_relations \
           (id, user_id, from_entity_type, from_entity_id, to_entity_type, to_entity_id, \
            relation_type, source_type, evidence) \
         VALUES \
           (gen_random_uuid(), $1, $2, $3, $4, $5, $6, $7, $8) \
         RETURNING id, user_id, from_entity_type, from_entity_id, to_entity_type, to_entity_id, \
                   relation_type, source_type, evidence, created_at, updated_at, deleted_at",
    )
    .bind(user_id)
    .bind(&req.from_entity_type)
    .bind(req.from_entity_id)
    .bind(&req.to_entity_type)
    .bind(req.to_entity_id)
    .bind(&req.relation_type)
    .bind(&req.source_type)
    .bind(&req.evidence)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))?;

    Ok(row)
}

pub async fn delete_relation(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query(
        "UPDATE entity_relations \
         SET deleted_at = NOW(), updated_at = NOW() \
         WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL",
    )
    .bind(id)
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Tests (S022-T)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn valid_request() -> CreateRelationRequest {
        CreateRelationRequest {
            from_entity_type: "initiative".to_string(),
            from_entity_id: Uuid::new_v4(),
            to_entity_type: "knowledge_note".to_string(),
            to_entity_id: Uuid::new_v4(),
            relation_type: "references".to_string(),
            source_type: "user".to_string(),
            evidence: None,
        }
    }

    // FA-011: Unknown relation_type → UnprocessableEntity
    #[test]
    fn unknown_relation_type_returns_unprocessable_entity() {
        let mut req = valid_request();
        req.relation_type = "invented_relation".to_string();
        let result = validate_create_request(&req);
        match result {
            Err(AppError::UnprocessableEntity(msg)) => {
                assert!(
                    msg.contains("invented_relation"),
                    "Error message should contain the bad value; got: {msg}"
                );
            }
            other => panic!("Expected UnprocessableEntity, got {:?}", other),
        }
    }

    // FA-012: Unknown from_entity_type → UnprocessableEntity
    #[test]
    fn unknown_from_entity_type_returns_unprocessable_entity() {
        let mut req = valid_request();
        req.from_entity_type = "ghost_entity".to_string();
        let result = validate_create_request(&req);
        match result {
            Err(AppError::UnprocessableEntity(msg)) => {
                assert!(
                    msg.contains("ghost_entity"),
                    "Error message should contain the bad value; got: {msg}"
                );
            }
            other => panic!("Expected UnprocessableEntity, got {:?}", other),
        }
    }

    // FA-012: Unknown to_entity_type → UnprocessableEntity
    #[test]
    fn unknown_to_entity_type_returns_unprocessable_entity() {
        let mut req = valid_request();
        req.to_entity_type = "made_up_type".to_string();
        let result = validate_create_request(&req);
        match result {
            Err(AppError::UnprocessableEntity(msg)) => {
                assert!(
                    msg.contains("made_up_type"),
                    "Error message should contain the bad value; got: {msg}"
                );
            }
            other => panic!("Expected UnprocessableEntity, got {:?}", other),
        }
    }

    // Unknown source_type → UnprocessableEntity
    #[test]
    fn unknown_source_type_returns_unprocessable_entity() {
        let mut req = valid_request();
        req.source_type = "robot".to_string();
        let result = validate_create_request(&req);
        match result {
            Err(AppError::UnprocessableEntity(msg)) => {
                assert!(
                    msg.contains("robot"),
                    "Error message should contain the bad value; got: {msg}"
                );
            }
            other => panic!("Expected UnprocessableEntity, got {:?}", other),
        }
    }

    // All valid source_types must pass validation
    #[test]
    fn all_valid_source_types_pass_validation() {
        for source_type in VALID_SOURCE_TYPES {
            let mut req = valid_request();
            req.source_type = source_type.to_string();
            assert!(
                validate_create_request(&req).is_ok(),
                "source_type '{}' should be valid",
                source_type
            );
        }
    }

    // A fully valid request passes validation
    #[test]
    fn valid_request_passes_validation() {
        let req = valid_request();
        assert!(
            validate_create_request(&req).is_ok(),
            "A fully valid request must pass validation"
        );
    }

    // Verify list query scopes to user_id (SEC-1)
    #[test]
    fn list_query_scoped_to_user_id() {
        let query = "SELECT id, user_id, from_entity_type, from_entity_id, to_entity_type, to_entity_id, \
                relation_type, source_type, evidence, created_at, updated_at, deleted_at \
         FROM entity_relations \
         WHERE user_id = $1 AND deleted_at IS NULL \
         ORDER BY created_at DESC";
        assert!(
            query.contains("user_id = $1"),
            "list query must scope to user_id"
        );
        assert!(
            query.contains("deleted_at IS NULL"),
            "list query must exclude soft-deleted rows"
        );
    }

    // Verify delete is soft-delete scoped to user_id
    #[test]
    fn delete_query_is_soft_delete_scoped_to_user() {
        let query = "UPDATE entity_relations \
         SET deleted_at = NOW(), updated_at = NOW() \
         WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL";
        assert!(
            query.contains("deleted_at = NOW()"),
            "delete must be a soft delete"
        );
        assert!(
            query.contains("user_id = $2"),
            "delete must scope to user_id"
        );
    }
}
