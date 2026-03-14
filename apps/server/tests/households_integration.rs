mod common;

#[cfg(test)]
mod tests {
	use crate::common::{MIGRATOR, create_test_app, create_test_session, create_test_user};
	use axum::http::{Request, StatusCode, header};
	use bytes::Bytes;
	use chrono::Utc;
	use http_body_util::Full;
	use serde::Deserialize;
	use sqlx::PgPool;
	use tower::ServiceExt;
	use uuid::Uuid;

	/// Household response struct for deserializing JSON responses.
	/// Matches the Household struct from handlers::households.
	#[derive(Debug, Clone, Deserialize)]
	#[allow(dead_code)]
	struct Household {
		id: Uuid,
		owner_user_id: Uuid,
		name: String,
		#[allow(dead_code)]
		slug: Option<String>,
		description: Option<String>,
		#[allow(dead_code)]
		created_at: chrono::DateTime<Utc>,
		#[allow(dead_code)]
		updated_at: chrono::DateTime<Utc>,
		#[allow(dead_code)]
		deleted_at: Option<chrono::DateTime<Utc>>,
	}

	/// Test that household owner can access their household.
	///
	/// Creates a household, verifies owner gets 200 on GET /core/households/{id}.
	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_household_owner_can_access(pool: PgPool) {
		// Create owner user and session
		let owner_id = create_test_user(&pool).await;
		let token = create_test_session(&pool, owner_id).await;

		// Create household (also creates owner membership via handler)
		let app = create_test_app(pool.clone()).into_inner();
		let create_response = app
			.oneshot(
				Request::builder()
					.method("POST")
					.uri("/core/households")
					.header(header::CONTENT_TYPE, "application/json")
					.header(header::COOKIE, format!("better-auth.session_token={token}"))
					.body(Full::new(Bytes::from(
						serde_json::json!({
							"name": "Test Household",
							"slug": "test-household",
							"description": "A test household"
						})
						.to_string()
						.into_bytes(),
					)))
					.unwrap(),
			)
			.await
			.unwrap();

		assert_eq!(create_response.status(), StatusCode::CREATED);
		let body = axum::body::to_bytes(create_response.into_body(), 1024 * 1024)
			.await
			.unwrap();
		let household: Household = serde_json::from_slice(&body).unwrap();

		// Now test GET access as owner
		let app = create_test_app(pool.clone()).into_inner();
		let response = app
			.oneshot(
				Request::builder()
					.uri(format!("/core/households/{}", household.id))
					.header(header::COOKIE, format!("better-auth.session_token={token}"))
					.body(Full::new(Bytes::new()))
					.unwrap(),
			)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::OK);

