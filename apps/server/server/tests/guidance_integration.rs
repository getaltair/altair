/// HTTP-level integration tests for the Guidance domain (S010).
///
/// These tests exercise the full HTTP request/response cycle via the real app router,
/// real database (via `#[sqlx::test]`), and real JWT issuance.
///
/// Covered assertions:
///   A-G-01 — POST /api/guidance/epics with valid initiative_id owned by auth user → 201
///   A-G-02 — POST /api/guidance/epics with initiative_id belonging to different user → 403
///   A-G-03 — GET /api/guidance/quests returns only the authenticated user's quests
///   A-G-12 — POST /api/guidance/daily-checkins first call → 201; second same date → 409
///   A-G-14 — Completing a quest (not_started → in_progress → completed) → 200
///             Note: the `QuestCompleted` tracing event is verified at the service unit-test
///             level (guidance/quests/service.rs). The HTTP test confirms the 200 response
///             which proves the code path that emits the event was reached.
use altair_server::{AppState, auth::service::issue_access_token, build_app_state, guidance};
use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use sqlx::PgPool;
use tower::ServiceExt;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn generate_test_rsa_pem() -> String {
    use rsa::pkcs8::EncodePrivateKey;
    use rsa::rand_core::OsRng;
    let private_key =
        rsa::RsaPrivateKey::new(&mut OsRng, 2048).expect("Failed to generate RSA key for tests");
    private_key
        .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
        .expect("Failed to encode RSA key as PEM")
        .to_string()
}

fn build_test_app(pool: PgPool) -> (Router, AppState) {
    let pem = generate_test_rsa_pem();
    let state: AppState =
        build_app_state(pool, &pem, false).expect("Failed to build AppState for test");
    let app = Router::new()
        .merge(guidance::router())
        .with_state(state.clone());
    (app, state)
}

fn bearer(state: &AppState, user_id: Uuid, email: &str) -> String {
    let token = issue_access_token(user_id, email, vec![], &state.enc_key)
        .expect("Failed to issue test access token");
    format!("Bearer {token}")
}

async fn post_json_auth(
    app: Router,
    uri: &str,
    body: &str,
    auth_header: &str,
) -> axum::response::Response {
    app.oneshot(
        Request::builder()
            .method("POST")
            .uri(uri)
            .header("Content-Type", "application/json")
            .header("Authorization", auth_header)
            .body(Body::from(body.to_string()))
            .expect("Failed to build request"),
    )
    .await
    .expect("Request failed")
}

async fn patch_json_auth(
    app: Router,
    uri: &str,
    body: &str,
    auth_header: &str,
) -> axum::response::Response {
    app.oneshot(
        Request::builder()
            .method("PATCH")
            .uri(uri)
            .header("Content-Type", "application/json")
            .header("Authorization", auth_header)
            .body(Body::from(body.to_string()))
            .expect("Failed to build request"),
    )
    .await
    .expect("Request failed")
}

