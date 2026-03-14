//! Household handler module.
//!
//! Handles CRUD operations for household resources.
//!
//! Households represent shared spaces where family members collaborate
//! on goals, tasks, and shared resources.

use crate::auth::{AuthenticatedUser, UserOwnableTable, can_access_household, require_user_owned};
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
use sqlx::{PgPool, Row, Transaction};
use utoipa::ToSchema;
use uuid::Uuid;

/// Household model representing the `households` table.
#[derive(Debug, Clone, sqlx::FromRow, Serialize, ToSchema)]
pub struct Household {
	pub id: Uuid,
	pub owner_user_id: Uuid,
	pub name: String,
	pub slug: Option<String>,
	pub description: Option<String>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
	pub deleted_at: Option<DateTime<Utc>>,
}

/// Household membership model representing the `household_memberships` table.
#[allow(dead_code)]
#[derive(Debug, Clone, sqlx::FromRow, Serialize, ToSchema)]
pub struct HouseholdMembership {
	pub id: Uuid,
	pub household_id: Uuid,
	pub user_id: Uuid,
	pub role: String,
	pub is_active: bool,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

/// Household member with user details for membership listing.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct HouseholdMember {
	pub id: Uuid,
	pub user_id: Uuid,
	pub user_name: String,
	pub user_email: String,
	pub role: String,
	pub is_active: bool,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

/// Request body for creating a new household.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateHouseholdRequest {
	pub name: String,
	pub slug: Option<String>,
	pub description: Option<String>,
}

/// Request body for updating a household.
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateHouseholdRequest {
	pub name: Option<String>,
	pub slug: Option<String>,
	pub description: Option<String>,
}

/// List all households accessible to the authenticated user.
///
/// Returns households where the user is either:
/// - The owner (`owner_user_id = user_id`)
/// - An active member (via `household_memberships`)
///
/// Soft-deleted households (`deleted_at IS NOT NULL`) are excluded.
#[utoipa::path(
	get,
	path = "/core/households",
	responses(
		(status = 200, description = "Households retrieved successfully", body = Vec<Household>)
	),
	security(("better_auth_session" = [])),
	tag = "households"
)]
#[axum::debug_handler]
pub async fn list(
	State(pool): State<PgPool>,
	user: AuthenticatedUser,
) -> Result<Json<Vec<Household>>, AppError> {
	let households = sqlx::query_as::<_, Household>(
		r#"
		SELECT DISTINCT h.id, h.owner_user_id, h.name, h.slug, h.description,
		       h.created_at, h.updated_at, h.deleted_at
		FROM households h
		LEFT JOIN household_memberships hm ON
			hm.household_id = h.id AND
			hm.user_id = $1 AND
			hm.is_active = true
		WHERE h.deleted_at IS NULL AND (h.owner_user_id = $1 OR hm.id IS NOT NULL)
		ORDER BY h.created_at DESC
		"#,
	)
	.bind(user.0.id)
	.fetch_all(&pool)
	.await?;

	Ok(Json(households))
}

/// Get a single household by ID.
///
/// Returns 404 if:
/// - Household doesn't exist
/// - Household is soft-deleted
/// - User is neither owner nor active member
#[utoipa::path(
	get,
	path = "/core/households/{id}",
	params(
		("id" = Uuid, Path, description = "Household ID")
	),
	responses(
		(status = 200, description = "Household retrieved successfully", body = Household),
		(status = 404, description = "Household not found or access denied")
	),
	security(("better_auth_session" = [])),
	tag = "households"
)]
#[axum::debug_handler]
pub async fn get_household(
	State(pool): State<PgPool>,
	user: AuthenticatedUser,
	Path(id): Path<Uuid>,
) -> Result<Json<Household>, AppError> {
	let can_access = can_access_household(&pool, user.0.id, id)
		.await
		.map_err(|e| AppError::Internal(format!("Authorization check failed: {}", e)))?;

	if !can_access {
		return Err(AppError::Forbidden);
	}

	let household = sqlx::query_as::<_, Household>(
		"SELECT * FROM households WHERE id = $1 AND deleted_at IS NULL",
	)
	.bind(id)
	.fetch_optional(&pool)
	.await?
	.ok_or_else(|| AppError::NotFound("Household not found".to_string()))?;

	Ok(Json(household))
}

/// Create a new household with automatic owner membership.
///
/// Creates both:
/// 1. The household record
/// 2. A household_memberships record with role='owner', is_active=true
///
/// Uses a database transaction to ensure atomicity.
#[utoipa::path(
	post,
	path = "/core/households",
	request_body = CreateHouseholdRequest,
	responses(
		(status = 201, description = "Household created successfully", body = Household)
	),
	security(("better_auth_session" = [])),
	tag = "households"
)]
#[axum::debug_handler]
pub async fn create(
	State(pool): State<PgPool>,
	user: AuthenticatedUser,
	Json(req): Json<CreateHouseholdRequest>,
) -> Result<(StatusCode, Json<Household>), AppError> {
	let mut tx: Transaction<'_, sqlx::Postgres> = pool.begin().await?;

	let household = sqlx::query_as::<_, Household>(
		r#"
		INSERT INTO households (owner_user_id, name, slug, description)
		VALUES ($1, $2, $3, $4)
		RETURNING *
		"#,
	)
	.bind(user.0.id)
	.bind(&req.name)
	.bind(&req.slug)
	.bind(&req.description)
	.fetch_one(&mut *tx)
	.await?;

	sqlx::query(
		r#"
		INSERT INTO household_memberships (household_id, user_id, role, is_active)
		VALUES ($1, $2, 'owner', true)
		"#,
	)
	.bind(household.id)
	.bind(user.0.id)
	.execute(&mut *tx)
	.await?;

	tx.commit().await?;

	Ok((StatusCode::CREATED, Json(household)))
}

