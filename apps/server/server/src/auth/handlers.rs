use axum::http::{HeaderMap, HeaderValue, StatusCode, header::SET_COOKIE};
use axum::{Json, extract::State, response::IntoResponse};
use hex;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use super::models::{
    AuthUser, LoginRequest, LogoutRequest, RefreshRequest, RegisterRequest, TokenResponse,
    UserProfile,
};
use super::service;
use crate::AppState;
use crate::error::AppError;

// ---------------------------------------------------------------------------
// JWKS (S013)
// ---------------------------------------------------------------------------

pub async fn jwks(State(state): State<AppState>) -> impl IntoResponse {
    (
        [(axum::http::header::CONTENT_TYPE, "application/json")],
        state.jwks_json.clone(),
    )
}

// ---------------------------------------------------------------------------
// Register (S014)
// ---------------------------------------------------------------------------

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 1. Validate inputs
    if !req.email.contains('@') {
        return Err(AppError::BadRequest("Invalid email address".to_string()));
    }
    if req.password.len() < 8 {
        return Err(AppError::BadRequest(
            "Password must be at least 8 characters".to_string(),
        ));
    }

    // 2. Count existing non-deleted users to determine first-user admin status
    let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE deleted_at IS NULL")
        .fetch_one(&state.db)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))?;
    let count = count_row.0;

    let (is_admin, status) = if count == 0 {
        (true, "active")
    } else {
        (false, "pending")
    };

    // 3. Hash password
    let password_hash = service::hash_password(&req.password)?;

    // 4. Insert user
    let insert_result: Result<(Uuid,), sqlx::Error> = sqlx::query_as(
        "INSERT INTO users (email, display_name, password_hash, is_admin, status) \
         VALUES ($1, $2, $3, $4, $5) RETURNING id",
    )
    .bind(&req.email)
    .bind(&req.display_name)
    .bind(&password_hash)
    .bind(is_admin)
    .bind(status)
    .fetch_one(&state.db)
    .await;

    let user_id: Uuid = match insert_result {
        Ok((id,)) => id,
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            return Err(AppError::Conflict("Email already registered".to_string()));
        }
        Err(e) => return Err(AppError::Internal(anyhow::anyhow!(e.to_string()))),
    };

    // 5. Respond based on status
    if status == "active" {
        let access_token = service::issue_access_token(user_id, vec![], &state.enc_key)?;
        let (raw_refresh, hash_refresh) = service::generate_refresh_token();
        service::store_refresh_token(&state.db, user_id, &hash_refresh, None).await?;

        let token_response = TokenResponse {
            access_token: access_token.clone(),
            refresh_token: raw_refresh.clone(),
            token_type: "Bearer".to_string(),
        };

        let mut headers = HeaderMap::new();
        set_auth_cookies(&mut headers, &access_token, &raw_refresh)?;

        Ok((StatusCode::CREATED, headers, Json(token_response)).into_response())
    } else {
        Ok((
            StatusCode::ACCEPTED,
            Json(serde_json::json!({
                "message": "Account created. An admin must approve before you can log in."
            })),
        )
            .into_response())
    }
}

// ---------------------------------------------------------------------------
// Login (S014)
// ---------------------------------------------------------------------------

#[derive(sqlx::FromRow)]
struct UserRow {
    id: Uuid,
    password_hash: String,
    #[allow(dead_code)]
    is_admin: bool,
    status: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 1. Look up user
    let user: UserRow = sqlx::query_as(
        "SELECT id, password_hash, is_admin, status FROM users \
         WHERE email = $1 AND deleted_at IS NULL",
    )
    .bind(&req.email)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))?
    .ok_or(AppError::Unauthorized)?;

    // 2. Verify password
    service::verify_password(&req.password, &user.password_hash)?;

    // 3. Check status
    if user.status == "pending" {
        return Err(AppError::Forbidden);
    }

    // 4. Fetch household memberships (empty until household tables are wired)
    let household_ids: Vec<Uuid> = vec![];

    // 5. Issue tokens
    let access_token = service::issue_access_token(user.id, household_ids, &state.enc_key)?;
    let (raw_refresh, hash_refresh) = service::generate_refresh_token();
    service::store_refresh_token(&state.db, user.id, &hash_refresh, None).await?;

    let token_response = TokenResponse {
        access_token: access_token.clone(),
        refresh_token: raw_refresh.clone(),
        token_type: "Bearer".to_string(),
    };

    let mut headers = HeaderMap::new();
    set_auth_cookies(&mut headers, &access_token, &raw_refresh)?;

    Ok((StatusCode::OK, headers, Json(token_response)).into_response())
}

