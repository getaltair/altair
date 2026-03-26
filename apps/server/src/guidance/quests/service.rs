use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

use crate::contracts::{Priority, QuestStatus};
use crate::error::AppError;
use super::models::*;

/// Create a new quest for the given user.
///
/// Priority defaults to `Medium` and status defaults to `Pending` when not
/// specified in the request.
pub async fn create_quest(
    pool: &PgPool,
    user_id: Uuid,
    req: &CreateQuestRequest,
) -> Result<GuidanceQuest, AppError> {
    let priority = req
        .priority
        .as_ref()
        .map_or(Priority::Medium.as_str(), |p| p.as_str());

    let quest = sqlx::query_as::<_, GuidanceQuest>(
        r#"INSERT INTO guidance_quests (epic_id, initiative_id, user_id, household_id, name, description, status, priority, due_date, estimated_minutes)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
           RETURNING id, epic_id, initiative_id, user_id, household_id, name, description,
                     status, priority, due_date, estimated_minutes, completed_at, created_at, updated_at"#,
    )
    .bind(req.epic_id)
    .bind(req.initiative_id)
    .bind(user_id)
    .bind(req.household_id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(QuestStatus::Pending.as_str())
    .bind(priority)
    .bind(req.due_date)
    .bind(req.estimated_minutes)
    .fetch_one(pool)
    .await
    .map_err(|e| match &e {
        sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
            AppError::Conflict("Quest already exists".to_string())
        }
        sqlx::Error::Database(db_err) if db_err.is_check_violation() => {
            AppError::BadRequest("Invalid field value".to_string())
        }
        sqlx::Error::Database(db_err) if db_err.is_foreign_key_violation() => {
            AppError::BadRequest("Referenced resource does not exist".to_string())
        }
        _ => AppError::Database(e),
    })?;

    tracing::info!(quest_id = %quest.id, user_id = %user_id, "Created guidance quest");
    Ok(quest)
}

/// List quests visible to the user (owned or belonging to their households)
pub async fn list_quests(
    pool: &PgPool,
    user_id: Uuid,
    household_ids: &[Uuid],
) -> Result<Vec<GuidanceQuest>, AppError> {
    sqlx::query_as::<_, GuidanceQuest>(
        r#"SELECT id, epic_id, initiative_id, user_id, household_id, name, description,
                  status, priority, due_date, estimated_minutes, completed_at, created_at, updated_at
           FROM guidance_quests
           WHERE user_id = $1 OR household_id = ANY($2)
           ORDER BY created_at DESC"#,
    )
    .bind(user_id)
    .bind(household_ids)
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)
}

/// Get a single quest by ID, accessible if user owns it or is in its household
pub async fn get_quest(
    pool: &PgPool,
    quest_id: Uuid,
    user_id: Uuid,
    household_ids: &[Uuid],
) -> Result<GuidanceQuest, AppError> {
    sqlx::query_as::<_, GuidanceQuest>(
        r#"SELECT id, epic_id, initiative_id, user_id, household_id, name, description,
                  status, priority, due_date, estimated_minutes, completed_at, created_at, updated_at
           FROM guidance_quests
           WHERE id = $1 AND (user_id = $2 OR household_id = ANY($3))"#,
    )
    .bind(quest_id)
    .bind(user_id)
    .bind(household_ids)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?
    .ok_or_else(|| AppError::NotFound("Quest not found".to_string()))
}

