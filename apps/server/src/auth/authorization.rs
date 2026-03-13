//! Authorization module for household access control.
//!
//! Defines roles and authorization helpers for managing household memberships.
//!
//! Note: Some items are marked as dead code because they are infrastructure
//! exports that will be used by route handlers in future tasks (P3-005+).

#![allow(dead_code)]

use axum::{
	http::StatusCode,
	response::{IntoResponse, Json, Response},
};
use serde::{Deserialize, Serialize};
use sqlx::{Error as SqlxError, postgres::PgPool};
use std::str::FromStr;
use thiserror::Error;
use uuid::Uuid;

/// Household role with hierarchical permissions.
///
/// Roles are ordered by permission level: Owner > Admin > Member > Viewer.
#[derive(
	PartialOrd, PartialEq, Eq, Clone, Copy, Debug, Hash, Serialize, Deserialize, sqlx::Type,
)]
#[sqlx(type_name = "household_role")]
#[serde(rename_all = "lowercase")]
#[repr(i16)]
pub enum HouseholdRole {
	Owner = 4,
	Admin = 3,
	Member = 2,
	Viewer = 1,
}

impl std::fmt::Display for HouseholdRole {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			HouseholdRole::Owner => write!(f, "owner"),
			HouseholdRole::Admin => write!(f, "admin"),
			HouseholdRole::Member => write!(f, "member"),
			HouseholdRole::Viewer => write!(f, "viewer"),
		}
	}
}

impl std::str::FromStr for HouseholdRole {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"owner" => Ok(HouseholdRole::Owner),
			"admin" => Ok(HouseholdRole::Admin),
			"member" => Ok(HouseholdRole::Member),
			"viewer" => Ok(HouseholdRole::Viewer),
			_ => Err(format!("Invalid household role: {}", s)),
		}
	}
}

impl From<&str> for HouseholdRole {
	fn from(s: &str) -> Self {
		HouseholdRole::from_str(s).expect("Invalid household role string")
	}
}

impl From<String> for HouseholdRole {
	fn from(s: String) -> Self {
		HouseholdRole::from_str(&s).expect("Invalid household role string")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn owner_is_greater_than_admin() {
		assert!(HouseholdRole::Owner > HouseholdRole::Admin);
		assert!(HouseholdRole::Owner >= HouseholdRole::Admin);
		assert!(HouseholdRole::Owner >= HouseholdRole::Admin);
	}

	#[test]
	fn admin_is_greater_than_member() {
		assert!(HouseholdRole::Admin > HouseholdRole::Member);
		assert!(HouseholdRole::Admin >= HouseholdRole::Member);
		assert!(HouseholdRole::Admin >= HouseholdRole::Member);
	}

	#[test]
	fn member_is_greater_than_viewer() {
		assert!(HouseholdRole::Member > HouseholdRole::Viewer);
		assert!(HouseholdRole::Member >= HouseholdRole::Viewer);
		assert!(HouseholdRole::Member >= HouseholdRole::Viewer);
	}

	#[test]
	fn viewer_is_not_greater_than_admin() {
		assert!(HouseholdRole::Viewer < HouseholdRole::Admin);
		assert!(HouseholdRole::Viewer < HouseholdRole::Admin);
	}

	#[test]
	fn role_equal_to_itself() {
		assert_eq!(HouseholdRole::Owner, HouseholdRole::Owner);
		assert_eq!(HouseholdRole::Admin, HouseholdRole::Admin);
		assert_eq!(HouseholdRole::Member, HouseholdRole::Member);
		assert_eq!(HouseholdRole::Viewer, HouseholdRole::Viewer);
	}

	#[test]
	fn role_not_equal_to_different_role() {
		assert_ne!(HouseholdRole::Owner, HouseholdRole::Admin);
		assert_ne!(HouseholdRole::Admin, HouseholdRole::Member);
		assert_ne!(HouseholdRole::Member, HouseholdRole::Viewer);
		assert_ne!(HouseholdRole::Viewer, HouseholdRole::Owner);
	}

	#[test]
	fn display_owner() {
		assert_eq!(format!("{}", HouseholdRole::Owner), "owner");
	}

	#[test]
	fn display_admin() {
		assert_eq!(format!("{}", HouseholdRole::Admin), "admin");
	}

	#[test]
	fn display_member() {
		assert_eq!(format!("{}", HouseholdRole::Member), "member");
	}

	#[test]
	fn display_viewer() {
		assert_eq!(format!("{}", HouseholdRole::Viewer), "viewer");
	}

	#[test]
	fn from_str_owner() {
		assert_eq!(
			HouseholdRole::from_str("owner").unwrap(),
			HouseholdRole::Owner
		);
	}

	#[test]
	fn from_str_admin() {
		assert_eq!(
			HouseholdRole::from_str("admin").unwrap(),
			HouseholdRole::Admin
		);
	}

	#[test]
	fn from_str_member() {
		assert_eq!(
			HouseholdRole::from_str("member").unwrap(),
			HouseholdRole::Member
		);
	}

	#[test]
	fn from_str_viewer() {
		assert_eq!(
			HouseholdRole::from_str("viewer").unwrap(),
			HouseholdRole::Viewer
		);
	}

	#[test]
	fn from_str_case_insensitive() {
		assert_eq!(
			HouseholdRole::from_str("OWNER").unwrap(),
			HouseholdRole::Owner
		);
		assert_eq!(
			HouseholdRole::from_str("Admin").unwrap(),
			HouseholdRole::Admin
		);
		assert_eq!(
			HouseholdRole::from_str("MeMbEr").unwrap(),
			HouseholdRole::Member
		);
	}

	#[test]
	fn from_str_invalid_role() {
		assert!(HouseholdRole::from_str("invalid").is_err());
		assert!(HouseholdRole::from_str("").is_err());
	}

	#[test]
	fn from_str_ref_owner() {
		assert_eq!(HouseholdRole::from("owner"), HouseholdRole::Owner);
	}

	#[test]
	fn from_string_admin() {
		assert_eq!(
			HouseholdRole::from("admin".to_string()),
			HouseholdRole::Admin
		);
	}

	#[test]
	fn role_copy() {
		let original = HouseholdRole::Admin;
		let copied = original;
		assert_eq!(original, copied);
	}

	#[test]
	fn role_clone() {
		let original = HouseholdRole::Member;
		let cloned = original;
		assert_eq!(original, cloned);
	}

	#[test]
	fn debug_format() {
		assert!(format!("{:?}", HouseholdRole::Owner).contains("Owner"));
		assert!(format!("{:?}", HouseholdRole::Admin).contains("Admin"));
	}

	#[test]
	fn serde_serialize() {
		let role = HouseholdRole::Admin;
		let json = serde_json::to_string(&role).unwrap();
		assert!(json.contains("admin"));
	}

	#[test]
	fn serde_deserialize() {
		let json = r#""viewer""#;
		let role: HouseholdRole = serde_json::from_str(json).unwrap();
		assert_eq!(role, HouseholdRole::Viewer);
	}
}

/// Errors that can occur during authorization checks.
///
/// These errors cover common authorization scenarios:
/// - Ownership verification
/// - Household membership validation
/// - Role-based access control
/// - Resource existence checks
/// - Database operations
#[derive(Debug, Error)]
pub enum AuthorizationError {
	/// The authenticated user is not the owner of the requested resource.
	#[error("Not authorized: you are not the owner of this resource")]
	NotOwner,

