use std::future::Future;

use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use jsonwebtoken::{Algorithm, Validation, decode};

use super::models::{AuthUser, Claims};
use crate::AppState;
use crate::error::AppError;

impl<S> FromRequestParts<S> for AuthUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        let app_state = AppState::from_ref(state);
        let headers = parts.headers.clone();

        async move {
            // 1. Try Authorization: Bearer <token> header
            let token = if let Some(auth_header) = headers.get("Authorization") {
                let val = auth_header.to_str().map_err(|_| AppError::Unauthorized)?;
                if let Some(token) = val.strip_prefix("Bearer ") {
                    token.to_string()
                } else {
                    return Err(AppError::Unauthorized);
                }
            } else if let Some(cookie_header) = headers.get("Cookie") {
                // 2. Fall back to access_token cookie
                // P4-027: string-parsing delegated to the shared `super::parse_cookie_value`.
                let cookies = cookie_header.to_str().map_err(|_| AppError::Unauthorized)?;
                super::parse_cookie_value(cookies, "access_token").ok_or(AppError::Unauthorized)?
            } else {
                return Err(AppError::Unauthorized);
            };

            let mut validation = Validation::new(Algorithm::RS256);
            validation.set_required_spec_claims(&["sub", "exp"]);

            // P4-004: match on error kind so non-expiry failures (algorithm confusion,
            // wrong deployment key, malformed token) are logged at warn level before
            // returning 401. ExpiredSignature is expected and logged at debug level only.
            let token_data = decode::<Claims>(&token, &app_state.dec_key, &validation)
                .map_err(|e| {
                    use jsonwebtoken::errors::ErrorKind;
                    match e.kind() {
                        ErrorKind::ExpiredSignature => {
                            tracing::debug!("JWT rejected: token expired");
                        }
                        kind => {
                            tracing::warn!(error_kind = ?kind, "JWT decode failed — possible key mismatch or malformed token");
                        }
                    }
                    AppError::Unauthorized
                })?;

            Ok(AuthUser {
                user_id: token_data.claims.sub,
                household_ids: token_data.claims.household_ids,
            })
        }
    }
}

// ---------------------------------------------------------------------------
// Tests (S016-T)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        Router,
        body::Body,
        extract::State,
        http::{Request, StatusCode},
        response::IntoResponse,
        routing::get,
    };
    use tower::ServiceExt;
    use uuid::Uuid;

    use crate::auth::service::issue_access_token;
    use crate::build_app_state;

    fn build_test_state() -> AppState {
        let pem = generate_test_rsa_pem();
        let pool = sqlx::PgPool::connect_lazy("postgres://localhost/test").unwrap();
        build_app_state(pool, &pem, false).unwrap()
    }

    fn generate_test_rsa_pem() -> String {
        use rsa::pkcs8::EncodePrivateKey;
        use rsa::rand_core::OsRng;
        let private_key = rsa::RsaPrivateKey::new(&mut OsRng, 2048).unwrap();
        private_key
            .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
            .unwrap()
            .to_string()
    }

    async fn auth_required_handler(
        State(_state): State<AppState>,
        auth_user: AuthUser,
    ) -> impl IntoResponse {
        axum::Json(serde_json::json!({ "user_id": auth_user.user_id.to_string() }))
    }

    fn make_app(state: AppState) -> Router {
        Router::new()
            .route("/protected", get(auth_required_handler))
            .with_state(state)
    }

    #[tokio::test]
    async fn test_no_auth_header_no_cookie_returns_401() {
        let state = build_test_state();
        let app = make_app(state);

        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/protected")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_valid_bearer_token_returns_user_id() {
        let state = build_test_state();
        let user_id = Uuid::new_v4();
        let token =
            issue_access_token(user_id, "user@example.com", vec![], &state.enc_key).unwrap();

        let app = make_app(state);

        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/protected")
                    .header("Authorization", format!("Bearer {}", token))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body_bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(json["user_id"].as_str().unwrap(), user_id.to_string());
    }

    #[tokio::test]
    async fn test_expired_bearer_token_returns_401() {
        let state = build_test_state();
        let user_id = Uuid::new_v4();

        // Build an already-expired token (exp in the past)
        let now = chrono::Utc::now().timestamp();
        let claims = Claims {
            sub: user_id,
            email: "user@example.com".to_string(),
            household_ids: vec![],
            iat: now - 1000,
            exp: now - 500, // expired
        };
        let mut header = jsonwebtoken::Header::new(Algorithm::RS256);
        header.kid = Some("altair-v1".to_string());
        let expired_token = jsonwebtoken::encode(&header, &claims, &state.enc_key).unwrap();

        let app = make_app(state);

        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/protected")
                    .header("Authorization", format!("Bearer {}", expired_token))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_valid_access_token_cookie_returns_user_id() {
        let state = build_test_state();
        let user_id = Uuid::new_v4();
        let token =
            issue_access_token(user_id, "user@example.com", vec![], &state.enc_key).unwrap();

        let app = make_app(state);

        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/protected")
                    .header("Cookie", format!("access_token={}", token))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body_bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(json["user_id"].as_str().unwrap(), user_id.to_string());
    }

    // P4-026: malformed Authorization header edge cases
    #[tokio::test]
    async fn test_non_bearer_scheme_returns_401() {
        let state = build_test_state();
        let app = make_app(state);

        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/protected")
                    .header("Authorization", "Basic dXNlcjpwYXNz")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_bearer_with_no_token_returns_401() {
        let state = build_test_state();
        let app = make_app(state);

        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/protected")
                    .header("Authorization", "Bearer")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // "Bearer" with no trailing space+token: strip_prefix("Bearer ") returns None → 401
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
}
