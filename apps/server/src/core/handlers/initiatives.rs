//! Initiative handler module.
//!
//! Handles CRUD operations for initiative resources.
//!
//! Initiatives represent long-term goals, projects, or objectives
//! that can be tracked and completed over time.

use crate::auth::{
	AuthenticatedUser, UserOwnableTable, can_access_household, can_access_initiative,
	require_user_owned,
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

/// Initiative model representing the `initiatives` table.
#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct Initiative {
	pub id: Uuid,
	pub owner_user_id: Uuid,
	pub household_id: Option<Uuid>,
	pub title: String,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub status: String,
	pub start_date: Option<DateTime<Utc>>,
	pub target_date: Option<DateTime<Utc>>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
	pub deleted_at: Option<DateTime<Utc>>,
}

/// Request body for creating a new initiative.
#[derive(Debug, Deserialize)]
pub struct CreateInitiativeRequest {
	pub title: String,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub status: Option<String>,
	pub start_date: Option<DateTime<Utc>>,
	pub target_date: Option<DateTime<Utc>>,
	pub household_id: Option<Uuid>,
}

/// Request body for updating an initiative.
#[derive(Debug, Deserialize)]
pub struct UpdateInitiativeRequest {
	pub title: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub status: Option<String>,
	pub start_date: Option<DateTime<Utc>>,
	pub target_date: Option<DateTime<Utc>>,
	pub household_id: Option<Uuid>,
}

/// List all initiatives accessible to the authenticated user.
///
/// Returns initiatives where the user is either:
/// - The owner (`owner_user_id = user_id`)
/// - An active member of the initiative's household (via `household_memberships`)
///
/// Soft-deleted initiatives (`deleted_at IS NOT NULL`) are excluded.
#[axum::debug_handler]
pub async fn list(
	State(pool): State<PgPool>,
	user: AuthenticatedUser,
) -> Result<Json<Vec<Initiative>>, AppError> {
	let initiatives = sqlx::query_as::<_, Initiative>(
		r#"
		SELECT DISTINCT i.id, i.owner_user_id, i.household_id, i.title, i.slug, i.description,
		       i.status, i.start_date, i.target_date, i.created_at, i.updated_at, i.deleted_at
		FROM initiatives i
		LEFT JOIN household_memberships hm ON
			hm.household_id = i.household_id AND
			hm.user_id = $1 AND
			hm.is_active = true
		WHERE i.deleted_at IS NULL AND (i.owner_user_id = $1 OR hm.id IS NOT NULL)
		ORDER BY i.created_at DESC
		"#,
	)
	.bind(user.0.id)
	.fetch_all(&pool)
	.await?;

	Ok(Json(initiatives))
}

/// Get a single initiative by ID.
///
/// Returns 404 if:
/// - Initiative doesn't exist
/// - Initiative is soft-deleted
/// - User is neither owner nor active household member
#[axum::debug_handler]
pub async fn get_initiative(
	State(pool): State<PgPool>,
	user: AuthenticatedUser,
	Path(id): Path<Uuid>,
) -> Result<Json<Initiative>, AppError> {
	let can_access = can_access_initiative(&pool, user.0.id, id)
		.await
		.map_err(|e| AppError::Internal(format!("Authorization check failed: {}", e)))?;

	if !can_access {
		return Err(AppError::Forbidden);
	}

	let initiative = sqlx::query_as::<_, Initiative>(
		"SELECT * FROM initiatives WHERE id = $1 AND deleted_at IS NULL",
	)
	.bind(id)
	.fetch_optional(&pool)
	.await?
	.ok_or_else(|| AppError::NotFound("Initiative not found".to_string()))?;

	Ok(Json(initiative))
}

