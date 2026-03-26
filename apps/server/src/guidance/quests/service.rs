use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

use crate::error::AppError;
use super::models::*;

/// Create a new quest for the given user
pub async fn create_quest(
    pool: &PgPool,
    user_id: Uuid,
    req: &CreateQuestRequest,
) -> Result<GuidanceQuest, AppError> {
    let priority = req.priority.as_deref().unwrap_or("medium");

    sqlx::query_as::<_, GuidanceQuest>(
        r#"INSERT INTO guidance_quests (epic_id, initiative_id, user_id, household_id, name, description, priority, due_date, estimated_minutes)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
           RETURNING id, epic_id, initiative_id, user_id, household_id, name, description,
                     status, priority, due_date, estimated_minutes, completed_at, created_at, updated_at"#,
    )
    .bind(req.epic_id)
    .bind(req.initiative_id)
    .bind(user_id)
    .bind(req.household_id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(priority)
    .bind(req.due_date)
    .bind(req.estimated_minutes)
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)
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

    if let Some(ref description) = req.description {
        qb.push(", description = ");
        qb.push_bind(description.clone());
    }

    if let Some(ref status) = req.status {
        qb.push(", status = ");
        qb.push_bind(status.clone());
    }

    if let Some(ref priority) = req.priority {
        qb.push(", priority = ");
        qb.push_bind(priority.clone());
    }

    if let Some(ref due_date) = req.due_date {
        qb.push(", due_date = ");
        qb.push_bind(*due_date);
    }

    if let Some(ref estimated_minutes) = req.estimated_minutes {
        qb.push(", estimated_minutes = ");
        qb.push_bind(*estimated_minutes);
    }

    if let Some(ref epic_id) = req.epic_id {
        qb.push(", epic_id = ");
        qb.push_bind(*epic_id);
    }

    qb.push(" WHERE id = ");
    qb.push_bind(quest_id);
    qb.push(" AND user_id = ");
    qb.push_bind(user_id);
    qb.push(r#" RETURNING id, epic_id, initiative_id, user_id, household_id, name, description,
                          status, priority, due_date, estimated_minutes, completed_at, created_at, updated_at"#);

    qb.build_query_as::<GuidanceQuest>()
        .fetch_optional(pool)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound("Quest not found".to_string()))
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
        .map_err(AppError::Database)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Quest not found".to_string()));
    }

    Ok(())
}

/// Mark a quest as completed.
///
/// The quest must be accessible to the user (owned or household member)
/// and must not already be completed or cancelled.
pub async fn complete_quest(
    pool: &PgPool,
    quest_id: Uuid,
    user_id: Uuid,
    household_ids: &[Uuid],
) -> Result<GuidanceQuest, AppError> {
    // Verify the quest is accessible
    let quest = sqlx::query_as::<_, GuidanceQuest>(
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
    .ok_or_else(|| AppError::NotFound("Quest not found".to_string()))?;

    if quest.status == "completed" || quest.status == "cancelled" {
        return Err(AppError::BadRequest(
            "Quest is already completed or cancelled".to_string(),
        ));
    }

    sqlx::query_as::<_, GuidanceQuest>(
        r#"UPDATE guidance_quests
           SET status = 'completed', completed_at = now(), updated_at = now()
           WHERE id = $1
           RETURNING id, epic_id, initiative_id, user_id, household_id, name, description,
                     status, priority, due_date, estimated_minutes, completed_at, created_at, updated_at"#,
    )
    .bind(quest_id)
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)
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
        .map_err(AppError::Database)?;

    Ok(())
}

/// Remove a tag from a quest
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

    sqlx::query("DELETE FROM quest_tags WHERE quest_id = $1 AND tag_id = $2")
        .bind(quest_id)
        .bind(tag_id)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;

    Ok(())
}