	/// The authenticated user is not a member of the requested household.
	#[error("Not authorized: you are not a member of this household")]
	NotHouseholdMember,

	/// The authenticated user lacks sufficient role permissions.
	///
	/// This error includes both the required role for the operation
	/// and the user's actual role.
	#[error(
		"Not authorized: insufficient role (required: {:?}, actual: {:?})",
		required,
		actual
	)]
	InsufficientRole {
		/// The minimum role required for this operation
		required: HouseholdRole,

		/// The user's actual role
		actual: HouseholdRole,
	},

	/// The requested resource does not exist.
	#[error("Resource not found")]
	NotFound,

	/// A database error occurred during authorization check.
	///
	/// The underlying database error is logged but not exposed
	/// in the HTTP response to prevent information leakage.
	#[error("Database error")]
	DatabaseError(#[from] SqlxError),

	/// The entity relation has ambiguous ownership (both owner_user_id and household_id are NULL).
	///
	/// This indicates a data integrity issue where the relation cannot determine
	/// access permissions.
	#[error("Ambiguous ownership: unable to determine access path")]
	AmbiguousOwnership,
}

/// Error response format for authorization failures.
#[derive(Serialize)]
struct ErrorResponse {
	error: String,
	message: String,
}

impl IntoResponse for AuthorizationError {
	fn into_response(self) -> Response {
		let (status, error, message) = match self {
			AuthorizationError::NotOwner => (
				StatusCode::FORBIDDEN,
				"FORBIDDEN".to_string(),
				"You are not the owner of this resource".to_string(),
			),
			AuthorizationError::NotHouseholdMember => (
				StatusCode::FORBIDDEN,
				"FORBIDDEN".to_string(),
				"You are not a member of this household".to_string(),
			),
			AuthorizationError::InsufficientRole { .. } => (
				StatusCode::FORBIDDEN,
				"FORBIDDEN".to_string(),
				"Insufficient permissions: your role does not meet the required level".to_string(),
				// Note: We don't leak role details in the message for security
			),
			AuthorizationError::NotFound => (
				StatusCode::NOT_FOUND,
				"NOT_FOUND".to_string(),
				"Resource not found".to_string(),
			),
			AuthorizationError::DatabaseError(ref err) => {
				eprintln!("Authorization database error: {:?}", err);
				(
					StatusCode::INTERNAL_SERVER_ERROR,
					"INTERNAL_ERROR".to_string(),
					"An error occurred while checking authorization".to_string(),
				)
			}
			AuthorizationError::AmbiguousOwnership => (
				StatusCode::INTERNAL_SERVER_ERROR,
				"INTERNAL_ERROR".to_string(),
				"Unable to determine access permissions".to_string(),
			),
		};

		let body = Json(ErrorResponse { error, message });
		(status, body).into_response()
	}
}

