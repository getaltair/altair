// Common test utilities and fixtures
//
// This module provides shared test infrastructure for integration tests:
// - MIGRATOR: Embedded database migrations for test databases
// - Helper functions for test assertions and setup

use sqlx::PgPool;
use sqlx::migrate::Migrator;

/// Embedded database migrations.
///
/// This static embeds all SQL migrations at compile time using sqlx::migrate!().
/// Use with `#[sqlx::test(migrator = "common::MIGRATOR")]` in integration tests
/// to automatically run migrations on a fresh test database.
///
/// # Example
///
/// ```rust,ignore
/// use sqlx::PgPool;
///
/// #[sqlx::test(migrator = "common::MIGRATOR")]
/// async fn test_users_table_exists(pool: PgPool) {
///     // pool has all migrations applied
///     let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
///         .fetch_one(&pool)
///         .await
///         .unwrap();
///     assert_eq!(row.0, 0); // empty table
/// }
/// ```
pub static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

/// Verify that all expected tables exist after migrations.
///
/// Returns a vector of missing table names, empty if all tables exist.
pub async fn verify_tables_exist(pool: &PgPool) -> Result<Vec<String>, sqlx::Error> {
	let required_tables = [
		"users",
		"households",
		"household_memberships",
		"initiatives",
		"guidance_focus_sessions",
		"notes",
		"locations",
		"categories",
		"items",
		"tags",
		"attachments",
		"entity_relations",
	];

	let existing_tables: Vec<String> = sqlx::query_scalar(
		r#"
        SELECT table_name
        FROM information_schema.tables
        WHERE table_schema = 'public'
        AND table_type = 'BASE TABLE'
        "#,
	)
	.fetch_all(pool)
	.await?;

	let missing: Vec<String> = required_tables
		.iter()
		.filter(|table| !existing_tables.contains(&table.to_string()))
		.map(|s| s.to_string())
		.collect();

	Ok(missing)
}

/// Verify that all expected enum types exist after migrations.
///
/// Returns a vector of missing enum names, empty if all enums exist.
pub async fn verify_enums_exist(pool: &PgPool) -> Result<Vec<String>, sqlx::Error> {
	let required_enums = [
		"entity_type",
		"relation_type",
		"source_type",
		"relation_status",
		"initiative_status",
		"quest_status",
		"routine_recurrence",
		"routine_status",
		"focus_session_status",
		"note_visibility",
		"location_type",
		"item_status",
		"tag_type",
		"household_role",
	];

	let existing_enums: Vec<String> = sqlx::query_scalar(
		r#"
        SELECT typname
        FROM pg_type
        WHERE typtype = 'e'
        AND typnamespace = (SELECT oid FROM pg_namespace WHERE nspname = 'public')
        "#,
	)
	.fetch_all(pool)
	.await?;

	let missing: Vec<String> = required_enums
		.iter()
		.filter(|enum_name| !existing_enums.contains(&enum_name.to_string()))
		.map(|s| s.to_string())
		.collect();

	Ok(missing)
}

// ============================================================================
// Test Application Infrastructure
// ============================================================================

use axum::{Router, routing::get};
use chrono::{Duration, Utc};
use uuid::Uuid;

use server::attachments;
use server::auth;
use server::core;
use server::guidance;
use server::handlers;
use server::knowledge;
use server::search;
use server::sync;
use server::tracking;

/// Test application wrapper that provides access to the router
/// with `ServiceExt` for making test requests.
///
/// # Example
///
/// ```rust,ignore
/// use tower::ServiceExt;
/// use http_body_util::Full;
/// use bytes::Bytes;
///
/// let app = create_test_app(pool);
///
/// // Use oneshot for single request testing
/// let response = app
///     .oneshot(Request::builder()
///         .uri("/health")
///         .body(Full::new(Bytes::empty()))
///         .unwrap())
///     .await
///     .unwrap();
/// ```
pub struct TestApp(Router);

impl TestApp {
	/// Create a new test app from a router.
	pub fn new(router: Router) -> Self {
		Self(router)
	}

	/// Get a reference to the inner router.
	#[allow(dead_code)]
	pub fn inner(&self) -> &Router {
		&self.0
	}

	/// Convert into the inner router.
	pub fn into_inner(self) -> Router {
		self.0
	}
}

