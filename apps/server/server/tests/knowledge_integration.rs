/// Integration tests for knowledge domain endpoints (S006-T).
///
/// Covered assertions:
///   SEC-2  — unauthenticated requests to /knowledge/* → 401
///   TC-K-1 — note lifecycle: POST → GET → PUT → GET → DELETE → GET 404
///   A-018  — client-generated UUID accepted; GET returns same UUID
///   TC-K-2 — user isolation: User A creates note; User B GET → 404 (SEC-1)
///   TC-K-3 — snapshot immutability: POST snapshot → 201; GET snapshots → 200 with content
///   TC-K-4 — backlinks end-to-end via /api/entity_relations
///   TC-K-5 — GET /notes?initiative_id filter
use altair_server::{
    AppState,
    auth::{auth_router, service::issue_access_token},
    build_app_state,
    core::relations::router as relations_router,
    knowledge::router as knowledge_router,
};
use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use rsa::pkcs8::EncodePrivateKey;
use sqlx::PgPool;
use tower::ServiceExt;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Test infrastructure
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

fn build_test_app(pool: PgPool) -> (Router, AppState) {
    let pem = generate_test_rsa_pem();
    let state: AppState =
        build_app_state(pool, &pem, false).expect("Failed to build AppState for test");
    let app = Router::new()
        .merge(auth_router())
        .merge(knowledge_router())
        .merge(relations_router())
        .with_state(state.clone());
    (app, state)
}