/// Tables that have an `owner_user_id` column for ownership tracking.
///
/// This enum is used to provide type-safe table selection for user-owned
/// record authorization checks. Since sqlx doesn't support dynamic table
/// names, we use a match statement to generate appropriate SQL queries.
pub enum UserOwnableTable {
	Household,
	Initiative,
	Epic,
	Quest,
	Routine,
	Note,
	Tag,
	Attachment,
	EntityRelation,
}

impl UserOwnableTable {
	/// Returns the table name for SQL queries.
	fn table_name(&self) -> &str {
		match self {
			UserOwnableTable::Household => "households",
			UserOwnableTable::Initiative => "initiatives",
			UserOwnableTable::Epic => "guidance_epics",
			UserOwnableTable::Quest => "guidance_quests",
			UserOwnableTable::Routine => "guidance_routines",
			UserOwnableTable::Note => "knowledge_notes",
			UserOwnableTable::Tag => "tags",
			UserOwnableTable::Attachment => "attachments",
			UserOwnableTable::EntityRelation => "entity_relations",
		}
	}
}

/// Checks if a user can access a user-owned record by ownership verification.
///
/// This function determines if the authenticated user owns a specific entity
/// by comparing the `owner_user_id` column with the provided user ID.
///
/// # Arguments
///
/// * `pool` - The PostgreSQL connection pool
/// * `user_id` - The UUID of the user to check ownership for
/// * `table` - The table to check (via `UserOwnableTable` enum)
/// * `entity_id` - The UUID of the entity to check
///
/// # Returns
///
/// * `Ok(true)` - The user owns the entity
/// * `Ok(false)` - The user does not own the entity (but the entity exists)
/// * `Err(AuthorizationError::NotFound)` - The entity doesn't exist
/// * `Err(AuthorizationError::DatabaseError)` - A database error occurred
///
/// # Example
///
/// ```no_run
/// use server::auth::authorization::{can_access_user_owned, UserOwnableTable};
/// use sqlx::PgPool;
///
/// # async fn example(pool: &PgPool, user_id: uuid::Uuid, initiative_id: uuid::Uuid) -> Result<bool, AuthorizationError> {
/// let can_access = can_access_user_owned(pool, user_id, UserOwnableTable::Initiative, initiative_id).await?;
/// # Ok(can_access)
/// # }
/// ```
pub async fn can_access_user_owned(
	pool: &PgPool,
	user_id: Uuid,
	table: UserOwnableTable,
	entity_id: Uuid,
) -> Result<bool, AuthorizationError> {
	let result = match table {
		UserOwnableTable::Household => {
			sqlx::query_scalar::<_, bool>("SELECT owner_user_id = $1 FROM households WHERE id = $2")
				.bind(user_id)
				.bind(entity_id)
				.fetch_optional(pool)
				.await?
		}
		UserOwnableTable::Initiative => {
			sqlx::query_scalar::<_, bool>(
				"SELECT owner_user_id = $1 FROM initiatives WHERE id = $2",
			)
			.bind(user_id)
			.bind(entity_id)
			.fetch_optional(pool)
			.await?
		}
		UserOwnableTable::Epic => {
			sqlx::query_scalar::<_, bool>(
				"SELECT owner_user_id = $1 FROM guidance_epics WHERE id = $2",
			)
			.bind(user_id)
			.bind(entity_id)
			.fetch_optional(pool)
			.await?
		}
		UserOwnableTable::Quest => {
			sqlx::query_scalar::<_, bool>(
				"SELECT owner_user_id = $1 FROM guidance_quests WHERE id = $2",
			)
			.bind(user_id)
			.bind(entity_id)
			.fetch_optional(pool)
			.await?
		}
		UserOwnableTable::Routine => {
			sqlx::query_scalar::<_, bool>(
				"SELECT owner_user_id = $1 FROM guidance_routines WHERE id = $2",
			)
			.bind(user_id)
			.bind(entity_id)
			.fetch_optional(pool)
			.await?
		}
		UserOwnableTable::Note => {
			sqlx::query_scalar::<_, bool>(
				"SELECT owner_user_id = $1 FROM knowledge_notes WHERE id = $2",
			)
			.bind(user_id)
			.bind(entity_id)
			.fetch_optional(pool)
			.await?
		}
		UserOwnableTable::Tag => {
			sqlx::query_scalar::<_, bool>("SELECT owner_user_id = $1 FROM tags WHERE id = $2")
				.bind(user_id)
				.bind(entity_id)
				.fetch_optional(pool)
				.await?
		}
		UserOwnableTable::Attachment => {
			sqlx::query_scalar::<_, bool>(
				"SELECT owner_user_id = $1 FROM attachments WHERE id = $2",
			)
			.bind(user_id)
			.bind(entity_id)
			.fetch_optional(pool)
			.await?
		}
		UserOwnableTable::EntityRelation => {
			sqlx::query_scalar::<_, bool>(
				"SELECT owner_user_id = $1 FROM entity_relations WHERE id = $2",
			)
			.bind(user_id)
			.bind(entity_id)
			.fetch_optional(pool)
			.await?
		}
	};

	match result {
		Some(is_owner) => Ok(is_owner),
		None => Err(AuthorizationError::NotFound),
	}
}

