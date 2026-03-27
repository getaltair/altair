use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

use super::models::{
    CreateRelationRequest, EntityRelation, RelationGraphNode, RelationGraphResponse,
    RelationQuery, UpdateRelationStatusRequest,
};
use crate::contracts::SourceType;
use crate::error::AppError;

/// Column list with NUMERIC -> FLOAT8 cast for the confidence field.
/// sqlx does not have the `numeric` feature enabled, so we cast in SQL.
pub const SELECT_COLUMNS: &str = r#"
    id, from_entity_type, from_entity_id, to_entity_type, to_entity_id,
    relation_type, source_type, status,
    CAST(confidence AS FLOAT8) AS confidence,
    evidence_json, created_by_user_id, created_by_process,
    created_at, updated_at, last_confirmed_at,
    household_id, initiative_id, owner_user_id, updated_by_user_id
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
          relation_type, source_type, confidence, evidence_json, created_by_user_id, \
          household_id, initiative_id, owner_user_id) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) \
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
    .bind(req.household_id)
    .bind(req.initiative_id)
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
    qb.push("(created_by_user_id = ");
    qb.push_bind(user_id);
    qb.push(" OR owner_user_id = ");
    qb.push_bind(user_id);
    qb.push(")");

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
         WHERE id = $3 AND (created_by_user_id = $4 OR owner_user_id = $4) \
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

/// Get all relations for a given entity in both directions (from and to).
/// Scoped to relations the user created or owns.
pub async fn get_relations_for_entity(
    pool: &PgPool,
    user_id: Uuid,
    entity_type: &str,
    entity_id: Uuid,
) -> Result<Vec<EntityRelation>, AppError> {
    let relations = sqlx::query_as::<_, EntityRelation>(&format!(
        "SELECT {SELECT_COLUMNS} FROM entity_relations \
         WHERE ((from_entity_type = $1 AND from_entity_id = $2) \
                OR (to_entity_type = $1 AND to_entity_id = $2)) \
           AND (created_by_user_id = $3 OR owner_user_id = $3) \
         ORDER BY created_at DESC"
    ))
    .bind(entity_type)
    .bind(entity_id)
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(relations)
}

/// Get the one-hop relation graph for an entity. Returns a structured response
/// with direction labels ("outgoing" if the entity is the source, "incoming" if target).
pub async fn get_relation_graph(
    pool: &PgPool,
    user_id: Uuid,
    entity_type: &str,
    entity_id: Uuid,
) -> Result<RelationGraphResponse, AppError> {
    let relations = get_relations_for_entity(pool, user_id, entity_type, entity_id).await?;

    let nodes: Vec<RelationGraphNode> = relations
        .into_iter()
        .map(|r| {
            let (other_type, other_id, direction) =
                if r.from_entity_type == entity_type && r.from_entity_id == entity_id {
                    (r.to_entity_type.clone(), r.to_entity_id, "outgoing")
                } else {
                    (r.from_entity_type.clone(), r.from_entity_id, "incoming")
                };

            RelationGraphNode {
                entity_type: other_type,
                entity_id: other_id,
                relation_id: r.id,
                relation_type: r.relation_type,
                direction: direction.to_string(),
                source_type: r.source_type,
                status: r.status,
                confidence: r.confidence,
                evidence_json: r.evidence_json,
                created_at: r.created_at,
            }
        })
        .collect();

    Ok(RelationGraphResponse {
        entity_type: entity_type.to_string(),
        entity_id,
        relations: nodes,
    })
}

/// Get all suggested relations for the authenticated user, ordered by most recent first.
pub async fn get_suggested_relations(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<EntityRelation>, AppError> {
    let relations = sqlx::query_as::<_, EntityRelation>(&format!(
        "SELECT {SELECT_COLUMNS} FROM entity_relations \
         WHERE status = 'suggested' \
           AND (created_by_user_id = $1 OR owner_user_id = $1) \
         ORDER BY created_at DESC"
    ))
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(relations)
}

/// Accept a relation: set status to 'accepted', update last_confirmed_at and updated_by.
pub async fn accept_relation(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
) -> Result<EntityRelation, AppError> {
    let relation = sqlx::query_as::<_, EntityRelation>(&format!(
        "UPDATE entity_relations \
         SET status = 'accepted', last_confirmed_at = now(), \
             updated_by_user_id = $1, updated_at = now() \
         WHERE id = $2 AND (created_by_user_id = $1 OR owner_user_id = $1) \
         RETURNING {SELECT_COLUMNS}"
    ))
    .bind(user_id)
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?
    .ok_or_else(|| AppError::NotFound(format!("Entity relation not found: {id}")))?;

    Ok(relation)
}

/// Dismiss a relation: set status to 'dismissed' and update updated_by.
pub async fn dismiss_relation(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
) -> Result<EntityRelation, AppError> {
    let relation = sqlx::query_as::<_, EntityRelation>(&format!(
        "UPDATE entity_relations \
         SET status = 'dismissed', updated_by_user_id = $1, updated_at = now() \
         WHERE id = $2 AND (created_by_user_id = $1 OR owner_user_id = $1) \
         RETURNING {SELECT_COLUMNS}"
    ))
    .bind(user_id)
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?
    .ok_or_else(|| AppError::NotFound(format!("Entity relation not found: {id}")))?;

    Ok(relation)
}

/// Reject a relation: set status to 'rejected' and update updated_by.
pub async fn reject_relation(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
) -> Result<EntityRelation, AppError> {
    let relation = sqlx::query_as::<_, EntityRelation>(&format!(
        "UPDATE entity_relations \
         SET status = 'rejected', updated_by_user_id = $1, updated_at = now() \
         WHERE id = $2 AND (created_by_user_id = $1 OR owner_user_id = $1) \
         RETURNING {SELECT_COLUMNS}"
    ))
    .bind(user_id)
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?
    .ok_or_else(|| AppError::NotFound(format!("Entity relation not found: {id}")))?;

    Ok(relation)
}
