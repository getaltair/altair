//! Entity relation handler module.
//!
//! Handles CRUD operations for cross-domain entity relationships.
//!
//! Entity relations connect different domain entities (initiatives, notes, tags, etc.)
//! with typed relationships like "references", "supports", "requires", etc.

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
use contracts::{EntityType, RelationSourceType, RelationStatusType, RelationType};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;

/// Entity relation model representing the `entity_relations` table.
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
	pub confidence: Option<Decimal>,
	pub evidence: Option<sqlx::types::Json<serde_json::Value>>,
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

/// Request body for creating a new entity relation.
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
	/// New status for the relation (accepted, dismissed, rejected, expired)
	pub status: String,
}

/// Validates an entity type string against contracts enum.
fn validate_entity_type(value: &str) -> Result<(), AppError> {
	serde_json::from_value::<EntityType>(serde_json::Value::String(value.to_string()))
		.map_err(|_| AppError::BadRequest(format!("Invalid entity_type: {}", value)))?;
	Ok(())
}

/// Validates a relation type string against contracts enum.
fn validate_relation_type(value: &str) -> Result<(), AppError> {
	serde_json::from_value::<RelationType>(serde_json::Value::String(value.to_string()))
		.map_err(|_| AppError::BadRequest(format!("Invalid relation_type: {}", value)))?;
	Ok(())
}

/// Validates a source type string against contracts enum.
fn validate_source_type(value: &str) -> Result<(), AppError> {
	serde_json::from_value::<RelationSourceType>(serde_json::Value::String(value.to_string()))
		.map_err(|_| AppError::BadRequest(format!("Invalid source_type: {}", value)))?;
	Ok(())
}

/// Validates a status string against contracts enum.
fn validate_status(value: &str) -> Result<(), AppError> {
	serde_json::from_value::<RelationStatusType>(serde_json::Value::String(value.to_string()))
		.map_err(|_| AppError::BadRequest(format!("Invalid status: {}", value)))?;
	Ok(())
}

/// Checks if a user can access a relation via dual-path authorization.
///
/// A relation is accessible if the user is either:
/// - The owner of the relation (`owner_user_id = user_id`)
/// - A member of the relation's household (via `household_memberships` with `is_active = true`)
async fn can_access_relation(
	pool: &PgPool,
	user_id: Uuid,
	relation_id: Uuid,
) -> Result<bool, AppError> {
	let can_access = sqlx::query_scalar::<_, bool>(
		r#"
		SELECT EXISTS(
			SELECT 1 FROM entity_relations er
			LEFT JOIN household_memberships hm ON
				hm.household_id = er.household_id AND
				hm.user_id = $1 AND
				hm.is_active = true
			WHERE er.id = $2 AND er.deleted_at IS NULL AND (
				er.owner_user_id = $1 OR
				hm.id IS NOT NULL
			)
		)
		"#,
	)
	.bind(user_id)
	.bind(relation_id)
	.fetch_one(pool)
	.await
	.map_err(|e| AppError::Internal(format!("Authorization check failed: {}", e)))?;

	Ok(can_access)
}