/// Update a quest's fields. Only the quest owner can update it.
///
/// Status changes are intentionally excluded; status transitions are handled
/// exclusively through `complete_quest`. Nullable fields use double-option
/// semantics: `None` = don't touch, `Some(None)` = set to NULL,
/// `Some(Some(val))` = set to value.
pub async fn update_quest(
    pool: &PgPool,
    quest_id: Uuid,
    user_id: Uuid,
    req: &UpdateQuestRequest,
) -> Result<GuidanceQuest, AppError> {
    let mut qb: QueryBuilder<sqlx::Postgres> =
        QueryBuilder::new("UPDATE guidance_quests SET updated_at = now()");

    if let Some(ref name) = req.name {
        qb.push(", name = ");
        qb.push_bind(name.clone());
    }

    match &req.description {
        Some(None) => {
            qb.push(", description = NULL");
        }
        Some(Some(val)) => {
            qb.push(", description = ");
            qb.push_bind(val.clone());
        }
        None => {}
    }

    if let Some(ref priority) = req.priority {
        qb.push(", priority = ");
        qb.push_bind(priority.as_str());
    }

    match &req.due_date {
        Some(None) => {
            qb.push(", due_date = NULL");
        }
        Some(Some(val)) => {
            qb.push(", due_date = ");
            qb.push_bind(*val);
        }
        None => {}
    }

    match &req.estimated_minutes {
        Some(None) => {
            qb.push(", estimated_minutes = NULL");
        }
        Some(Some(val)) => {
            qb.push(", estimated_minutes = ");
            qb.push_bind(*val);
        }
        None => {}
    }

    match &req.epic_id {
        Some(None) => {
            qb.push(", epic_id = NULL");
        }
        Some(Some(val)) => {
            qb.push(", epic_id = ");
            qb.push_bind(*val);
        }
        None => {}
    }

    qb.push(" WHERE id = ");
    qb.push_bind(quest_id);
    qb.push(" AND user_id = ");
    qb.push_bind(user_id);
    qb.push(r#" RETURNING id, epic_id, initiative_id, user_id, household_id, name, description,
                          status, priority, due_date, estimated_minutes, completed_at, created_at, updated_at"#);

    let quest = qb
        .build_query_as::<GuidanceQuest>()
        .fetch_optional(pool)
        .await
        .map_err(|e| match &e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                AppError::Conflict("Quest already exists".to_string())
            }
            sqlx::Error::Database(db_err) if db_err.is_check_violation() => {
                AppError::BadRequest("Invalid field value".to_string())
            }
            sqlx::Error::Database(db_err) if db_err.is_foreign_key_violation() => {
                AppError::BadRequest("Referenced resource does not exist".to_string())
            }
            _ => AppError::Database(e),
        })?
        .ok_or_else(|| AppError::NotFound("Quest not found".to_string()))?;

    tracing::info!(quest_id = %quest_id, user_id = %user_id, "Updated guidance quest");
    Ok(quest)
}

/// Delete a quest by ID. Only the quest owner can delete it.
pub async fn delete_quest(
    pool: &PgPool,
    quest_id: Uuid,
    user_id: Uuid,
) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM guidance_quests WHERE id = $1 AND user_id = $2")
        .bind(quest_id)
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| match &e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                AppError::Conflict("Resource already exists".to_string())
            }
            sqlx::Error::Database(db_err) if db_err.is_check_violation() => {
                AppError::BadRequest("Invalid field value".to_string())
            }
            sqlx::Error::Database(db_err) if db_err.is_foreign_key_violation() => {
                AppError::BadRequest("Referenced resource does not exist".to_string())
            }
            _ => AppError::Database(e),
        })?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Quest not found".to_string()));
    }

    tracing::info!(quest_id = %quest_id, user_id = %user_id, "Deleted guidance quest");
    Ok(())
}

