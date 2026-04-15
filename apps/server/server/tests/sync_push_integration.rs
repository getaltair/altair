/// Integration tests for the sync push, conflicts list, and conflict resolve endpoints
/// (S010-T, S011-T, S015-T, S016-T).
///
/// Exercises the full HTTP request/response cycle via tower::ServiceExt::oneshot.
/// Each DB-backed test gets an isolated database via `#[sqlx::test]`.
///
/// Covered assertions (FA-001 through FA-009, plus S015/S016 scenarios):
///   FA-001 — POST /api/sync/push without JWT → 401
///   FA-002 — valid JWT + valid create mutation → 200 + Accepted
///   FA-003 — replay same mutation_id → 200 + Deduplicated
///   FA-004 — invalid entity_type string → 422
///   FA-005 — mutation for another user's entity → 403
///   FA-006 — two Update mutations with same base_version → sync_conflicts row + Conflicted
///   FA-007 — knowledge_note Update conflict → conflict copy note + entity_relations duplicates + conflict_copy resolution
///   FA-008 — tracking_item_event Update with quantity field → 409
///   FA-009 — delete mutation → deleted_at set, row still queryable
///   S015 — GET /api/sync/conflicts: user isolation, pagination, empty list, 401
///   S016 — POST /api/sync/conflicts/:id/resolve: accepted, rejected, 404, 403, 401
use altair_server::{AppState, auth::service::issue_access_token, build_app_state, sync};
use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use chrono::{DateTime, Utc};
use rsa::pkcs8::EncodePrivateKey;
use sqlx::PgPool;
use tower::ServiceExt;
use uuid::Uuid;

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

fn build_test_app_with_state(pool: PgPool) -> (Router, AppState) {
    let pem = generate_test_rsa_pem();
    let state: AppState =
        build_app_state(pool, &pem, false).expect("Failed to build AppState for test");
    let app = Router::new()
        .merge(sync::router())
        .with_state(state.clone());
    (app, state)
}

/// Issue a test JWT for the given user_id.
fn make_token(state: &AppState, user_id: Uuid) -> String {
    issue_access_token(user_id, "test@example.com", vec![], &state.enc_key)
        .expect("Failed to issue test token")
}

/// POST to /api/sync/push with a bearer token and JSON body.
async fn push(app: Router, token: &str, body: serde_json::Value) -> axum::response::Response {
    app.oneshot(
        Request::builder()
            .method("POST")
            .uri("/api/sync/push")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::from(serde_json::to_string(&body).unwrap()))
            .expect("Failed to build push request"),
    )
    .await
    .expect("Push request failed")
}

/// POST to /api/sync/push with no auth headers.
async fn push_unauthenticated(app: Router, body: serde_json::Value) -> axum::response::Response {
    app.oneshot(
        Request::builder()
            .method("POST")
            .uri("/api/sync/push")
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_string(&body).unwrap()))
            .expect("Failed to build push request"),
    )
    .await
    .expect("Push request failed")
}

async fn body_json(resp: axum::response::Response) -> serde_json::Value {
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");
    serde_json::from_slice(&bytes).expect("Failed to parse response body as JSON")
}

// ---------------------------------------------------------------------------
// DB fixtures
// ---------------------------------------------------------------------------

async fn insert_test_user(pool: &PgPool, user_id: Uuid, email: &str) {
    sqlx::query(
        "INSERT INTO users (id, email, display_name, password_hash, is_admin, status) \
         VALUES ($1, $2, 'Test User', 'hashed', false, 'active')",
    )
    .bind(user_id)
    .bind(email)
    .execute(pool)
    .await
    .expect("insert_test_user failed");
}

async fn insert_test_note(
    pool: &PgPool,
    note_id: Uuid,
    user_id: Uuid,
    title: &str,
) -> DateTime<Utc> {
    let row: (DateTime<Utc>,) = sqlx::query_as(
        "INSERT INTO knowledge_notes (id, title, user_id) \
         VALUES ($1, $2, $3) RETURNING updated_at",
    )
    .bind(note_id)
    .bind(title)
    .bind(user_id)
    .fetch_one(pool)
    .await
    .expect("insert_test_note failed");
    row.0
}

async fn insert_test_household(pool: &PgPool, household_id: Uuid, owner_id: Uuid) {
    sqlx::query("INSERT INTO households (id, owner_id, name) VALUES ($1, $2, 'Test Household')")
        .bind(household_id)
        .bind(owner_id)
        .execute(pool)
        .await
        .expect("insert_test_household failed");
}

async fn insert_test_tracking_item(
    pool: &PgPool,
    item_id: Uuid,
    user_id: Uuid,
    household_id: Uuid,
) {
    sqlx::query(
        "INSERT INTO tracking_items (id, name, user_id, household_id) VALUES ($1, 'Test Item', $2, $3)",
    )
    .bind(item_id)
    .bind(user_id)
    .bind(household_id)
    .execute(pool)
    .await
    .expect("insert_test_tracking_item failed");
}

async fn insert_test_item_event(pool: &PgPool, event_id: Uuid, item_id: Uuid) {
    sqlx::query(
        "INSERT INTO tracking_item_events \
         (id, item_id, event_type, quantity_change, occurred_at) \
         VALUES ($1, $2, 'use', 1, now())",
    )
    .bind(event_id)
    .bind(item_id)
    .execute(pool)
    .await
    .expect("insert_test_item_event failed");
}

/// Build a SyncUploadRequest JSON with a single mutation envelope.
fn single_mutation(
    entity_type: &str,
    entity_id: Uuid,
    operation: &str,
    payload: Option<serde_json::Value>,
    base_version: Option<DateTime<Utc>>,
) -> serde_json::Value {
    single_mutation_with_id(
        Uuid::new_v4(),
        entity_type,
        entity_id,
        operation,
        payload,
        base_version,
    )
}