/// List all relations accessible to the authenticated user.
///
/// Returns relations where the user is either:
/// - The owner (`owner_user_id = user_id`)
/// - An active member of the relation's household (via `household_memberships`)
///
/// Query params: `from_entity_id` OR `to_entity_id` (one required)
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
#[axum::debug_handler]
pub async fn list(
	State(pool): State<PgPool>,
	user: AuthenticatedUser,
	Query(query): Query<ListRelationsQuery>,
) -> Result<Json<Vec<EntityRelation>>, AppError> {
	if query.from_entity_id.is_none() && query.to_entity_id.is_none() {
		return Err(AppError::BadRequest(
			"Either from_entity_id or to_entity_id is required".to_string(),
		));
	}

	let relations = match (query.from_entity_id, query.to_entity_id) {
		(Some(from_id), Some(to_id)) => {
			sqlx::query_as::<_, EntityRelation>(
				r#"
				SELECT DISTINCT er.id, er.from_entity_type::text, er.from_entity_id, er.to_entity_type::text,
				       er.to_entity_id, er.relation_type::text, er.source_type::text, er.status::text, er.confidence,
				       er.evidence, er.owner_user_id, er.household_id, er.notes, er.created_at,
				       er.updated_at, er.deleted_at
				FROM entity_relations er
				LEFT JOIN household_memberships hm ON
					hm.household_id = er.household_id AND
					hm.user_id = $1 AND
					hm.is_active = true
				WHERE er.deleted_at IS NULL
				  AND (er.owner_user_id = $1 OR hm.id IS NOT NULL)
				  AND er.from_entity_id = $2
				  AND er.to_entity_id = $3
				ORDER BY er.created_at DESC
				"#,
			)
			.bind(user.0.id)
			.bind(from_id)
			.bind(to_id)
			.fetch_all(&pool)
			.await?
		}
		(Some(from_id), None) => {
			sqlx::query_as::<_, EntityRelation>(
				r#"
				SELECT DISTINCT er.id, er.from_entity_type::text, er.from_entity_id, er.to_entity_type::text,
				       er.to_entity_id, er.relation_type::text, er.source_type::text, er.status::text, er.confidence,
				       er.evidence, er.owner_user_id, er.household_id, er.notes, er.created_at,
				       er.updated_at, er.deleted_at
				FROM entity_relations er
				LEFT JOIN household_memberships hm ON
					hm.household_id = er.household_id AND
					hm.user_id = $1 AND
					hm.is_active = true
				WHERE er.deleted_at IS NULL
				  AND (er.owner_user_id = $1 OR hm.id IS NOT NULL)
				  AND er.from_entity_id = $2
				ORDER BY er.created_at DESC
				"#,
			)
			.bind(user.0.id)
			.bind(from_id)
			.fetch_all(&pool)
			.await?
		}
		(None, Some(to_id)) => {
			sqlx::query_as::<_, EntityRelation>(
				r#"
				SELECT DISTINCT er.id, er.from_entity_type::text, er.from_entity_id, er.to_entity_type::text,
				       er.to_entity_id, er.relation_type::text, er.source_type::text, er.status::text, er.confidence,
				       er.evidence, er.owner_user_id, er.household_id, er.notes, er.created_at,
				       er.updated_at, er.deleted_at
				FROM entity_relations er
				LEFT JOIN household_memberships hm ON
					hm.household_id = er.household_id AND
					hm.user_id = $1 AND
					hm.is_active = true
				WHERE er.deleted_at IS NULL
				  AND (er.owner_user_id = $1 OR hm.id IS NOT NULL)
				  AND er.to_entity_id = $2
				ORDER BY er.created_at DESC
				"#,
			)
			.bind(user.0.id)
			.bind(to_id)
			.fetch_all(&pool)
			.await?
		}
		(None, None) => {
			return Err(AppError::BadRequest(
				"Either from_entity_id or to_entity_id is required".to_string(),
			));
		}
	};

	Ok(Json(relations))
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
#[axum::debug_handler]
pub async fn get_single(
	State(pool): State<PgPool>,
	user: AuthenticatedUser,
	Path(id): Path<Uuid>,
) -> Result<Json<EntityRelation>, AppError> {
	let can_access = can_access_relation(&pool, user.0.id, id).await?;

	if !can_access {
		return Err(AppError::Forbidden);
	}

	let relation = sqlx::query_as::<_, EntityRelation>(
		r#"SELECT id, from_entity_type::text, from_entity_id, to_entity_type::text, to_entity_id,
		   relation_type::text, source_type::text, status::text, confidence, evidence,
		   owner_user_id, household_id, notes, created_at, updated_at, deleted_at
		   FROM entity_relations WHERE id = $1 AND deleted_at IS NULL"#,
	)
	.bind(id)
	.fetch_optional(&pool)
	.await?
	.ok_or_else(|| AppError::NotFound("Relation not found".to_string()))?;

	Ok(Json(relation))
}

/// Create a new entity relation.
///
/// Validates all enum inputs against contracts.
/// Sets `owner_user_id` = authenticated user.
/// Sets `status` = "suggested" (or "accepted" if source_type="user").
/// Returns 201 Created.
/// Returns 409 Conflict if duplicate relation exists.
#[utoipa::path(
	post,
	path = "/core/relations",
	tag = "Relations",
	request_body = CreateRelationRequest,
	responses(
		(status = 201, description = "Relation created successfully", body = EntityRelation),
		(status = 409, description = "Duplicate relation exists", body = crate::auth::ErrorResponse),
		(status = 400, description = "Invalid request parameters", body = crate::auth::ErrorResponse),
		(status = 401, description = "Unauthorized - invalid session token", body = crate::auth::ErrorResponse)
	),
	security(
		("better_auth_session" = [])
	)
)]
#[axum::debug_handler]
pub async fn create(
	State(pool): State<PgPool>,
	user: AuthenticatedUser,
	Json(req): Json<CreateRelationRequest>,
) -> Result<(StatusCode, Json<EntityRelation>), AppError> {
	validate_entity_type(&req.from_entity_type)?;
	validate_entity_type(&req.to_entity_type)?;

	validate_relation_type(&req.relation_type)?;

	let source_type = req.source_type.unwrap_or_else(|| "user".to_string());
	validate_source_type(&source_type)?;

	let status = if source_type == "user" {
		"accepted".to_string()
	} else {
		"suggested".to_string()
	};

	let confidence_decimal = match req.confidence {
		Some(c) if c.is_finite() && (0.0..=1.0).contains(&c) => {
			Some(Decimal::from_f64_retain(c).unwrap_or(Decimal::ZERO))
		}
		Some(_) => {
			return Err(AppError::BadRequest(
				"confidence must be a finite number between 0 and 1".to_string(),
			));
		}
		None => None,
	};

	let relation = sqlx::query_as::<_, EntityRelation>(
		r#"
		INSERT INTO entity_relations (
			from_entity_type, from_entity_id, to_entity_type, to_entity_id,
			relation_type, source_type, status, confidence, evidence,
			owner_user_id, household_id, notes
		)
		VALUES ($1::entity_type_v2, $2, $3::entity_type_v2, $4, $5::relation_type_v2, $6::source_type_v2, $7::relation_status_v2, $8, $9, $10, $11, $12)
		RETURNING
			id, from_entity_type::text, from_entity_id, to_entity_type::text, to_entity_id,
			relation_type::text, source_type::text, status::text, confidence, evidence,
			owner_user_id, household_id, notes, created_at, updated_at, deleted_at
		"#,
	)
	.bind(&req.from_entity_type)
	.bind(req.from_entity_id)
	.bind(&req.to_entity_type)
	.bind(req.to_entity_id)
	.bind(&req.relation_type)
	.bind(&source_type)
	.bind(&status)
	.bind(confidence_decimal)
	.bind(req.evidence.map(sqlx::types::Json))
	.bind(user.0.id)
	.bind(req.household_id)
	.bind(&req.notes)
	.fetch_one(&pool)
	.await
	.map_err(|e| {
		#[allow(clippy::collapsible_if)]
		if let sqlx::Error::Database(ref db_err) = e {
			if db_err.code().as_deref() == Some("23505") {
				return AppError::Conflict(
					"A relation with these entities and type already exists".to_string(),
				);
			}
		}
		AppError::from(e)
	})?;

	Ok((StatusCode::CREATED, Json(relation)))
}