/// Create a test application with all routes mounted.
///
/// This mirrors the router setup in `main.rs` but without:
/// - TraceLayer (not needed for tests)
/// - Graceful shutdown handling
///
/// # Arguments
///
/// * `pool` - PostgreSQL connection pool to use as app state
///
/// # Returns
///
/// A `TestApp` that can be used to make test requests via `ServiceExt`.
///
/// # Example
///
/// ```rust,ignore
/// let app = create_test_app(pool);
/// let response = app.oneshot(request).await.unwrap();
/// ```
pub fn create_test_app(pool: PgPool) -> TestApp {
	let router = Router::new()
		.route("/health", get(handlers::health::health_check))
		.route("/users/me", get(handlers::users::me))
		.nest("/auth", auth::router())
		.nest("/core", core::handlers::router())
		.nest("/guidance", guidance::router())
		.nest("/knowledge", knowledge::router())
		.nest("/tracking", tracking::router())
		.nest("/attachments", attachments::router())
		.nest("/sync", sync::router())
		.nest("/search", search::router())
		.with_state(pool);

	TestApp::new(router)
}

// ============================================================================
// Test Data Factory Functions
// ============================================================================

/// Create a test user in the `users` table.
///
/// Inserts a user with:
/// - Unique UUID
/// - Unique email (using UUID for uniqueness)
/// - Display name "Test User"
/// - Default timezone "America/Chicago"
/// - `is_active = true`
///
/// # Arguments
///
/// * `pool` - PostgreSQL connection pool
///
/// # Returns
///
/// The UUID of the created user.
///
/// # Example
///
/// ```rust,ignore
/// let user_id = create_test_user(&pool).await;
/// assert!(user_id != Uuid::nil());
/// ```
pub async fn create_test_user(pool: &PgPool) -> Uuid {
	let user_id = Uuid::new_v4();
	let email = format!("test-{}@example.com", user_id);
	let now = Utc::now();

	sqlx::query(
		r#"
		INSERT INTO users (id, email, display_name, timezone, is_active, created_at, updated_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7)
		"#,
	)
	.bind(user_id)
	.bind(&email)
	.bind("Test User")
	.bind("America/Chicago")
	.bind(true)
	.bind(now)
	.bind(now)
	.execute(pool)
	.await
	.expect("Failed to create test user");

	user_id
}

/// Create a test session in Better-Auth's `session` table.
///
/// This inserts a session directly into the database, bypassing
/// the normal authentication flow. Useful for testing authenticated
/// endpoints without going through login.
///
/// # Arguments
///
/// * `pool` - PostgreSQL connection pool
/// * `user_id` - UUID of the user to associate the session with
///
/// # Returns
///
/// The session token string that can be used in the `Cookie` header.
///
/// # Example
///
/// ```rust,ignore
/// let user_id = create_test_user(&pool).await;
/// let token = create_test_session(&pool, user_id).await;
///
/// // Use in request:
/// // Cookie: session=<token>
/// ```
pub async fn create_test_session(pool: &PgPool, user_id: Uuid) -> String {
	let session_id = Uuid::new_v4();
	let token = format!("test-session-token-{}", Uuid::new_v4());
	let now = Utc::now();
	let expires_at = now + Duration::hours(1);

	sqlx::query(
		r#"
		INSERT INTO session (id, expires_at, token, created_at, updated_at, user_id)
		VALUES ($1, $2, $3, $4, $5, $6)
		"#,
	)
	.bind(session_id)
	.bind(expires_at)
	.bind(&token)
	.bind(now)
	.bind(now)
	.bind(user_id)
	.execute(pool)
	.await
	.expect("Failed to create test session");

	token
}

/// Create an expired test session in Better-Auth's `session` table.
///
/// This inserts a session with `expires_at` set to 1 hour in the past,
/// useful for testing expired session handling.
///
/// # Arguments
///
/// * `pool` - PostgreSQL connection pool
/// * `user_id` - UUID of the user to associate the session with
///
/// # Returns
///
/// The session token string that can be used in the `Cookie` header.
#[allow(dead_code)]
pub async fn create_expired_test_session(pool: &PgPool, user_id: Uuid) -> String {
	let session_id = Uuid::new_v4();
	let token = format!("test-session-token-{}", Uuid::new_v4());
	let now = Utc::now();
	let expires_at = now - Duration::hours(1); // Expired 1 hour ago

	sqlx::query(
		r#"
		INSERT INTO session (id, expires_at, token, created_at, updated_at, user_id)
		VALUES ($1, $2, $3, $4, $5, $6)
		"#,
	)
	.bind(session_id)
	.bind(expires_at)
	.bind(&token)
	.bind(now)
	.bind(now)
	.bind(user_id)
	.execute(pool)
	.await
	.expect("Failed to create expired test session");

	token
}