fn single_mutation_with_id(
    mutation_id: Uuid,
    entity_type: &str,
    entity_id: Uuid,
    operation: &str,
    payload: Option<serde_json::Value>,
    base_version: Option<DateTime<Utc>>,
) -> serde_json::Value {
    serde_json::json!({
        "mutations": [{
            "mutation_id": mutation_id.to_string(),
            "device_id": Uuid::new_v4().to_string(),
            "entity_type": entity_type,
            "entity_id": entity_id.to_string(),
            "operation": operation,
            "payload": payload,
            "base_version": base_version.map(|v| v.to_rfc3339()),
            "occurred_at": Utc::now().to_rfc3339()
        }]
    })
}

// ---------------------------------------------------------------------------
// FA-001: no JWT → 401
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn fa001_no_jwt_returns_401(pool: PgPool) {
    let (app, _state) = build_test_app_with_state(pool);

    let body = serde_json::json!({ "mutations": [] });
    let resp = push_unauthenticated(app, body).await;

    assert_eq!(
        resp.status(),
        StatusCode::UNAUTHORIZED,
        "FA-001: missing JWT must return 401"
    );
}

// ---------------------------------------------------------------------------
// FA-002: valid JWT + valid create mutation → 200 + Accepted
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn fa002_valid_create_mutation_returns_200_accepted(pool: PgPool) {
    let user_id = Uuid::new_v4();
    insert_test_user(&pool, user_id, "fa002@example.com").await;

    let (app, state) = build_test_app_with_state(pool.clone());
    let token = make_token(&state, user_id);
    let note_id = Uuid::new_v4();

    let body = single_mutation(
        "knowledge_note",
        note_id,
        "create",
        Some(serde_json::json!({ "title": "FA-002 Note" })),
        None,
    );

    let resp = push(app, &token, body).await;
    assert_eq!(resp.status(), StatusCode::OK, "FA-002: expect 200");

    let json = body_json(resp).await;
    let results = json["results"].as_array().expect("results must be array");
    assert_eq!(results.len(), 1);
    assert_eq!(
        results[0]["status"].as_str().unwrap(),
        "accepted",
        "FA-002: mutation must be accepted"
    );
}

// ---------------------------------------------------------------------------
// FA-003: replay same mutation_id → 200 + Deduplicated
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn fa003_replay_mutation_id_returns_deduplicated(pool: PgPool) {
    let user_id = Uuid::new_v4();
    insert_test_user(&pool, user_id, "fa003@example.com").await;

    let note_id = Uuid::new_v4();
    let mutation_id = Uuid::new_v4();
    let body = single_mutation_with_id(
        mutation_id,
        "knowledge_note",
        note_id,
        "create",
        Some(serde_json::json!({ "title": "FA-003 Note" })),
        None,
    );

    // First submission
    {
        let (app, state) = build_test_app_with_state(pool.clone());
        let token = make_token(&state, user_id);
        let resp = push(app, &token, body.clone()).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let json = body_json(resp).await;
        assert_eq!(json["results"][0]["status"].as_str().unwrap(), "accepted");
    }

    // Second submission — same mutation_id
    {
        let (app, state) = build_test_app_with_state(pool.clone());
        let token = make_token(&state, user_id);
        let resp = push(app, &token, body).await;
        assert_eq!(
            resp.status(),
            StatusCode::OK,
            "FA-003: expect 200 on replay"
        );
        let json = body_json(resp).await;
        assert_eq!(
            json["results"][0]["status"].as_str().unwrap(),
            "deduplicated",
            "FA-003: second submission must be deduplicated"
        );
    }

    // Verify DB row count is exactly 1 (not double-applied)
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM knowledge_notes WHERE id = $1")
        .bind(note_id)
        .fetch_one(&pool)
        .await
        .expect("count query failed");
    assert_eq!(
        count.0, 1,
        "FA-003: exactly 1 note must exist after dedup replay"
    );
}

// ---------------------------------------------------------------------------
// FA-004: invalid entity_type → 422
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn fa004_invalid_entity_type_returns_422(pool: PgPool) {
    let user_id = Uuid::new_v4();
    insert_test_user(&pool, user_id, "fa004@example.com").await;

    let (app, state) = build_test_app_with_state(pool);
    let token = make_token(&state, user_id);

    // Use an entity_type string that is not in the registry
    let body = serde_json::json!({
        "mutations": [{
            "mutation_id": Uuid::new_v4().to_string(),
            "device_id": Uuid::new_v4().to_string(),
            "entity_type": "unknown_type",
            "entity_id": Uuid::new_v4().to_string(),
            "operation": "create",
            "payload": null,
            "base_version": null,
            "occurred_at": Utc::now().to_rfc3339()
        }]
    });

    let resp = push(app, &token, body).await;
    assert_eq!(
        resp.status(),
        StatusCode::UNPROCESSABLE_ENTITY,
        "FA-004: unknown entity_type must return 422"
    );
}

// ---------------------------------------------------------------------------
// FA-005: mutation for another user's entity → 403
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn fa005_mutation_for_other_users_entity_returns_403(pool: PgPool) {
    let owner_id = Uuid::new_v4();
    let attacker_id = Uuid::new_v4();
    insert_test_user(&pool, owner_id, "fa005_owner@example.com").await;
    insert_test_user(&pool, attacker_id, "fa005_attacker@example.com").await;

    // Create note owned by owner
    let note_id = Uuid::new_v4();
    insert_test_note(&pool, note_id, owner_id, "Owner's Note").await;

    // Attacker tries to update it
    let (app, state) = build_test_app_with_state(pool);
    let token = make_token(&state, attacker_id);

    let body = single_mutation(
        "knowledge_note",
        note_id,
        "update",
        Some(serde_json::json!({ "title": "Hacked" })),
        None,
    );

    let resp = push(app, &token, body).await;
    assert_eq!(
        resp.status(),
        StatusCode::FORBIDDEN,
        "FA-005: mutation for another user's entity must return 403"
    );
}