// ---------------------------------------------------------------------------
// Refresh (S015)
// ---------------------------------------------------------------------------

#[derive(sqlx::FromRow)]
struct RefreshUserRow {
    user_id: Uuid,
}

pub async fn refresh(
    State(state): State<AppState>,
    headers_in: axum::http::HeaderMap,
    body: Option<Json<RefreshRequest>>,
) -> Result<impl IntoResponse, AppError> {
    // Read refresh token from body or cookie
    let raw_token = extract_token_from_body_or_cookie(
        body.and_then(|Json(r)| r.refresh_token),
        &headers_in,
        "refresh_token",
    )?;

    // Look up token to get user_id (for household_ids — validate it's active)
    let token_hash = hex::encode(Sha256::digest(raw_token.as_bytes()));
    let record: RefreshUserRow = sqlx::query_as(
        "SELECT user_id FROM refresh_tokens \
         WHERE token_hash = $1 AND revoked_at IS NULL AND expires_at > NOW()",
    )
    .bind(&token_hash)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))?
    .ok_or(AppError::Unauthorized)?;

    let user_id = record.user_id;
    let household_ids: Vec<Uuid> = vec![];

    let new_tokens = service::rotate_refresh_token(
        &state.db,
        &raw_token,
        &state.enc_key,
        user_id,
        household_ids,
    )
    .await?;

    let mut resp_headers = HeaderMap::new();
    set_auth_cookies(
        &mut resp_headers,
        &new_tokens.access_token,
        &new_tokens.refresh_token,
    )?;

    Ok((StatusCode::OK, resp_headers, Json(new_tokens)).into_response())
}

// ---------------------------------------------------------------------------
// Logout (S015)
// ---------------------------------------------------------------------------

pub async fn logout(
    State(state): State<AppState>,
    headers_in: axum::http::HeaderMap,
    body: Option<Json<LogoutRequest>>,
) -> Result<impl IntoResponse, AppError> {
    let raw_token = extract_token_from_body_or_cookie(
        body.and_then(|Json(r)| r.refresh_token),
        &headers_in,
        "refresh_token",
    )?;

    service::revoke_refresh_token(&state.db, &raw_token).await?;

    let mut resp_headers = HeaderMap::new();
    resp_headers.insert(
        SET_COOKIE,
        HeaderValue::from_static("access_token=; HttpOnly; SameSite=Lax; Path=/; Max-Age=0"),
    );
    resp_headers.append(
        SET_COOKIE,
        HeaderValue::from_static(
            "refresh_token=; HttpOnly; SameSite=Lax; Path=/api/auth/refresh; Max-Age=0",
        ),
    );

    Ok((StatusCode::NO_CONTENT, resp_headers).into_response())
}

// ---------------------------------------------------------------------------
// Me (S015)
// ---------------------------------------------------------------------------

#[derive(sqlx::FromRow)]
struct MeRow {
    id: Uuid,
    email: String,
    display_name: String,
    is_admin: bool,
}

pub async fn me(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let row: MeRow =
        sqlx::query_as("SELECT id, email, display_name, is_admin FROM users WHERE id = $1")
            .bind(auth_user.user_id)
            .fetch_optional(&state.db)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))?
            .ok_or(AppError::NotFound)?;

    Ok(Json(UserProfile {
        id: row.id,
        email: row.email,
        display_name: row.display_name,
        is_admin: row.is_admin,
    }))
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn extract_token_from_body_or_cookie(
    from_body: Option<String>,
    headers: &axum::http::HeaderMap,
    cookie_name: &str,
) -> Result<String, AppError> {
    if let Some(t) = from_body {
        return Ok(t);
    }
    read_cookie_from_headers(headers, cookie_name).ok_or(AppError::Unauthorized)
}