		let body = axum::body::to_bytes(response.into_body(), 1024 * 1024)
			.await
			.unwrap();
		let fetched: Household = serde_json::from_slice(&body).unwrap();
		assert_eq!(fetched.id, household.id);
		assert_eq!(fetched.owner_user_id, owner_id);
	}

	/// Test that household members (non-owners) can access the household.
	///
	/// Creates a household with owner, adds a member, verifies member gets 200 on GET.
	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_household_member_can_access(pool: PgPool) {
		// Create owner user
		let owner_id = create_test_user(&pool).await;
		let owner_token = create_test_session(&pool, owner_id).await;

		// Create household via API (to get owner membership)
		let app = create_test_app(pool.clone()).into_inner();
		let create_response = app
			.oneshot(
				Request::builder()
					.method("POST")
					.uri("/core/households")
					.header(header::CONTENT_TYPE, "application/json")
					.header(
						header::COOKIE,
						format!("better-auth.session_token={owner_token}"),
					)
					.body(Full::new(Bytes::from(
						serde_json::json!({
							"name": "Member Test Household",
							"slug": "member-test-household"
						})
						.to_string()
						.into_bytes(),
					)))
					.unwrap(),
			)
			.await
			.unwrap();

		let body = axum::body::to_bytes(create_response.into_body(), 1024 * 1024)
			.await
			.unwrap();
		let household: Household = serde_json::from_slice(&body).unwrap();
		let household_id = household.id;

		// Create member user
		let member_id = create_test_user(&pool).await;
		let member_token = create_test_session(&pool, member_id).await;

		// Insert membership directly for member
		sqlx::query(
			r#"
			INSERT INTO household_memberships (id, household_id, user_id, role, is_active, created_at, updated_at)
			VALUES ($1, $2, $3, 'member', true, NOW(), NOW())
			"#,
		)
		.bind(Uuid::new_v4())
		.bind(household_id)
		.bind(member_id)
		.execute(&pool)
		.await
		.expect("Failed to create membership");

		// Test GET access as member (not owner)
		let app = create_test_app(pool).into_inner();
		let response = app
			.oneshot(
				Request::builder()
					.uri(format!("/core/households/{household_id}"))
					.header(
						header::COOKIE,
						format!("better-auth.session_token={member_token}"),
					)
					.body(Full::new(Bytes::new()))
					.unwrap(),
			)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::OK);
	}

	/// Test that non-members cannot access a household.
	///
	/// Creates a household with owner, creates a different user,
	/// verifies that user gets 403 on GET.
	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_non_member_cannot_access(pool: PgPool) {
		// Create owner and household
		let owner_id = create_test_user(&pool).await;
		let owner_token = create_test_session(&pool, owner_id).await;

		let app = create_test_app(pool.clone()).into_inner();
		let create_response = app
			.oneshot(
				Request::builder()
					.method("POST")
					.uri("/core/households")
					.header(header::CONTENT_TYPE, "application/json")
					.header(
						header::COOKIE,
						format!("better-auth.session_token={owner_token}"),
					)
					.body(Full::new(Bytes::from(
						serde_json::json!({
							"name": "Private Household",
							"slug": "private-household"
						})
						.to_string()
						.into_bytes(),
					)))
					.unwrap(),
			)
			.await
			.unwrap();

		let body = axum::body::to_bytes(create_response.into_body(), 1024 * 1024)
			.await
			.unwrap();
		let household: Household = serde_json::from_slice(&body).unwrap();
		let household_id = household.id;

		// Create a different user (not a member)
		let non_member_id = create_test_user(&pool).await;
		let non_member_token = create_test_session(&pool, non_member_id).await;

		// Test GET access as non-member
		let app = create_test_app(pool).into_inner();
		let response = app
			.oneshot(
				Request::builder()
					.uri(format!("/core/households/{household_id}"))
					.header(
						header::COOKIE,
						format!("better-auth.session_token={non_member_token}"),
					)
					.body(Full::new(Bytes::new()))
					.unwrap(),
			)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::FORBIDDEN);
	}

	/// Test that household owner can update the household.
	///
	/// Owner PATCH /core/households/{id} → 200.
	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_owner_can_update(pool: PgPool) {
		// Create owner and household
		let owner_id = create_test_user(&pool).await;
		let owner_token = create_test_session(&pool, owner_id).await;

		let app = create_test_app(pool.clone()).into_inner();
		let create_response = app
			.oneshot(
				Request::builder()
					.method("POST")
					.uri("/core/households")
					.header(header::CONTENT_TYPE, "application/json")
					.header(
						header::COOKIE,
						format!("better-auth.session_token={owner_token}"),
					)
					.body(Full::new(Bytes::from(
						serde_json::json!({
							"name": "Original Name",
							"slug": "original-slug"
						})
						.to_string()
						.into_bytes(),
					)))
					.unwrap(),
			)
			.await
			.unwrap();

		let body = axum::body::to_bytes(create_response.into_body(), 1024 * 1024)
			.await
			.unwrap();
		let household: Household = serde_json::from_slice(&body).unwrap();
		let household_id = household.id;

		// Test PATCH as owner
		let app = create_test_app(pool).into_inner();
		let response = app
			.oneshot(
				Request::builder()
					.method("PATCH")
					.uri(format!("/core/households/{household_id}"))
					.header(header::CONTENT_TYPE, "application/json")
					.header(
						header::COOKIE,
						format!("better-auth.session_token={owner_token}"),
					)
					.body(Full::new(Bytes::from(
						serde_json::json!({
							"name": "Updated Name",
							"description": "New description"
						})
						.to_string()
						.into_bytes(),
					)))
					.unwrap(),
			)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::OK);

		let body = axum::body::to_bytes(response.into_body(), 1024 * 1024)
			.await
			.unwrap();
		let updated: Household = serde_json::from_slice(&body).unwrap();
		assert_eq!(updated.name, "Updated Name");
		assert_eq!(updated.description, Some("New description".to_string()));
	}

	/// Test that non-owner members cannot update the household.
	///
	/// Member PATCH → 403 (only owner can update).
	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_member_cannot_update(pool: PgPool) {
		// Create owner and household
		let owner_id = create_test_user(&pool).await;
		let owner_token = create_test_session(&pool, owner_id).await;

		let app = create_test_app(pool.clone()).into_inner();
		let create_response = app
			.oneshot(
				Request::builder()
					.method("POST")
					.uri("/core/households")
					.header(header::CONTENT_TYPE, "application/json")
					.header(
						header::COOKIE,
						format!("better-auth.session_token={owner_token}"),
					)
					.body(Full::new(Bytes::from(
						serde_json::json!({
							"name": "Owner's Household",
							"slug": "owners-household"
						})
						.to_string()
						.into_bytes(),
					)))
					.unwrap(),
			)
			.await
			.unwrap();

		let body = axum::body::to_bytes(create_response.into_body(), 1024 * 1024)
			.await
			.unwrap();
		let household: Household = serde_json::from_slice(&body).unwrap();
		let household_id = household.id;

		// Create member user
		let member_id = create_test_user(&pool).await;
		let member_token = create_test_session(&pool, member_id).await;

		// Insert membership for member
		sqlx::query(
			r#"
			INSERT INTO household_memberships (id, household_id, user_id, role, is_active, created_at, updated_at)
			VALUES ($1, $2, $3, 'member', true, NOW(), NOW())
			"#,
		)
		.bind(Uuid::new_v4())
		.bind(household_id)
		.bind(member_id)
		.execute(&pool)
		.await
		.expect("Failed to create membership");

		// Test PATCH as member (should fail - only owner can update)
		let app = create_test_app(pool).into_inner();
		let response = app
			.oneshot(
				Request::builder()
					.method("PATCH")
					.uri(format!("/core/households/{household_id}"))
					.header(header::CONTENT_TYPE, "application/json")
					.header(
						header::COOKIE,
						format!("better-auth.session_token={member_token}"),
					)
					.body(Full::new(Bytes::from(
						serde_json::json!({
							"name": "Hacked Name"
						})
						.to_string()
						.into_bytes(),
					)))
					.unwrap(),
			)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::FORBIDDEN);
	}

	/// Test that list returns only households where user is owner or member.
	///
	/// Creates 2 households (one where user is member, one not),
	/// verifies list returns only the member's household.
	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_list_includes_member_households(pool: PgPool) {
		// Create user who will be the member
		let member_id = create_test_user(&pool).await;
		let member_token = create_test_session(&pool, member_id).await;

		// Create another user who will own a household (not shared with member)
		let other_owner_id = create_test_user(&pool).await;
		let other_owner_token = create_test_session(&pool, other_owner_id).await;

		// Create household #1 where member IS a member
		let app = create_test_app(pool.clone()).into_inner();
		let create_response1 = app
			.oneshot(
				Request::builder()
					.method("POST")
					.uri("/core/households")
					.header(header::CONTENT_TYPE, "application/json")
					.header(
						header::COOKIE,
						format!("better-auth.session_token={member_token}"),
					)
					.body(Full::new(Bytes::from(
						serde_json::json!({
							"name": "Member's Household",
							"slug": "members-household"
						})
						.to_string()
						.into_bytes(),
					)))
					.unwrap(),
			)
			.await
			.unwrap();

		let body = axum::body::to_bytes(create_response1.into_body(), 1024 * 1024)
			.await
			.unwrap();
		let household1: Household = serde_json::from_slice(&body).unwrap();

		// Create household #2 where member is NOT a member
		let app = create_test_app(pool.clone()).into_inner();
		let _create_response2 = app
			.oneshot(
				Request::builder()
					.method("POST")
					.uri("/core/households")
					.header(header::CONTENT_TYPE, "application/json")
					.header(
						header::COOKIE,
						format!("better-auth.session_token={other_owner_token}"),
					)
					.body(Full::new(Bytes::from(
						serde_json::json!({
							"name": "Other's Household",
							"slug": "others-household"
						})
						.to_string()
						.into_bytes(),
					)))
					.unwrap(),
			)
			.await
			.unwrap();

		// List households as member
		let app = create_test_app(pool).into_inner();
		let response = app
			.oneshot(
				Request::builder()
					.uri("/core/households")
					.header(
						header::COOKIE,
						format!("better-auth.session_token={member_token}"),
					)
					.body(Full::new(Bytes::new()))
					.unwrap(),
			)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::OK);

		let body = axum::body::to_bytes(response.into_body(), 1024 * 1024)
			.await
			.unwrap();
		let households: Vec<Household> = serde_json::from_slice(&body).unwrap();

		// Should only see household #1 (member's own household)
		assert_eq!(households.len(), 1, "Should only see 1 household");
		assert_eq!(households[0].id, household1.id);
		assert_eq!(households[0].name, "Member's Household");
	}
}