/// Update a household.
///
/// Only the household owner can update the household.
/// Soft-deleted households cannot be updated.
#[utoipa::path(
	patch,
	path = "/core/households/{id}",
	params(
		("id" = Uuid, Path, description = "Household ID")
	),
	request_body = UpdateHouseholdRequest,
	responses(
		(status = 200, description = "Household updated successfully", body = Household),
		(status = 403, description = "User is not the household owner")
	),
	security(("better_auth_session" = [])),
	tag = "households"
)]
#[axum::debug_handler]
#[allow(unused_assignments)]
pub async fn update(
	State(pool): State<PgPool>,
	user: AuthenticatedUser,
	Path(id): Path<Uuid>,
	Json(req): Json<UpdateHouseholdRequest>,
) -> Result<Json<Household>, AppError> {
	require_user_owned(&pool, user.0.id, UserOwnableTable::Household, id)
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

	if query_parts.is_empty() {
		let household = sqlx::query_as::<_, Household>(
			"SELECT * FROM households WHERE id = $1 AND deleted_at IS NULL",
		)
		.bind(id)
		.fetch_one(&pool)
		.await?;
		return Ok(Json(household));
	}

	query_parts.push("updated_at = NOW()".to_string());

	let set_clause = query_parts.join(", ");
	let sql = format!(
		"UPDATE households SET {} WHERE id = $1 AND deleted_at IS NULL RETURNING *",
		set_clause
	);

	let mut query = sqlx::query_as::<_, Household>(&sql);
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

	let household = query.fetch_one(&pool).await?;

	Ok(Json(household))
}

/// Soft delete a household.
///
/// Only the household owner can delete the household.
/// Sets `deleted_at = NOW()` instead of removing the record.
#[utoipa::path(
	delete,
	path = "/core/households/{id}",
	params(
		("id" = Uuid, Path, description = "Household ID")
	),
	responses(
		(status = 204, description = "Household deleted successfully"),
		(status = 403, description = "User is not the household owner")
	),
	security(("better_auth_session" = [])),
	tag = "households"
)]
#[axum::debug_handler]
pub async fn delete_household(
	State(pool): State<PgPool>,
	user: AuthenticatedUser,
	Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
	require_user_owned(&pool, user.0.id, UserOwnableTable::Household, id)
		.await
		.map_err(|_| AppError::Forbidden)?;

	sqlx::query("UPDATE households SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL")
		.bind(id)
		.execute(&pool)
		.await?;

	Ok(StatusCode::NO_CONTENT)
}

/// List all members of a household.
///
/// Returns 404 if:
/// - Household doesn't exist
/// - Household is soft-deleted
/// - User is neither owner nor active member
#[utoipa::path(
	get,
	path = "/core/households/{id}/memberships",
	params(
		("id" = Uuid, Path, description = "Household ID")
	),
	responses(
		(status = 200, description = "Household memberships retrieved successfully", body = Vec<HouseholdMember>),
		(status = 403, description = "User is not a member of the household")
	),
	security(("better_auth_session" = [])),
	tag = "households"
)]
#[axum::debug_handler]
pub async fn list_memberships(
	State(pool): State<PgPool>,
	user: AuthenticatedUser,
	Path(id): Path<Uuid>,
) -> Result<Json<Vec<HouseholdMember>>, AppError> {
	let can_access = can_access_household(&pool, user.0.id, id)
		.await
		.map_err(|e| AppError::Internal(format!("Authorization check failed: {}", e)))?;

	if !can_access {
		return Err(AppError::Forbidden);
	}

	let rows = sqlx::query(
		r#"
		SELECT
			hm.id,
			hm.user_id,
			u.name as user_name,
			u.email as user_email,
			hm.role,
			hm.is_active,
			hm.created_at,
			hm.updated_at
		FROM household_memberships hm
		JOIN users u ON u.id = hm.user_id
		WHERE hm.household_id = $1
		ORDER BY hm.created_at ASC
		"#,
	)
	.bind(id)
	.fetch_all(&pool)
	.await?;

	let members: Vec<HouseholdMember> = rows
		.into_iter()
		.map(|row| HouseholdMember {
			id: row.get("id"),
			user_id: row.get("user_id"),
			user_name: row.get("user_name"),
			user_email: row.get("user_email"),
			role: row.get("role"),
			is_active: row.get("is_active"),
			created_at: row.get("created_at"),
			updated_at: row.get("updated_at"),
		})
		.collect();

	Ok(Json(members))
}

/// Create routes for household operations.
///
/// Returns a router with all household endpoints.
#[allow(dead_code)]
pub fn routes() -> Router<PgPool> {
	Router::new()
		.route("/", get(list))
		.route("/", post(create))
		.route("/{id}", get(get_household))
		.route("/{id}", patch(update))
		.route("/{id}", delete(delete_household))
		.route("/{id}/memberships", get(list_memberships))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn routes_is_mountable() {
		let _router: Router<PgPool> = routes();
	}
}
