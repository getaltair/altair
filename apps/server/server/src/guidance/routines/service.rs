use chrono::NaiveDate;
use sqlx::PgPool;
use sqlx::types::Json;
use uuid::Uuid;

use super::models::{CreateRoutineRequest, FrequencyConfig, Routine, UpdateRoutineRequest};
use crate::error::AppError;
use crate::guidance::quests::models::Quest;

const ROUTINE_COLUMNS: &str = "id, user_id, title, description, frequency_type, frequency_config, \
     status, created_at, updated_at, deleted_at";

const QUEST_COLUMNS: &str = "id, user_id, initiative_id, epic_id, routine_id, title, description, \
     status, priority, due_date, created_at, updated_at, deleted_at";

pub async fn list_routines(pool: &PgPool, user_id: Uuid) -> Result<Vec<Routine>, AppError> {
    let rows = sqlx::query_as::<_, Routine>(&format!(
        "SELECT {ROUTINE_COLUMNS} \
         FROM guidance_routines \
         WHERE user_id = $1 AND deleted_at IS NULL \
         ORDER BY created_at ASC"
    ))
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_routine(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<Routine, AppError> {
    let row = sqlx::query_as::<_, Routine>(&format!(
        "SELECT {ROUTINE_COLUMNS} \
         FROM guidance_routines \
         WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL"
    ))
    .bind(id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    row.ok_or(AppError::NotFound)
}

pub async fn create_routine(
    pool: &PgPool,
    user_id: Uuid,
    req: CreateRoutineRequest,
) -> Result<Routine, AppError> {
    if let Some(err_msg) = req.frequency_config.validate() {
        return Err(AppError::UnprocessableEntity(err_msg));
    }

    let frequency_type = req.frequency_config.frequency_type();
    let status = if req.is_active.unwrap_or(true) {
        "active"
    } else {
        "paused"
    };

    let routine = sqlx::query_as::<_, Routine>(&format!(
        "INSERT INTO guidance_routines \
            (id, user_id, title, description, frequency_type, frequency_config, status) \
         VALUES \
            (gen_random_uuid(), $1, $2, $3, $4, $5, $6) \
         RETURNING {ROUTINE_COLUMNS}"
    ))
    .bind(user_id)
    .bind(&req.title)
    .bind(&req.description)
    .bind(&frequency_type)
    .bind(Json(&req.frequency_config))
    .bind(status)
    .fetch_one(pool)
    .await?;

    // A-G-09: spawn the first quest occurrence on routine creation.
    let today = chrono::Local::now().date_naive();
    spawn_routine_quest(pool, routine.id, user_id, today).await?;

    Ok(routine)
}

pub async fn update_routine(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    req: UpdateRoutineRequest,
) -> Result<Routine, AppError> {
    if let Some(err_msg) = req.frequency_config.as_ref().and_then(|c| c.validate()) {
        return Err(AppError::UnprocessableEntity(err_msg));
    }

    // Fetch current to derive frequency_type if config is changing.
    let current = get_routine(pool, id, user_id).await?;

    let new_config: Option<Json<FrequencyConfig>> =
        req.frequency_config.as_ref().map(|c| Json(c.clone()));
    let new_freq_type = req
        .frequency_config
        .as_ref()
        .map(|c| c.frequency_type())
        .unwrap_or_else(|| current.frequency_type.clone());
    let new_status = req.is_active.map(|a| if a { "active" } else { "paused" });

    let row = sqlx::query_as::<_, Routine>(&format!(
        "UPDATE guidance_routines \
         SET \
           title            = COALESCE($3, title), \
           description      = COALESCE($4, description), \
           frequency_config = COALESCE($5, frequency_config), \
           frequency_type   = $6, \
           status           = COALESCE($7, status), \
           updated_at       = NOW() \
         WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL \
         RETURNING {ROUTINE_COLUMNS}"
    ))
    .bind(id)
    .bind(user_id)
    .bind(&req.title)
    .bind(&req.description)
    .bind(new_config)
    .bind(&new_freq_type)
    .bind(new_status)
    .fetch_optional(pool)
    .await?;

    row.ok_or(AppError::NotFound)
}

pub async fn delete_routine(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query(
        "UPDATE guidance_routines \
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

/// Spawn a quest occurrence for the given routine on `due_date`.
/// Idempotent: if a non-deleted quest with `routine_id = id AND due_date = due_date`
/// already exists, returns it without inserting a duplicate.
pub async fn spawn_routine_quest(
    pool: &PgPool,
    routine_id: Uuid,
    user_id: Uuid,
    due_date: NaiveDate,
) -> Result<Quest, AppError> {
    // Check for an existing quest for this routine+date (idempotency guard).
    let existing = sqlx::query_as::<_, Quest>(&format!(
        "SELECT {QUEST_COLUMNS} \
         FROM guidance_quests \
         WHERE routine_id = $1 AND due_date = $2 AND user_id = $3 AND deleted_at IS NULL"
    ))
    .bind(routine_id)
    .bind(due_date)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    if let Some(q) = existing {
        return Ok(q);
    }

    // Fetch the routine title to use for the quest.
    let routine = sqlx::query_as::<_, Routine>(&format!(
        "SELECT {ROUTINE_COLUMNS} \
         FROM guidance_routines \
         WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL"
    ))
    .bind(routine_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound)?;

    let quest = sqlx::query_as::<_, Quest>(&format!(
        "INSERT INTO guidance_quests \
            (id, user_id, routine_id, title, status, priority, due_date) \
         VALUES \
            (gen_random_uuid(), $1, $2, $3, 'not_started', 'medium', $4) \
         RETURNING {QUEST_COLUMNS}"
    ))
    .bind(user_id)
    .bind(routine_id)
    .bind(&routine.title)
    .bind(due_date)
    .fetch_one(pool)
    .await?;

    Ok(quest)
}

// ---------------------------------------------------------------------------
// Tests (S007-T) — sqlx::test integration tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::AppError;
    use crate::guidance::routines::models::{
        CreateRoutineRequest, FrequencyConfig, UpdateRoutineRequest,
    };
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

    fn daily_req(title: &str) -> CreateRoutineRequest {
        CreateRoutineRequest {
            title: title.to_string(),
            description: None,
            frequency_config: FrequencyConfig::Daily,
            is_active: None,
        }
    }

    // A-G-08: weekly routine with empty days_of_week returns UnprocessableEntity.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn create_weekly_routine_with_empty_days_returns_unprocessable(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "weekly@example.com").await;

        let result = create_routine(
            &pool,
            user_id,
            CreateRoutineRequest {
                title: "Empty weekly".to_string(),
                description: None,
                frequency_config: FrequencyConfig::Weekly {
                    days_of_week: vec![],
                },
                is_active: None,
            },
        )
        .await;

        assert!(
            matches!(result, Err(AppError::UnprocessableEntity(_))),
            "expected UnprocessableEntity, got: {result:?}"
        );
    }

    // A-G-09: creating a valid daily routine succeeds and spawns first quest with routine_id set.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn create_daily_routine_spawns_first_quest(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "daily@example.com").await;

        let routine = create_routine(&pool, user_id, daily_req("Morning Run"))
            .await
            .expect("create_routine should succeed");

        assert_eq!(routine.title, "Morning Run");
        assert_eq!(routine.status, "active");

        // Verify at least one quest was spawned with routine_id set.
        let quest_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM guidance_quests \
             WHERE routine_id = $1 AND user_id = $2 AND deleted_at IS NULL",
        )
        .bind(routine.id)
        .bind(user_id)
        .fetch_one(&pool)
        .await
        .expect("count query failed");

        assert_eq!(quest_count, 1, "exactly one quest should be spawned");

        // Confirm the spawned quest has routine_id set.
        let routine_id_on_quest: Option<Uuid> = sqlx::query_scalar(
            "SELECT routine_id FROM guidance_quests \
             WHERE routine_id = $1 AND deleted_at IS NULL",
        )
        .bind(routine.id)
        .fetch_one(&pool)
        .await
        .expect("quest lookup failed");

        assert_eq!(
            routine_id_on_quest,
            Some(routine.id),
            "quest must have routine_id set"
        );
    }

    // spawn_routine_quest called twice for same routine+date does not duplicate.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn spawn_routine_quest_is_idempotent(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "idempotent@example.com").await;

        let routine = create_routine(&pool, user_id, daily_req("Idempotent Routine"))
            .await
            .expect("create_routine should succeed");

        let today = chrono::Local::now().date_naive();

        // Second spawn call for the same date.
        spawn_routine_quest(&pool, routine.id, user_id, today)
            .await
            .expect("second spawn should succeed");

        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM guidance_quests \
             WHERE routine_id = $1 AND due_date = $2 AND deleted_at IS NULL",
        )
        .bind(routine.id)
        .bind(today)
        .fetch_one(&pool)
        .await
        .expect("count query failed");

        assert_eq!(count, 1, "spawn must be idempotent — exactly one quest");
    }

    // routine soft-delete excludes from list.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn soft_delete_excludes_from_list(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "softdel@example.com").await;

        let routine = create_routine(&pool, user_id, daily_req("To Delete"))
            .await
            .expect("create_routine should succeed");

        let before = list_routines(&pool, user_id)
            .await
            .expect("list_routines failed");
        assert_eq!(before.len(), 1);

        delete_routine(&pool, routine.id, user_id)
            .await
            .expect("delete_routine failed");

        let after = list_routines(&pool, user_id)
            .await
            .expect("list_routines failed");
        assert!(
            after.is_empty(),
            "soft-deleted routine must not appear in list"
        );
    }

    // frequency_config JSONB round-trips correctly through serde (create then get).
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn frequency_config_jsonb_round_trips(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "jsonb@example.com").await;

        let routine = create_routine(
            &pool,
            user_id,
            CreateRoutineRequest {
                title: "Weekly Routine".to_string(),
                description: None,
                frequency_config: FrequencyConfig::Weekly {
                    days_of_week: vec!["monday".to_string(), "friday".to_string()],
                },
                is_active: None,
            },
        )
        .await
        .expect("create_routine should succeed");

        let fetched = get_routine(&pool, routine.id, user_id)
            .await
            .expect("get_routine should succeed");

        match &fetched.frequency_config.0 {
            FrequencyConfig::Weekly { days_of_week } => {
                assert_eq!(
                    days_of_week,
                    &vec!["monday".to_string(), "friday".to_string()],
                    "days_of_week must round-trip correctly"
                );
            }
            other => panic!("expected FrequencyConfig::Weekly, got: {other:?}"),
        }
    }

    // interval with interval_days <= 0 returns UnprocessableEntity.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn create_interval_routine_with_zero_days_returns_unprocessable(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "interval@example.com").await;

        let result = create_routine(
            &pool,
            user_id,
            CreateRoutineRequest {
                title: "Bad Interval".to_string(),
                description: None,
                frequency_config: FrequencyConfig::Interval { interval_days: 0 },
                is_active: None,
            },
        )
        .await;

        assert!(
            matches!(result, Err(AppError::UnprocessableEntity(_))),
            "expected UnprocessableEntity, got: {result:?}"
        );
    }

    // partial update only changes specified fields.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn partial_update_leaves_unspecified_fields_unchanged(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "update@example.com").await;

        let routine = create_routine(&pool, user_id, daily_req("Original"))
            .await
            .expect("create_routine should succeed");

        let updated = update_routine(
            &pool,
            routine.id,
            user_id,
            UpdateRoutineRequest {
                title: Some("Updated Title".to_string()),
                description: None,
                frequency_config: None,
                is_active: None,
            },
        )
        .await
        .expect("update_routine should succeed");

        assert_eq!(updated.title, "Updated Title");
        assert_eq!(updated.status, "active", "status must remain active");
    }
}
