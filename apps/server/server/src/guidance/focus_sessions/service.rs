use sqlx::PgPool;
use uuid::Uuid;

use super::models::{CreateFocusSessionRequest, FocusSession, UpdateFocusSessionRequest};
use crate::error::AppError;

const SESSION_COLUMNS: &str =
    "id, quest_id, started_at, ended_at, duration_minutes, user_id, created_at, updated_at, deleted_at";

pub async fn list_sessions(
    pool: &PgPool,
    user_id: Uuid,
    quest_id: Option<Uuid>,
) -> Result<Vec<FocusSession>, AppError> {
    let rows = sqlx::query_as::<_, FocusSession>(&format!(
        "SELECT {SESSION_COLUMNS} \
         FROM guidance_focus_sessions \
         WHERE user_id = $1 \
           AND deleted_at IS NULL \
           AND ($2::uuid IS NULL OR quest_id = $2) \
         ORDER BY started_at ASC"
    ))
    .bind(user_id)
    .bind(quest_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_session(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<FocusSession, AppError> {
    let row = sqlx::query_as::<_, FocusSession>(&format!(
        "SELECT {SESSION_COLUMNS} \
         FROM guidance_focus_sessions \
         WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL"
    ))
    .bind(id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    row.ok_or(AppError::NotFound)
}

pub async fn create_session(
    pool: &PgPool,
    user_id: Uuid,
    req: CreateFocusSessionRequest,
) -> Result<FocusSession, AppError> {
    // Verify quest exists and belongs to the calling user.
    let quest_status: Option<String> = sqlx::query_scalar(
        "SELECT status FROM guidance_quests \
         WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL",
    )
    .bind(req.quest_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    let quest_status = quest_status.ok_or(AppError::NotFound)?;

    // A-G-10: auto-transition not_started → in_progress and the session INSERT must be
    // atomic. If the INSERT fails after the UPDATE, the quest would be permanently stuck
    // in_progress with no session attached.
    let mut tx = pool.begin().await?;

    if quest_status == "not_started" {
        sqlx::query(
            "UPDATE guidance_quests \
             SET status = 'in_progress', updated_at = NOW() \
             WHERE id = $1 AND user_id = $2",
        )
        .bind(req.quest_id)
        .bind(user_id)
        .execute(&mut *tx)
        .await?;
    }
    // If already in_progress, leave unchanged.

    let session = sqlx::query_as::<_, FocusSession>(&format!(
        "INSERT INTO guidance_focus_sessions \
            (id, user_id, quest_id, started_at) \
         VALUES \
            (gen_random_uuid(), $1, $2, COALESCE($3, NOW())) \
         RETURNING {SESSION_COLUMNS}"
    ))
    .bind(user_id)
    .bind(req.quest_id)
    .bind(req.started_at)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(session)
}

pub async fn update_session(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    req: UpdateFocusSessionRequest,
) -> Result<FocusSession, AppError> {
    // Fetch current session to verify ownership and get started_at for duration computation.
    let current = get_session(pool, id, user_id).await?;

    let updated = if let Some(ended_at) = req.ended_at {
        // Reject negative or zero durations — clock drift or malformed client input.
        if ended_at <= current.started_at {
            return Err(AppError::UnprocessableEntity(
                "ended_at must be after started_at".to_string(),
            ));
        }

        // A-G-11: compute duration_minutes = floor((ended_at - started_at).num_seconds() / 60).
        let duration_secs = (ended_at - current.started_at).num_seconds();
        let duration_minutes = (duration_secs / 60) as i32;

        sqlx::query_as::<_, FocusSession>(&format!(
            "UPDATE guidance_focus_sessions \
             SET ended_at = $3, duration_minutes = $4, updated_at = NOW() \
             WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL \
             RETURNING {SESSION_COLUMNS}"
        ))
        .bind(id)
        .bind(user_id)
        .bind(ended_at)
        .bind(duration_minutes)
        .fetch_optional(pool)
        .await?
        .ok_or(AppError::NotFound)?
    } else {
        // No ended_at provided — nothing to update; return current state.
        current
    };

    Ok(updated)
}

pub async fn delete_session(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query(
        "UPDATE guidance_focus_sessions \
         SET deleted_at = NOW(), updated_at = NOW() \
         WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL",
    )
    .bind(id)
    .bind(user_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Tests (S008-T) — sqlx::test integration tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::guidance::focus_sessions::models::{
        CreateFocusSessionRequest, UpdateFocusSessionRequest,
    };
    use chrono::{Duration, SubsecRound, Utc};
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

    async fn insert_test_quest(
        pool: &PgPool,
        quest_id: Uuid,
        user_id: Uuid,
        status: &str,
    ) {
        sqlx::query(
            "INSERT INTO guidance_quests \
                (id, user_id, title, status, priority) \
             VALUES ($1, $2, 'Test Quest', $3, 'medium')",
        )
        .bind(quest_id)
        .bind(user_id)
        .bind(status)
        .execute(pool)
        .await
        .expect("Failed to insert test quest");
    }

    // A-G-10: creating a session on a not_started quest transitions it to in_progress.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn create_session_on_not_started_quest_transitions_to_in_progress(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let quest_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "fs1@example.com").await;
        insert_test_quest(&pool, quest_id, user_id, "not_started").await;

        create_session(
            &pool,
            user_id,
            CreateFocusSessionRequest {
                quest_id,
                started_at: None,
            },
        )
        .await
        .expect("create_session should succeed");

        let quest_status: String =
            sqlx::query_scalar("SELECT status FROM guidance_quests WHERE id = $1")
                .bind(quest_id)
                .fetch_one(&pool)
                .await
                .unwrap();

        assert_eq!(
            quest_status, "in_progress",
            "not_started quest must transition to in_progress"
        );
    }

    // Creating a session on an already in_progress quest leaves quest status unchanged.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn create_session_on_in_progress_quest_leaves_status_unchanged(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let quest_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "fs2@example.com").await;
        insert_test_quest(&pool, quest_id, user_id, "in_progress").await;

        create_session(
            &pool,
            user_id,
            CreateFocusSessionRequest {
                quest_id,
                started_at: None,
            },
        )
        .await
        .expect("create_session should succeed");

        let quest_status: String =
            sqlx::query_scalar("SELECT status FROM guidance_quests WHERE id = $1")
                .bind(quest_id)
                .fetch_one(&pool)
                .await
                .unwrap();

        assert_eq!(
            quest_status, "in_progress",
            "in_progress quest must remain in_progress"
        );
    }

    // A-G-11: PATCH with ended_at = started_at + 90s → duration_minutes = 1 (floor(90/60)).
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn update_with_ended_at_computes_duration_floor_90s(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let quest_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "fs3@example.com").await;
        insert_test_quest(&pool, quest_id, user_id, "in_progress").await;

        let started_at = Utc::now() - Duration::hours(1);
        let session = create_session(
            &pool,
            user_id,
            CreateFocusSessionRequest {
                quest_id,
                started_at: Some(started_at),
            },
        )
        .await
        .expect("create_session should succeed");

        let ended_at = started_at + Duration::seconds(90);
        let updated = update_session(
            &pool,
            session.id,
            user_id,
            UpdateFocusSessionRequest {
                ended_at: Some(ended_at),
            },
        )
        .await
        .expect("update_session should succeed");

        assert_eq!(
            updated.duration_minutes,
            Some(1),
            "floor(90s / 60) must be 1"
        );
        // Postgres stores timestamptz at microsecond precision; truncate before comparing.
        assert_eq!(updated.ended_at, Some(ended_at.trunc_subsecs(6)));
    }

    // A-G-11: PATCH with ended_at = started_at + 120s → duration_minutes = 2.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn update_with_ended_at_computes_duration_exact_120s(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let quest_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "fs4@example.com").await;
        insert_test_quest(&pool, quest_id, user_id, "in_progress").await;

        let started_at = Utc::now() - Duration::hours(1);
        let session = create_session(
            &pool,
            user_id,
            CreateFocusSessionRequest {
                quest_id,
                started_at: Some(started_at),
            },
        )
        .await
        .expect("create_session should succeed");

        let ended_at = started_at + Duration::seconds(120);
        let updated = update_session(
            &pool,
            session.id,
            user_id,
            UpdateFocusSessionRequest {
                ended_at: Some(ended_at),
            },
        )
        .await
        .expect("update_session should succeed");

        assert_eq!(
            updated.duration_minutes,
            Some(2),
            "floor(120s / 60) must be 2"
        );
    }

    // Verify floor behavior: 119 seconds → 1 minute, not 2.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn update_duration_floors_correctly_119s(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let quest_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "fs5@example.com").await;
        insert_test_quest(&pool, quest_id, user_id, "in_progress").await;

        let started_at = Utc::now() - Duration::hours(1);
        let session = create_session(
            &pool,
            user_id,
            CreateFocusSessionRequest {
                quest_id,
                started_at: Some(started_at),
            },
        )
        .await
        .expect("create_session should succeed");

        let ended_at = started_at + Duration::seconds(119);
        let updated = update_session(
            &pool,
            session.id,
            user_id,
            UpdateFocusSessionRequest {
                ended_at: Some(ended_at),
            },
        )
        .await
        .expect("update_session should succeed");

        assert_eq!(
            updated.duration_minutes,
            Some(1),
            "floor(119s / 60) must be 1, not 2"
        );
    }

    // Soft-delete excludes session from list.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn soft_delete_excludes_from_list(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let quest_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "fs6@example.com").await;
        insert_test_quest(&pool, quest_id, user_id, "in_progress").await;

        let session = create_session(
            &pool,
            user_id,
            CreateFocusSessionRequest {
                quest_id,
                started_at: None,
            },
        )
        .await
        .expect("create_session should succeed");

        let before = list_sessions(&pool, user_id, None).await.unwrap();
        assert_eq!(before.len(), 1);

        delete_session(&pool, session.id, user_id).await.unwrap();

        let after = list_sessions(&pool, user_id, None).await.unwrap();
        assert!(after.is_empty(), "soft-deleted session must not appear in list");
    }

    // create_session returns NotFound when quest belongs to a different user.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn create_session_with_other_users_quest_returns_not_found(pool: PgPool) {
        let owner = Uuid::new_v4();
        let attacker = Uuid::new_v4();
        let quest_id = Uuid::new_v4();
        insert_test_user(&pool, owner, "owner_fs@example.com").await;
        insert_test_user(&pool, attacker, "attacker_fs@example.com").await;
        insert_test_quest(&pool, quest_id, owner, "not_started").await;

        let result = create_session(
            &pool,
            attacker,
            CreateFocusSessionRequest {
                quest_id,
                started_at: None,
            },
        )
        .await;

        assert!(
            matches!(result, Err(AppError::NotFound)),
            "must return NotFound for another user's quest, got: {result:?}"
        );
    }
}
