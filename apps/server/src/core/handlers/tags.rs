//! Tag handler module.
//!
//! Handles CRUD operations for tag resources.
//!
//! Tags provide flexible categorization for initiatives, tasks,
//! and other domain entities for better organization and filtering.

use crate::auth::{
	AuthenticatedUser, UserOwnableTable, can_access_household, can_access_tag, require_user_owned,
};
use crate::error::AppError;
use axum::{
	Router,
	extract::{Path, State},
	http::StatusCode,
	response::Json,
	routing::{delete, get, patch, post},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

/// Tag model representing the `tags` table.
#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct Tag {
	pub id: Uuid,
	pub owner_user_id: Uuid,
	pub household_id: Option<Uuid>,
	pub name: String,
	pub slug: String,
	pub description: Option<String>,
	pub color: Option<String>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
	pub deleted_at: Option<DateTime<Utc>>,
}

/// Request body for creating a new tag.
#[derive(Debug, Deserialize)]
pub struct CreateTagRequest {
	pub name: String,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub color: Option<String>,
	pub household_id: Option<Uuid>,
}

/// Request body for updating a tag.
#[derive(Debug, Deserialize)]
pub struct UpdateTagRequest {
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub color: Option<String>,
	pub household_id: Option<Uuid>,
}

/// List all tags accessible to the authenticated user.
///
/// Returns tags where the user is either:
/// - The owner (`owner_user_id = user_id`)
/// - An active member of the tag's household (via `household_memberships`)
///
/// Soft-deleted tags (`deleted_at IS NOT NULL`) are excluded.
#[axum::debug_handler]
pub async fn list(
	State(pool): State<PgPool>,
	user: AuthenticatedUser,
) -> Result<Json<Vec<Tag>>, AppError> {
	let tags = sqlx::query_as::<_, Tag>(
		r#"
		SELECT DISTINCT t.id, t.owner_user_id, t.household_id, t.name, t.slug,
		       t.description, t.color, t.created_at, t.updated_at, t.deleted_at
		FROM tags t
		LEFT JOIN household_memberships hm ON
			hm.household_id = t.household_id AND
			hm.user_id = $1 AND
			hm.is_active = true
		WHERE t.deleted_at IS NULL AND (t.owner_user_id = $1 OR hm.id IS NOT NULL)
		ORDER BY t.created_at DESC
		"#,
	)
	.bind(user.0.id)
	.fetch_all(&pool)
	.await?;

	Ok(Json(tags))
}

/// Get a single tag by ID.
///
/// Returns 404 if:
/// - Tag doesn't exist
/// - Tag is soft-deleted
/// - User is neither owner nor active household member
#[axum::debug_handler]
pub async fn get_tag(
	State(pool): State<PgPool>,
	user: AuthenticatedUser,
	Path(id): Path<Uuid>,
) -> Result<Json<Tag>, AppError> {
	let can_access = can_access_tag(&pool, user.0.id, id)
		.await
		.map_err(|e| AppError::Internal(format!("Authorization check failed: {}", e)))?;

	if !can_access {
		return Err(AppError::Forbidden);
	}

	let tag = sqlx::query_as::<_, Tag>("SELECT * FROM tags WHERE id = $1 AND deleted_at IS NULL")
		.bind(id)
		.fetch_optional(&pool)
		.await?
		.ok_or_else(|| AppError::NotFound("Tag not found".to_string()))?;

	Ok(Json(tag))
}

