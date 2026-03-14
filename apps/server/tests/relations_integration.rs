mod common;

#[cfg(test)]
mod tests {
	use crate::common::{
		MIGRATOR, create_test_app, create_test_initiative, create_test_relation,
		create_test_session, create_test_user,
	};
	use axum::http::{Request, StatusCode, header};
	use http_body_util::Full;
	use serde_json::{Value, json};
	use sqlx::PgPool;
	use tower::ServiceExt;
	use uuid::Uuid;

	/// Test: POST /core/relations without authentication returns 401
	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_create_relation_requires_auth(pool: PgPool) {
		let app = create_test_app(pool).into_inner();

		let response = app
			.oneshot(
				Request::builder()
					.method("POST")
					.uri("/core/relations")
					.header(header::CONTENT_TYPE, "application/json")
					.body(Full::new(
						json!({
							"from_entity_type": "initiative",
							"from_entity_id": Uuid::new_v4(),
							"to_entity_type": "initiative",
							"to_entity_id": Uuid::new_v4(),
							"relation_type": "references"
						})
						.to_string()
						.into_bytes()
						.into(),
					))
					.unwrap(),
			)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
	}

	/// Test: POST /core/relations with valid data returns 201
	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_create_relation_with_valid_data(pool: PgPool) {
		let user_id = create_test_user(&pool).await;
		let token = create_test_session(&pool, user_id).await;
		let initiative1 = create_test_initiative(&pool, user_id, None).await;
		let initiative2 = create_test_initiative(&pool, user_id, None).await;
		let app = create_test_app(pool).into_inner();

		let response = app
			.oneshot(
				Request::builder()
					.method("POST")
					.uri("/core/relations")
					.header(header::COOKIE, format!("better-auth.session_token={token}"))
					.header(header::CONTENT_TYPE, "application/json")
					.body(Full::new(
						json!({
							"from_entity_type": "initiative",
							"from_entity_id": initiative1,
							"to_entity_type": "initiative",
							"to_entity_id": initiative2,
							"relation_type": "references",
							"source_type": "ai"
						})
						.to_string()
						.into_bytes()
						.into(),
					))
					.unwrap(),
			)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::CREATED);

		let body = axum::body::to_bytes(response.into_body(), 1024 * 1024)
			.await
			.unwrap();
		let relation: Value = serde_json::from_slice(&body).unwrap();