// ---------------------------------------------------------------------------
// FA-006: two Update mutations with same base_version → sync_conflicts row + Conflicted
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn fa006_two_updates_same_base_version_produces_conflict_row(pool: PgPool) {
    let user_id = Uuid::new_v4();
    insert_test_user(&pool, user_id, "fa006@example.com").await;

    let note_id = Uuid::new_v4();
    let base_version = insert_test_note(&pool, note_id, user_id, "FA-006 Note").await;

    // Simulate a server-side update so the note has moved past base_version
    sqlx::query(
        "UPDATE knowledge_notes SET updated_at = now() + interval '10 seconds' WHERE id = $1",
    )
    .bind(note_id)
    .execute(&pool)
    .await
    .expect("simulated server update failed");

    // Submit an update with the old base_version (conflict)
    let (app, state) = build_test_app_with_state(pool.clone());
    let token = make_token(&state, user_id);

    let body = single_mutation(
        "knowledge_note",
        note_id,
        "update",
        Some(serde_json::json!({ "title": "Device A version" })),
        Some(base_version),
    );

    let resp = push(app, &token, body).await;
    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "FA-006: expect 200 even on conflict"
    );

    let json = body_json(resp).await;
    assert_eq!(
        json["results"][0]["status"].as_str().unwrap(),
        "conflicted",
        "FA-006: conflicting update must return conflicted status"
    );

    // Verify sync_conflicts row was written
    let conflict_count: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM sync_conflicts WHERE entity_id = $1")
            .bind(note_id)
            .fetch_one(&pool)
            .await
            .expect("count sync_conflicts failed");
    assert!(
        conflict_count.0 > 0,
        "FA-006: sync_conflicts row must exist after conflict"
    );
}

// ---------------------------------------------------------------------------
// FA-007: knowledge_note Update + conflict → conflict copy + duplicates relation + conflict_copy resolution
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn fa007_knowledge_note_conflict_creates_conflict_copy(pool: PgPool) {
    let user_id = Uuid::new_v4();
    insert_test_user(&pool, user_id, "fa007@example.com").await;

    let note_id = Uuid::new_v4();
    let base_version = insert_test_note(&pool, note_id, user_id, "FA-007 Original").await;

    // Simulate server-side update
    sqlx::query(
        "UPDATE knowledge_notes SET updated_at = now() + interval '10 seconds' WHERE id = $1",
    )
    .bind(note_id)
    .execute(&pool)
    .await
    .expect("simulated server update failed");

    let (app, state) = build_test_app_with_state(pool.clone());
    let token = make_token(&state, user_id);

    let body = single_mutation(
        "knowledge_note",
        note_id,
        "update",
        Some(serde_json::json!({ "title": "FA-007 Conflict Version" })),
        Some(base_version),
    );

    let resp = push(app, &token, body).await;
    assert_eq!(resp.status(), StatusCode::OK, "FA-007: expect 200");
    let json = body_json(resp).await;
    assert_eq!(
        json["results"][0]["status"].as_str().unwrap(),
        "conflicted",
        "FA-007: must return conflicted"
    );

    // Two knowledge_notes rows must exist (original + conflict copy)
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM knowledge_notes WHERE user_id = $1")
        .bind(user_id)
        .fetch_one(&pool)
        .await
        .expect("count notes failed");
    assert_eq!(
        count.0, 2,
        "FA-007: two knowledge_notes rows must exist (original + conflict copy)"
    );

    // entity_relations row with relation_type = 'duplicates' must exist
    let rel: Option<(String,)> = sqlx::query_as(
        "SELECT relation_type FROM entity_relations \
         WHERE to_entity_id = $1 AND relation_type = 'duplicates'",
    )
    .bind(note_id)
    .fetch_optional(&pool)
    .await
    .expect("query entity_relations failed");
    assert!(
        rel.is_some(),
        "FA-007: entity_relations 'duplicates' row must exist after conflict copy"
    );

    // sync_conflicts row with resolution = 'conflict_copy' must exist
    let resolution: Option<(String,)> =
        sqlx::query_as("SELECT resolution FROM sync_conflicts WHERE entity_id = $1")
            .bind(note_id)
            .fetch_optional(&pool)
            .await
            .expect("query sync_conflicts failed");
    assert_eq!(
        resolution.map(|r| r.0),
        Some("conflict_copy".to_string()),
        "FA-007: sync_conflicts resolution must be 'conflict_copy'"
    );
}

// ---------------------------------------------------------------------------
// FA-008: tracking_item_event Update with quantity field → 409
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn fa008_tracking_item_event_quantity_conflict_returns_409(pool: PgPool) {
    let user_id = Uuid::new_v4();
    insert_test_user(&pool, user_id, "fa008@example.com").await;
    let hh_id = Uuid::new_v4();
    insert_test_household(&pool, hh_id, user_id).await;

    let item_id = Uuid::new_v4();
    insert_test_tracking_item(&pool, item_id, user_id, hh_id).await;

    let event_id = Uuid::new_v4();
    insert_test_item_event(&pool, event_id, item_id).await;

    let (app, state) = build_test_app_with_state(pool);
    let token = make_token(&state, user_id);

    // Update with quantity field → should get 409
    let body = single_mutation(
        "tracking_item_event",
        event_id,
        "update",
        Some(serde_json::json!({ "quantity": 10 })),
        None,
    );

    let resp = push(app, &token, body).await;
    assert_eq!(
        resp.status(),
        StatusCode::CONFLICT,
        "FA-008: quantity conflict must return 409"
    );
}

// ---------------------------------------------------------------------------
// FA-009: delete mutation → deleted_at set, row still queryable
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn fa009_delete_mutation_sets_deleted_at_row_remains(pool: PgPool) {
    let user_id = Uuid::new_v4();
    insert_test_user(&pool, user_id, "fa009@example.com").await;

    let note_id = Uuid::new_v4();
    insert_test_note(&pool, note_id, user_id, "FA-009 Note").await;

    let (app, state) = build_test_app_with_state(pool.clone());
    let token = make_token(&state, user_id);

    let body = single_mutation("knowledge_note", note_id, "delete", None, None);

    let resp = push(app, &token, body).await;
    assert_eq!(resp.status(), StatusCode::OK, "FA-009: expect 200");

    let json = body_json(resp).await;
    assert_eq!(
        json["results"][0]["status"].as_str().unwrap(),
        "accepted",
        "FA-009: delete must be accepted"
    );

    // Row must still exist with deleted_at IS NOT NULL
    let row: Option<(Option<DateTime<Utc>>,)> =
        sqlx::query_as("SELECT deleted_at FROM knowledge_notes WHERE id = $1")
            .bind(note_id)
            .fetch_optional(&pool)
            .await
            .expect("query failed");

    assert!(
        row.is_some(),
        "FA-009: row must still exist after soft delete"
    );
    assert!(
        row.unwrap().0.is_some(),
        "FA-009: deleted_at must be non-null after delete mutation"
    );
}

