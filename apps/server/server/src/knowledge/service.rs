use chrono::{DateTime, Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use super::models::{
    CreateNoteRequest, CreateSnapshotRequest, NoteResponse, SnapshotResponse, UpdateNoteRequest,
};
use crate::error::AppError;

// ---------------------------------------------------------------------------
// Private DB mapping type — carries `deleted_at` for query filtering but is
// not exposed in the public API contract (NoteResponse omits it).
// ---------------------------------------------------------------------------

#[derive(sqlx::FromRow)]
struct NoteRow {
    id: Uuid,
    title: String,
    content: Option<String>,
    initiative_id: Option<Uuid>,
    user_id: Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    #[allow(dead_code)]
    deleted_at: Option<DateTime<Utc>>,
}

impl From<NoteRow> for NoteResponse {
    fn from(row: NoteRow) -> Self {
        NoteResponse {
            id: row.id,
            title: row.title,
            content: row.content,
            initiative_id: row.initiative_id,
            user_id: row.user_id,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

// ---------------------------------------------------------------------------
// Note CRUD
// ---------------------------------------------------------------------------

pub async fn create_note(
    db: &PgPool,
    req: CreateNoteRequest,
    user_id: Uuid,
) -> Result<NoteResponse, AppError> {
    if req.title.trim().is_empty() {
        return Err(AppError::BadRequest("title must not be empty".to_string()));
    }
    let id = req.id.unwrap_or_else(Uuid::new_v4);
    // TODO: emit NoteLinked domain event when this note is linked to another note
    let row = sqlx::query_as::<_, NoteRow>(
        "INSERT INTO knowledge_notes (id, title, content, initiative_id, user_id) \
         VALUES ($1, $2, $3, $4, $5) \
         RETURNING *",
    )
    .bind(id)
    .bind(&req.title)
    .bind(&req.content)
    .bind(req.initiative_id)
    .bind(user_id)
    .fetch_one(db)
    .await?;

    Ok(NoteResponse::from(row))
}

pub async fn list_notes(
    db: &PgPool,
    user_id: Uuid,
    initiative_id: Option<Uuid>,
) -> Result<Vec<NoteResponse>, AppError> {
    if let Some(init_id) = initiative_id {
        let rows = sqlx::query_as::<_, NoteRow>(
            "SELECT * FROM knowledge_notes \
             WHERE user_id = $1 AND deleted_at IS NULL AND initiative_id = $2 \
             ORDER BY created_at DESC",
        )
        .bind(user_id)
        .bind(init_id)
        .fetch_all(db)
        .await?;
        Ok(rows.into_iter().map(NoteResponse::from).collect())
    } else {
        let rows = sqlx::query_as::<_, NoteRow>(
            "SELECT * FROM knowledge_notes \
             WHERE user_id = $1 AND deleted_at IS NULL \
             ORDER BY created_at DESC",
        )
        .bind(user_id)
        .fetch_all(db)
        .await?;
        Ok(rows.into_iter().map(NoteResponse::from).collect())
    }
}

pub async fn get_note(
    db: &PgPool,
    note_id: Uuid,
    user_id: Uuid,
) -> Result<NoteResponse, AppError> {
    let row = sqlx::query_as::<_, NoteRow>(
        "SELECT * FROM knowledge_notes \
         WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL",
    )
    .bind(note_id)
    .bind(user_id)
    .fetch_optional(db)
    .await?;

    row.map(NoteResponse::from).ok_or(AppError::NotFound)
}

pub async fn update_note(
    db: &PgPool,
    note_id: Uuid,
    req: UpdateNoteRequest,
    user_id: Uuid,
) -> Result<NoteResponse, AppError> {
    if req.title.is_none() && req.content.is_none() {
        return Err(AppError::BadRequest(
            "at least one field required".to_string(),
        ));
    }
    if let Some(ref title) = req.title {
        if title.trim().is_empty() {
            return Err(AppError::BadRequest("title must not be empty".to_string()));
        }
    }
    // `updated_at` is intentionally omitted — the `knowledge_notes_updated_at` trigger
    // sets it on every UPDATE; a manual SET would be redundant and mislead readers.
    let row = sqlx::query_as::<_, NoteRow>(
        "UPDATE knowledge_notes \
         SET title = COALESCE($1, title), content = COALESCE($2, content) \
         WHERE id = $3 AND user_id = $4 AND deleted_at IS NULL \
         RETURNING *",
    )
    .bind(req.title)
    .bind(req.content)
    .bind(note_id)
    .bind(user_id)
    .fetch_optional(db)
    .await?;

    row.map(NoteResponse::from).ok_or(AppError::NotFound)
}

pub async fn delete_note(db: &PgPool, note_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
    // `updated_at` is intentionally omitted — the trigger handles it.
    let result = sqlx::query(
        "UPDATE knowledge_notes \
         SET deleted_at = NOW() \
         WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL",
    )
    .bind(note_id)
    .bind(user_id)
    .execute(db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Snapshot operations (immutable — no update or delete, invariant E-6)
// ---------------------------------------------------------------------------

pub async fn create_snapshot(
    db: &PgPool,
    note_id: Uuid,
    req: CreateSnapshotRequest,
    user_id: Uuid,
) -> Result<SnapshotResponse, AppError> {
    // Reject captured_at values more than 30 seconds in the future to prevent
    // timestamp manipulation that would float snapshots above real captures in
    // the DESC ordering of list_snapshots.
    if req.captured_at > Utc::now() + Duration::seconds(30) {
        return Err(AppError::BadRequest(
            "captured_at must not be more than 30 seconds in the future".to_string(),
        ));
    }

    // SEC-1 (Spec.md): verify note ownership before inserting a snapshot.
    // Returns NotFound — not Forbidden — to avoid leaking whether the note exists at all.
    get_note(db, note_id, user_id).await?;

    // COALESCE(n.content, '') handles notes with NULL content — the snapshot column is
    // TEXT NOT NULL (migration 000018), so we map NULL → empty string rather than crashing.
    // fetch_optional handles TOCTOU: if the note is soft-deleted between the get_note
    // pre-check and this INSERT, the SELECT returns no rows → NotFound (not 500).
    let row = sqlx::query_as::<_, SnapshotResponse>(
        "INSERT INTO knowledge_note_snapshots (id, note_id, content, captured_at) \
         SELECT gen_random_uuid(), n.id, COALESCE(n.content, ''), $2 \
         FROM knowledge_notes n \
         WHERE n.id = $1 AND n.user_id = $3 AND n.deleted_at IS NULL \
         RETURNING *",
    )
    .bind(note_id)
    .bind(req.captured_at)
    .bind(user_id)
    .fetch_optional(db)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(row)
}

pub async fn list_snapshots(
    db: &PgPool,
    note_id: Uuid,
    user_id: Uuid,
) -> Result<Vec<SnapshotResponse>, AppError> {
    // SEC-1 (Spec.md): verify note ownership before returning snapshots.
    // Returns NotFound — not Forbidden — to avoid leaking whether the note exists at all.
    get_note(db, note_id, user_id).await?;

    let rows = sqlx::query_as::<_, SnapshotResponse>(
        "SELECT s.* FROM knowledge_note_snapshots s \
         WHERE s.note_id = $1 \
         ORDER BY s.captured_at DESC",
    )
    .bind(note_id)
    .fetch_all(db)
    .await?;

    Ok(rows)
}

// ---------------------------------------------------------------------------
// Backlink query (derived from entity_relations, invariant E-5)
// ---------------------------------------------------------------------------

pub async fn list_backlinks(
    db: &PgPool,
    note_id: Uuid,
    user_id: Uuid,
) -> Result<Vec<NoteResponse>, AppError> {
    // SEC-1 (Spec.md): verify target note ownership before returning backlinks.
    // Returns NotFound — not Forbidden — to avoid leaking whether the note exists at all.
    get_note(db, note_id, user_id).await?;

    let rows = sqlx::query_as::<_, NoteRow>(
        "SELECT n.* \
         FROM knowledge_notes n \
         JOIN entity_relations er \
             ON er.from_entity_id = n.id \
             AND er.from_entity_type = 'knowledge_note' \
         WHERE er.to_entity_id = $1 \
           AND er.to_entity_type = 'knowledge_note' \
           AND er.deleted_at IS NULL \
           AND n.deleted_at IS NULL \
           AND n.user_id = $2",
    )
    .bind(note_id)
    .bind(user_id)
    .fetch_all(db)
    .await?;

    Ok(rows.into_iter().map(NoteResponse::from).collect())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    async fn insert_test_user(pool: &PgPool, user_id: Uuid, email: &str) {
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

    // A-018 partial: create with explicit UUID stores and returns the same UUID
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn create_with_explicit_uuid_stores_same_uuid(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "explicit_uuid@example.com").await;

        let note_id = Uuid::new_v4();
        let result = create_note(
            &pool,
            CreateNoteRequest {
                id: Some(note_id),
                title: "Explicit UUID Note".to_string(),
                content: Some("content".to_string()),
                initiative_id: None,
            },
            user_id,
        )
        .await
        .expect("create_note failed");

        assert_eq!(result.id, note_id, "stored UUID must match provided UUID");
    }

    // A-018 partial: create without UUID auto-generates a non-nil UUID
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn create_without_uuid_generates_non_nil(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "auto_uuid@example.com").await;

        let result = create_note(
            &pool,
            CreateNoteRequest {
                id: None,
                title: "Auto UUID Note".to_string(),
                content: None,
                initiative_id: None,
            },
            user_id,
        )
        .await
        .expect("create_note failed");

        assert!(!result.id.is_nil(), "auto-generated UUID must be non-nil");
    }

    // TC-K-5: list with initiative_id filter returns only matching notes
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn list_with_initiative_filter_returns_only_matching(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "filter_user@example.com").await;

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

        create_note(
            &pool,
            CreateNoteRequest {
                id: None,
                title: "Linked Note".to_string(),
                content: None,
                initiative_id: Some(initiative_id),
            },
            user_id,
        )
        .await
        .expect("create_note failed");

        create_note(
            &pool,
            CreateNoteRequest {
                id: None,
                title: "Unlinked Note".to_string(),
                content: None,
                initiative_id: None,
            },
            user_id,
        )
        .await
        .expect("create_note failed");

        let filtered = list_notes(&pool, user_id, Some(initiative_id))
            .await
            .expect("list_notes failed");
        assert_eq!(filtered.len(), 1, "only the linked note should be returned");
        assert_eq!(filtered[0].title, "Linked Note");

        let other_filtered = list_notes(&pool, user_id, Some(other_initiative_id))
            .await
            .expect("list_notes failed");
        assert_eq!(
            other_filtered.len(),
            0,
            "other initiative filter returns empty"
        );
    }

    // COALESCE: update with only title leaves content unchanged
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn update_only_title_leaves_content_unchanged(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "update_title@example.com").await;

        let note = create_note(
            &pool,
            CreateNoteRequest {
                id: None,
                title: "Original Title".to_string(),
                content: Some("Original Content".to_string()),
                initiative_id: None,
            },
            user_id,
        )
        .await
        .expect("create_note failed");

        let updated = update_note(
            &pool,
            note.id,
            UpdateNoteRequest {
                title: Some("New Title".to_string()),
                content: None,
            },
            user_id,
        )
        .await
        .expect("update_note failed");

        assert_eq!(updated.title, "New Title");
        assert_eq!(
            updated.content,
            Some("Original Content".to_string()),
            "content must remain unchanged"
        );
    }

    // COALESCE: update with only content leaves title unchanged
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn update_only_content_leaves_title_unchanged(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "update_content@example.com").await;

        let note = create_note(
            &pool,
            CreateNoteRequest {
                id: None,
                title: "Stable Title".to_string(),
                content: Some("Old Content".to_string()),
                initiative_id: None,
            },
            user_id,
        )
        .await
        .expect("create_note failed");

        let updated = update_note(
            &pool,
            note.id,
            UpdateNoteRequest {
                title: None,
                content: Some("New Content".to_string()),
            },
            user_id,
        )
        .await
        .expect("update_note failed");

        assert_eq!(updated.title, "Stable Title", "title must remain unchanged");
        assert_eq!(updated.content, Some("New Content".to_string()));
    }

    // get non-existent note returns NotFound
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn get_nonexistent_note_returns_not_found(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "notfound@example.com").await;

        let result = get_note(&pool, Uuid::new_v4(), user_id).await;
        assert!(
            matches!(result, Err(AppError::NotFound)),
            "get on unknown id must return NotFound"
        );
    }

    // TC-K-1: delete → subsequent get returns NotFound
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn delete_then_get_returns_not_found(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "delete_get@example.com").await;

        let note = create_note(
            &pool,
            CreateNoteRequest {
                id: None,
                title: "To Delete".to_string(),
                content: None,
                initiative_id: None,
            },
            user_id,
        )
        .await
        .expect("create_note failed");

        delete_note(&pool, note.id, user_id)
            .await
            .expect("delete_note failed");

        let result = get_note(&pool, note.id, user_id).await;
        assert!(
            matches!(result, Err(AppError::NotFound)),
            "get after delete must return NotFound"
        );
    }

    // list excludes soft-deleted notes
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn list_excludes_soft_deleted_notes(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "soft_delete_list@example.com").await;

        let note = create_note(
            &pool,
            CreateNoteRequest {
                id: None,
                title: "Will Be Deleted".to_string(),
                content: None,
                initiative_id: None,
            },
            user_id,
        )
        .await
        .expect("create_note failed");

        delete_note(&pool, note.id, user_id)
            .await
            .expect("delete_note failed");

        let notes = list_notes(&pool, user_id, None)
            .await
            .expect("list_notes failed");
        assert!(
            notes.iter().all(|n| n.id != note.id),
            "soft-deleted note must not appear in list"
        );
    }

    // ---------------------------------------------------------------------------
    // Snapshot tests (S004-T)
    // ---------------------------------------------------------------------------

    // TC-K-3 partial: snapshot content matches note content at time of capture
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn snapshot_content_matches_note_content_at_capture(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "snapshot_content@example.com").await;

        let note = create_note(
            &pool,
            CreateNoteRequest {
                id: None,
                title: "Snapshot Note".to_string(),
                content: Some("original content".to_string()),
                initiative_id: None,
            },
            user_id,
        )
        .await
        .expect("create_note failed");

        let snapshot = create_snapshot(
            &pool,
            note.id,
            CreateSnapshotRequest {
                captured_at: Utc::now(),
            },
            user_id,
        )
        .await
        .expect("create_snapshot failed");

        assert_eq!(
            snapshot.content, "original content",
            "snapshot content must equal note content at capture time"
        );
    }

    // GET snapshots → ordered by captured_at DESC
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn list_snapshots_ordered_by_captured_at_desc(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "snapshot_order@example.com").await;

        let note = create_note(
            &pool,
            CreateNoteRequest {
                id: None,
                title: "Order Note".to_string(),
                content: Some("content".to_string()),
                initiative_id: None,
            },
            user_id,
        )
        .await
        .expect("create_note failed");

        let now = Utc::now();
        let later = now + Duration::seconds(1);
        let earlier = now - Duration::seconds(1);

        // Insert "later" first to confirm ordering is not insertion order.
        create_snapshot(
            &pool,
            note.id,
            CreateSnapshotRequest { captured_at: later },
            user_id,
        )
        .await
        .expect("create_snapshot later failed");

        create_snapshot(
            &pool,
            note.id,
            CreateSnapshotRequest { captured_at: earlier },
            user_id,
        )
        .await
        .expect("create_snapshot earlier failed");

        let snapshots = list_snapshots(&pool, note.id, user_id)
            .await
            .expect("list_snapshots failed");

        assert_eq!(snapshots.len(), 2, "expected two snapshots");
        assert!(
            snapshots[0].captured_at >= snapshots[1].captured_at,
            "snapshots must be ordered by captured_at DESC"
        );
    }

    // SEC-1: snapshot for note owned by other user → NotFound
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn snapshot_for_other_users_note_returns_not_found(pool: PgPool) {
        let user_a_id = Uuid::new_v4();
        let user_b_id = Uuid::new_v4();
        insert_test_user(&pool, user_a_id, "snap_user_a@example.com").await;
        insert_test_user(&pool, user_b_id, "snap_user_b@example.com").await;

        let note = create_note(
            &pool,
            CreateNoteRequest {
                id: None,
                title: "User A Note".to_string(),
                content: Some("content".to_string()),
                initiative_id: None,
            },
            user_a_id,
        )
        .await
        .expect("create_note failed");

        let result = create_snapshot(
            &pool,
            note.id,
            CreateSnapshotRequest {
                captured_at: Utc::now(),
            },
            user_b_id,
        )
        .await;

        assert!(
            matches!(result, Err(AppError::NotFound)),
            "create_snapshot for another user's note must return NotFound"
        );
    }

    // E-6: update_snapshot and delete_snapshot intentionally absent — snapshots are immutable

    // Snapshot content preserved after note update (A-021)
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn snapshot_content_preserved_after_note_update(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "snap_preserve@example.com").await;

        let note = create_note(
            &pool,
            CreateNoteRequest {
                id: None,
                title: "Preserve Note".to_string(),
                content: Some("original".to_string()),
                initiative_id: None,
            },
            user_id,
        )
        .await
        .expect("create_note failed");

        create_snapshot(
            &pool,
            note.id,
            CreateSnapshotRequest {
                captured_at: Utc::now(),
            },
            user_id,
        )
        .await
        .expect("create_snapshot failed");

        update_note(
            &pool,
            note.id,
            UpdateNoteRequest {
                title: None,
                content: Some("updated".to_string()),
            },
            user_id,
        )
        .await
        .expect("update_note failed");

        let snapshots = list_snapshots(&pool, note.id, user_id)
            .await
            .expect("list_snapshots failed");

        assert_eq!(snapshots.len(), 1, "expected one snapshot");
        assert_eq!(
            snapshots[0].content, "original",
            "snapshot content must remain 'original' after note update"
        );
    }

    // A-021: create_snapshot on soft-deleted note → NotFound (P6-017)
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn create_snapshot_on_deleted_note_returns_not_found(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "snap_deleted@example.com").await;

        let note = create_note(
            &pool,
            CreateNoteRequest {
                id: None,
                title: "To Delete".to_string(),
                content: Some("content".to_string()),
                initiative_id: None,
            },
            user_id,
        )
        .await
        .expect("create_note failed");

        delete_note(&pool, note.id, user_id)
            .await
            .expect("delete_note failed");

        let result = create_snapshot(
            &pool,
            note.id,
            CreateSnapshotRequest {
                captured_at: Utc::now(),
            },
            user_id,
        )
        .await;
        assert!(
            matches!(result, Err(AppError::NotFound)),
            "create_snapshot on soft-deleted note must return NotFound"
        );
    }

    // ---------------------------------------------------------------------------
    // Backlink tests (S005-T)
    // ---------------------------------------------------------------------------

    // TC-K-4: backlink end-to-end
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn backlink_end_to_end(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "backlink_e2e@example.com").await;

        let note_a = create_note(
            &pool,
            CreateNoteRequest {
                id: None,
                title: "Note A".to_string(),
                content: None,
                initiative_id: None,
            },
            user_id,
        )
        .await
        .expect("create_note A failed");

        let note_b = create_note(
            &pool,
            CreateNoteRequest {
                id: None,
                title: "Note B".to_string(),
                content: None,
                initiative_id: None,
            },
            user_id,
        )
        .await
        .expect("create_note B failed");

        sqlx::query(
            "INSERT INTO entity_relations \
             (id, from_entity_type, from_entity_id, to_entity_type, to_entity_id, relation_type, source_type, user_id) \
             VALUES (gen_random_uuid(), 'knowledge_note', $1, 'knowledge_note', $2, 'link', 'manual', $3)",
        )
        .bind(note_a.id)
        .bind(note_b.id)
        .bind(user_id)
        .execute(&pool)
        .await
        .expect("Failed to insert entity_relation");

        let backlinks = list_backlinks(&pool, note_b.id, user_id)
            .await
            .expect("list_backlinks failed");

        assert_eq!(backlinks.len(), 1, "expected one backlink for note B");
        assert_eq!(
            backlinks[0].id, note_a.id,
            "backlink source must be note A"
        );
    }

    // No backlinks → empty list
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn no_backlinks_returns_empty(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "backlink_empty@example.com").await;

        let note_a = create_note(
            &pool,
            CreateNoteRequest {
                id: None,
                title: "Isolated Note".to_string(),
                content: None,
                initiative_id: None,
            },
            user_id,
        )
        .await
        .expect("create_note failed");

        let backlinks = list_backlinks(&pool, note_a.id, user_id)
            .await
            .expect("list_backlinks failed");

        assert!(backlinks.is_empty(), "expected empty backlinks list");
    }

    // SEC-1: backlinks for note owned by other user → NotFound
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn backlinks_for_other_users_note_returns_not_found(pool: PgPool) {
        let user_a_id = Uuid::new_v4();
        let user_b_id = Uuid::new_v4();
        insert_test_user(&pool, user_a_id, "bl_user_a@example.com").await;
        insert_test_user(&pool, user_b_id, "bl_user_b@example.com").await;

        let note = create_note(
            &pool,
            CreateNoteRequest {
                id: None,
                title: "User A Note".to_string(),
                content: None,
                initiative_id: None,
            },
            user_a_id,
        )
        .await
        .expect("create_note failed");

        let result = list_backlinks(&pool, note.id, user_b_id).await;

        assert!(
            matches!(result, Err(AppError::NotFound)),
            "list_backlinks for another user's note must return NotFound"
        );
    }

    // Deleted entity_relation not returned in backlinks
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn deleted_entity_relation_not_returned_in_backlinks(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "backlink_deleted@example.com").await;

        let note_a = create_note(
            &pool,
            CreateNoteRequest {
                id: None,
                title: "Source Note".to_string(),
                content: None,
                initiative_id: None,
            },
            user_id,
        )
        .await
        .expect("create_note A failed");

        let note_b = create_note(
            &pool,
            CreateNoteRequest {
                id: None,
                title: "Target Note".to_string(),
                content: None,
                initiative_id: None,
            },
            user_id,
        )
        .await
        .expect("create_note B failed");

        sqlx::query(
            "INSERT INTO entity_relations \
             (id, from_entity_type, from_entity_id, to_entity_type, to_entity_id, relation_type, source_type, user_id) \
             VALUES (gen_random_uuid(), 'knowledge_note', $1, 'knowledge_note', $2, 'link', 'manual', $3)",
        )
        .bind(note_a.id)
        .bind(note_b.id)
        .bind(user_id)
        .execute(&pool)
        .await
        .expect("Failed to insert entity_relation");

        sqlx::query(
            "UPDATE entity_relations SET deleted_at = NOW() \
             WHERE from_entity_id = $1 AND to_entity_id = $2",
        )
        .bind(note_a.id)
        .bind(note_b.id)
        .execute(&pool)
        .await
        .expect("Failed to soft-delete entity_relation");

        let backlinks = list_backlinks(&pool, note_b.id, user_id)
            .await
            .expect("list_backlinks failed");

        assert!(
            backlinks.is_empty(),
            "deleted entity_relation must not appear in backlinks"
        );
    }

    // ---------------------------------------------------------------------------
    // Cross-user mutation isolation (SEC-1, P6-014, P6-015)
    // ---------------------------------------------------------------------------

    // SEC-1: User B cannot update User A's note (P6-014)
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn update_other_users_note_returns_not_found(pool: PgPool) {
        let user_a_id = Uuid::new_v4();
        let user_b_id = Uuid::new_v4();
        insert_test_user(&pool, user_a_id, "update_iso_a@example.com").await;
        insert_test_user(&pool, user_b_id, "update_iso_b@example.com").await;

        let note = create_note(
            &pool,
            CreateNoteRequest {
                id: None,
                title: "User A Note".to_string(),
                content: None,
                initiative_id: None,
            },
            user_a_id,
        )
        .await
        .expect("create_note failed");

        let result = update_note(
            &pool,
            note.id,
            UpdateNoteRequest {
                title: Some("Tampered".to_string()),
                content: None,
            },
            user_b_id,
        )
        .await;
        assert!(
            matches!(result, Err(AppError::NotFound)),
            "User B must not update User A's note"
        );
    }

    // SEC-1: User B cannot delete User A's note (P6-015)
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn delete_other_users_note_returns_not_found(pool: PgPool) {
        let user_a_id = Uuid::new_v4();
        let user_b_id = Uuid::new_v4();
        insert_test_user(&pool, user_a_id, "delete_iso_a@example.com").await;
        insert_test_user(&pool, user_b_id, "delete_iso_b@example.com").await;

        let note = create_note(
            &pool,
            CreateNoteRequest {
                id: None,
                title: "User A Note".to_string(),
                content: None,
                initiative_id: None,
            },
            user_a_id,
        )
        .await
        .expect("create_note failed");

        let result = delete_note(&pool, note.id, user_b_id).await;
        assert!(
            matches!(result, Err(AppError::NotFound)),
            "User B must not delete User A's note"
        );
    }
}