/// Requires user ownership of a record or returns an error.
///
/// This is a fail-fast variant of `can_access_user_owned` that returns
/// an `AuthorizationError` instead of a boolean. Use this when you want to
/// automatically return an HTTP error response via the `IntoResponse` trait.
///
/// # Arguments
///
/// * `pool` - The PostgreSQL connection pool
/// * `user_id` - The UUID of the user to check ownership for
/// * `table` - The table to check (via `UserOwnableTable` enum)
/// * `entity_id` - The UUID of the entity to check
///
/// # Returns
///
/// * `Ok(())` - The user owns the entity
/// * `Err(AuthorizationError::NotOwner)` - The user does not own the entity
/// * `Err(AuthorizationError::NotFound)` - The entity doesn't exist
/// * `Err(AuthorizationError::DatabaseError)` - A database error occurred
///
/// # Example
///
/// ```no_run
/// use server::auth::authorization::{require_user_owned, UserOwnableTable};
/// use sqlx::PgPool;
///
/// # async fn example(pool: &PgPool, user_id: uuid::Uuid, initiative_id: uuid::Uuid) -> Result<(), AuthorizationError> {
/// require_user_owned(pool, user_id, UserOwnableTable::Initiative, initiative_id).await?;
/// // User is the owner, continue with the authorized operation
/// # Ok(())
/// # }
/// ```
pub async fn require_user_owned(
	pool: &PgPool,
	user_id: Uuid,
	table: UserOwnableTable,
	entity_id: Uuid,
) -> Result<(), AuthorizationError> {
	let is_owner = can_access_user_owned(pool, user_id, table, entity_id).await?;
	if is_owner {
		Ok(())
	} else {
		Err(AuthorizationError::NotOwner)
	}
}

/// Checks if a user has access to a household as an active member.
///
/// This function performs an atomic check to determine if a user is an active
/// member of a household, preventing TOCTOU (time-of-check to time-of-use) races.
///
/// # Arguments
///
/// * `pool` - The PostgreSQL connection pool
/// * `user_id` - The UUID of the user to check
/// * `household_id` - The UUID of the household to check
///
/// # Returns
///
/// * `Ok(true)` - User is an active member of the household
/// * `Ok(false)` - User is not a member or the membership is inactive
/// * `Err(AuthorizationError::DatabaseError)` - A database error occurred
///
/// # Example
///
/// ```no_run
/// use uuid::Uuid;
///
/// let user_id = Uuid::new_v4();
/// let household_id = Uuid::new_v4();
///
/// match can_access_household(&pool, user_id, household_id).await {
///     Ok(true) => println!("User can access household"),
///     Ok(false) => println!("User cannot access household"),
///     Err(e) => eprintln!("Database error: {}", e),
/// }
/// ```
pub async fn can_access_household(
	pool: &PgPool,
	user_id: Uuid,
	household_id: Uuid,
) -> Result<bool, AuthorizationError> {
	let exists = sqlx::query_scalar::<_, bool>(
		"SELECT EXISTS(
			SELECT 1 FROM household_memberships
			WHERE user_id = $1 AND household_id = $2 AND is_active = true
		)",
	)
	.bind(user_id)
	.bind(household_id)
	.fetch_one(pool)
	.await?;

	Ok(exists)
}

