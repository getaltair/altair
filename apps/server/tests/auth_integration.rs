mod common;

#[cfg(test)]
mod tests {
	use crate::common::{
		MIGRATOR, create_expired_test_session, create_test_app, create_test_session,
		create_test_user,
	};
	use axum::http::{Request, StatusCode, header};
	use http_body_util::Full;
	use serde_json::Value;
	use sqlx::PgPool;
	use tower::ServiceExt;

	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_valid_session_returns_user(pool: PgPool) {
		let user_id = create_test_user(&pool).await;
		let token = create_test_session(&pool, user_id).await;
		let app = create_test_app(pool).into_inner();

		let response = app
			.oneshot(
				Request::builder()
					.uri("/auth/me")
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
		let user: Value = serde_json::from_slice(&body).unwrap();

		assert_eq!(user["id"], user_id.to_string());
		assert!(user["email"].as_str().unwrap().contains("@example.com"));
	}

	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_invalid_session_returns_401(pool: PgPool) {
		let user_id = create_test_user(&pool).await;
		let _token = create_test_session(&pool, user_id).await;
		let app = create_test_app(pool).into_inner();

		let response = app
			.oneshot(
				Request::builder()
					.uri("/auth/me")
					.header(
						header::COOKIE,
						"better-auth.session_token=invalid-token-never-exists",
					)
					.body(Full::new(bytes::Bytes::new()))
					.unwrap(),
			)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
	}

	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_expired_session_returns_401(pool: PgPool) {
		let user_id = create_test_user(&pool).await;
		let token = create_expired_test_session(&pool, user_id).await;
		let app = create_test_app(pool).into_inner();

		let response = app
			.oneshot(
				Request::builder()
					.uri("/auth/me")
					.header(header::COOKIE, format!("better-auth.session_token={token}"))
					.body(Full::new(bytes::Bytes::new()))
					.unwrap(),
			)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
	}

	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_missing_cookie_returns_401(pool: PgPool) {
		let user_id = create_test_user(&pool).await;
		let _token = create_test_session(&pool, user_id).await;
		let app = create_test_app(pool).into_inner();

		let response = app
			.oneshot(
				Request::builder()
					.uri("/auth/me")
					.body(Full::new(bytes::Bytes::new()))
					.unwrap(),
			)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
	}
}