/// Create a new tag.
///
/// Creates a single tag record with the authenticated user as owner.
/// If a household_id is provided, validates that the user is an active member.
#[axum::debug_handler]
pub async fn create(
	State(pool): State<PgPool>,
	user: AuthenticatedUser,
	Json(req): Json<CreateTagRequest>,
) -> Result<(StatusCode, Json<Tag>), AppError> {
	if let Some(household_id) = req.household_id {
		let can_access = can_access_household(&pool, user.0.id, household_id)
			.await
			.map_err(|e| AppError::Internal(format!("Household access check failed: {}", e)))?;
		if !can_access {
			return Err(AppError::BadRequest(
				"Not a member of the specified household".to_string(),
			));
		}
	}

	let tag = sqlx::query_as::<_, Tag>(
		r#"
		INSERT INTO tags (owner_user_id, household_id, name, slug, description, color)
		VALUES ($1, $2, $3, $4, $5, $6)
		RETURNING *
		"#,
	)
	.bind(user.0.id)
	.bind(req.household_id)
	.bind(&req.name)
	.bind(&req.slug)
	.bind(&req.description)
	.bind(&req.color)
	.fetch_one(&pool)
	.await?;

	Ok((StatusCode::CREATED, Json(tag)))
}

/// Update a tag.
///
/// Only the tag owner can update the tag.
/// Soft-deleted tags cannot be updated.
#[axum::debug_handler]
pub async fn update(
	State(pool): State<PgPool>,
	user: AuthenticatedUser,
	Path(id): Path<Uuid>,
	Json(req): Json<UpdateTagRequest>,
) -> Result<Json<Tag>, AppError> {
	require_user_owned(&pool, user.0.id, UserOwnableTable::Tag, id)
		.await
		.map_err(|_| AppError::Forbidden)?;

	let mut query_parts = Vec::new();
	let mut param_index = 2;

	if req.name.is_some() {
		query_parts.push(format!("name = ${}", param_index));
		param_index += 1;
	}
	if req.slug.is_some() {
		query_parts.push(format!("slug = ${}", param_index));
		param_index += 1;
	}
	if req.description.is_some() {
		query_parts.push(format!("description = ${}", param_index));
		param_index += 1;
	}
	if req.color.is_some() {
		query_parts.push(format!("color = ${}", param_index));
		param_index += 1;
	}
	if req.household_id.is_some() {
		query_parts.push(format!("household_id = ${}", param_index));
	}

	if query_parts.is_empty() {
		let tag =
			sqlx::query_as::<_, Tag>("SELECT * FROM tags WHERE id = $1 AND deleted_at IS NULL")
				.bind(id)
				.fetch_one(&pool)
				.await?;
		return Ok(Json(tag));
	}

	query_parts.push("updated_at = NOW()".to_string());

	let set_clause = query_parts.join(", ");
	let sql = format!(
		"UPDATE tags SET {} WHERE id = $1 AND deleted_at IS NULL RETURNING *",
		set_clause
	);

	let mut query = sqlx::query_as::<_, Tag>(&sql);
	query = query.bind(id);

	if let Some(name) = req.name {
		query = query.bind(name);
	}
	if let Some(slug) = req.slug {
		query = query.bind(slug);
	}
	if let Some(description) = req.description {
		query = query.bind(description);
	}
	if let Some(color) = req.color {
		query = query.bind(color);
	}
	if let Some(household_id) = req.household_id {
		query = query.bind(household_id);
	}

	let tag = query.fetch_one(&pool).await?;

	Ok(Json(tag))
}

/// Soft delete a tag.
///
/// Only the tag owner can delete the tag.
/// Sets `deleted_at = NOW()` instead of removing the record.
#[axum::debug_handler]
pub async fn delete_tag(
	State(pool): State<PgPool>,
	user: AuthenticatedUser,
	Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
	require_user_owned(&pool, user.0.id, UserOwnableTable::Tag, id)
		.await
		.map_err(|_| AppError::Forbidden)?;

	sqlx::query("UPDATE tags SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL")
		.bind(id)
		.execute(&pool)
		.await?;

	Ok(StatusCode::NO_CONTENT)
}

/// Create routes for tag operations.
///
/// Returns a router with all tag endpoints.
#[allow(dead_code)] // Wired in Task 20
pub fn routes() -> Router<PgPool> {
	Router::new()
		.route("/", get(list))
		.route("/", post(create))
		.route("/{id}", get(get_tag))
		.route("/{id}", patch(update))
		.route("/{id}", delete(delete_tag))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn routes_is_mountable() {
		let _router: Router<PgPool> = routes();
	}
}