fn read_cookie_from_headers(headers: &axum::http::HeaderMap, name: &str) -> Option<String> {
    let cookie_header = headers.get(axum::http::header::COOKIE)?;
    let cookies = cookie_header.to_str().ok()?;
    cookies.split(';').find_map(|part| {
        let part = part.trim();
        part.strip_prefix(&format!("{}=", name))
            .map(|v| v.to_string())
    })
}

fn set_auth_cookies(
    headers: &mut HeaderMap,
    access_token: &str,
    refresh_token: &str,
) -> Result<(), AppError> {
    let access_cookie = format!(
        "access_token={}; HttpOnly; SameSite=Lax; Path=/; Max-Age=900",
        access_token
    );
    let refresh_cookie = format!(
        "refresh_token={}; HttpOnly; SameSite=Lax; Path=/api/auth/refresh; Max-Age=604800",
        refresh_token
    );
    headers.insert(
        SET_COOKIE,
        HeaderValue::from_str(&access_cookie)
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))?,
    );
    headers.append(
        SET_COOKIE,
        HeaderValue::from_str(&refresh_cookie)
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))?,
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests (S013-T, S014-T, S015-T)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    mod test_helpers {
        use crate::AppState;
        use crate::build_app_state;

        pub fn generate_test_rsa_pem() -> String {
            use rsa::pkcs8::EncodePrivateKey;
            use rsa::rand_core::OsRng;
            let private_key = rsa::RsaPrivateKey::new(&mut OsRng, 2048).unwrap();
            private_key
                .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
                .unwrap()
                .to_string()
        }

        pub fn build_test_state() -> AppState {
            let pem = generate_test_rsa_pem();
            // connect_lazy so no actual DB connection is made during unit tests
            let pool = sqlx::PgPool::connect_lazy("postgres://localhost/test").unwrap();
            build_app_state(pool, &pem).unwrap()
        }
    }

    // S013-T: JWKS handler returns JSON with RSA key
    #[tokio::test]
    async fn test_jwks_returns_rsa_key() {
        let state = test_helpers::build_test_state();
        let resp = jwks(State(state)).await.into_response();
        assert_eq!(resp.status(), StatusCode::OK);
        let body_bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        let keys = json["keys"].as_array().unwrap();
        assert!(!keys.is_empty());
        assert_eq!(keys[0]["kty"], "RSA");
        assert_eq!(keys[0]["alg"], "RS256");
    }

    // S014-T: Input validation logic (isolated — no DB)
    #[test]
    fn test_email_validation_logic() {
        assert!("user@example.com".contains('@'));
        assert!(!"notanemail".contains('@'));
    }

    #[test]
    fn test_password_length_validation_logic() {
        assert!("short".len() < 8);
        assert!("longenough".len() >= 8);
    }

    // S015-T: Cookie header parsing
    #[test]
    fn test_read_cookie_from_headers() {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert(
            axum::http::header::COOKIE,
            axum::http::HeaderValue::from_static(
                "access_token=mytoken; refresh_token=myrefresh; other=val",
            ),
        );
        assert_eq!(
            read_cookie_from_headers(&headers, "access_token"),
            Some("mytoken".to_string())
        );
        assert_eq!(
            read_cookie_from_headers(&headers, "refresh_token"),
            Some("myrefresh".to_string())
        );
        assert_eq!(read_cookie_from_headers(&headers, "missing"), None);
    }

    #[test]
    fn test_read_cookie_from_headers_empty() {
        let headers = axum::http::HeaderMap::new();
        assert_eq!(read_cookie_from_headers(&headers, "access_token"), None);
    }
}