// ---------------------------------------------------------------------------
// Helpers for S015 / S016 tests
// ---------------------------------------------------------------------------

/// Insert a row into sync_conflicts with a given resolution and explicit created_at.
async fn insert_test_conflict(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    resolution: &str,
    created_at: &str, // ISO 8601
) {
    sqlx::query(
        "INSERT INTO sync_conflicts \
         (id, entity_type, entity_id, user_id, resolution, created_at) \
         VALUES ($1, 'knowledge_note', $2, $3, $4, $5::timestamptz)",
    )
    .bind(id)
    .bind(Uuid::new_v4())
    .bind(user_id)
    .bind(resolution)
    .bind(created_at)
    .execute(pool)
    .await
    .expect("insert_test_conflict failed");
}

/// GET /api/sync/conflicts with a bearer token.
async fn get_conflicts(app: Router, token: &str, query: &str) -> axum::response::Response {
    let uri = if query.is_empty() {
        "/api/sync/conflicts".to_string()
    } else {
        format!("/api/sync/conflicts?{}", query)
    };
    app.oneshot(
        Request::builder()
            .method("GET")
            .uri(&uri)
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty())
            .expect("Failed to build get_conflicts request"),
    )
    .await
    .expect("get_conflicts request failed")
}

/// GET /api/sync/conflicts with no auth headers.
async fn get_conflicts_unauthenticated(app: Router) -> axum::response::Response {
    app.oneshot(
        Request::builder()
            .method("GET")
            .uri("/api/sync/conflicts")
            .body(Body::empty())
            .expect("Failed to build unauthenticated get_conflicts request"),
    )
    .await
    .expect("get_conflicts_unauthenticated request failed")
}

/// POST /api/sync/conflicts/:id/resolve with a bearer token.
async fn resolve_conflict(
    app: Router,
    token: &str,
    id: Uuid,
    resolution: &str,
) -> axum::response::Response {
    let body = serde_json::json!({ "resolution": resolution });
    app.oneshot(
        Request::builder()
            .method("POST")
            .uri(&format!("/api/sync/conflicts/{}/resolve", id))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::from(serde_json::to_string(&body).unwrap()))
            .expect("Failed to build resolve_conflict request"),
    )
    .await
    .expect("resolve_conflict request failed")
}

/// POST /api/sync/conflicts/:id/resolve with no auth headers.
async fn resolve_conflict_unauthenticated(app: Router, id: Uuid) -> axum::response::Response {
    let body = serde_json::json!({ "resolution": "accepted" });
    app.oneshot(
        Request::builder()
            .method("POST")
            .uri(&format!("/api/sync/conflicts/{}/resolve", id))
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_string(&body).unwrap()))
            .expect("Failed to build unauthenticated resolve request"),
    )
    .await
    .expect("resolve_conflict_unauthenticated request failed")
}

// ---------------------------------------------------------------------------
// S015: GET /api/sync/conflicts — user isolation
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn s015_conflicts_returns_only_auth_users_pending(pool: PgPool) {
    let user_a = Uuid::new_v4();
    let user_b = Uuid::new_v4();
    insert_test_user(&pool, user_a, "s015a@example.com").await;
    insert_test_user(&pool, user_b, "s015b@example.com").await;

    // User A has 2 pending conflicts
    let a1 = Uuid::new_v4();
    let a2 = Uuid::new_v4();
    insert_test_conflict(&pool, a1, user_a, "pending", "2026-01-01T00:00:00Z").await;
    insert_test_conflict(&pool, a2, user_a, "pending", "2026-01-02T00:00:00Z").await;

    // User B has 1 pending conflict — must NOT appear in User A's results
    let b1 = Uuid::new_v4();
    insert_test_conflict(&pool, b1, user_b, "pending", "2026-01-03T00:00:00Z").await;

    let (app, state) = build_test_app_with_state(pool.clone());
    let token_a = make_token(&state, user_a);

    let resp = get_conflicts(app, &token_a, "").await;
    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "S015: expect 200 for conflicts list"
    );

    let json = body_json(resp).await;
    let conflicts = json["conflicts"]
        .as_array()
        .expect("conflicts must be array");
    assert_eq!(
        conflicts.len(),
        2,
        "S015: user A must see only their 2 conflicts"
    );

    // Verify none of user B's conflict IDs appear
    let ids: Vec<&str> = conflicts
        .iter()
        .map(|c| c["id"].as_str().unwrap())
        .collect();
    assert!(
        !ids.contains(&b1.to_string().as_str()),
        "S015: user B's conflict must not appear in user A's results"
    );
}