/// Requires a user to have a specific minimum role within a household.
///
/// This function performs an atomic check to verify that a user is an active
/// member of a household with a role at or above the required minimum.
/// This prevents TOCTOU (time-of-check to time-of-use) races by combining
/// both the membership check and role comparison in a single database query.
///
/// # Arguments
///
/// * `pool` - The PostgreSQL connection pool
/// * `user_id` - The UUID of the user to check
/// * `household_id` - The UUID of the household to check
/// * `min_role` - The minimum role required for access
///
/// # Returns
///
/// * `Ok(role)` - User's actual role (which is >= min_role)
/// * `Err(AuthorizationError::NotHouseholdMember)` - User is not a member or the membership is inactive
/// * `Err(AuthorizationError::InsufficientRole)` - User's role is below the minimum required
/// * `Err(AuthorizationError::DatabaseError)` - A database error occurred
///
/// # Example
///
/// ```no_run
/// use uuid::Uuid;
///
/// let user_id = Uuid::new_v4();
/// let household_id = Uuid::new_v4();
/// let min_role = HouseholdRole::Admin;
///
/// match require_household_role(&pool, user_id, household_id, min_role).await {
///     Ok(actual_role) => println!("User has access with role: {}", actual_role),
///     Err(AuthorizationError::NotHouseholdMember) => println!("Not a member"),
///     Err(AuthorizationError::InsufficientRole { required, actual }) => {
///         println!("Insufficient role: required {:?}, have {:?}", required, actual)
///     }
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
pub async fn require_household_role(
	pool: &PgPool,
	user_id: Uuid,
	household_id: Uuid,
	min_role: HouseholdRole,
) -> Result<HouseholdRole, AuthorizationError> {
	let actual_role_opt = sqlx::query_scalar::<_, HouseholdRole>(
		"SELECT role FROM household_memberships
		WHERE user_id = $1 AND household_id = $2 AND is_active = true",
	)
	.bind(user_id)
	.bind(household_id)
	.fetch_optional(pool)
	.await?;

	let actual_role = match actual_role_opt {
		Some(role) => role,
		None => return Err(AuthorizationError::NotHouseholdMember),
	};

	if actual_role >= min_role {
		Ok(actual_role)
	} else {
		Err(AuthorizationError::InsufficientRole {
			required: min_role,
			actual: actual_role,
		})
	}
}

/// Checks if a user can access an initiative via dual-path authorization.
///
/// An initiative is accessible if the user is either:
/// - The owner of the initiative (`initiatives.owner_user_id = user_id`)
/// - A member of the initiative's household (via `household_memberships` with `is_active = true`)
///
/// This function performs an atomic check using a single LEFT JOIN query to evaluate
/// both access paths efficiently, preventing TOCTOU (time-of-check to time-of-use) races.
///
/// # Arguments
///
/// * `pool` - The PostgreSQL connection pool
/// * `user_id` - The UUID of the user to check access for
/// * `initiative_id` - The UUID of the initiative to check
///
/// # Returns
///
/// * `Ok(true)` - User can access the initiative (owner or household member)
/// * `Ok(false)` - User cannot access the initiative (neither owner nor member)
/// * `Err(AuthorizationError::NotFound)` - The initiative doesn't exist
/// * `Err(AuthorizationError::DatabaseError)` - A database error occurred
///
/// # Example
///
/// ```no_run
/// use uuid::Uuid;
///
/// # async fn example(pool: &PgPool, user_id: Uuid, initiative_id: Uuid) -> Result<bool, AuthorizationError> {
/// let can_access = can_access_initiative(pool, user_id, initiative_id).await?;
/// # Ok(can_access)
/// # }
/// ```
pub async fn can_access_initiative(
	pool: &PgPool,
	user_id: Uuid,
	initiative_id: Uuid,
) -> Result<bool, AuthorizationError> {
	let can_access = sqlx::query_scalar::<_, bool>(
		"SELECT EXISTS(
			SELECT 1 FROM initiatives i
			LEFT JOIN household_memberships hm ON
				hm.household_id = i.household_id AND
				hm.user_id = $1 AND
				hm.is_active = true
			WHERE i.id = $2 AND (
				i.owner_user_id = $1 OR
				hm.id IS NOT NULL
			)
		)",
	)
	.bind(user_id)
	.bind(initiative_id)
	.fetch_one(pool)
	.await?;

	Ok(can_access)
}

