use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

use super::models::{
    CreateRelationRequest, EntityRelation, RelationQuery, UpdateRelationStatusRequest,
};
use crate::contracts;
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

/// Create a new entity relation after validating all domain constraints
pub async fn create_relation(
    pool: &PgPool,
    user_id: Uuid,
    req: CreateRelationRequest,
) -> Result<EntityRelation, AppError> {
    if !contracts::is_valid_entity_type(&req.from_entity_type) {
        return Err(AppError::BadRequest(format!(
            "Unknown entity type: {}",
            req.from_entity_type
        )));
    }
    if !contracts::is_valid_entity_type(&req.to_entity_type) {
        return Err(AppError::BadRequest(format!(
            "Unknown entity type: {}",
            req.to_entity_type
        )));
    }
    if !contracts::is_valid_relation_type(&req.relation_type) {
        return Err(AppError::BadRequest(format!(
            "Unknown relation type: {}",
            req.relation_type
        )));
    }

    let source_type = req.source_type.as_deref().unwrap_or("user");
    if !contracts::is_valid_source_type(source_type) {
        return Err(AppError::BadRequest(format!(
            "Unknown source type: {}",
            source_type
        )));
    }

    let relation = sqlx::query_as::<_, EntityRelation>(&format!(
        "INSERT INTO entity_relations \
         (from_entity_type, from_entity_id, to_entity_type, to_entity_id, \
          relation_type, source_type, confidence, evidence_json, created_by_user_id) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) \
         RETURNING {SELECT_COLUMNS}"
    ))
    .bind(&req.from_entity_type)
    .bind(req.from_entity_id)
    .bind(&req.to_entity_type)
    .bind(req.to_entity_id)
    .bind(&req.relation_type)
    .bind(source_type)
    .bind(req.confidence)
    .bind(&req.evidence_json)
    .bind(user_id)
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(relation)
}

/// Query entity relations with optional filters. At least one of from_entity_id
/// or to_entity_id must be provided to prevent unbounded table scans.
pub async fn query_relations(
    pool: &PgPool,
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

    let mut has_condition = false;

    if let Some(ref v) = query.from_entity_type {
        qb.push("from_entity_type = ");
        qb.push_bind(v.clone());
        has_condition = true;
    }

    if let Some(v) = query.from_entity_id {
        if has_condition {
            qb.push(" AND ");
        }
        qb.push("from_entity_id = ");
        qb.push_bind(v);
        has_condition = true;
    }

    if let Some(ref v) = query.to_entity_type {
        if has_condition {
            qb.push(" AND ");
        }
        qb.push("to_entity_type = ");
        qb.push_bind(v.clone());
        has_condition = true;
    }

    if let Some(v) = query.to_entity_id {
        if has_condition {
            qb.push(" AND ");
        }
        qb.push("to_entity_id = ");
        qb.push_bind(v);
        has_condition = true;
    }

    if let Some(ref v) = query.relation_type {
        if has_condition {
            qb.push(" AND ");
        }
        qb.push("relation_type = ");
        qb.push_bind(v.clone());
        has_condition = true;
    }

    if let Some(ref v) = query.status {
        if has_condition {
            qb.push(" AND ");
        }
        qb.push("status = ");
        qb.push_bind(v.clone());
        // Mark used so the compiler doesn't warn about the final assignment
        has_condition = true;
    }

    // Safety: at least from_entity_id or to_entity_id was provided, so has_condition is true
    let _ = has_condition;

    qb.push(" ORDER BY created_at DESC");

    let relations = qb
        .build_query_as::<EntityRelation>()
        .fetch_all(pool)
        .await
        .map_err(AppError::Database)?;

    Ok(relations)
}

/// Update the status (and optionally last_confirmed_at) of an existing entity relation
pub async fn update_relation_status(
    pool: &PgPool,
    id: Uuid,
    _user_id: Uuid,
    req: UpdateRelationStatusRequest,
) -> Result<EntityRelation, AppError> {
    if !contracts::is_valid_relation_status(&req.status) {
        return Err(AppError::BadRequest(format!(
            "Unknown relation status: {}",
            req.status
        )));
    }

    let relation = sqlx::query_as::<_, EntityRelation>(&format!(
        "UPDATE entity_relations \
         SET status = $1, updated_at = now(), last_confirmed_at = COALESCE($2, last_confirmed_at) \
         WHERE id = $3 \
         RETURNING {SELECT_COLUMNS}"
    ))
    .bind(&req.status)
    .bind(req.last_confirmed_at)
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?
    .ok_or_else(|| AppError::NotFound(format!("Entity relation not found: {id}")))?;

    Ok(relation)
}