/// Mark a quest as completed using an atomic UPDATE.
///
/// The quest must be accessible to the user (owned or household member)
/// and must not already be completed or cancelled. Uses a single UPDATE
/// with a WHERE guard to avoid TOCTOU races.
pub async fn complete_quest(
    pool: &PgPool,
    quest_id: Uuid,
    user_id: Uuid,
    household_ids: &[Uuid],
) -> Result<GuidanceQuest, AppError> {
    let quest = sqlx::query_as::<_, GuidanceQuest>(
        r#"UPDATE guidance_quests
           SET status = 'completed', completed_at = now(), updated_at = now()
           WHERE id = $1
             AND (user_id = $2 OR household_id = ANY($3))
             AND status NOT IN ('completed', 'cancelled')
           RETURNING id, epic_id, initiative_id, user_id, household_id, name, description,
                     status, priority, due_date, estimated_minutes, completed_at,
                     created_at, updated_at"#,
    )
    .bind(quest_id)
    .bind(user_id)
    .bind(household_ids)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?;

    match quest {
        Some(q) => {
            tracing::info!(quest_id = %quest_id, "Completed guidance quest");
            Ok(q)
        }
        None => {
            // Distinguish "not found" from "already completed/cancelled"
            let status = sqlx::query_scalar::<_, String>(
                "SELECT status FROM guidance_quests WHERE id = $1 AND (user_id = $2 OR household_id = ANY($3))",
            )
            .bind(quest_id)
            .bind(user_id)
            .bind(household_ids)
            .fetch_optional(pool)
            .await
            .map_err(AppError::Database)?;

            match status.as_deref() {
                Some("completed") | Some("cancelled") => {
                    tracing::warn!(quest_id = %quest_id, "Attempted to complete already finished quest");
                    Err(AppError::BadRequest(
                        "Quest is already completed or cancelled".to_string(),
                    ))
                }
                _ => Err(AppError::NotFound("Quest not found".to_string())),
            }
        }
    }
}

/// Add a tag to a quest. Verifies quest ownership before adding.
pub async fn add_quest_tag(
    pool: &PgPool,
    quest_id: Uuid,
    tag_id: Uuid,
    user_id: Uuid,
) -> Result<(), AppError> {
    // Verify quest belongs to user
    let exists: Option<(Uuid,)> = sqlx::query_as(
        "SELECT id FROM guidance_quests WHERE id = $1 AND user_id = $2",
    )
    .bind(quest_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?;

    if exists.is_none() {
        return Err(AppError::NotFound("Quest not found".to_string()));
    }

    sqlx::query("INSERT INTO quest_tags (quest_id, tag_id) VALUES ($1, $2) ON CONFLICT DO NOTHING")
        .bind(quest_id)
        .bind(tag_id)
        .execute(pool)
        .await
        .map_err(|e| match &e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                AppError::Conflict("Resource already exists".to_string())
            }
            sqlx::Error::Database(db_err) if db_err.is_check_violation() => {
                AppError::BadRequest("Invalid field value".to_string())
            }
            sqlx::Error::Database(db_err) if db_err.is_foreign_key_violation() => {
                AppError::BadRequest("Referenced resource does not exist".to_string())
            }
            _ => AppError::Database(e),
        })?;

    tracing::info!(quest_id = %quest_id, tag_id = %tag_id, "Added tag to quest");
    Ok(())
}

/// Remove a tag from a quest.
///
/// Returns 404 if the quest does not belong to the user or the tag is not
/// associated with the quest.
pub async fn remove_quest_tag(
    pool: &PgPool,
    quest_id: Uuid,
    tag_id: Uuid,
    user_id: Uuid,
) -> Result<(), AppError> {
    // Verify quest belongs to user
    let exists: Option<(Uuid,)> = sqlx::query_as(
        "SELECT id FROM guidance_quests WHERE id = $1 AND user_id = $2",
    )
    .bind(quest_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?;

    if exists.is_none() {
        return Err(AppError::NotFound("Quest not found".to_string()));
    }

    let result = sqlx::query("DELETE FROM quest_tags WHERE quest_id = $1 AND tag_id = $2")
        .bind(quest_id)
        .bind(tag_id)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;

    if result.rows_affected() == 0 {
        tracing::warn!(quest_id = %quest_id, tag_id = %tag_id, "Tag not found on quest");
        return Err(AppError::NotFound("Tag not found on this quest".to_string()));
    }

    tracing::info!(quest_id = %quest_id, tag_id = %tag_id, "Removed tag from quest");
    Ok(())
}