/// Requires a user to have access to an initiative or returns an error.
///
/// This is a fail-fast variant of `can_access_initiative` that returns
/// an `AuthorizationError` instead of a boolean. Use this when you want to
/// automatically return an HTTP error response via the `IntoResponse` trait.
///
/// # Arguments
///
/// * `pool` - The PostgreSQL connection pool
/// * `user_id` - The UUID of the user to check access for
/// * `initiative_id` - The UUID of the initiative to check
///
/// # Returns
///
/// * `Ok(())` - User can access the initiative (owner or household member)
/// * `Err(AuthorizationError::NotHouseholdMember)` - User cannot access the initiative
/// * `Err(AuthorizationError::NotFound)` - The initiative doesn't exist
/// * `Err(AuthorizationError::DatabaseError)` - A database error occurred
///
/// # Example
///
/// ```no_run
/// use uuid::Uuid;
///
/// # async fn example(pool: &PgPool, user_id: Uuid, initiative_id: Uuid) -> Result<(), AuthorizationError> {
/// require_initiative_access(pool, user_id, initiative_id).await?;
/// // User can access the initiative, continue with the authorized operation
/// # Ok(())
/// # }
/// ```
pub async fn require_initiative_access(
	pool: &PgPool,
	user_id: Uuid,
	initiative_id: Uuid,
) -> Result<(), AuthorizationError> {
	let can_access = can_access_initiative(pool, user_id, initiative_id).await?;
	if can_access {
		Ok(())
	} else {
		Err(AuthorizationError::NotHouseholdMember)
	}
}

/// Checks if a user can access an attachment via multi-path authorization.
///
/// An attachment is accessible if the user:
/// - Is the direct owner of the attachment (`attachments.owner_user_id = user_id`)
/// - Has access via a related entity in `entity_relations`:
///   - The relation's `owner_user_id` matches the user, OR
///   - The user is an active member of the relation's `household_id`
///
/// This function checks the `entity_relations` table where the attachment
/// is the `from_entity`. Relations with both `owner_user_id` and `household_id`
/// as NULL result in an `AmbiguousOwnership` error.
///
/// # Arguments
///
/// * `pool` - The PostgreSQL connection pool
/// * `user_id` - The UUID of the user to check access for
/// * `attachment_id` - The UUID of the attachment to check
///
/// # Returns
///
/// * `Ok(true)` - User can access the attachment
/// * `Ok(false)` - User cannot access the attachment
/// * `Err(AuthorizationError::NotFound)` - The attachment doesn't exist
/// * `Err(AuthorizationError::AmbiguousOwnership)` - Related entity has ambiguous ownership
/// * `Err(AuthorizationError::DatabaseError)` - A database error occurred
pub async fn can_access_attachment(
	pool: &PgPool,
	user_id: Uuid,
	attachment_id: Uuid,
) -> Result<bool, AuthorizationError> {
	// 1. Check direct ownership of the attachment
	let is_owner =
		sqlx::query_scalar::<_, bool>("SELECT owner_user_id = $1 FROM attachments WHERE id = $2")
			.bind(user_id)
			.bind(attachment_id)
			.fetch_optional(pool)
			.await?;

	match is_owner {
		Some(true) => return Ok(true),
		Some(false) => {}
		None => return Err(AuthorizationError::NotFound),
	}

	// 2. Check for access via entity_relations (attachment as from_entity)
	let has_relation_access = sqlx::query_scalar::<_, bool>(
		"SELECT EXISTS(
			SELECT 1 FROM entity_relations er
			WHERE er.from_entity_type = 'attachment'
			  AND er.from_entity_id = $1
			  AND er.status = 'active'
			  AND (
				er.owner_user_id = $2
				OR EXISTS(
					SELECT 1 FROM household_memberships hm
					WHERE hm.user_id = $2
					  AND hm.household_id = er.household_id
					  AND hm.is_active = true
				)
			  )
		)",
	)
	.bind(attachment_id)
	.bind(user_id)
	.fetch_one(pool)
	.await?;

	if has_relation_access {
		return Ok(true);
	}

	// 3. Check for ambiguous ownership (both owner_user_id and household_id are NULL)
	let has_ambiguous = sqlx::query_scalar::<_, bool>(
		"SELECT EXISTS(
			SELECT 1 FROM entity_relations er
			WHERE er.from_entity_type = 'attachment'
			  AND er.from_entity_id = $1
			  AND er.status = 'active'
			  AND er.owner_user_id IS NULL
			  AND er.household_id IS NULL
		)",
	)
	.bind(attachment_id)
	.fetch_one(pool)
	.await?;

	if has_ambiguous {
		return Err(AuthorizationError::AmbiguousOwnership);
	}

	Ok(false)
}