// ---------------------------------------------------------------------------
// S015: GET /api/sync/conflicts — pagination
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn s015_conflicts_pagination(pool: PgPool) {
    let user_id = Uuid::new_v4();
    insert_test_user(&pool, user_id, "s015p@example.com").await;

    // Insert 2 conflicts with distinct created_at; newer first when sorted DESC
    let older_id = Uuid::new_v4();
    let newer_id = Uuid::new_v4();
    insert_test_conflict(&pool, older_id, user_id, "pending", "2026-01-01T00:00:00Z").await;
    insert_test_conflict(&pool, newer_id, user_id, "pending", "2026-01-02T00:00:00Z").await;

    // Page 1: limit=1 — must return the newer conflict + next_cursor set
    let (app, state) = build_test_app_with_state(pool.clone());
    let token = make_token(&state, user_id);

    let resp = get_conflicts(app, &token, "limit=1").await;
    assert_eq!(resp.status(), StatusCode::OK, "S015: page 1 expect 200");

    let json = body_json(resp).await;
    let conflicts = json["conflicts"]
        .as_array()
        .expect("conflicts must be array");
    assert_eq!(
        conflicts.len(),
        1,
        "S015: page 1 must return exactly 1 conflict"
    );
    assert_eq!(
        conflicts[0]["id"].as_str().unwrap(),
        newer_id.to_string(),
        "S015: page 1 must return the newer conflict"
    );

    let next_cursor = json["next_cursor"]
        .as_str()
        .expect("S015: next_cursor must be set on page 1");
    assert_eq!(
        next_cursor,
        newer_id.to_string(),
        "S015: next_cursor must be the last returned conflict id"
    );

    // Page 2: cursor=next_cursor, limit=1 — must return the older conflict + no next_cursor
    let (app2, state2) = build_test_app_with_state(pool.clone());
    let token2 = make_token(&state2, user_id);

    let query2 = format!("limit=1&cursor={}", next_cursor);
    let resp2 = get_conflicts(app2, &token2, &query2).await;
    assert_eq!(resp2.status(), StatusCode::OK, "S015: page 2 expect 200");

    let json2 = body_json(resp2).await;
    let conflicts2 = json2["conflicts"]
        .as_array()
        .expect("conflicts must be array");
    assert_eq!(
        conflicts2.len(),
        1,
        "S015: page 2 must return exactly 1 conflict"
    );
    assert_eq!(
        conflicts2[0]["id"].as_str().unwrap(),
        older_id.to_string(),
        "S015: page 2 must return the older conflict"
    );
    assert!(
        json2["next_cursor"].is_null(),
        "S015: next_cursor must be null on last page"
    );
}

// ---------------------------------------------------------------------------
// S015: GET /api/sync/conflicts — empty list
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn s015_conflicts_empty_list(pool: PgPool) {
    let user_id = Uuid::new_v4();
    insert_test_user(&pool, user_id, "s015e@example.com").await;

    let (app, state) = build_test_app_with_state(pool.clone());
    let token = make_token(&state, user_id);

    let resp = get_conflicts(app, &token, "").await;
    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "S015: expect 200 with no conflicts"
    );

    let json = body_json(resp).await;
    let conflicts = json["conflicts"]
        .as_array()
        .expect("conflicts must be array");
    assert_eq!(
        conflicts.len(),
        0,
        "S015: empty list when no pending conflicts"
    );
    assert!(
        json["next_cursor"].is_null(),
        "S015: next_cursor must be null when empty"
    );
}

// ---------------------------------------------------------------------------
// S015: GET /api/sync/conflicts — 401 without JWT
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn s015_conflicts_401_without_jwt(pool: PgPool) {
    let (app, _state) = build_test_app_with_state(pool);
    let resp = get_conflicts_unauthenticated(app).await;
    assert_eq!(
        resp.status(),
        StatusCode::UNAUTHORIZED,
        "S015: missing JWT must return 401"
    );
}

// ---------------------------------------------------------------------------
// S016: POST /api/sync/conflicts/:id/resolve — accepted → 200
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn s016_resolve_accepted_returns_200(pool: PgPool) {
    let user_id = Uuid::new_v4();
    insert_test_user(&pool, user_id, "s016a@example.com").await;

    let conflict_id = Uuid::new_v4();
    insert_test_conflict(
        &pool,
        conflict_id,
        user_id,
        "pending",
        "2026-01-01T00:00:00Z",
    )
    .await;

    let (app, state) = build_test_app_with_state(pool.clone());
    let token = make_token(&state, user_id);

    let resp = resolve_conflict(app, &token, conflict_id, "accepted").await;
    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "S016: accepted must return 200"
    );

    // Verify DB: resolution = 'accepted', resolved_at IS NOT NULL
    let row: (String, Option<DateTime<Utc>>) =
        sqlx::query_as("SELECT resolution, resolved_at FROM sync_conflicts WHERE id = $1")
            .bind(conflict_id)
            .fetch_one(&pool)
            .await
            .expect("S016: query failed");
    assert_eq!(row.0, "accepted", "S016: resolution must be 'accepted'");
    assert!(row.1.is_some(), "S016: resolved_at must be set");
}

// ---------------------------------------------------------------------------
// S016: POST /api/sync/conflicts/:id/resolve — rejected → 200
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn s016_resolve_rejected_returns_200(pool: PgPool) {
    let user_id = Uuid::new_v4();
    insert_test_user(&pool, user_id, "s016r@example.com").await;

    let conflict_id = Uuid::new_v4();
    insert_test_conflict(
        &pool,
        conflict_id,
        user_id,
        "pending",
        "2026-01-01T00:00:00Z",
    )
    .await;

    let (app, state) = build_test_app_with_state(pool.clone());
    let token = make_token(&state, user_id);

    let resp = resolve_conflict(app, &token, conflict_id, "rejected").await;
    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "S016: rejected must return 200"
    );

    let row: (String, Option<DateTime<Utc>>) =
        sqlx::query_as("SELECT resolution, resolved_at FROM sync_conflicts WHERE id = $1")
            .bind(conflict_id)
            .fetch_one(&pool)
            .await
            .expect("S016: query failed");
    assert_eq!(row.0, "rejected", "S016: resolution must be 'rejected'");
    assert!(row.1.is_some(), "S016: resolved_at must be set");
}

// ---------------------------------------------------------------------------
// S016: POST /api/sync/conflicts/:id/resolve — unknown id → 404
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn s016_resolve_unknown_id_returns_404(pool: PgPool) {
    let user_id = Uuid::new_v4();
    insert_test_user(&pool, user_id, "s016n@example.com").await;

    let (app, state) = build_test_app_with_state(pool);
    let token = make_token(&state, user_id);

    let nonexistent = Uuid::new_v4();
    let resp = resolve_conflict(app, &token, nonexistent, "accepted").await;
    assert_eq!(
        resp.status(),
        StatusCode::NOT_FOUND,
        "S016: unknown id must return 404"
    );
}