/// Create a new initiative.
///
/// Creates a single initiative record with the authenticated user as owner.
/// If a household_id is provided, validates that the user is an active member.
#[axum::debug_handler]
pub async fn create(
	State(pool): State<PgPool>,
	user: AuthenticatedUser,
	Json(req): Json<CreateInitiativeRequest>,
) -> Result<(StatusCode, Json<Initiative>), AppError> {
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

	let status = req.status.unwrap_or_else(|| "planned".to_string());

	let initiative = sqlx::query_as::<_, Initiative>(
		r#"
		INSERT INTO initiatives (owner_user_id, household_id, title, slug, description, status, start_date, target_date)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
		RETURNING *
		"#,
	)
	.bind(user.0.id)
	.bind(req.household_id)
	.bind(&req.title)
	.bind(&req.slug)
	.bind(&req.description)
	.bind(&status)
	.bind(req.start_date)
	.bind(req.target_date)
	.fetch_one(&pool)
	.await?;

	Ok((StatusCode::CREATED, Json(initiative)))
}

/// Update an initiative.
///
/// Only the initiative owner can update the initiative.
/// Soft-deleted initiatives cannot be updated.
#[axum::debug_handler]
pub async fn update(
	State(pool): State<PgPool>,
	user: AuthenticatedUser,
	Path(id): Path<Uuid>,
	Json(req): Json<UpdateInitiativeRequest>,
) -> Result<Json<Initiative>, AppError> {
	require_user_owned(&pool, user.0.id, UserOwnableTable::Initiative, id)
		.await
		.map_err(|_| AppError::Forbidden)?;

	let mut query_parts = Vec::new();
	let mut param_index = 2;

	if req.title.is_some() {
		query_parts.push(format!("title = ${}", param_index));
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
	if req.status.is_some() {
		query_parts.push(format!("status = ${}", param_index));
		param_index += 1;
	}
	if req.start_date.is_some() {
		query_parts.push(format!("start_date = ${}", param_index));
		param_index += 1;
	}
	if req.target_date.is_some() {
		query_parts.push(format!("target_date = ${}", param_index));
		param_index += 1;
	}
	if req.household_id.is_some() {
		query_parts.push(format!("household_id = ${}", param_index));
	}

	if query_parts.is_empty() {
		let initiative = sqlx::query_as::<_, Initiative>(
			"SELECT * FROM initiatives WHERE id = $1 AND deleted_at IS NULL",
		)
		.bind(id)
		.fetch_one(&pool)
		.await?;
		return Ok(Json(initiative));
	}

	query_parts.push("updated_at = NOW()".to_string());

	let set_clause = query_parts.join(", ");
	let sql = format!(
		"UPDATE initiatives SET {} WHERE id = $1 AND deleted_at IS NULL RETURNING *",
		set_clause
	);

	let mut query = sqlx::query_as::<_, Initiative>(&sql);
	query = query.bind(id);

	if let Some(title) = req.title {
		query = query.bind(title);
	}
	if let Some(slug) = req.slug {
		query = query.bind(slug);
	}
	if let Some(description) = req.description {
		query = query.bind(description);
	}
	if let Some(status) = req.status {
		query = query.bind(status);
	}
	if let Some(start_date) = req.start_date {
		query = query.bind(start_date);
	}
	if let Some(target_date) = req.target_date {
		query = query.bind(target_date);
	}
	if let Some(household_id) = req.household_id {
		query = query.bind(household_id);
	}

	let initiative = query.fetch_one(&pool).await?;

	Ok(Json(initiative))
}

/// Soft delete an initiative.
///
/// Only the initiative owner can delete the initiative.
/// Sets `deleted_at = NOW()` instead of removing the record.
#[axum::debug_handler]
pub async fn delete_initiative(
	State(pool): State<PgPool>,
	user: AuthenticatedUser,
	Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
	require_user_owned(&pool, user.0.id, UserOwnableTable::Initiative, id)
		.await
		.map_err(|_| AppError::Forbidden)?;

	sqlx::query("UPDATE initiatives SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL")
		.bind(id)
		.execute(&pool)
		.await?;

	Ok(StatusCode::NO_CONTENT)
}

/// Create routes for initiative operations.
///
/// Returns a router with all initiative endpoints.
#[allow(dead_code)]
pub fn routes() -> Router<PgPool> {
	Router::new()
		.route("/", get(list))
		.route("/", post(create))
		.route("/{id}", get(get_initiative))
		.route("/{id}", patch(update))
		.route("/{id}", delete(delete_initiative))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn routes_is_mountable() {
		let _router: Router<PgPool> = routes();
	}
}