/// Create a test household in the `households` table.
///
/// # Arguments
///
/// * `pool` - PostgreSQL connection pool
/// * `owner_id` - UUID of the user who owns the household
///
/// # Returns
///
/// The UUID of the created household.
///
/// # Example
///
/// ```rust,ignore
/// let user_id = create_test_user(&pool).await;
/// let household_id = create_test_household(&pool, user_id).await;
/// ```
pub async fn create_test_household(pool: &PgPool, owner_id: Uuid) -> Uuid {
	let household_id = Uuid::new_v4();
	let name = format!("Test Household {}", household_id);
	let slug = format!("test-household-{}", household_id);
	let now = Utc::now();

	sqlx::query(
		r#"
		INSERT INTO households (id, owner_user_id, name, slug, description, created_at, updated_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7)
		"#,
	)
	.bind(household_id)
	.bind(owner_id)
	.bind(&name)
	.bind(&slug)
	.bind("A test household for integration tests")
	.bind(now)
	.bind(now)
	.execute(pool)
	.await
	.expect("Failed to create test household");

	household_id
}

/// Create a test initiative in the `initiatives` table.
///
/// # Arguments
///
/// * `pool` - PostgreSQL connection pool
/// * `owner_id` - UUID of the user who owns the initiative
/// * `household_id` - Optional UUID of the household to associate
///
/// # Returns
///
/// The UUID of the created initiative.
///
/// # Example
///
/// ```rust,ignore
/// let user_id = create_test_user(&pool).await;
/// let initiative_id = create_test_initiative(&pool, user_id, None).await;
/// ```
pub async fn create_test_initiative(
	pool: &PgPool,
	owner_id: Uuid,
	household_id: Option<Uuid>,
) -> Uuid {
	let initiative_id = Uuid::new_v4();
	let title = format!("Test Initiative {}", initiative_id);
	let slug = format!("test-initiative-{}", initiative_id);
	let now = Utc::now();

	sqlx::query(
		r#"
		INSERT INTO initiatives (id, owner_user_id, household_id, title, slug, description, status, created_at, updated_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7::initiative_status, $8, $9)
		"#,
	)
	.bind(initiative_id)
	.bind(owner_id)
	.bind(household_id)
	.bind(&title)
	.bind(&slug)
	.bind("A test initiative for integration tests")
	.bind("active")
	.bind(now)
	.bind(now)
	.execute(pool)
	.await
	.expect("Failed to create test initiative");

	initiative_id
}

/// Create a test relation in the `entity_relations` table.
///
/// Creates a relation between two entities with default values:
/// - `relation_type`: "relates_to"
/// - `source_type`: "manual"
/// - `status`: "active"
///
/// # Arguments
///
/// * `pool` - PostgreSQL connection pool
/// * `owner_id` - UUID of the user who owns the relation
/// * `from_id` - UUID of the source entity
/// * `to_id` - UUID of the target entity
///
/// # Returns
///
/// The UUID of the created relation.
///
/// # Example
///
/// ```rust,ignore
/// let user_id = create_test_user(&pool).await;
/// let initiative1 = create_test_initiative(&pool, user_id, None).await;
/// let initiative2 = create_test_initiative(&pool, user_id, None).await;
/// let relation_id = create_test_relation(&pool, user_id, initiative1, initiative2).await;
/// ```
pub async fn create_test_relation(
	pool: &PgPool,
	owner_id: Uuid,
	from_id: Uuid,
	to_id: Uuid,
) -> Uuid {
	let relation_id = Uuid::new_v4();
	let now = Utc::now();

	sqlx::query(
		r#"
		INSERT INTO entity_relations (
			id,
			from_entity_type,
			from_entity_id,
			to_entity_type,
			to_entity_id,
			relation_type,
			source_type,
			status,
			owner_user_id,
			created_at,
			updated_at
		)
		VALUES ($1, $2::entity_type_v2, $3, $4::entity_type_v2, $5, $6::relation_type_v2, $7::source_type_v2, $8::relation_status_v2, $9, $10, $11)
		"#,
	)
	.bind(relation_id)
	.bind("initiative")
	.bind(from_id)
	.bind("initiative")
	.bind(to_id)
	.bind("related_to")
	.bind("user")
	.bind("accepted")
	.bind(owner_id)
	.bind(now)
	.bind(now)
	.execute(pool)
	.await
	.expect("Failed to create test relation");

	relation_id
}

#[cfg(test)]
mod tests {
	use super::*;