// ---------------------------------------------------------------------------
// S016: POST /api/sync/conflicts/:id/resolve — other user's conflict → 403
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn s016_resolve_other_users_conflict_returns_403(pool: PgPool) {
    let owner_id = Uuid::new_v4();
    let other_id = Uuid::new_v4();
    insert_test_user(&pool, owner_id, "s016o@example.com").await;
    insert_test_user(&pool, other_id, "s016x@example.com").await;

    let conflict_id = Uuid::new_v4();
    insert_test_conflict(
        &pool,
        conflict_id,
        owner_id,
        "pending",
        "2026-01-01T00:00:00Z",
    )
    .await;

    // other_id tries to resolve owner_id's conflict
    let (app, state) = build_test_app_with_state(pool);
    let token = make_token(&state, other_id);

    let resp = resolve_conflict(app, &token, conflict_id, "accepted").await;
    assert_eq!(
        resp.status(),
        StatusCode::FORBIDDEN,
        "S016: resolving another user's conflict must return 403"
    );
}

// ---------------------------------------------------------------------------
// S016: POST /api/sync/conflicts/:id/resolve — 401 without JWT
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn s016_resolve_401_without_jwt(pool: PgPool) {
    let (app, _state) = build_test_app_with_state(pool);
    let conflict_id = Uuid::new_v4();
    let resp = resolve_conflict_unauthenticated(app, conflict_id).await;
    assert_eq!(
        resp.status(),
        StatusCode::UNAUTHORIZED,
        "S016: missing JWT must return 401"
    );
}

// ---------------------------------------------------------------------------
// Additional fixtures for S018-T through S025-T
// ---------------------------------------------------------------------------

async fn insert_test_tag(pool: &PgPool, tag_id: Uuid, user_id: Uuid) {
    sqlx::query("INSERT INTO tags (id, user_id, name) VALUES ($1, $2, 'test-tag')")
        .bind(tag_id)
        .bind(user_id)
        .execute(pool)
        .await
        .expect("insert_test_tag failed");
}

async fn insert_test_snapshot(pool: &PgPool, snapshot_id: Uuid, note_id: Uuid) {
    sqlx::query(
        "INSERT INTO knowledge_note_snapshots (id, note_id, content, captured_at) \
         VALUES ($1, $2, 'snapshot', now())",
    )
    .bind(snapshot_id)
    .bind(note_id)
    .execute(pool)
    .await
    .expect("insert_test_snapshot failed");
}

async fn insert_test_quest(
    pool: &PgPool,
    quest_id: Uuid,
    user_id: Uuid,
    title: &str,
) -> DateTime<Utc> {
    let row: (DateTime<Utc>,) = sqlx::query_as(
        "INSERT INTO guidance_quests (id, user_id, title) VALUES ($1, $2, $3) RETURNING updated_at",
    )
    .bind(quest_id)
    .bind(user_id)
    .bind(title)
    .fetch_one(pool)
    .await
    .expect("insert_test_quest failed");
    row.0
}

// ---------------------------------------------------------------------------
// S018-T: Mixed-validity batch — partial commit (P5-015)
// ---------------------------------------------------------------------------

/// Verifies the documented partial-commit semantic of apply_mutations:
/// mutations are processed serially, each in its own transaction.
/// A later error (e.g., 403 on the second mutation) does NOT roll back
/// earlier committed mutations. The HTTP response reflects the error of
/// the failing mutation, while the first mutation's row is already persisted.
#[sqlx::test(migrations = "../../../infra/migrations")]
async fn s018_mixed_batch_partial_commit_first_row_persists_on_second_403(pool: PgPool) {
    let owner_id = Uuid::new_v4();
    let attacker_id = Uuid::new_v4();
    insert_test_user(&pool, owner_id, "s018_owner@example.com").await;
    insert_test_user(&pool, attacker_id, "s018_attacker@example.com").await;

    // The attacker will issue two mutations:
    //   1. A valid create of their own note.
    //   2. An update on owner's note — forbidden.
    let owner_note_id = Uuid::new_v4();
    insert_test_note(&pool, owner_note_id, owner_id, "Owner Note").await;

    let attacker_note_id = Uuid::new_v4();

    let (app, state) = build_test_app_with_state(pool.clone());
    let token = make_token(&state, attacker_id);

    // Build a two-mutation batch manually.
    let body = serde_json::json!({
        "mutations": [
            {
                "mutation_id": Uuid::new_v4().to_string(),
                "device_id": Uuid::new_v4().to_string(),
                "entity_type": "knowledge_note",
                "entity_id": attacker_note_id.to_string(),
                "operation": "create",
                "payload": { "title": "Attacker's Own Note" },
                "base_version": null,
                "occurred_at": Utc::now().to_rfc3339()
            },
            {
                "mutation_id": Uuid::new_v4().to_string(),
                "device_id": Uuid::new_v4().to_string(),
                "entity_type": "knowledge_note",
                "entity_id": owner_note_id.to_string(),
                "operation": "update",
                "payload": { "title": "Hacked Title" },
                "base_version": null,
                "occurred_at": Utc::now().to_rfc3339()
            }
        ]
    });

    let resp = push(app, &token, body).await;

    // The second mutation is forbidden → the whole call returns 403.
    assert_eq!(
        resp.status(),
        StatusCode::FORBIDDEN,
        "S018: second mutation forbidden must make overall response 403"
    );

    // The first mutation's row MUST be present in the DB — it was committed before the error.
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM knowledge_notes WHERE id = $1")
        .bind(attacker_note_id)
        .fetch_one(&pool)
        .await
        .expect("S018: count query failed");
    assert_eq!(
        count.0, 1,
        "S018: first mutation's row must be persisted despite later 403"
    );
}

