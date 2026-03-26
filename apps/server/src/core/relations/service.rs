use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

use super::models::{
    CreateRelationRequest, EntityRelation, RelationQuery, UpdateRelationStatusRequest,
};
use crate::contracts::SourceType;
use crate::error::AppError;

/// Column list with NUMERIC -> FLOAT8 cast for the confidence field.
/// sqlx does not have the `numeric` feature enabled, so we cast in SQL.
const SELECT_COLUMNS: &str = r#"
    id, from_entity_type, from_entity_id, to_entity_type, to_entity_id,
    relation_type, source_type, status,
    CAST(confidence AS FLOAT8) AS confidence,
    evidence_json, created_by_user_id, created_by_process,
    created_at, updated_at, last_confirmed_at
"#;

/// Create a new entity relation. Validation of entity types, relation types,
/// source types, and statuses is handled by serde deserialization of the
/// request enums, so no manual `is_valid_*` checks are needed.
pub async fn create_relation(
    pool: &PgPool,
    user_id: Uuid,
    req: CreateRelationRequest,
) -> Result<EntityRelation, AppError> {
    let source_type = req.source_type.unwrap_or(SourceType::User);

    let relation = sqlx::query_as::<_, EntityRelation>(&format!(
        "INSERT INTO entity_relations \
         (from_entity_type, from_entity_id, to_entity_type, to_entity_id, \
          relation_type, source_type, confidence, evidence_json, created_by_user_id) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) \
         RETURNING {SELECT_COLUMNS}"
    ))
    .bind(req.from_entity_type.as_str())
    .bind(req.from_entity_id)
    .bind(req.to_entity_type.as_str())
    .bind(req.to_entity_id)
    .bind(req.relation_type.as_str())
    .bind(source_type.as_str())
    .bind(req.confidence)
    .bind(&req.evidence_json)
    .bind(user_id)
    .fetch_one(pool)
    .await
    .map_err(|e| match &e {
        sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
            AppError::Conflict("Entity relation already exists".to_string())
        }
        sqlx::Error::Database(db_err) if db_err.is_check_violation() => {
            AppError::BadRequest("Invalid field value".to_string())
        }
        _ => AppError::Database(e),
    })?;

    Ok(relation)
}

/// Query entity relations with optional filters. At least one of from_entity_id
/// or to_entity_id must be provided to prevent unbounded table scans. Results
/// are scoped to relations created by the authenticated user.
pub async fn query_relations(
    pool: &PgPool,
    user_id: Uuid,
    query: RelationQuery,
) -> Result<Vec<EntityRelation>, AppError> {
    if query.from_entity_id.is_none() && query.to_entity_id.is_none() {
        return Err(AppError::BadRequest(
            "At least one of from_entity_id or to_entity_id must be provided".to_string(),
        ));
    }

    let mut qb: QueryBuilder<sqlx::Postgres> = QueryBuilder::new(format!(
        "SELECT {SELECT_COLUMNS} FROM entity_relations WHERE "
    ));

    // Always scope to the authenticated user's relations
    qb.push("created_by_user_id = ");
    qb.push_bind(user_id);

    if let Some(ref v) = query.from_entity_type {
        qb.push(" AND from_entity_type = ");
        qb.push_bind(v.as_str().to_owned());
    }

    if let Some(v) = query.from_entity_id {
        qb.push(" AND from_entity_id = ");
        qb.push_bind(v);
    }

    if let Some(ref v) = query.to_entity_type {
        qb.push(" AND to_entity_type = ");
        qb.push_bind(v.as_str().to_owned());
    }

    if let Some(v) = query.to_entity_id {
        qb.push(" AND to_entity_id = ");
        qb.push_bind(v);
    }

    if let Some(ref v) = query.relation_type {
        qb.push(" AND relation_type = ");
        qb.push_bind(v.as_str().to_owned());
    }

    if let Some(ref v) = query.status {
        qb.push(" AND status = ");
        qb.push_bind(v.as_str().to_owned());
    }

    qb.push(" ORDER BY created_at DESC");

    let relations = qb
        .build_query_as::<EntityRelation>()
        .fetch_all(pool)
        .await
        .map_err(AppError::Database)?;

    Ok(relations)
}

// TODO: DB-backed test needed for query_relations entity-id validation.
// The check at the top of query_relations (both from_entity_id and to_entity_id are None
// -> BadRequest) is synchronous and fires before any DB call, but we cannot construct
// a dummy PgPool without a live database. When #[sqlx::test] infrastructure is added,
// add a test that passes an empty RelationQuery and asserts BadRequest. For now, the
// validation logic is covered by the serde-level tests in models.rs and by the
// RelationQuery::default() test verifying all fields start as None.

/// Update the status (and optionally last_confirmed_at) of an existing entity relation.
/// Only the relation creator can update status. Status validation is handled by serde
/// deserialization of `RelationStatus`.
pub async fn update_relation_status(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    req: UpdateRelationStatusRequest,
) -> Result<EntityRelation, AppError> {
    let relation = sqlx::query_as::<_, EntityRelation>(&format!(
        "UPDATE entity_relations \
         SET status = $1, updated_at = now(), last_confirmed_at = COALESCE($2, last_confirmed_at) \
         WHERE id = $3 AND created_by_user_id = $4 \
         RETURNING {SELECT_COLUMNS}"
    ))
    .bind(req.status.as_str())
    .bind(req.last_confirmed_at)
    .bind(id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| match &e {
        sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
            AppError::Conflict("Entity relation already exists".to_string())
        }
        sqlx::Error::Database(db_err) if db_err.is_check_violation() => {
            AppError::BadRequest("Invalid field value".to_string())
        }
        _ => AppError::Database(e),
    })?
    .ok_or_else(|| AppError::NotFound(format!("Entity relation not found: {id}")))?;

    Ok(relation)
}