/// Issue a valid JWT for the given user_id without a DB round-trip.
fn make_token(state: &AppState, user_id: Uuid) -> String {
    issue_access_token(user_id, "test@example.com", vec![], &state.enc_key)
        .expect("Failed to issue access token")
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

async fn request(
    app: Router,
    method: &str,
    uri: &str,
    token: Option<&str>,
    body: Option<&str>,
) -> axum::response::Response {
    let mut builder = Request::builder().method(method).uri(uri);
    if let Some(t) = token {
        builder = builder.header("Authorization", format!("Bearer {t}"));
    }
    if body.is_some() {
        builder = builder.header("Content-Type", "application/json");
    }
    let body_bytes = body.map(|b| b.to_string()).unwrap_or_default();
    app.oneshot(
        builder
            .body(Body::from(body_bytes))
            .expect("Failed to build request"),
    )
    .await
    .expect("Request failed")
}

async fn body_json(resp: axum::response::Response) -> serde_json::Value {
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");
    serde_json::from_slice(&bytes).expect("Failed to parse JSON body")
}

// ---------------------------------------------------------------------------
// SEC-2: Unauthenticated requests to /knowledge/* → 401
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn test_unauthenticated_list_notes_returns_401(pool: PgPool) {
    let (app, _state) = build_test_app(pool);
    let resp = request(app, "GET", "/api/knowledge/notes", None, None).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED, "no auth → 401");
}

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn test_unauthenticated_post_note_returns_401(pool: PgPool) {
    let (app, _state) = build_test_app(pool);
    let resp = request(
        app,
        "POST",
        "/api/knowledge/notes",
        None,
        Some(r#"{"title":"x"}"#),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED, "no auth → 401");
}

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn test_unauthenticated_get_note_returns_401(pool: PgPool) {
    let (app, _state) = build_test_app(pool);
    let note_id = Uuid::new_v4();
    let resp = request(
        app,
        "GET",
        &format!("/api/knowledge/notes/{note_id}"),
        None,
        None,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED, "no auth → 401");
}

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn test_unauthenticated_snapshots_returns_401(pool: PgPool) {
    let (app, _state) = build_test_app(pool);
    let note_id = Uuid::new_v4();
    let resp = request(
        app,
        "POST",
        &format!("/api/knowledge/notes/{note_id}/snapshots"),
        None,
        Some(r#"{"captured_at":"2026-01-01T00:00:00Z"}"#),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED, "no auth → 401");
}

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn test_unauthenticated_backlinks_returns_401(pool: PgPool) {
    let (app, _state) = build_test_app(pool);
    let note_id = Uuid::new_v4();
    let resp = request(
        app,
        "GET",
        &format!("/api/knowledge/notes/{note_id}/backlinks"),
        None,
        None,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED, "no auth → 401");
}

// ---------------------------------------------------------------------------
// TC-K-1: Note lifecycle — POST → GET → PUT → GET → DELETE → GET 404
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn test_note_lifecycle(pool: PgPool) {
    let user_id = Uuid::new_v4();
    insert_user(&pool, user_id, "lifecycle@example.com").await;

    let (app, state) = build_test_app(pool);
    let token = make_token(&state, user_id);

    // POST → 201
    let resp = request(
        app.clone(),
        "POST",
        "/api/knowledge/notes",
        Some(&token),
        Some(r#"{"title":"My Note","content":"hello"}"#),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::CREATED, "POST → 201");
    let note = body_json(resp).await;
    let note_id = note["id"].as_str().expect("id must be string");

    // GET → 200
    let resp = request(
        app.clone(),
        "GET",
        &format!("/api/knowledge/notes/{note_id}"),
        Some(&token),
        None,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK, "GET → 200");
    let fetched = body_json(resp).await;
    assert_eq!(fetched["title"], "My Note");
    assert_eq!(fetched["content"], "hello");

    // PUT → 200
    let resp = request(
        app.clone(),
        "PUT",
        &format!("/api/knowledge/notes/{note_id}"),
        Some(&token),
        Some(r#"{"title":"Updated Title"}"#),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK, "PUT → 200");

    // GET → updated title, content unchanged
    let resp = request(
        app.clone(),
        "GET",
        &format!("/api/knowledge/notes/{note_id}"),
        Some(&token),
        None,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    let updated = body_json(resp).await;
    assert_eq!(updated["title"], "Updated Title");
    assert_eq!(updated["content"], "hello", "content must be unchanged");

    // DELETE → 204
    let resp = request(
        app.clone(),
        "DELETE",
        &format!("/api/knowledge/notes/{note_id}"),
        Some(&token),
        None,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT, "DELETE → 204");

    // GET → 404
    let resp = request(
        app.clone(),
        "GET",
        &format!("/api/knowledge/notes/{note_id}"),
        Some(&token),
        None,
    )
    .await;
    assert_eq!(
        resp.status(),
        StatusCode::NOT_FOUND,
        "GET after DELETE → 404"
    );
}

// ---------------------------------------------------------------------------
// A-018: Client-generated UUID accepted; GET returns same UUID
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn test_client_uuid_stored_and_returned(pool: PgPool) {
    let user_id = Uuid::new_v4();
    insert_user(&pool, user_id, "client_uuid@example.com").await;

    let (app, state) = build_test_app(pool);
    let token = make_token(&state, user_id);
    let client_id = Uuid::new_v4();

    let body = serde_json::json!({ "id": client_id, "title": "Offline Note" }).to_string();
    let resp = request(
        app.clone(),
        "POST",
        "/api/knowledge/notes",
        Some(&token),
        Some(&body),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let note = body_json(resp).await;
    assert_eq!(
        note["id"].as_str().unwrap(),
        client_id.to_string(),
        "server must store the client-provided UUID"
    );

    // GET returns same UUID
    let resp = request(
        app.clone(),
        "GET",
        &format!("/api/knowledge/notes/{client_id}"),
        Some(&token),
        None,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    let fetched = body_json(resp).await;
    assert_eq!(fetched["id"].as_str().unwrap(), client_id.to_string());
}

// ---------------------------------------------------------------------------
// TC-K-2 / SEC-1: User A creates note; User B GET → 404
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn test_user_isolation_get_other_users_note_returns_404(pool: PgPool) {
    let user_a_id = Uuid::new_v4();
    let user_b_id = Uuid::new_v4();
    insert_user(&pool, user_a_id, "user_a_iso@example.com").await;
    insert_user(&pool, user_b_id, "user_b_iso@example.com").await;

    let (app, state) = build_test_app(pool);
    let token_a = make_token(&state, user_a_id);
    let token_b = make_token(&state, user_b_id);

    // User A creates a note
    let resp = request(
        app.clone(),
        "POST",
        "/api/knowledge/notes",
        Some(&token_a),
        Some(r#"{"title":"User A Note"}"#),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let note = body_json(resp).await;
    let note_id = note["id"].as_str().unwrap().to_string();

    // User B tries to GET it → 404
    let resp = request(
        app.clone(),
        "GET",
        &format!("/api/knowledge/notes/{note_id}"),
        Some(&token_b),
        None,
    )
    .await;
    assert_eq!(
        resp.status(),
        StatusCode::NOT_FOUND,
        "User B must not access User A's note"
    );
}

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn test_user_isolation_list_does_not_show_other_users_notes(pool: PgPool) {
    let user_a_id = Uuid::new_v4();
    let user_b_id = Uuid::new_v4();
    insert_user(&pool, user_a_id, "list_a@example.com").await;
    insert_user(&pool, user_b_id, "list_b@example.com").await;

    let (app, state) = build_test_app(pool);
    let token_a = make_token(&state, user_a_id);
    let token_b = make_token(&state, user_b_id);

    // User A creates a note
    request(
        app.clone(),
        "POST",
        "/api/knowledge/notes",
        Some(&token_a),
        Some(r#"{"title":"User A Private"}"#),
    )
    .await;

    // User B list → empty
    let resp = request(
        app.clone(),
        "GET",
        "/api/knowledge/notes",
        Some(&token_b),
        None,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    let notes = body_json(resp).await;
    assert!(
        notes.as_array().unwrap().is_empty(),
        "User B must see an empty list"
    );
}

// ---------------------------------------------------------------------------
// TC-K-3: Snapshot immutability
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn test_create_snapshot_returns_201_with_content(pool: PgPool) {
    let user_id = Uuid::new_v4();
    insert_user(&pool, user_id, "snapshot_201@example.com").await;

    let (app, state) = build_test_app(pool);
    let token = make_token(&state, user_id);

    // Create note
    let resp = request(
        app.clone(),
        "POST",
        "/api/knowledge/notes",
        Some(&token),
        Some(r#"{"title":"Snap Note","content":"snap content"}"#),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let note = body_json(resp).await;
    let note_id = note["id"].as_str().unwrap().to_string();

    // POST snapshot → 201
    let resp = request(
        app.clone(),
        "POST",
        &format!("/api/knowledge/notes/{note_id}/snapshots"),
        Some(&token),
        Some(r#"{"captured_at":"2026-01-01T12:00:00Z"}"#),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::CREATED, "POST snapshot → 201");
    let snapshot = body_json(resp).await;
    assert_eq!(
        snapshot["content"], "snap content",
        "snapshot must capture note content"
    );

    // GET snapshots → 200 with the snapshot
    let resp = request(
        app.clone(),
        "GET",
        &format!("/api/knowledge/notes/{note_id}/snapshots"),
        Some(&token),
        None,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK, "GET snapshots → 200");
    let snapshots = body_json(resp).await;
    let arr = snapshots.as_array().expect("snapshots must be array");
    assert_eq!(arr.len(), 1, "must have one snapshot");
    assert_eq!(arr[0]["content"], "snap content");

    // No PATCH/PUT endpoint for snapshots — verify 404/405
    let snap_id = arr[0]["id"].as_str().unwrap().to_string();
    let resp = request(
        app.clone(),
        "PUT",
        &format!("/api/knowledge/notes/{note_id}/snapshots/{snap_id}"),
        Some(&token),
        Some(r#"{"content":"tampered"}"#),
    )
    .await;
    assert!(
        resp.status() == StatusCode::NOT_FOUND || resp.status() == StatusCode::METHOD_NOT_ALLOWED,
        "no PUT on snapshot — got: {}",
        resp.status()
    );
}

// ---------------------------------------------------------------------------
// TC-K-4: Backlinks end-to-end via /api/entity_relations
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn test_backlinks_end_to_end(pool: PgPool) {
    let user_id = Uuid::new_v4();
    insert_user(&pool, user_id, "backlink_http@example.com").await;

    let (app, state) = build_test_app(pool);
    let token = make_token(&state, user_id);

    // Create Note A
    let resp = request(
        app.clone(),
        "POST",
        "/api/knowledge/notes",
        Some(&token),
        Some(r#"{"title":"Note A"}"#),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let note_a = body_json(resp).await;
    let note_a_id = note_a["id"].as_str().unwrap().to_string();

    // Create Note B
    let resp = request(
        app.clone(),
        "POST",
        "/api/knowledge/notes",
        Some(&token),
        Some(r#"{"title":"Note B"}"#),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let note_b = body_json(resp).await;
    let note_b_id = note_b["id"].as_str().unwrap().to_string();

    // Create relation A → B via /api/entity_relations
    let relation_body = serde_json::json!({
        "from_entity_type": "knowledge_note",
        "from_entity_id": note_a_id,
        "to_entity_type": "knowledge_note",
        "to_entity_id": note_b_id,
        "relation_type": "references",
        "source_type": "user"
    })
    .to_string();
    let resp = request(
        app.clone(),
        "POST",
        "/api/entity_relations",
        Some(&token),
        Some(&relation_body),
    )
    .await;
    assert_eq!(
        resp.status(),
        StatusCode::CREATED,
        "create relation must return 201"
    );

    // GET /knowledge/notes/:B/backlinks → note A must appear
    let resp = request(
        app.clone(),
        "GET",
        &format!("/api/knowledge/notes/{note_b_id}/backlinks"),
        Some(&token),
        None,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK, "GET backlinks → 200");
    let backlinks = body_json(resp).await;
    let arr = backlinks.as_array().expect("backlinks must be array");
    assert_eq!(arr.len(), 1, "expected one backlink");
    assert_eq!(
        arr[0]["id"].as_str().unwrap(),
        note_a_id,
        "backlink source must be note A"
    );
}

// ---------------------------------------------------------------------------
// TC-K-5: GET /notes?initiative_id filter
// ---------------------------------------------------------------------------

#[sqlx::test(migrations = "../../../infra/migrations")]
async fn test_initiative_id_filter(pool: PgPool) {
    let user_id = Uuid::new_v4();
    insert_user(&pool, user_id, "filter_http@example.com").await;

    // Insert an initiative for FK validity
    let initiative_id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO initiatives (id, user_id, title, status) VALUES ($1, $2, 'Init', 'draft')",
    )
    .bind(initiative_id)
    .bind(user_id)
    .execute(&pool)
    .await
    .expect("Failed to insert initiative");

    let other_initiative_id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO initiatives (id, user_id, title, status) VALUES ($1, $2, 'Other', 'draft')",
    )
    .bind(other_initiative_id)
    .bind(user_id)
    .execute(&pool)
    .await
    .expect("Failed to insert other initiative");

    let (app, state) = build_test_app(pool);
    let token = make_token(&state, user_id);

    // Create note linked to initiative
    let body = serde_json::json!({
        "title": "Linked Note",
        "initiative_id": initiative_id
    })
    .to_string();
    let resp = request(
        app.clone(),
        "POST",
        "/api/knowledge/notes",
        Some(&token),
        Some(&body),
    )
    .await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    // Create note with no initiative
    request(
        app.clone(),
        "POST",
        "/api/knowledge/notes",
        Some(&token),
        Some(r#"{"title":"Unlinked Note"}"#),
    )
    .await;

    // Filter by initiative_id → 1 note
    let resp = request(
        app.clone(),
        "GET",
        &format!("/api/knowledge/notes?initiative_id={initiative_id}"),
        Some(&token),
        None,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    let notes = body_json(resp).await;
    let arr = notes.as_array().unwrap();
    assert_eq!(arr.len(), 1, "only one note linked to this initiative");
    assert_eq!(arr[0]["title"], "Linked Note");

    // Filter by other_initiative_id → empty
    let resp = request(
        app.clone(),
        "GET",
        &format!("/api/knowledge/notes?initiative_id={other_initiative_id}"),
        Some(&token),
        None,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    let notes = body_json(resp).await;
    let arr = notes.as_array().unwrap();
    assert_eq!(arr.len(), 0, "no notes linked to other initiative");
}