// ---------------------------------------------------------------------------
// S019-T: Delete on non-deletable entity types → 400 (P5-016)
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn s019_delete_tracking_item_event_returns_400(pool: PgPool) {
    let user_id = Uuid::new_v4();
    insert_test_user(&pool, user_id, "s019_tie@example.com").await;
    let hh_id = Uuid::new_v4();
    insert_test_household(&pool, hh_id, user_id).await;
    let item_id = Uuid::new_v4();
    insert_test_tracking_item(&pool, item_id, user_id, hh_id).await;
    let event_id = Uuid::new_v4();
    insert_test_item_event(&pool, event_id, item_id).await;

    let (app, state) = build_test_app_with_state(pool);
    let token = make_token(&state, user_id);

    let body = single_mutation("tracking_item_event", event_id, "delete", None, None);
    let resp = push(app, &token, body).await;

    assert_eq!(
        resp.status(),
        StatusCode::BAD_REQUEST,
        "S019: delete on tracking_item_event must return 400"
    );
}

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn s019_delete_knowledge_note_snapshot_returns_400(pool: PgPool) {
    let user_id = Uuid::new_v4();
    insert_test_user(&pool, user_id, "s019_kns@example.com").await;
    let note_id = Uuid::new_v4();
    insert_test_note(&pool, note_id, user_id, "S019 Note").await;
    let snapshot_id = Uuid::new_v4();
    insert_test_snapshot(&pool, snapshot_id, note_id).await;

    let (app, state) = build_test_app_with_state(pool);
    let token = make_token(&state, user_id);

    let body = single_mutation("knowledge_note_snapshot", snapshot_id, "delete", None, None);
    let resp = push(app, &token, body).await;

    assert_eq!(
        resp.status(),
        StatusCode::BAD_REQUEST,
        "S019: delete on knowledge_note_snapshot must return 400"
    );
}

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn s019_delete_tag_returns_400(pool: PgPool) {
    let user_id = Uuid::new_v4();
    insert_test_user(&pool, user_id, "s019_tag@example.com").await;
    let tag_id = Uuid::new_v4();
    insert_test_tag(&pool, tag_id, user_id).await;

    let (app, state) = build_test_app_with_state(pool);
    let token = make_token(&state, user_id);

    let body = single_mutation("tag", tag_id, "delete", None, None);
    let resp = push(app, &token, body).await;

    assert_eq!(
        resp.status(),
        StatusCode::BAD_REQUEST,
        "S019: delete on tag must return 400"
    );
}

// ---------------------------------------------------------------------------
// S020-T: Update on immutable KnowledgeNoteSnapshot → 400 (P5-017)
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn s020_update_knowledge_note_snapshot_returns_400_immutable(pool: PgPool) {
    let user_id = Uuid::new_v4();
    insert_test_user(&pool, user_id, "s020@example.com").await;
    let note_id = Uuid::new_v4();
    insert_test_note(&pool, note_id, user_id, "S020 Note").await;
    let snapshot_id = Uuid::new_v4();
    insert_test_snapshot(&pool, snapshot_id, note_id).await;

    let (app, state) = build_test_app_with_state(pool);
    let token = make_token(&state, user_id);

    let body = single_mutation(
        "knowledge_note_snapshot",
        snapshot_id,
        "update",
        Some(serde_json::json!({ "content": "new content" })),
        None,
    );
    let resp = push(app, &token, body).await;

    assert_eq!(
        resp.status(),
        StatusCode::BAD_REQUEST,
        "S020: update on knowledge_note_snapshot must return 400"
    );

    let json = body_json(resp).await;
    let body_str = json.to_string();
    assert!(
        body_str.contains("knowledge_note_snapshot is immutable"),
        "S020: response body must contain 'knowledge_note_snapshot is immutable'; got: {body_str}"
    );
}

// ---------------------------------------------------------------------------
// S021-T: Ownership checks — Household, TrackingItem, TrackingItemEvent (P5-018)
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn s021_household_update_by_non_owner_returns_403(pool: PgPool) {
    let owner_id = Uuid::new_v4();
    let other_id = Uuid::new_v4();
    insert_test_user(&pool, owner_id, "s021_hh_owner@example.com").await;
    insert_test_user(&pool, other_id, "s021_hh_other@example.com").await;

    let hh_id = Uuid::new_v4();
    insert_test_household(&pool, hh_id, owner_id).await;

    let (app, state) = build_test_app_with_state(pool);
    let token = make_token(&state, other_id);

    let body = single_mutation(
        "household",
        hh_id,
        "update",
        Some(serde_json::json!({ "name": "Hijacked" })),
        None,
    );
    let resp = push(app, &token, body).await;

    assert_eq!(
        resp.status(),
        StatusCode::FORBIDDEN,
        "S021: non-owner household update must return 403"
    );
}

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn s021_tracking_item_update_by_non_member_returns_403(pool: PgPool) {
    let owner_id = Uuid::new_v4();
    let other_id = Uuid::new_v4();
    insert_test_user(&pool, owner_id, "s021_ti_owner@example.com").await;
    insert_test_user(&pool, other_id, "s021_ti_other@example.com").await;

    let hh_id = Uuid::new_v4();
    insert_test_household(&pool, hh_id, owner_id).await;

    let item_id = Uuid::new_v4();
    insert_test_tracking_item(&pool, item_id, owner_id, hh_id).await;

    // other_id is not owner of the item and not a household member.
    let (app, state) = build_test_app_with_state(pool);
    let token = make_token(&state, other_id);

    let body = single_mutation(
        "tracking_item",
        item_id,
        "update",
        Some(serde_json::json!({ "name": "Hijacked Item" })),
        None,
    );
    let resp = push(app, &token, body).await;

    assert_eq!(
        resp.status(),
        StatusCode::FORBIDDEN,
        "S021: non-member tracking_item update must return 403"
    );
}

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn s021_tracking_item_event_update_by_non_member_returns_403(pool: PgPool) {
    let owner_id = Uuid::new_v4();
    let other_id = Uuid::new_v4();
    insert_test_user(&pool, owner_id, "s021_tie_owner@example.com").await;
    insert_test_user(&pool, other_id, "s021_tie_other@example.com").await;

    let hh_id = Uuid::new_v4();
    insert_test_household(&pool, hh_id, owner_id).await;

    let item_id = Uuid::new_v4();
    insert_test_tracking_item(&pool, item_id, owner_id, hh_id).await;

    let event_id = Uuid::new_v4();
    insert_test_item_event(&pool, event_id, item_id).await;

    // other_id is not owner of the parent item and not a household member.
    let (app, state) = build_test_app_with_state(pool);
    let token = make_token(&state, other_id);

    let body = single_mutation(
        "tracking_item_event",
        event_id,
        "update",
        Some(serde_json::json!({ "notes": "hijacked" })),
        None,
    );
    let resp = push(app, &token, body).await;

    assert_eq!(
        resp.status(),
        StatusCode::FORBIDDEN,
        "S021: non-member tracking_item_event update must return 403"
    );
}