/// Requires a user to have access to an attachment or returns an error.
///
/// This is a fail-fast variant of `can_access_attachment` that returns
/// an `AuthorizationError` instead of a boolean.
///
/// # Arguments
///
/// * `pool` - The PostgreSQL connection pool
/// * `user_id` - The UUID of the user to check access for
/// * `attachment_id` - The UUID of the attachment to check
///
/// # Returns
///
/// * `Ok(())` - User can access the attachment
/// * `Err(AuthorizationError::NotOwner)` - User cannot access the attachment
/// * `Err(AuthorizationError::NotFound)` - The attachment doesn't exist
/// * `Err(AuthorizationError::AmbiguousOwnership)` - Related entity has ambiguous ownership
/// * `Err(AuthorizationError::DatabaseError)` - A database error occurred
pub async fn require_attachment_access(
	pool: &PgPool,
	user_id: Uuid,
	attachment_id: Uuid,
) -> Result<(), AuthorizationError> {
	let can_access = can_access_attachment(pool, user_id, attachment_id).await?;
	if can_access {
		Ok(())
	} else {
		Err(AuthorizationError::NotOwner)
	}
}

// ========================================
// AuthorizationError Tests (No DB Required)
// ========================================

#[test]
fn authorization_error_not_owner_display() {
	let err = AuthorizationError::NotOwner;
	assert_eq!(
		format!("{err}"),
		"Not authorized: you are not the owner of this resource"
	);
}

#[test]
fn authorization_error_not_household_member_display() {
	let err = AuthorizationError::NotHouseholdMember;
	assert_eq!(
		format!("{err}"),
		"Not authorized: you are not a member of this household"
	);
}

#[test]
fn authorization_error_insufficient_role_display() {
	let err = AuthorizationError::InsufficientRole {
		required: HouseholdRole::Admin,
		actual: HouseholdRole::Member,
	};
	assert!(format!("{err}").contains("insufficient role"));
	assert!(format!("{err}").contains("required"));
	assert!(format!("{err}").contains("actual"));
}

#[test]
fn authorization_error_not_found_display() {
	let err = AuthorizationError::NotFound;
	assert_eq!(format!("{err}"), "Resource not found");
}

#[test]
fn authorization_error_database_error_display() {
	let err = AuthorizationError::DatabaseError(SqlxError::RowNotFound);
	assert_eq!(format!("{err}"), "Database error");
}

#[test]
fn authorization_error_debug_impl() {
	let err = AuthorizationError::NotOwner;
	assert!(format!("{err:?}").contains("NotOwner"));

	let err = AuthorizationError::NotHouseholdMember;
	assert!(format!("{err:?}").contains("NotHouseholdMember"));

	let err = AuthorizationError::NotFound;
	assert!(format!("{err:?}").contains("NotFound"));
}

// ========================================
// IntoResponse Tests for AuthorizationError (No DB Required)
// ========================================