/// Soft delete a relation.
///
/// Only relation owner can delete relation.
/// Sets `deleted_at = NOW()` instead of removing record.
/// Returns 204 No Content.
#[utoipa::path(
	delete,
	path = "/core/relations/{id}",
	tag = "Relations",
	responses(
		(status = 204, description = "Relation deleted successfully"),
		(status = 404, description = "Relation not found", body = crate::auth::ErrorResponse),
		(status = 403, description = "Forbidden - not relation owner", body = crate::auth::ErrorResponse),
		(status = 401, description = "Unauthorized - invalid session token", body = crate::auth::ErrorResponse)
	),
	params(
		("id" = Uuid, Path, description = "Relation ID")
	),
	security(
		("better_auth_session" = [])
	)
)]
#[axum::debug_handler]
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

/// Update a relation's status.
///
/// Only relation owner can update status.
/// Validates status: "accepted" | "dismissed" | "rejected" | "expired".
/// No state machine validation (any transition allowed).
/// Returns 200 with updated relation.
#[utoipa::path(
	patch,
	path = "/core/relations/{id}/status",
	tag = "Relations",
	request_body = UpdateStatusRequest,
	responses(
		(status = 200, description = "Status updated successfully", body = EntityRelation),
		(status = 404, description = "Relation not found", body = crate::auth::ErrorResponse),
		(status = 403, description = "Forbidden - not relation owner", body = crate::auth::ErrorResponse),
		(status = 401, description = "Unauthorized - invalid session token", body = crate::auth::ErrorResponse)
	),
	params(
		("id" = Uuid, Path, description = "Relation ID")
	),
	security(
		("better_auth_session" = [])
	)
)]
#[axum::debug_handler]
pub async fn update_status(
	State(pool): State<PgPool>,
	user: AuthenticatedUser,
	Path(id): Path<Uuid>,
	Json(req): Json<UpdateStatusRequest>,
) -> Result<Json<EntityRelation>, AppError> {
	require_user_owned(&pool, user.0.id, UserOwnableTable::EntityRelation, id)
		.await
		.map_err(|_| AppError::Forbidden)?;

	validate_status(&req.status)?;

	let relation = sqlx::query_as::<_, EntityRelation>(
		r#"
		UPDATE entity_relations
		SET status = $1::relation_status_v2, updated_at = NOW()
		WHERE id = $2 AND deleted_at IS NULL
		RETURNING
			id, from_entity_type::text, from_entity_id, to_entity_type::text, to_entity_id,
			relation_type::text, source_type::text, status::text, confidence, evidence,
			owner_user_id, household_id, notes, created_at, updated_at, deleted_at
		"#,
	)
	.bind(&req.status)
	.bind(id)
	.fetch_optional(&pool)
	.await?
	.ok_or_else(|| AppError::NotFound("Relation not found".to_string()))?;

	Ok(Json(relation))
}

