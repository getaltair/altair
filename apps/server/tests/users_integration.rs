mod common;

#[cfg(test)]
mod tests {
	use crate::common::{MIGRATOR, create_test_app, create_test_session, create_test_user};
	use axum::http::{Request, StatusCode, header};
	use bytes::Bytes;
	use http_body_util::Full;
	use server::handlers::users::AppUser;
	use sqlx::PgPool;
	use tower::ServiceExt;

	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_users_me_returns_authenticated_user(pool: PgPool) {
		let user_id = create_test_user(&pool).await;
		let token = create_test_session(&pool, user_id).await;
		let app = create_test_app(pool).into_inner();

		let response = app
			.oneshot(
				Request::builder()
					.uri("/users/me")
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
		let app_user: AppUser = serde_json::from_slice(&body).unwrap();

		assert_eq!(app_user.id, user_id);
		assert!(app_user.email.contains("@example.com"));
		assert_eq!(app_user.display_name, "Test User");
		assert_eq!(app_user.timezone, "America/Chicago");
		assert!(app_user.is_active);
	}

	#[sqlx::test(migrator = "MIGRATOR")]
	async fn test_users_me_without_auth_returns_401(pool: PgPool) {
		let app = create_test_app(pool).into_inner();

		let response = app
			.oneshot(
				Request::builder()
					.uri("/users/me")
					.body(Full::new(Bytes::new()))
					.unwrap(),
			)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
	}
}
