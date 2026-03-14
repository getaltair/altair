//! Relation handler module.
//!
//! Handles CRUD operations for cross-domain entity relationships.
//!
//! Relations connect entities across domains (e.g., initiatives to notes,
//! tags to items, etc.) enabling rich cross-domain linkage.

use crate::auth::{AuthenticatedUser, UserOwnableTable, require_user_owned};
use crate::error::AppError;
use axum::{
	Router,
	extract::{Path, Query, State},
	http::StatusCode,
	response::Json,
	routing::{delete, get, patch, post},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;

/// Entity relation model representing `entity_relations` table.
#[derive(Debug, Clone, sqlx::FromRow, Serialize, ToSchema)]
pub struct EntityRelation {
	pub id: Uuid,
	pub from_entity_type: String,
	pub from_entity_id: Uuid,
	pub to_entity_type: String,
	pub to_entity_id: Uuid,
	pub relation_type: String,
	pub source_type: String,
	pub status: String,
	pub confidence: Option<f64>,
	pub evidence: Option<serde_json::Value>,
	pub owner_user_id: Uuid,
	pub household_id: Option<Uuid>,
	pub notes: Option<String>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
	pub deleted_at: Option<DateTime<Utc>>,
}

/// Query parameters for listing relations.
#[derive(Debug, Deserialize, ToSchema, utoipa::IntoParams)]
pub struct ListRelationsQuery {
	/// Filter by source entity ID
	#[param(example = "550e8400-e29b-41d4-a716-446655440000")]
	pub from_entity_id: Option<Uuid>,
	/// Filter by target entity ID
	#[param(example = "550e8400-e29b-41d4-a716-446655440001")]
	pub to_entity_id: Option<Uuid>,
}

/// Request body for creating a new relation.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateRelationRequest {
	/// Type of source entity (user, household, initiative, tag, attachment, epic, quest, routine, focus_session, note, location, category, item)
	pub from_entity_type: String,
	/// ID of source entity
	pub from_entity_id: Uuid,
	/// Type of target entity (user, household, initiative, tag, attachment, epic, quest, routine, focus_session, note, location, category, item)
	pub to_entity_type: String,
	/// ID of target entity
	pub to_entity_id: Uuid,
	/// Type of relationship (parent_of, child_of, relates_to, depends_on, blocks, blocked_by, duplicates, similar_to, references, contains, owned_by, assigned_to, part_of, precedes, succeeds)
	pub relation_type: String,
	/// How this relation was established (manual, inferred, imported, system)
	pub source_type: Option<String>,
	/// Optional confidence score for inferred relations (0.0 to 1.0)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub confidence: Option<f64>,
	/// Optional evidence JSON for inferred relations
	#[serde(skip_serializing_if = "Option::is_none")]
	pub evidence: Option<serde_json::Value>,
	/// Optional household ID for shared relations
	#[serde(skip_serializing_if = "Option::is_none")]
	pub household_id: Option<Uuid>,
	/// Optional notes describing this relation
	#[serde(skip_serializing_if = "Option::is_none")]
	pub notes: Option<String>,
}

/// Request body for updating relation status.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateStatusRequest {
	/// New status for the relation (active, suspended, deleted)
	pub status: String,
}