// ---------------------------------------------------------------------------
// S022-T: Resolve endpoint edge cases (P5-019)
// ---------------------------------------------------------------------------

/// Resolving an already-resolved conflict returns 409 Conflict.
#[sqlx::test(migrations = "../../../infra/migrations")]
async fn s022_resolve_already_resolved_conflict_returns_409(pool: PgPool) {
    let user_id = Uuid::new_v4();
    insert_test_user(&pool, user_id, "s022_409@example.com").await;

    let conflict_id = Uuid::new_v4();
    // Insert with resolution = 'accepted' (already resolved).
    insert_test_conflict(
        &pool,
        conflict_id,
        user_id,
        "accepted",
        "2026-01-01T00:00:00Z",
    )
    .await;

    let (app, state) = build_test_app_with_state(pool);
    let token = make_token(&state, user_id);

    let resp = resolve_conflict(app, &token, conflict_id, "accepted").await;

    assert_eq!(
        resp.status(),
        StatusCode::CONFLICT,
        "S022: resolving an already-resolved conflict must return 409"
    );
}

/// Resolving with an unknown enum variant returns 422 (serde rejects at deserialization).
#[sqlx::test(migrations = "../../../infra/migrations")]
async fn s022_resolve_invalid_resolution_returns_422(pool: PgPool) {
    let user_id = Uuid::new_v4();
    insert_test_user(&pool, user_id, "s022_422@example.com").await;

    let conflict_id = Uuid::new_v4();
    insert_test_conflict(
        &pool,
        conflict_id,
        user_id,
        "pending",
        "2026-01-01T00:00:00Z",
    )
    .await;

    let (app, state) = build_test_app_with_state(pool);
    let token = make_token(&state, user_id);

    // Pass an invalid resolution string — serde cannot deserialize it into ConflictResolution.
    let resp = resolve_conflict(app, &token, conflict_id, "garbage").await;

    assert_eq!(
        resp.status(),
        StatusCode::UNPROCESSABLE_ENTITY,
        "S022: unknown resolution enum variant must return 422"
    );
}

// ---------------------------------------------------------------------------
// S025-T: Non-KnowledgeNote LWW Rejected path (P5-026)
// ---------------------------------------------------------------------------

/// For a user-scoped entity with updated_at (GuidanceQuest):
/// - Submit an Update where occurred_at is older than the server row's updated_at.
/// - Expect: response status 200 with status "conflicted" and a non-null conflict_id.
/// - The quest row's data is UNCHANGED (server value wins under LWW reject).
/// - A sync_conflicts row exists for the user with resolution "pending".
#[sqlx::test(migrations = "../../../infra/migrations")]
async fn s025_guidance_quest_lww_rejected_returns_conflicted_and_row_unchanged(pool: PgPool) {
    let user_id = Uuid::new_v4();
    insert_test_user(&pool, user_id, "s025@example.com").await;

    let quest_id = Uuid::new_v4();
    insert_test_quest(&pool, quest_id, user_id, "Original Title").await;

    // Advance the server's updated_at far into the future to simulate a concurrent server write.
    sqlx::query("UPDATE guidance_quests SET updated_at = now() + interval '1 hour' WHERE id = $1")
        .bind(quest_id)
        .execute(&pool)
        .await
        .expect("S025: failed to advance updated_at");

    // Read back the actual updated_at to avoid host/DB clock-skew dependency.
    let (server_updated_at,): (DateTime<Utc>,) =
        sqlx::query_as("SELECT updated_at FROM guidance_quests WHERE id = $1")
            .bind(quest_id)
            .fetch_one(&pool)
            .await
            .expect("S025: failed to fetch updated_at");

    // Set occurred_at to 2 hours BEFORE server's updated_at — guaranteed stale regardless of clock skew.
    let stale_occurred_at = server_updated_at - chrono::Duration::hours(2);

    let (app, state) = build_test_app_with_state(pool.clone());
    let token = make_token(&state, user_id);

    // Submit an update with explicit stale occurred_at; detect_conflict sees occurred_at < updated_at → Rejected.
    let body = serde_json::json!({
        "mutations": [{
            "mutation_id": Uuid::new_v4().to_string(),
            "device_id": Uuid::new_v4().to_string(),
            "entity_type": "guidance_quest",
            "entity_id": quest_id.to_string(),
            "operation": "update",
            "payload": { "title": "Client Stale Title" },
            "base_version": null,
            "occurred_at": stale_occurred_at.to_rfc3339()
        }]
    });

    let resp = push(app, &token, body).await;

    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "S025: LWW-rejected update must return 200"
    );

    let json = body_json(resp).await;
    assert_eq!(
        json["results"][0]["status"].as_str().unwrap(),
        "conflicted",
        "S025: LWW-rejected update must return 'conflicted' status"
    );
    assert!(
        !json["results"][0]["conflict_id"].is_null(),
        "S025: conflict_id must be non-null in conflicted result"
    );

    // Quest row data must be unchanged — server title wins.
    let title: (String,) = sqlx::query_as("SELECT title FROM guidance_quests WHERE id = $1")
        .bind(quest_id)
        .fetch_one(&pool)
        .await
        .expect("S025: quest query failed");
    assert_eq!(
        title.0, "Original Title",
        "S025: quest title must be unchanged after LWW reject"
    );

    // A sync_conflicts row with resolution 'pending' must exist for this user.
    let conflict: Option<(String,)> = sqlx::query_as(
        "SELECT resolution FROM sync_conflicts WHERE entity_id = $1 AND user_id = $2",
    )
    .bind(quest_id)
    .bind(user_id)
    .fetch_optional(&pool)
    .await
    .expect("S025: sync_conflicts query failed");
    assert_eq!(
        conflict.map(|r| r.0),
        Some("pending".to_string()),
        "S025: sync_conflicts row must exist with resolution 'pending'"
    );
}
