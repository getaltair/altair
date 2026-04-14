/// Integration tests for auth handlers (S029).
///
/// These tests exercise the full HTTP request/response cycle including real DB operations.
/// Each test gets an isolated database via `#[sqlx::test]` with migrations applied.
///
/// Covered assertions:
///   FA-002 — first register → 201, user is admin and active
///   FA-003 — second register → 202, user is pending
///   FA-004 — duplicate email → 409
///   FA-005 — login active user → 200 with valid JWT containing email claim
///   FA-006 — login pending user → 403
///   FA-007 — login wrong password → 401
///   logout with valid token → 204, refresh token is revoked in DB
///   FA-010 (partial) — replay old refresh token after rotate → 401
use altair_server::{AppState, auth::auth_router, build_app_state};
use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use rsa::pkcs8::EncodePrivateKey;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use tower::ServiceExt;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn generate_test_rsa_pem() -> String {
    use rsa::rand_core::OsRng;
    let private_key =
        rsa::RsaPrivateKey::new(&mut OsRng, 2048).expect("Failed to generate RSA key for tests");
    private_key
        .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
        .expect("Failed to encode RSA key as PEM")
        .to_string()
}

fn build_test_app(pool: PgPool) -> Router {
    let pem = generate_test_rsa_pem();
    let state: AppState =
        build_app_state(pool, &pem, false).expect("Failed to build AppState for test");
    Router::new().merge(auth_router()).with_state(state)
}

fn build_test_app_with_state(pool: PgPool) -> (Router, AppState) {
    let pem = generate_test_rsa_pem();
    let state: AppState =
        build_app_state(pool, &pem, false).expect("Failed to build AppState for test");
    let app = Router::new().merge(auth_router()).with_state(state.clone());
    (app, state)
}

async fn post_json(app: Router, uri: &str, body: &str) -> axum::response::Response {
    app.oneshot(
        Request::builder()
            .method("POST")
            .uri(uri)
            .header("Content-Type", "application/json")
            .body(Body::from(body.to_string()))
            .expect("Failed to build request"),
    )
    .await
    .expect("Request failed")
}

async fn body_json(resp: axum::response::Response) -> serde_json::Value {
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");
    serde_json::from_slice(&bytes).expect("Failed to parse response body as JSON")
}

// ---------------------------------------------------------------------------
// FA-002: First register → 201, user is admin and active
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn test_first_register_returns_201_admin_active(pool: PgPool) {
    let app = build_test_app(pool.clone());

    let resp = post_json(
        app,
        "/api/auth/register",
        r#"{"email":"admin@example.com","display_name":"Admin","password":"password123"}"#,
    )
    .await;

    assert_eq!(
        resp.status(),
        StatusCode::CREATED,
        "First register must return 201"
    );

    let json = body_json(resp).await;
    assert!(
        json["access_token"].is_string(),
        "access_token must be present"
    );
    assert!(
        json["refresh_token"].is_string(),
        "refresh_token must be present"
    );
    assert_eq!(json["token_type"], "Bearer");

    // Verify DB row: is_admin=true, status=active
    let row: (bool, String) = sqlx::query_as("SELECT is_admin, status FROM users WHERE email = $1")
        .bind("admin@example.com")
        .fetch_one(&pool)
        .await
        .expect("User must exist in DB after first register");
    assert!(row.0, "First registered user must be admin");
    assert_eq!(
        row.1, "active",
        "First registered user must have status=active"
    );
}

