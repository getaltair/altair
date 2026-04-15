use sqlx::PgPool;
use uuid::Uuid;

use super::models::{CreateQuestRequest, Quest, QuestListParams, QuestStatus, UpdateQuestRequest};
use crate::error::AppError;
use crate::guidance::epics::models::EpicStatus;
use crate::guidance::epics::service::check_initiative_ownership;

const QUEST_COLUMNS: &str =
    "id, user_id, initiative_id, epic_id, routine_id, title, description, \
     status, priority, due_date, created_at, updated_at, deleted_at";

pub async fn list_quests(
    pool: &PgPool,
    user_id: Uuid,
    params: QuestListParams,
) -> Result<Vec<Quest>, AppError> {
    // Single fixed-arity query; $N IS NULL means "no filter applied".
    // The due_date filter additionally excludes terminal statuses (A-G-15).
    let rows = sqlx::query_as::<_, Quest>(&format!(
        "SELECT {QUEST_COLUMNS} \
         FROM guidance_quests \
         WHERE user_id = $1 \
           AND deleted_at IS NULL \
           AND ($2::varchar IS NULL OR status = $2) \
           AND ($3::varchar IS NULL OR priority = $3) \
           AND ($4::date   IS NULL OR (due_date = $4 \
                AND status NOT IN ('completed', 'cancelled', 'deferred'))) \
           AND ($5::uuid   IS NULL OR initiative_id = $5) \
         ORDER BY created_at ASC"
    ))
    .bind(user_id)
    .bind(params.status)
    .bind(params.priority)
    .bind(params.due_date)
    .bind(params.initiative_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_quest(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<Quest, AppError> {
    let row = sqlx::query_as::<_, Quest>(&format!(
        "SELECT {QUEST_COLUMNS} \
         FROM guidance_quests \
         WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL"
    ))
    .bind(id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    row.ok_or(AppError::NotFound)
}

pub async fn create_quest(
    pool: &PgPool,
    user_id: Uuid,
    req: CreateQuestRequest,
) -> Result<Quest, AppError> {
    if let Some(init_id) = req.initiative_id {
        check_initiative_ownership(pool, init_id, user_id).await?;
    }

    let row = sqlx::query_as::<_, Quest>(&format!(
        "INSERT INTO guidance_quests \
            (id, user_id, initiative_id, epic_id, title, description, status, priority, due_date) \
         VALUES \
            (gen_random_uuid(), $1, $2, $3, $4, $5, \
             COALESCE($6, 'not_started'), COALESCE($7, 'medium'), $8) \
         RETURNING {QUEST_COLUMNS}"
    ))
    .bind(user_id)
    .bind(req.initiative_id)
    .bind(req.epic_id)
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.status)
    .bind(&req.priority)
    .bind(req.due_date)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn update_quest(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    req: UpdateQuestRequest,
) -> Result<Quest, AppError> {
    // Fetch current quest first to validate the status transition.
    let current = get_quest(pool, id, user_id).await?;

    if req
        .status
        .as_ref()
        .is_some_and(|s| !current.status.can_transition_to(s))
    {
        return Err(AppError::UnprocessableEntity(
            "Invalid status transition".to_string(),
        ));
    }

    if let Some(init_id) = req.initiative_id {
        check_initiative_ownership(pool, init_id, user_id).await?;
    }

    let transitioning_to_completed =
        matches!(&req.status, Some(s) if *s == QuestStatus::Completed);
    let transitioning_to_in_progress =
        matches!(&req.status, Some(s) if *s == QuestStatus::InProgress);

    // Trigger epic recalculation when transitioning to in_progress (not_started → in_progress
    // should flip the epic not_started → in_progress) or completed. Both need a transaction
    // to keep the quest update and epic status change atomic.
    if let Some(epic_id) = current
        .epic_id
        .filter(|_| transitioning_to_completed || transitioning_to_in_progress)
    {
        let mut tx = pool.begin().await?;

        let updated = sqlx::query_as::<_, Quest>(&format!(
            "UPDATE guidance_quests \
             SET \
               initiative_id = COALESCE($3, initiative_id), \
               epic_id       = COALESCE($4, epic_id), \
               title         = COALESCE($5, title), \
               description   = COALESCE($6, description), \
               status        = COALESCE($7, status), \
               priority      = COALESCE($8, priority), \
               due_date      = COALESCE($9, due_date), \
               updated_at    = NOW() \
             WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL \
             RETURNING {QUEST_COLUMNS}"
        ))
        .bind(id)
        .bind(user_id)
        .bind(req.initiative_id)
        .bind(req.epic_id)
        .bind(&req.title)
        .bind(&req.description)
        .bind(&req.status)
        .bind(&req.priority)
        .bind(req.due_date)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(AppError::NotFound)?;

        #[allow(clippy::explicit_auto_deref)]
        recalculate_epic_status(&mut *tx, epic_id, user_id).await?;

        tx.commit().await?;

        if transitioning_to_completed {
            tracing::info!(quest_id = %id, "QuestCompleted");
        }
        return Ok(updated);
    }

    let updated = sqlx::query_as::<_, Quest>(&format!(
        "UPDATE guidance_quests \
         SET \
           initiative_id = COALESCE($3, initiative_id), \
           epic_id       = COALESCE($4, epic_id), \
           title         = COALESCE($5, title), \
           description   = COALESCE($6, description), \
           status        = COALESCE($7, status), \
           priority      = COALESCE($8, priority), \
           due_date      = COALESCE($9, due_date), \
           updated_at    = NOW() \
         WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL \
         RETURNING {QUEST_COLUMNS}"
    ))
    .bind(id)
    .bind(user_id)
    .bind(req.initiative_id)
    .bind(req.epic_id)
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.status)
    .bind(&req.priority)
    .bind(req.due_date)
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound)?;

    if transitioning_to_completed {
        tracing::info!(quest_id = %id, "QuestCompleted");
    }

    Ok(updated)
}

pub async fn delete_quest(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query(
        "UPDATE guidance_quests \
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

/// Recompute and persist the epic's derived status from its child quests.
/// Must be called within an open transaction.
async fn recalculate_epic_status(
    tx: &mut sqlx::PgConnection,
    epic_id: Uuid,
    user_id: Uuid,
) -> Result<(), AppError> {
    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM guidance_quests \
         WHERE epic_id = $1 AND deleted_at IS NULL AND status != 'cancelled'",
    )
    .bind(epic_id)
    .fetch_one(&mut *tx)
    .await?;

    let completed: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM guidance_quests \
         WHERE epic_id = $1 AND deleted_at IS NULL AND status = 'completed'",
    )
    .bind(epic_id)
    .fetch_one(&mut *tx)
    .await?;

    let in_progress: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM guidance_quests \
         WHERE epic_id = $1 AND deleted_at IS NULL AND status = 'in_progress'",
    )
    .bind(epic_id)
    .fetch_one(&mut *tx)
    .await?;

    let new_status = if total > 0 && completed == total {
        EpicStatus::Completed
    } else if in_progress > 0 {
        EpicStatus::InProgress
    } else {
        EpicStatus::NotStarted
    };

    sqlx::query(
        "UPDATE guidance_epics SET status = $1, updated_at = NOW() WHERE id = $2 AND user_id = $3",
    )
    .bind(new_status)
    .bind(epic_id)
    .bind(user_id)
    .execute(&mut *tx)
    .await?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Tests (S005-T) — sqlx::test integration tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::AppError;
    use crate::guidance::quests::models::{
        CreateQuestRequest, QuestListParams, QuestStatus, UpdateQuestRequest,
    };
    use chrono::Local;
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

    async fn insert_test_initiative(
        pool: &PgPool,
        initiative_id: Uuid,
        user_id: Uuid,
        title: &str,
    ) {
        sqlx::query(
            "INSERT INTO initiatives (id, user_id, title, status) \
             VALUES ($1, $2, $3, 'draft')",
        )
        .bind(initiative_id)
        .bind(user_id)
        .bind(title)
        .execute(pool)
        .await
        .expect("Failed to insert test initiative");
    }

    async fn insert_test_epic(
        pool: &PgPool,
        epic_id: Uuid,
        user_id: Uuid,
        initiative_id: Uuid,
        title: &str,
    ) {
        sqlx::query(
            "INSERT INTO guidance_epics \
                (id, user_id, initiative_id, title, status, sort_order) \
             VALUES ($1, $2, $3, $4, 'not_started', 0)",
        )
        .bind(epic_id)
        .bind(user_id)
        .bind(initiative_id)
        .bind(title)
        .execute(pool)
        .await
        .expect("Failed to insert test epic");
    }

    fn make_create_req(title: &str) -> CreateQuestRequest {
        CreateQuestRequest {
            initiative_id: None,
            epic_id: None,
            title: title.to_string(),
            description: None,
            status: None,
            priority: None,
            due_date: None,
        }
    }

    fn no_update() -> UpdateQuestRequest {
        UpdateQuestRequest {
            initiative_id: None,
            epic_id: None,
            title: None,
            description: None,
            status: None,
            priority: None,
            due_date: None,
        }
    }

    // create quest returns row with correct user_id
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn create_quest_returns_correct_user_id(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "user@example.com").await;

        let quest = create_quest(&pool, user_id, make_create_req("My Quest"))
            .await
            .expect("create_quest should succeed");

        assert_eq!(quest.user_id, user_id);
        assert_eq!(quest.title, "My Quest");
        assert_eq!(quest.status, QuestStatus::NotStarted);
        assert!(quest.deleted_at.is_none());
    }

    // A-G-07: create quest with initiative_id from other user returns Forbidden
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn create_quest_with_other_users_initiative_returns_forbidden(pool: PgPool) {
        let owner = Uuid::new_v4();
        let attacker = Uuid::new_v4();
        let initiative_id = Uuid::new_v4();
        insert_test_user(&pool, owner, "owner@example.com").await;
        insert_test_user(&pool, attacker, "attacker@example.com").await;
        insert_test_initiative(&pool, initiative_id, owner, "Owner's Init").await;

        let result = create_quest(
            &pool,
            attacker,
            CreateQuestRequest {
                initiative_id: Some(initiative_id),
                ..make_create_req("Malicious Quest")
            },
        )
        .await;

        assert!(
            matches!(result, Err(AppError::Forbidden)),
            "expected Forbidden, got: {result:?}"
        );
    }

    // A-G-03: list returns only calling user's quests
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn list_returns_only_calling_users_quests(pool: PgPool) {
        let user1 = Uuid::new_v4();
        let user2 = Uuid::new_v4();
        insert_test_user(&pool, user1, "user1@example.com").await;
        insert_test_user(&pool, user2, "user2@example.com").await;

        create_quest(&pool, user1, make_create_req("User1 Quest"))
            .await
            .unwrap();
        create_quest(&pool, user2, make_create_req("User2 Quest"))
            .await
            .unwrap();

        let user1_quests = list_quests(
            &pool,
            user1,
            QuestListParams {
                status: None,
                priority: None,
                due_date: None,
                initiative_id: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(user1_quests.len(), 1);
        assert_eq!(user1_quests[0].user_id, user1);
    }

    // A-G-04: PATCH status not_started → in_progress succeeds
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn patch_status_not_started_to_in_progress_succeeds(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "patch1@example.com").await;

        let quest = create_quest(&pool, user_id, make_create_req("Quest"))
            .await
            .unwrap();

        let updated = update_quest(
            &pool,
            quest.id,
            user_id,
            UpdateQuestRequest {
                status: Some(QuestStatus::InProgress),
                ..no_update()
            },
        )
        .await
        .expect("transition should succeed");

        assert_eq!(updated.status, QuestStatus::InProgress);
    }

    // A-G-05: PATCH status not_started → completed returns UnprocessableEntity
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn patch_status_not_started_to_completed_returns_unprocessable(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "patch2@example.com").await;

        let quest = create_quest(&pool, user_id, make_create_req("Quest"))
            .await
            .unwrap();

        let result = update_quest(
            &pool,
            quest.id,
            user_id,
            UpdateQuestRequest {
                status: Some(QuestStatus::Completed),
                ..no_update()
            },
        )
        .await;

        assert!(
            matches!(result, Err(AppError::UnprocessableEntity(_))),
            "expected UnprocessableEntity, got: {result:?}"
        );
    }

    // A-G-06: PATCH status completed → not_started returns UnprocessableEntity
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn patch_status_completed_to_not_started_returns_unprocessable(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "patch3@example.com").await;

        let quest = create_quest(&pool, user_id, make_create_req("Quest"))
            .await
            .unwrap();

        // Advance to in_progress then completed.
        update_quest(
            &pool,
            quest.id,
            user_id,
            UpdateQuestRequest {
                status: Some(QuestStatus::InProgress),
                ..no_update()
            },
        )
        .await
        .unwrap();
        update_quest(
            &pool,
            quest.id,
            user_id,
            UpdateQuestRequest {
                status: Some(QuestStatus::Completed),
                ..no_update()
            },
        )
        .await
        .unwrap();

        let result = update_quest(
            &pool,
            quest.id,
            user_id,
            UpdateQuestRequest {
                status: Some(QuestStatus::NotStarted),
                ..no_update()
            },
        )
        .await;

        assert!(
            matches!(result, Err(AppError::UnprocessableEntity(_))),
            "expected UnprocessableEntity, got: {result:?}"
        );
    }

    // A-G-07: PATCH initiative_id to other user's initiative returns Forbidden
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn patch_initiative_id_to_other_users_initiative_returns_forbidden(pool: PgPool) {
        let user1 = Uuid::new_v4();
        let user2 = Uuid::new_v4();
        let other_initiative = Uuid::new_v4();
        insert_test_user(&pool, user1, "u1@example.com").await;
        insert_test_user(&pool, user2, "u2@example.com").await;
        insert_test_initiative(&pool, other_initiative, user2, "User2 Init").await;

        let quest = create_quest(&pool, user1, make_create_req("Quest"))
            .await
            .unwrap();

        let result = update_quest(
            &pool,
            quest.id,
            user1,
            UpdateQuestRequest {
                initiative_id: Some(other_initiative),
                ..no_update()
            },
        )
        .await;

        assert!(
            matches!(result, Err(AppError::Forbidden)),
            "expected Forbidden, got: {result:?}"
        );
    }

    // quest completion with epic_id updates epic status in same transaction
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn completing_quest_updates_epic_status(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let initiative_id = Uuid::new_v4();
        let epic_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "epic@example.com").await;
        insert_test_initiative(&pool, initiative_id, user_id, "Init").await;
        insert_test_epic(&pool, epic_id, user_id, initiative_id, "Epic").await;

        let quest = create_quest(
            &pool,
            user_id,
            CreateQuestRequest {
                epic_id: Some(epic_id),
                ..make_create_req("Epic Quest")
            },
        )
        .await
        .unwrap();

        update_quest(
            &pool,
            quest.id,
            user_id,
            UpdateQuestRequest {
                status: Some(QuestStatus::InProgress),
                ..no_update()
            },
        )
        .await
        .unwrap();

        update_quest(
            &pool,
            quest.id,
            user_id,
            UpdateQuestRequest {
                status: Some(QuestStatus::Completed),
                ..no_update()
            },
        )
        .await
        .unwrap();

        let epic_status: String =
            sqlx::query_scalar("SELECT status FROM guidance_epics WHERE id = $1")
                .bind(epic_id)
                .fetch_one(&pool)
                .await
                .unwrap();

        assert_eq!(epic_status, "completed", "epic status must be completed");
    }

    // A-G-13: soft-delete excludes row from subsequent list
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn soft_delete_excludes_from_list(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "del@example.com").await;

        let quest = create_quest(&pool, user_id, make_create_req("To Delete"))
            .await
            .unwrap();

        let before = list_quests(
            &pool,
            user_id,
            QuestListParams {
                status: None,
                priority: None,
                due_date: None,
                initiative_id: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(before.len(), 1);

        delete_quest(&pool, quest.id, user_id).await.unwrap();

        let after = list_quests(
            &pool,
            user_id,
            QuestListParams {
                status: None,
                priority: None,
                due_date: None,
                initiative_id: None,
            },
        )
        .await
        .unwrap();
        assert!(after.is_empty(), "soft-deleted quest must not appear in list");
    }

    // A-G-15: list?due_date=today returns only due today and non-terminal status
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn list_due_date_today_returns_only_due_today_non_terminal(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "today@example.com").await;

        let today = Local::now().date_naive();
        let tomorrow = today.succ_opt().unwrap();

        // Quest due today, not_started (should appear).
        let q_today = create_quest(
            &pool,
            user_id,
            CreateQuestRequest {
                due_date: Some(today),
                ..make_create_req("Today Quest")
            },
        )
        .await
        .unwrap();

        // Quest due today but completed (should NOT appear).
        let q_completed = create_quest(
            &pool,
            user_id,
            CreateQuestRequest {
                due_date: Some(today),
                ..make_create_req("Completed Today Quest")
            },
        )
        .await
        .unwrap();
        update_quest(
            &pool,
            q_completed.id,
            user_id,
            UpdateQuestRequest {
                status: Some(QuestStatus::InProgress),
                ..no_update()
            },
        )
        .await
        .unwrap();
        update_quest(
            &pool,
            q_completed.id,
            user_id,
            UpdateQuestRequest {
                status: Some(QuestStatus::Completed),
                ..no_update()
            },
        )
        .await
        .unwrap();

        // Quest due tomorrow (should NOT appear).
        create_quest(
            &pool,
            user_id,
            CreateQuestRequest {
                due_date: Some(tomorrow),
                ..make_create_req("Tomorrow Quest")
            },
        )
        .await
        .unwrap();

        let results = list_quests(
            &pool,
            user_id,
            QuestListParams {
                due_date: Some(today),
                status: None,
                priority: None,
                initiative_id: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(
            results.len(),
            1,
            "only the non-terminal today quest should appear"
        );
        assert_eq!(results[0].id, q_today.id);
    }
}