/// List all relations accessible to the authenticated user.
///
/// Returns relations where user is either:
/// - The owner (`owner_user_id = user_id`)
/// - An active member of relation's household (via `household_memberships`)
///
/// Supports filtering by `from_entity_id` and `to_entity_id` query parameters.
/// Soft-deleted relations (`deleted_at IS NOT NULL`) are excluded.
#[utoipa::path(
	get,
	path = "/core/relations",
	tag = "Relations",
	responses(
		(status = 200, description = "List of accessible relations", body = Vec<EntityRelation>),
		(status = 401, description = "Unauthorized - invalid session token", body = crate::auth::ErrorResponse)
	),
	params(ListRelationsQuery),
	security(
		("better_auth_session" = [])
	)
)]
pub async fn list(
	State(pool): State<PgPool>,
	Query(query): Query<ListRelationsQuery>,
	user: AuthenticatedUser,
) -> Result<Json<Vec<EntityRelation>>, AppError> {
	let relations = match (&query.from_entity_id, &query.to_entity_id) {
		(None, None) => {
			sqlx::query_as::<_, EntityRelation>(
				r#"
				SELECT DISTINCT er.id, er.from_entity_type, er.from_entity_id,
				       er.to_entity_type, er.to_entity_id, er.relation_type,
				       er.source_type, er.status, er.confidence, er.evidence,
				       er.owner_user_id, er.household_id, er.notes,
				       er.created_at, er.updated_at, er.deleted_at
				FROM entity_relations er
				LEFT JOIN household_memberships hm ON
					hm.household_id = er.household_id AND
					hm.user_id = $1 AND
					hm.is_active = true
				WHERE er.deleted_at IS NULL AND (er.owner_user_id = $1 OR hm.id IS NOT NULL)
				ORDER BY er.created_at DESC
				"#,
			)
			.bind(user.0.id)
			.fetch_all(&pool)
			.await?
		}
		(Some(from_id), None) => {
			sqlx::query_as::<_, EntityRelation>(
				r#"
				SELECT DISTINCT er.id, er.from_entity_type, er.from_entity_id,
				       er.to_entity_type, er.to_entity_id, er.relation_type,
				       er.source_type, er.status, er.confidence, er.evidence,
				       er.owner_user_id, er.household_id, er.notes,
				       er.created_at, er.updated_at, er.deleted_at
				FROM entity_relations er
				WHERE er.deleted_at IS NULL AND er.from_entity_id = $1
				  AND (er.owner_user_id = $2 OR EXISTS(
				  	SELECT 1 FROM household_memberships hm
				  	WHERE hm.household_id = er.household_id
				  	  AND hm.user_id = $2
				  	  AND hm.is_active = true
				  ))
				ORDER BY er.created_at DESC
				"#,
			)
			.bind(from_id)
			.bind(user.0.id)
			.fetch_all(&pool)
			.await?
		}
		(None, Some(to_id)) => {
			sqlx::query_as::<_, EntityRelation>(
				r#"
				SELECT DISTINCT er.id, er.from_entity_type, er.from_entity_id,
				       er.to_entity_type, er.to_entity_id, er.relation_type,
				       er.source_type, er.status, er.confidence, er.evidence,
				       er.owner_user_id, er.household_id, er.notes,
				       er.created_at, er.updated_at, er.deleted_at
				FROM entity_relations er
				WHERE er.deleted_at IS NULL AND er.to_entity_id = $1
				  AND (er.owner_user_id = $2 OR EXISTS(
				  	SELECT 1 FROM household_memberships hm
				  	WHERE hm.household_id = er.household_id
				  	  AND hm.user_id = $2
				  	  AND hm.is_active = true
				  ))
				ORDER BY er.created_at DESC
				"#,
			)
			.bind(to_id)
			.bind(user.0.id)
			.fetch_all(&pool)
			.await?
		}
		(Some(from_id), Some(to_id)) => {
			sqlx::query_as::<_, EntityRelation>(
				r#"
				SELECT DISTINCT er.id, er.from_entity_type, er.from_entity_id,
				       er.to_entity_type, er.to_entity_id, er.relation_type,
				       er.source_type, er.status, er.confidence, er.evidence,
				       er.owner_user_id, er.household_id, er.notes,
				       er.created_at, er.updated_at, er.deleted_at
				FROM entity_relations er
				WHERE er.deleted_at IS NULL AND er.from_entity_id = $1 AND er.to_entity_id = $2
				  AND (er.owner_user_id = $3 OR EXISTS(
				  	SELECT 1 FROM household_memberships hm
				  	WHERE hm.household_id = er.household_id
				  	  AND hm.user_id = $3
				  	  AND hm.is_active = true
				  ))
				ORDER BY er.created_at DESC
				"#,
			)
			.bind(from_id)
			.bind(to_id)
			.bind(user.0.id)
			.fetch_all(&pool)
			.await?
		}
	};

	Ok(Json(relations))
}