// ---------------------------------------------------------------------------
// FA-003: Second register → 202, user is pending
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn test_second_register_returns_202_pending(pool: PgPool) {
    // Register first user (admin)
    post_json(
        build_test_app(pool.clone()),
        "/api/auth/register",
        r#"{"email":"admin@example.com","display_name":"Admin","password":"password123"}"#,
    )
    .await;

    // Register second user — must be pending
    let resp = post_json(
        build_test_app(pool.clone()),
        "/api/auth/register",
        r#"{"email":"user@example.com","display_name":"User","password":"password123"}"#,
    )
    .await;

    assert_eq!(
        resp.status(),
        StatusCode::ACCEPTED,
        "Second register must return 202"
    );

    let json = body_json(resp).await;
    assert!(
        json["access_token"].is_null() || json.get("access_token").is_none(),
        "Pending user must not receive access_token, got: {json}"
    );
    assert!(
        json["message"].is_string(),
        "Pending response must include a message"
    );

    // Verify DB row: status=pending, is_admin=false
    let row: (bool, String) = sqlx::query_as("SELECT is_admin, status FROM users WHERE email = $1")
        .bind("user@example.com")
        .fetch_one(&pool)
        .await
        .expect("Second user must exist in DB");
    assert!(!row.0, "Second registered user must not be admin");
    assert_eq!(
        row.1, "pending",
        "Second registered user must have status=pending"
    );
}

// ---------------------------------------------------------------------------
// FA-004: Duplicate email → 409
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn test_duplicate_email_returns_409(pool: PgPool) {
    post_json(
        build_test_app(pool.clone()),
        "/api/auth/register",
        r#"{"email":"dupe@example.com","display_name":"First","password":"password123"}"#,
    )
    .await;

    let resp = post_json(
        build_test_app(pool.clone()),
        "/api/auth/register",
        r#"{"email":"dupe@example.com","display_name":"Second","password":"password456"}"#,
    )
    .await;

    assert_eq!(
        resp.status(),
        StatusCode::CONFLICT,
        "Duplicate email must return 409"
    );
}

// ---------------------------------------------------------------------------
// FA-005: Login active user → 200 with valid JWT containing email claim
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn test_login_active_user_returns_200_with_jwt(pool: PgPool) {
    // Register first user (becomes admin/active)
    post_json(
        build_test_app(pool.clone()),
        "/api/auth/register",
        r#"{"email":"active@example.com","display_name":"Active","password":"password123"}"#,
    )
    .await;

    let (app, state) = build_test_app_with_state(pool.clone());
    let resp = post_json(
        app,
        "/api/auth/login",
        r#"{"email":"active@example.com","password":"password123"}"#,
    )
    .await;

    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "Login with valid active user must return 200"
    );

    let json = body_json(resp).await;
    let access_token = json["access_token"]
        .as_str()
        .expect("access_token must be a string in login response");

    // Decode the JWT and verify email claim is present and correct
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_required_spec_claims(&["sub", "exp"]);
    let token_data = decode::<serde_json::Value>(access_token, &state.dec_key, &validation)
        .expect("JWT must be valid and decodable");
    let claims = &token_data.claims;

    // sub must be a UUID string
    let sub = claims["sub"].as_str().expect("sub claim must be a string");
    uuid::Uuid::parse_str(sub).expect("sub claim must be a valid UUID");

    // email claim must match
    assert_eq!(
        claims["email"]
            .as_str()
            .expect("email claim must be a string"),
        "active@example.com",
        "email claim must match registered email"
    );
}

// ---------------------------------------------------------------------------
// FA-006: Login pending user → 403
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn test_login_pending_user_returns_403(pool: PgPool) {
    // Register first (admin) to ensure second is pending
    post_json(
        build_test_app(pool.clone()),
        "/api/auth/register",
        r#"{"email":"admin@example.com","display_name":"Admin","password":"adminpass1"}"#,
    )
    .await;

    post_json(
        build_test_app(pool.clone()),
        "/api/auth/register",
        r#"{"email":"pending@example.com","display_name":"Pending","password":"pendpass1"}"#,
    )
    .await;

    let resp = post_json(
        build_test_app(pool.clone()),
        "/api/auth/login",
        r#"{"email":"pending@example.com","password":"pendpass1"}"#,
    )
    .await;

    assert_eq!(
        resp.status(),
        StatusCode::FORBIDDEN,
        "Login for pending user must return 403"
    );
}

