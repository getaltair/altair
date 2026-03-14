mod common;

#[cfg(test)]
mod tests {
	use crate::common::{
		MIGRATOR, create_test_app, create_test_household, create_test_initiative,
		create_test_session, create_test_user,
	};
	use axum::http::{Request, StatusCode, header};
	use http_body_util::Full;
	use serde_json::Value;
	use sqlx::PgPool;
	use tower::ServiceExt;
	use uuid::Uuid;

	async fn create_household_membership(pool: &PgPool, household_id: Uuid, user_id: Uuid) {
		sqlx::query(
			r#"
			INSERT INTO household_memberships (id, household_id, user_id, role, is_active, created_at, updated_at)
			VALUES ($1, $2, $3, 'member', true, NOW(), NOW())
			"#,
		)
		.bind(Uuid::new_v4())
		.bind(household_id)
		.bind(user_id)
		.execute(pool)
		.await
		.expect("Failed to create household membership");
	}

	async fn soft_delete_initiative(pool: &PgPool, initiative_id: Uuid) {
		sqlx::query("UPDATE initiatives SET deleted_at = NOW() WHERE id = $1")
			.bind(initiative_id)
			.execute(pool)
			.await
			.expect("Failed to soft-delete initiative");
	}

	/// Given: User owns an initiative
	/// When: Owner requests GET /core/initiatives/{id}
	/// Then: Returns 200 with initiative data
	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_initiative_owner_can_access(pool: PgPool) {
		let owner_id = create_test_user(&pool).await;
		let token = create_test_session(&pool, owner_id).await;
		let initiative_id = create_test_initiative(&pool, owner_id, None).await;

		let app = create_test_app(pool).into_inner();

		let response = app
			.oneshot(
				Request::builder()
					.uri(format!("/core/initiatives/{initiative_id}"))
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
		let initiative: Value = serde_json::from_slice(&body).unwrap();

		assert_eq!(initiative["id"], initiative_id.to_string());
		assert_eq!(initiative["owner_user_id"], owner_id.to_string());
	}

	/// Given: Initiative is in a household, user is a household member
	/// When: Member requests GET /core/initiatives/{id}
	/// Then: Returns 200 with initiative data (dual-path: household membership)
	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_initiative_household_member_can_access(pool: PgPool) {
		let household_owner_id = create_test_user(&pool).await;
		let household_id = create_test_household(&pool, household_owner_id).await;

		let member_id = create_test_user(&pool).await;
		let member_token = create_test_session(&pool, member_id).await;
		create_household_membership(&pool, household_id, member_id).await;

		let initiative_id =
			create_test_initiative(&pool, household_owner_id, Some(household_id)).await;

		let app = create_test_app(pool).into_inner();

		let response = app
			.oneshot(
				Request::builder()
					.uri(format!("/core/initiatives/{initiative_id}"))
					.header(
						header::COOKIE,
						format!("better-auth.session_token={member_token}"),
					)
					.body(Full::new(bytes::Bytes::new()))
					.unwrap(),
			)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::OK);

		let body = axum::body::to_bytes(response.into_body(), 1024 * 1024)
			.await
			.unwrap();
		let initiative: Value = serde_json::from_slice(&body).unwrap();

		assert_eq!(initiative["id"], initiative_id.to_string());
		assert_eq!(initiative["household_id"], household_id.to_string());
	}

	/// Given: User is not owner and not a household member
	/// When: User requests GET /core/initiatives/{id}
	/// Then: Returns 403 Forbidden
	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_non_member_cannot_access_initiative(pool: PgPool) {
		let owner_id = create_test_user(&pool).await;
		let initiative_id = create_test_initiative(&pool, owner_id, None).await;

		let other_user_id = create_test_user(&pool).await;
		let other_token = create_test_session(&pool, other_user_id).await;

		let app = create_test_app(pool).into_inner();

		let response = app
			.oneshot(
				Request::builder()
					.uri(format!("/core/initiatives/{initiative_id}"))
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

	/// Given: 3 initiatives exist (owned, household, invisible)
	/// When: User requests GET /core/initiatives
	/// Then: Returns only owned and household initiatives (2 total)
	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_list_includes_visible_initiatives(pool: PgPool) {
		let user_id = create_test_user(&pool).await;
		let token = create_test_session(&pool, user_id).await;

		let household_owner_id = create_test_user(&pool).await;
		let household_id = create_test_household(&pool, household_owner_id).await;
		create_household_membership(&pool, household_id, user_id).await;

		let owned_initiative_id = create_test_initiative(&pool, user_id, None).await;
		let household_initiative_id =
			create_test_initiative(&pool, household_owner_id, Some(household_id)).await;

		let other_owner_id = create_test_user(&pool).await;
		let _invisible_initiative_id = create_test_initiative(&pool, other_owner_id, None).await;

		let app = create_test_app(pool).into_inner();

		let response = app
			.oneshot(
				Request::builder()
					.uri("/core/initiatives")
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
		let initiatives: Value = serde_json::from_slice(&body).unwrap();

		let initiatives_arr = initiatives.as_array().expect("Expected JSON array");
		assert_eq!(initiatives_arr.len(), 2);

		let ids: Vec<String> = initiatives_arr
			.iter()
			.map(|i| i["id"].as_str().unwrap().to_string())
			.collect();

		assert!(ids.contains(&owned_initiative_id.to_string()));
		assert!(ids.contains(&household_initiative_id.to_string()));
	}

	/// Given: 2 initiatives, one soft-deleted
	/// When: User requests GET /core/initiatives
	/// Then: Returns only non-deleted initiative
	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_soft_delete_filtering(pool: PgPool) {
		let user_id = create_test_user(&pool).await;
		let token = create_test_session(&pool, user_id).await;

		let active_initiative_id = create_test_initiative(&pool, user_id, None).await;
		let deleted_initiative_id = create_test_initiative(&pool, user_id, None).await;

		soft_delete_initiative(&pool, deleted_initiative_id).await;

		let app = create_test_app(pool).into_inner();

		let response = app
			.oneshot(
				Request::builder()
					.uri("/core/initiatives")
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
		let initiatives: Value = serde_json::from_slice(&body).unwrap();

		let initiatives_arr = initiatives.as_array().expect("Expected JSON array");
		assert_eq!(initiatives_arr.len(), 1);
		assert_eq!(
			initiatives_arr[0]["id"].as_str().unwrap(),
			active_initiative_id.to_string()
		);
	}

	/// Given: Initiative is soft-deleted, user is owner
	/// When: Owner requests GET /core/initiatives/{id}
	/// Then: Returns 403 Forbidden (deleted_at filter blocks access)
	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_soft_deleted_initiative_returns_403(pool: PgPool) {
		let owner_id = create_test_user(&pool).await;
		let token = create_test_session(&pool, owner_id).await;
		let initiative_id = create_test_initiative(&pool, owner_id, None).await;

		soft_delete_initiative(&pool, initiative_id).await;

		let app = create_test_app(pool).into_inner();

		let response = app
			.oneshot(
				Request::builder()
					.uri(format!("/core/initiatives/{initiative_id}"))
					.header(header::COOKIE, format!("better-auth.session_token={token}"))
					.body(Full::new(bytes::Bytes::new()))
					.unwrap(),
			)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::FORBIDDEN);
	}
}