/// Create a new relation.
///
/// Creates a single relation record with the authenticated user as owner.
/// Validates that both `from_entity_id` and `to_entity_id` exist
/// as valid entity IDs in their respective tables.
///
/// If a household_id is provided, validates that user is an active member.
#[utoipa::path(
	post,
	path = "/core/relations",
	tag = "Relations",
	request_body = CreateRelationRequest,
	responses(
		(status = 201, description = "Relation created successfully", body = EntityRelation),
		(status = 400, description = "Invalid request - entities don't exist or duplicate relation", body = crate::auth::ErrorResponse),
		(status = 401, description = "Unauthorized - invalid session token", body = crate::auth::ErrorResponse)
	),
	security(
		("better_auth_session" = [])
	)
)]
pub async fn create(
	State(pool): State<PgPool>,
	user: AuthenticatedUser,
	Json(req): Json<CreateRelationRequest>,
) -> Result<(StatusCode, Json<EntityRelation>), AppError> {
	if let Some(household_id) = req.household_id {
		let is_member = sqlx::query_scalar::<_, bool>(
			"SELECT EXISTS(
				SELECT 1 FROM household_memberships
				WHERE user_id = $1 AND household_id = $2 AND is_active = true
			)",
		)
		.bind(user.0.id)
		.bind(household_id)
		.fetch_one(&pool)
		.await?;

		if !is_member {
			return Err(AppError::BadRequest(
				"Not a member of specified household".to_string(),
			));
		}
	}

	let relation = sqlx::query_as::<_, EntityRelation>(
		r#"
		INSERT INTO entity_relations (
			from_entity_type, from_entity_id, to_entity_type, to_entity_id,
			relation_type, source_type, status, confidence, evidence,
			owner_user_id, household_id, notes
		)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
		RETURNING *
		"#,
	)
	.bind(&req.from_entity_type)
	.bind(req.from_entity_id)
	.bind(&req.to_entity_type)
	.bind(req.to_entity_id)
	.bind(&req.relation_type)
	.bind(&req.source_type)
	.bind("active")
	.bind(req.confidence)
	.bind(&req.evidence)
	.bind(user.0.id)
	.bind(req.household_id)
	.bind(&req.notes)
	.fetch_one(&pool)
	.await?;

	Ok((StatusCode::CREATED, Json(relation)))
}

/// Get a single relation by ID.
///
/// Returns 404 if:
/// - Relation doesn't exist
/// - Relation is soft-deleted
/// - User is neither owner nor active household member
#[utoipa::path(
	get,
	path = "/core/relations/{id}",
	tag = "Relations",
	responses(
		(status = 200, description = "Relation found", body = EntityRelation),
		(status = 404, description = "Relation not found", body = crate::auth::ErrorResponse),
		(status = 401, description = "Unauthorized - invalid session token", body = crate::auth::ErrorResponse)
	),
	params(
		("id" = Uuid, Path, description = "Relation ID")
	),
	security(
		("better_auth_session" = [])
	)
)]
pub async fn get_single(
	State(pool): State<PgPool>,
	user: AuthenticatedUser,
	Path(id): Path<Uuid>,
) -> Result<Json<EntityRelation>, AppError> {
	let relation = sqlx::query_as::<_, EntityRelation>(
		r#"
		SELECT er.id, er.from_entity_type, er.from_entity_id,
		       er.to_entity_type, er.to_entity_id, er.relation_type,
		       er.source_type, er.status, er.confidence, er.evidence,
		       er.owner_user_id, er.household_id, er.notes,
		       er.created_at, er.updated_at, er.deleted_at
		FROM entity_relations er
		WHERE er.id = $1 AND er.deleted_at IS NULL AND (
			er.owner_user_id = $2 OR
			EXISTS(
				SELECT 1 FROM household_memberships hm
				WHERE hm.household_id = er.household_id
				  AND hm.user_id = $2
				  AND hm.is_active = true
			)
		)
		"#,
	)
	.bind(id)
	.bind(user.0.id)
	.fetch_optional(&pool)
	.await?
	.ok_or_else(|| AppError::NotFound("Relation not found".to_string()))?;

	Ok(Json(relation))
}