// ---------------------------------------------------------------------------
// FA-007: Login wrong password → 401
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn test_login_wrong_password_returns_401(pool: PgPool) {
    post_json(
        build_test_app(pool.clone()),
        "/api/auth/register",
        r#"{"email":"user@example.com","display_name":"User","password":"correctpass1"}"#,
    )
    .await;

    let resp = post_json(
        build_test_app(pool.clone()),
        "/api/auth/login",
        r#"{"email":"user@example.com","password":"wrongpassword"}"#,
    )
    .await;

    assert_eq!(
        resp.status(),
        StatusCode::UNAUTHORIZED,
        "Wrong password must return 401"
    );
}

// ---------------------------------------------------------------------------
// Logout: valid token → 204, refresh token is revoked in DB
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn test_logout_revokes_refresh_token(pool: PgPool) {
    // Register + login to get tokens
    post_json(
        build_test_app(pool.clone()),
        "/api/auth/register",
        r#"{"email":"user@example.com","display_name":"User","password":"password123"}"#,
    )
    .await;

    let login_resp = post_json(
        build_test_app(pool.clone()),
        "/api/auth/login",
        r#"{"email":"user@example.com","password":"password123"}"#,
    )
    .await;
    let login_json = body_json(login_resp).await;
    let refresh_token = login_json["refresh_token"]
        .as_str()
        .expect("refresh_token must be present after login")
        .to_string();

    // Logout using the refresh token in request body
    let logout_body = serde_json::json!({ "refresh_token": refresh_token }).to_string();
    let logout_resp = post_json(
        build_test_app(pool.clone()),
        "/api/auth/logout",
        &logout_body,
    )
    .await;

    assert_eq!(
        logout_resp.status(),
        StatusCode::NO_CONTENT,
        "Logout with valid token must return 204"
    );

    // Verify the token is revoked in DB
    let token_hash = hex::encode(Sha256::digest(refresh_token.as_bytes()));
    let row: (Option<chrono::DateTime<chrono::Utc>>,) =
        sqlx::query_as("SELECT revoked_at FROM refresh_tokens WHERE token_hash = $1")
            .bind(&token_hash)
            .fetch_one(&pool)
            .await
            .expect("Refresh token record must exist after logout");
    assert!(
        row.0.is_some(),
        "refresh_token must have revoked_at set after logout"
    );
}

// ---------------------------------------------------------------------------
// FA-010 (partial): Replay old refresh token after rotate → 401
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn test_replay_old_refresh_token_after_rotate_returns_401(pool: PgPool) {
    // Register + login
    post_json(
        build_test_app(pool.clone()),
        "/api/auth/register",
        r#"{"email":"user@example.com","display_name":"User","password":"password123"}"#,
    )
    .await;

    let login_resp = post_json(
        build_test_app(pool.clone()),
        "/api/auth/login",
        r#"{"email":"user@example.com","password":"password123"}"#,
    )
    .await;
    let login_json = body_json(login_resp).await;
    let old_refresh_token = login_json["refresh_token"]
        .as_str()
        .expect("refresh_token must be present after login")
        .to_string();

    // Rotate (use the refresh token once)
    let refresh_body = serde_json::json!({ "refresh_token": old_refresh_token }).to_string();
    let rotate_resp = post_json(
        build_test_app(pool.clone()),
        "/api/auth/refresh",
        &refresh_body,
    )
    .await;
    assert_eq!(
        rotate_resp.status(),
        StatusCode::OK,
        "First refresh must succeed"
    );

    // Replay the old refresh token — must be rejected
    let replay_resp = post_json(
        build_test_app(pool.clone()),
        "/api/auth/refresh",
        &refresh_body,
    )
    .await;
    assert_eq!(
        replay_resp.status(),
        StatusCode::UNAUTHORIZED,
        "Replayed (already-rotated) refresh token must return 401"
    );
}