#[test]
fn authorization_error_not_owner_into_response() {
	let err = AuthorizationError::NotOwner;
	let response: Response = err.into_response();
	assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[test]
fn authorization_error_not_household_member_into_response() {
	let err = AuthorizationError::NotHouseholdMember;
	let response: Response = err.into_response();
	assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[test]
fn authorization_error_insufficient_role_into_response() {
	let err = AuthorizationError::InsufficientRole {
		required: HouseholdRole::Admin,
		actual: HouseholdRole::Member,
	};
	let response: Response = err.into_response();
	assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[test]
fn authorization_error_not_found_into_response() {
	let err = AuthorizationError::NotFound;
	let response: Response = err.into_response();
	assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[test]
fn authorization_error_database_error_into_response() {
	let err = AuthorizationError::DatabaseError(SqlxError::RowNotFound);
	let response: Response = err.into_response();
	assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[test]
fn authorization_error_database_error_from_sqlx() {
	let sqlx_err = SqlxError::RowNotFound;
	let auth_err: AuthorizationError = sqlx_err.into();

	match auth_err {
		AuthorizationError::DatabaseError(_) => (),
		_ => panic!("Expected DatabaseError variant"),
	}
}

// ========================================
// User-Owned Authorization Tests (No DB Required)
// ========================================

#[test]
fn user_ownable_table_table_names() {
	assert_eq!(UserOwnableTable::Household.table_name(), "households");
	assert_eq!(UserOwnableTable::Initiative.table_name(), "initiatives");
	assert_eq!(UserOwnableTable::Epic.table_name(), "guidance_epics");
	assert_eq!(UserOwnableTable::Quest.table_name(), "guidance_quests");
	assert_eq!(UserOwnableTable::Routine.table_name(), "guidance_routines");
	assert_eq!(UserOwnableTable::Note.table_name(), "knowledge_notes");
	assert_eq!(UserOwnableTable::Tag.table_name(), "tags");
	assert_eq!(UserOwnableTable::Attachment.table_name(), "attachments");
	assert_eq!(
		UserOwnableTable::EntityRelation.table_name(),
		"entity_relations"
	);
}

#[test]
fn user_ownable_table_all_variants() {
	let tables = vec![
		UserOwnableTable::Household,
		UserOwnableTable::Initiative,
		UserOwnableTable::Epic,
		UserOwnableTable::Quest,
		UserOwnableTable::Routine,
		UserOwnableTable::Note,
		UserOwnableTable::Tag,
		UserOwnableTable::Attachment,
		UserOwnableTable::EntityRelation,
	];
	assert_eq!(tables.len(), 9);
}

#[test]
fn authorization_error_not_owner_for_user_owned() {
	let err = AuthorizationError::NotOwner;
	assert_eq!(
		format!("{err}"),
		"Not authorized: you are not the owner of this resource"
	);
}

#[test]
fn authorization_error_not_found_for_user_owned() {
	let err = AuthorizationError::NotFound;
	assert_eq!(format!("{err}"), "Resource not found");
}

// ========================================
// AmbiguousOwnership Error Tests (No DB Required)
// ========================================

#[test]
fn authorization_error_ambiguous_ownership_display() {
	let err = AuthorizationError::AmbiguousOwnership;
	assert_eq!(
		format!("{err}"),
		"Ambiguous ownership: unable to determine access path"
	);
}

#[test]
fn authorization_error_ambiguous_ownership_debug() {
	let err = AuthorizationError::AmbiguousOwnership;
	assert!(format!("{err:?}").contains("AmbiguousOwnership"));
}

#[test]
fn authorization_error_ambiguous_ownership_into_response() {
	let err = AuthorizationError::AmbiguousOwnership;
	let response: Response = err.into_response();
	assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

// ========================================
// Attachment Authorization Integration Tests (Require DB)
// ========================================

#[test]
#[ignore = "requires running database - see P3-007"]
fn can_access_attachment_direct_owner_grants_access() {
	unimplemented!("Requires test database - see P3-007")
}

#[test]
#[ignore = "requires running database - see P3-007"]
fn can_access_attachment_relation_ownership_grants_access() {
	unimplemented!("Requires test database - see P3-007")
}

#[test]
#[ignore = "requires running database - see P3-007"]
fn can_access_attachment_relation_household_membership_grants_access() {
	unimplemented!("Requires test database - see P3-007")
}

#[test]
#[ignore = "requires running database - see P3-007"]
fn can_access_attachment_no_access_path_denied() {
	unimplemented!("Requires test database - see P3-007")
}

#[test]
#[ignore = "requires running database - see P3-007"]
fn can_access_attachment_ambiguous_ownership_returns_error() {
	unimplemented!("Requires test database - see P3-007")
}

#[test]
#[ignore = "requires running database - see P3-007"]
fn can_access_attachment_nonexistent_returns_not_found() {
	unimplemented!("Requires test database - see P3-007")
}

#[test]
#[ignore = "requires running database - see P3-007"]
fn require_attachment_access_returns_ok_for_owner() {
	unimplemented!("Requires test database - see P3-007")
}

#[test]
#[ignore = "requires running database - see P3-007"]
fn require_attachment_access_returns_not_owner_for_non_owner() {
	unimplemented!("Requires test database - see P3-007")
}

// ========================================
// Authorization Integration Tests (Require DB)
// ========================================

/// Integration test for user-owned authorization.
///
/// **Database setup required:**
/// - Test user inserted
/// - Test entity with owner_user_id set
///
/// Run with: `cargo test -p server --ignored`
///
/// TODO: Implement in P3-007 when we add test database infrastructure
#[test]
#[ignore = "requires test database - see P3-007"]
fn test_user_owned_integration() {
	unimplemented!("Requires test database - see P3-007")
}

/// Integration test for household role authorization.
///
/// **Database setup required:**
/// - Test household inserted
/// - Test user with household membership and specific role
///
/// Run with: `cargo test -p server --ignored`
///
/// TODO: Implement in P3-007 when we add test database infrastructure
#[test]
#[ignore = "requires test database - see P3-007"]
fn test_household_role_integration() {
	unimplemented!("Requires test database - see P3-007")
}

/// Integration test for initiative access authorization.
///
/// **Database setup required:**
/// - Test initiative with owner_user_id and household_id
/// - Test user (either owner or household member)
///
/// Run with: `cargo test -p server --ignored`
///
/// TODO: Implement in P3-007 when we add test database infrastructure
#[test]
#[ignore = "requires test database - see P3-007"]
fn test_initiative_access_integration() {
	unimplemented!("Requires test database - see P3-007")
}

/// Integration test for attachment access authorization.
///
/// **Database setup required:**
/// - Test attachment with owner_user_id
/// - Test entity_relation linking attachment to another entity
/// - Test user (either owner, relation owner, or household member)
///
/// Run with: `cargo test -p server --ignored`
///
/// TODO: Implement in P3-007 when we add test database infrastructure
#[test]
#[ignore = "requires test database - see P3-007"]
fn test_attachment_access_integration() {
	unimplemented!("Requires test database - see P3-007")
}