/// Create routes for relation operations.
///
/// Returns a router with all relation endpoints.
#[allow(dead_code)] // Wired in Task 20
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

	#[test]
	fn validate_entity_type_valid() {
		assert!(validate_entity_type("initiative").is_ok());
		assert!(validate_entity_type("tag").is_ok());
		assert!(validate_entity_type("knowledge_note").is_ok());
	}

	#[test]
	fn validate_entity_type_invalid() {
		assert!(validate_entity_type("invalid_type").is_err());
		assert!(validate_entity_type("").is_err());
	}

	#[test]
	fn validate_relation_type_valid() {
		assert!(validate_relation_type("references").is_ok());
		assert!(validate_relation_type("supports").is_ok());
		assert!(validate_relation_type("depends_on").is_ok());
	}

	#[test]
	fn validate_relation_type_invalid() {
		assert!(validate_relation_type("invalid").is_err());
	}

	#[test]
	fn validate_source_type_valid() {
		assert!(validate_source_type("user").is_ok());
		assert!(validate_source_type("ai").is_ok());
		assert!(validate_source_type("import").is_ok());
	}

	#[test]
	fn validate_source_type_invalid() {
		assert!(validate_source_type("invalid").is_err());
	}

	#[test]
	fn validate_status_valid() {
		assert!(validate_status("accepted").is_ok());
		assert!(validate_status("suggested").is_ok());
		assert!(validate_status("dismissed").is_ok());
		assert!(validate_status("rejected").is_ok());
		assert!(validate_status("expired").is_ok());
	}

	#[test]
	fn validate_status_invalid() {
		assert!(validate_status("invalid").is_err());
	}

	#[test]
	fn list_relations_query_deserialization() {
		let query: ListRelationsQuery =
			serde_json::from_str(r#"{"from_entity_id": "00000000-0000-0000-0000-000000000001"}"#)
				.unwrap();
		assert!(query.from_entity_id.is_some());
		assert!(query.to_entity_id.is_none());
	}

	#[test]
	fn create_relation_request_deserialization() {
		let req: CreateRelationRequest = serde_json::from_str(
			r#"{
				"from_entity_type": "initiative",
				"from_entity_id": "00000000-0000-0000-0000-000000000001",
				"to_entity_type": "tag",
				"to_entity_id": "00000000-0000-0000-0000-000000000002",
				"relation_type": "references"
			}"#,
		)
		.unwrap();
		assert_eq!(req.from_entity_type, "initiative");
		assert_eq!(req.relation_type, "references");
		assert!(req.source_type.is_none());
	}

	#[test]
	fn update_status_request_deserialization() {
		let req: UpdateStatusRequest = serde_json::from_str(r#"{"status": "accepted"}"#).unwrap();
		assert_eq!(req.status, "accepted");
	}
}