	/// Test that the MIGRATOR can be used with sqlx::test attribute.
	/// This verifies:
	/// - All migrations run successfully
	/// - Test database receives all schema changes
	/// - Core tables are created
	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_migrator_creates_all_tables(pool: PgPool) {
		// Verify all expected tables exist
		let missing_tables = verify_tables_exist(&pool).await.unwrap();
		assert!(
			missing_tables.is_empty(),
			"Missing tables after migration: {:?}",
			missing_tables
		);
	}

	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_migrator_creates_all_enums(pool: PgPool) {
		// Verify all expected enum types exist
		let missing_enums = verify_enums_exist(&pool).await.unwrap();
		assert!(
			missing_enums.is_empty(),
			"Missing enums after migration: {:?}",
			missing_enums
		);
	}

	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_migrator_users_table_schema(pool: PgPool) {
		// Verify users table has expected columns
		let column_count: i64 = sqlx::query_scalar(
			r#"
            SELECT COUNT(*)
            FROM information_schema.columns
            WHERE table_schema = 'public'
            AND table_name = 'users'
            "#,
		)
		.fetch_one(&pool)
		.await
		.unwrap();

		// users table should have: id, email, display_name, timezone, is_active,
		// created_at, updated_at, deleted_at = 8 columns
		assert_eq!(column_count, 8, "users table should have 8 columns");
	}

	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_migrator_entity_relations_schema(pool: PgPool) {
		// Verify entity_relations table exists with correct structure
		let exists: bool = sqlx::query_scalar(
			r#"
            SELECT EXISTS (
                SELECT 1 FROM information_schema.tables
                WHERE table_schema = 'public'
                AND table_name = 'entity_relations'
            )
            "#,
		)
		.fetch_one(&pool)
		.await
		.unwrap();

		assert!(exists, "entity_relations table should exist");
	}

	// ========================================
	// Factory Function Tests
	// ========================================

	#[test]
	fn test_app_can_be_created() {
		// This test verifies the TestApp struct can be instantiated
		// (Router construction without pool is not possible, so we just test the type)
		let _ = std::mem::size_of::<TestApp>();
	}

	#[test]
	fn test_user_id_generation() {
		let id = Uuid::new_v4();
		assert!(!id.is_nil());
	}

	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_create_test_user(pool: PgPool) {
		let user_id = create_test_user(&pool).await;

		// Verify user was created with correct data
		let row: (String, String, bool) = sqlx::query_as(
			r#"
            SELECT email, display_name, is_active
            FROM users
            WHERE id = $1
            "#,
		)
		.bind(user_id)
		.fetch_one(&pool)
		.await
		.unwrap();

		assert!(row.0.contains("@example.com"));
		assert_eq!(row.1, "Test User");
		assert!(row.2);
	}

	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_create_test_household(pool: PgPool) {
		let user_id = create_test_user(&pool).await;
		let household_id = create_test_household(&pool, user_id).await;

		// Verify household was created
		let row: (String, Uuid) = sqlx::query_as(
			r#"
            SELECT name, owner_user_id
            FROM households
            WHERE id = $1
            "#,
		)
		.bind(household_id)
		.fetch_one(&pool)
		.await
		.unwrap();

		assert!(row.0.contains("Test Household"));
		assert_eq!(row.1, user_id);
	}

	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_create_test_initiative(pool: PgPool) {
		let user_id = create_test_user(&pool).await;
		let initiative_id = create_test_initiative(&pool, user_id, None).await;

		// Verify initiative was created
		let row: (String, String) = sqlx::query_as(
			r#"
            SELECT title, status::text
            FROM initiatives
            WHERE id = $1
            "#,
		)
		.bind(initiative_id)
		.fetch_one(&pool)
		.await
		.unwrap();

		assert!(row.0.contains("Test Initiative"));
		assert_eq!(row.1, "active");
	}

	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_create_test_initiative_with_household(pool: PgPool) {
		let user_id = create_test_user(&pool).await;
		let household_id = create_test_household(&pool, user_id).await;
		let initiative_id = create_test_initiative(&pool, user_id, Some(household_id)).await;

		// Verify initiative was created with household
		let row: (String, Option<Uuid>) = sqlx::query_as(
			r#"
            SELECT title, household_id
            FROM initiatives
            WHERE id = $1
            "#,
		)
		.bind(initiative_id)
		.fetch_one(&pool)
		.await
		.unwrap();

		assert!(row.0.contains("Test Initiative"));
		assert_eq!(row.1, Some(household_id));
	}

	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_create_test_relation(pool: PgPool) {
		let user_id = create_test_user(&pool).await;
		let initiative1 = create_test_initiative(&pool, user_id, None).await;
		let initiative2 = create_test_initiative(&pool, user_id, None).await;
		let relation_id = create_test_relation(&pool, user_id, initiative1, initiative2).await;

		// Verify relation was created
		let row: (String, String, String) = sqlx::query_as(
			r#"
            SELECT relation_type::text, source_type::text, status::text
            FROM entity_relations
            WHERE id = $1
            "#,
		)
		.bind(relation_id)
		.fetch_one(&pool)
		.await
		.unwrap();

		assert_eq!(row.0, "related_to");
		assert_eq!(row.1, "user");
		assert_eq!(row.2, "accepted");
	}
}