		// Verify relation_type
		assert_eq!(relation["relation_type"], "references");
		// source_type "ai" means status should be "suggested" (not "user")
		assert_eq!(relation["source_type"], "ai");
		assert_eq!(relation["status"], "suggested");
		assert_eq!(relation["from_entity_id"], initiative1.to_string());
		assert_eq!(relation["to_entity_id"], initiative2.to_string());
	}

	/// Test: GET /core/relations?from_entity_id=X returns relations
	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_list_relations_by_from_entity(pool: PgPool) {
		let user_id = create_test_user(&pool).await;
		let token = create_test_session(&pool, user_id).await;
		let initiative1 = create_test_initiative(&pool, user_id, None).await;
		let initiative2 = create_test_initiative(&pool, user_id, None).await;
		let _relation_id = create_test_relation(&pool, user_id, initiative1, initiative2).await;
		let app = create_test_app(pool).into_inner();

		let response = app
			.oneshot(
				Request::builder()
					.uri(format!("/core/relations?from_entity_id={initiative1}"))
					.header(header::COOKIE, format!("better-auth.session_token={token}"))
					.body(Full::new(bytes::Bytes::new()))
					.unwrap(),
			)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::OK);

		let body = axum::body::to_bytes(response.into_body(), 1024 * 1024)
			.await
			.unwrap();
		let relations: Value = serde_json::from_slice(&body).unwrap();

		// Should have exactly 1 relation
		let relations_array = relations.as_array().expect("Response should be an array");
		assert_eq!(relations_array.len(), 1);
		assert_eq!(
			relations_array[0]["from_entity_id"],
			initiative1.to_string()
		);
		assert_eq!(relations_array[0]["to_entity_id"], initiative2.to_string());
	}

	/// Test: DELETE /core/relations/{id} by owner returns 204
	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_owner_can_delete_relation(pool: PgPool) {
		let user_id = create_test_user(&pool).await;
		let token = create_test_session(&pool, user_id).await;
		let initiative1 = create_test_initiative(&pool, user_id, None).await;
		let initiative2 = create_test_initiative(&pool, user_id, None).await;
		let relation_id = create_test_relation(&pool, user_id, initiative1, initiative2).await;
		let app = create_test_app(pool).into_inner();

		let response = app
			.oneshot(
				Request::builder()
					.method("DELETE")
					.uri(format!("/core/relations/{relation_id}"))
					.header(header::COOKIE, format!("better-auth.session_token={token}"))
					.body(Full::new(bytes::Bytes::new()))
					.unwrap(),
			)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::NO_CONTENT);
	}

	/// Test: DELETE /core/relations/{id} by non-owner returns 403
	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_non_owner_cannot_delete_relation(pool: PgPool) {
		// Create owner user and relation
		let owner_id = create_test_user(&pool).await;
		let initiative1 = create_test_initiative(&pool, owner_id, None).await;
		let initiative2 = create_test_initiative(&pool, owner_id, None).await;
		let relation_id = create_test_relation(&pool, owner_id, initiative1, initiative2).await;

		// Create a different user with their own session
		let other_user_id = create_test_user(&pool).await;
		let other_token = create_test_session(&pool, other_user_id).await;
		let app = create_test_app(pool).into_inner();

		let response = app
			.oneshot(
				Request::builder()
					.method("DELETE")
					.uri(format!("/core/relations/{relation_id}"))
					.header(
						header::COOKIE,
						format!("better-auth.session_token={other_token}"),
					)
					.body(Full::new(bytes::Bytes::new()))
					.unwrap(),
			)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::FORBIDDEN);
	}

	/// Test: PATCH /core/relations/{id}/status by owner returns 200
	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_owner_can_update_status(pool: PgPool) {
		let user_id = create_test_user(&pool).await;
		let token = create_test_session(&pool, user_id).await;
		let initiative1 = create_test_initiative(&pool, user_id, None).await;
		let initiative2 = create_test_initiative(&pool, user_id, None).await;
		let relation_id = create_test_relation(&pool, user_id, initiative1, initiative2).await;
		let app = create_test_app(pool).into_inner();

		let response = app
			.oneshot(
				Request::builder()
					.method("PATCH")
					.uri(format!("/core/relations/{relation_id}/status"))
					.header(header::COOKIE, format!("better-auth.session_token={token}"))
					.header(header::CONTENT_TYPE, "application/json")
					.body(Full::new(
						json!({"status": "accepted"})
							.to_string()
							.into_bytes()
							.into(),
					))
					.unwrap(),
			)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::OK);

		let body = axum::body::to_bytes(response.into_body(), 1024 * 1024)
			.await
			.unwrap();
		let relation: Value = serde_json::from_slice(&body).unwrap();

		assert_eq!(relation["status"], "accepted");
	}

	/// Test: POST /core/relations with duplicate returns 409 Conflict
	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_create_duplicate_relation_returns_409(pool: PgPool) {
		let user_id = create_test_user(&pool).await;
		let token = create_test_session(&pool, user_id).await;
		let initiative1 = create_test_initiative(&pool, user_id, None).await;
		let initiative2 = create_test_initiative(&pool, user_id, None).await;

		// Create first relation
		let _relation_id = create_test_relation(&pool, user_id, initiative1, initiative2).await;

		let app = create_test_app(pool).into_inner();

		// Try to create duplicate (same entities, same relation type)
		let response = app
			.oneshot(
				Request::builder()
					.method("POST")
					.uri("/core/relations")
					.header(header::COOKIE, format!("better-auth.session_token={token}"))
					.header(header::CONTENT_TYPE, "application/json")
					.body(Full::new(
						json!({
							"from_entity_type": "initiative",
							"from_entity_id": initiative1,
							"to_entity_type": "initiative",
							"to_entity_id": initiative2,
							"relation_type": "related_to"
						})
						.to_string()
						.into_bytes()
						.into(),
					))
					.unwrap(),
			)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::CONFLICT);
	}

	/// Test: POST /core/relations with source_type "user" sets status to "accepted"
	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_create_relation_with_user_source_sets_accepted_status(pool: PgPool) {
		let user_id = create_test_user(&pool).await;
		let token = create_test_session(&pool, user_id).await;
		let initiative1 = create_test_initiative(&pool, user_id, None).await;
		let initiative2 = create_test_initiative(&pool, user_id, None).await;
		let app = create_test_app(pool).into_inner();

		let response = app
			.oneshot(
				Request::builder()
					.method("POST")
					.uri("/core/relations")
					.header(header::COOKIE, format!("better-auth.session_token={token}"))
					.header(header::CONTENT_TYPE, "application/json")
					.body(Full::new(
						json!({
							"from_entity_type": "initiative",
							"from_entity_id": initiative1,
							"to_entity_type": "initiative",
							"to_entity_id": initiative2,
							"relation_type": "supports",
							"source_type": "user"  // Explicit user source
						})
						.to_string()
						.into_bytes()
						.into(),
					))
					.unwrap(),
			)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::CREATED);

		let body = axum::body::to_bytes(response.into_body(), 1024 * 1024)
			.await
			.unwrap();
		let relation: Value = serde_json::from_slice(&body).unwrap();

		// source_type "user" should set status to "accepted"
		assert_eq!(relation["source_type"], "user");
		assert_eq!(relation["status"], "accepted");
	}

	/// Test: POST /core/relations with invalid relation_type returns 400
	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_create_relation_with_invalid_relation_type(pool: PgPool) {
		let user_id = create_test_user(&pool).await;
		let token = create_test_session(&pool, user_id).await;
		let initiative1 = create_test_initiative(&pool, user_id, None).await;
		let initiative2 = create_test_initiative(&pool, user_id, None).await;
		let app = create_test_app(pool).into_inner();

		let response = app
			.oneshot(
				Request::builder()
					.method("POST")
					.uri("/core/relations")
					.header(header::COOKIE, format!("better-auth.session_token={token}"))
					.header(header::CONTENT_TYPE, "application/json")
					.body(Full::new(
						json!({
							"from_entity_type": "initiative",
							"from_entity_id": initiative1,
							"to_entity_type": "initiative",
							"to_entity_id": initiative2,
							"relation_type": "invalid_relation_type"
						})
						.to_string()
						.into_bytes()
						.into(),
					))
					.unwrap(),
			)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::BAD_REQUEST);
	}

	/// Test: GET /core/relations without query params returns 400
	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_list_relations_without_query_params(pool: PgPool) {
		let user_id = create_test_user(&pool).await;
		let token = create_test_session(&pool, user_id).await;
		let app = create_test_app(pool).into_inner();

		let response = app
			.oneshot(
				Request::builder()
					.uri("/core/relations")
					.header(header::COOKIE, format!("better-auth.session_token={token}"))
					.body(Full::new(bytes::Bytes::new()))
					.unwrap(),
			)
			.await
			.unwrap();

		// Requires either from_entity_id or to_entity_id
		assert_eq!(response.status(), StatusCode::BAD_REQUEST);
	}
}