async fn get_auth(app: Router, uri: &str, auth_header: &str) -> axum::response::Response {
    app.oneshot(
        Request::builder()
            .method("GET")
            .uri(uri)
            .header("Authorization", auth_header)
            .body(Body::empty())
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

async fn insert_user(pool: &PgPool, user_id: Uuid, email: &str) {
    sqlx::query(
        "INSERT INTO users (id, email, display_name, password_hash, is_admin, status) \
         VALUES ($1, $2, 'Test User', 'hashed_password', false, 'active')",
    )
    .bind(user_id)
    .bind(email)
    .execute(pool)
    .await
    .expect("Failed to insert test user");
}

async fn insert_initiative(pool: &PgPool, initiative_id: Uuid, user_id: Uuid) {
    sqlx::query(
        "INSERT INTO initiatives (id, user_id, title, status) \
         VALUES ($1, $2, 'Test Initiative', 'draft')",
    )
    .bind(initiative_id)
    .bind(user_id)
    .execute(pool)
    .await
    .expect("Failed to insert test initiative");
}

// ---------------------------------------------------------------------------
// A-G-01: POST /api/guidance/epics with valid initiative_id → 201
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn test_create_epic_valid_initiative_returns_201(pool: PgPool) {
    let user_id = Uuid::new_v4();
    let initiative_id = Uuid::new_v4();
    insert_user(&pool, user_id, "user@example.com").await;
    insert_initiative(&pool, initiative_id, user_id).await;

    let (app, state) = build_test_app(pool);
    let auth = bearer(&state, user_id, "user@example.com");

    let body = serde_json::json!({
        "initiative_id": initiative_id,
        "title": "Phase 1",
        "description": "First phase"
    })
    .to_string();

    let resp = post_json_auth(app, "/api/guidance/epics", &body, &auth).await;

    assert_eq!(
        resp.status(),
        StatusCode::CREATED,
        "POST /api/guidance/epics with valid initiative_id must return 201"
    );

    let json = body_json(resp).await;
    assert!(json["id"].is_string(), "response must include id");
    assert_eq!(
        json["title"].as_str().unwrap(),
        "Phase 1",
        "title must match request"
    );
    assert_eq!(
        json["initiative_id"].as_str().unwrap(),
        initiative_id.to_string(),
        "initiative_id must match request"
    );
}

// ---------------------------------------------------------------------------
// A-G-02: POST /api/guidance/epics with initiative_id from different user → 403
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn test_create_epic_other_users_initiative_returns_403(pool: PgPool) {
    let user_a = Uuid::new_v4();
    let user_b = Uuid::new_v4();
    let initiative_id = Uuid::new_v4();

    insert_user(&pool, user_a, "user_a@example.com").await;
    insert_user(&pool, user_b, "user_b@example.com").await;
    insert_initiative(&pool, initiative_id, user_a).await;

    let (app, state) = build_test_app(pool);
    // Authenticate as user B, who does not own the initiative
    let auth = bearer(&state, user_b, "user_b@example.com");

    let body = serde_json::json!({
        "initiative_id": initiative_id,
        "title": "Unauthorized Epic"
    })
    .to_string();

    let resp = post_json_auth(app, "/api/guidance/epics", &body, &auth).await;

    assert_eq!(
        resp.status(),
        StatusCode::FORBIDDEN,
        "POST /api/guidance/epics with another user's initiative_id must return 403"
    );
}

// ---------------------------------------------------------------------------
// A-G-03: GET /api/guidance/quests returns only the authenticated user's quests
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn test_list_quests_returns_only_authed_users_quests(pool: PgPool) {
    let user_a = Uuid::new_v4();
    let user_b = Uuid::new_v4();

    insert_user(&pool, user_a, "user_a@example.com").await;
    insert_user(&pool, user_b, "user_b@example.com").await;

    let (app, state) = build_test_app(pool.clone());

    // Create a quest for user A
    let auth_a = bearer(&state, user_a, "user_a@example.com");
    let quest_a_body = serde_json::json!({"title": "User A Quest"}).to_string();
    let create_a =
        post_json_auth(app.clone(), "/api/guidance/quests", &quest_a_body, &auth_a).await;
    assert_eq!(
        create_a.status(),
        StatusCode::CREATED,
        "User A quest creation must return 201"
    );

    // Create a quest for user B
    let auth_b = bearer(&state, user_b, "user_b@example.com");
    let quest_b_body = serde_json::json!({"title": "User B Quest"}).to_string();
    let create_b =
        post_json_auth(app.clone(), "/api/guidance/quests", &quest_b_body, &auth_b).await;
    assert_eq!(
        create_b.status(),
        StatusCode::CREATED,
        "User B quest creation must return 201"
    );

    // GET quests as user A — must only see user A's quest
    let list_resp = get_auth(app, "/api/guidance/quests", &auth_a).await;
    assert_eq!(
        list_resp.status(),
        StatusCode::OK,
        "GET /api/guidance/quests must return 200"
    );

    let json = body_json(list_resp).await;
    let quests = json.as_array().expect("response must be an array");

    assert_eq!(
        quests.len(),
        1,
        "User A must see exactly 1 quest, not user B's"
    );
    assert_eq!(
        quests[0]["title"].as_str().unwrap(),
        "User A Quest",
        "user A's quest title must match"
    );
    assert_eq!(
        quests[0]["user_id"].as_str().unwrap(),
        user_a.to_string(),
        "quest must belong to user A"
    );
}

// ---------------------------------------------------------------------------
// A-G-12: POST /api/guidance/daily-checkins first call → 201; second same date → 409
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn test_daily_checkin_duplicate_date_returns_409(pool: PgPool) {
    let user_id = Uuid::new_v4();
    insert_user(&pool, user_id, "checkin_user@example.com").await;

    let (app, state) = build_test_app(pool);
    let auth = bearer(&state, user_id, "checkin_user@example.com");

    let body = serde_json::json!({
        "checkin_date": "2026-04-15",
        "energy_level": 7,
        "mood": "good"
    })
    .to_string();

    // First call must succeed with 201
    let first_resp =
        post_json_auth(app.clone(), "/api/guidance/daily-checkins", &body, &auth).await;
    assert_eq!(
        first_resp.status(),
        StatusCode::CREATED,
        "First POST /api/guidance/daily-checkins must return 201"
    );

    // Second call with same date must return 409
    let second_resp = post_json_auth(app, "/api/guidance/daily-checkins", &body, &auth).await;
    assert_eq!(
        second_resp.status(),
        StatusCode::CONFLICT,
        "Second POST /api/guidance/daily-checkins with same date must return 409"
    );
}

// ---------------------------------------------------------------------------
// A-G-14: Completing a quest via status transitions → 200
//
// The `QuestCompleted` tracing event is emitted in service.rs (lines 149 and 181)
// whenever `status` transitions to `completed`. This HTTP test verifies that the
// full request path succeeds (200) for both required transitions:
//   not_started → in_progress → completed
// Capturing the tracing event at the HTTP layer is not done here; it is already
// covered by the service-level unit tests in guidance/quests/service.rs.
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn test_quest_completion_transitions_return_200(pool: PgPool) {
    let user_id = Uuid::new_v4();
    insert_user(&pool, user_id, "quest_user@example.com").await;

    let (app, state) = build_test_app(pool);
    let auth = bearer(&state, user_id, "quest_user@example.com");

    // Create a quest (starts as not_started)
    let create_body = serde_json::json!({"title": "My Quest"}).to_string();
    let create_resp =
        post_json_auth(app.clone(), "/api/guidance/quests", &create_body, &auth).await;
    assert_eq!(
        create_resp.status(),
        StatusCode::CREATED,
        "Quest creation must return 201"
    );

    let quest_json = body_json(create_resp).await;
    let quest_id = quest_json["id"]
        .as_str()
        .expect("quest id must be a string");

    // PATCH not_started → in_progress (required intermediate step)
    let patch_uri = format!("/api/guidance/quests/{quest_id}");
    let in_progress_body = serde_json::json!({"status": "in_progress"}).to_string();
    let patch1_resp = patch_json_auth(app.clone(), &patch_uri, &in_progress_body, &auth).await;
    assert_eq!(
        patch1_resp.status(),
        StatusCode::OK,
        "PATCH not_started → in_progress must return 200"
    );

    let patch1_json = body_json(patch1_resp).await;
    assert_eq!(
        patch1_json["status"].as_str().unwrap(),
        "in_progress",
        "status must be in_progress after first patch"
    );

    // PATCH in_progress → completed (emits QuestCompleted tracing event in service.rs)
    let completed_body = serde_json::json!({"status": "completed"}).to_string();
    let patch2_resp = patch_json_auth(app, &patch_uri, &completed_body, &auth).await;
    assert_eq!(
        patch2_resp.status(),
        StatusCode::OK,
        "PATCH in_progress → completed must return 200"
    );

    let patch2_json = body_json(patch2_resp).await;
    assert_eq!(
        patch2_json["status"].as_str().unwrap(),
        "completed",
        "status must be completed after second patch"
    );
}