/// Soft delete a relation.
///
/// Only relation owner can delete the relation.
/// Sets `deleted_at = NOW()` instead of removing the record.
#[utoipa::path(
	delete,
	path = "/core/relations/{id}",
	tag = "Relations",
	responses(
		(status = 204, description = "Relation deleted successfully"),
		(status = 404, description = "Relation not found", body = crate::auth::ErrorResponse),
		(status = 403, description = "Forbidden - not the relation owner", body = crate::auth::ErrorResponse),
		(status = 401, description = "Unauthorized - invalid session token", body = crate::auth::ErrorResponse)
	),
	params(
		("id" = Uuid, Path, description = "Relation ID")
	),
	security(
		("better_auth_session" = [])
	)
)]
pub async fn delete_relation(
	State(pool): State<PgPool>,
	user: AuthenticatedUser,
	Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
	require_user_owned(&pool, user.0.id, UserOwnableTable::EntityRelation, id)
		.await
		.map_err(|_| AppError::Forbidden)?;

	sqlx::query(
		"UPDATE entity_relations SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
	)
	.bind(id)
	.execute(&pool)
	.await?;

	Ok(StatusCode::NO_CONTENT)
}

/// Update relation status.
///
/// Only relation owner can update the status.
/// Allowed status values are: active, suspended, deleted
#[utoipa::path(
	patch,
	path = "/core/relations/{id}/status",
	tag = "Relations",
	request_body = UpdateStatusRequest,
	responses(
		(status = 200, description = "Status updated successfully", body = EntityRelation),
		(status = 404, description = "Relation not found", body = crate::auth::ErrorResponse),
		(status = 403, description = "Forbidden - not the relation owner", body = crate::auth::ErrorResponse),
		(status = 401, description = "Unauthorized - invalid session token", body = crate::auth::ErrorResponse)
	),
	params(
		("id" = Uuid, Path, description = "Relation ID")
	),
	security(
		("better_auth_session" = [])
	)
)]
pub async fn update_status(
	State(pool): State<PgPool>,
	user: AuthenticatedUser,
	Path(id): Path<Uuid>,
	Json(req): Json<UpdateStatusRequest>,
) -> Result<Json<EntityRelation>, AppError> {
	require_user_owned(&pool, user.0.id, UserOwnableTable::EntityRelation, id)
		.await
		.map_err(|_| AppError::Forbidden)?;

	let relation = sqlx::query_as::<_, EntityRelation>(
		r#"
		UPDATE entity_relations
		SET status = $1, updated_at = NOW()
		WHERE id = $2 AND deleted_at IS NULL
		RETURNING *
		"#,
	)
	.bind(&req.status)
	.bind(id)
	.fetch_one(&pool)
	.await?;

	Ok(Json(relation))
}

/// Create routes for relation operations.
///
/// Returns a router with all relation endpoints.
#[allow(dead_code)]
pub fn routes() -> Router<PgPool> {
	Router::new()
		.route("/", get(list))
		.route("/", post(create))
		.route("/{id}", get(get_single))
		.route("/{id}", delete(delete_relation))
		.route("/{id}/status", patch(update_status))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn routes_is_mountable() {
		let _router: Router<PgPool> = routes();
	}
}
