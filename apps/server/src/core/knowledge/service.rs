use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

use crate::contracts::ContentType;
use crate::core::relations::models::EntityRelation;
use crate::core::relations::service::SELECT_COLUMNS as RELATION_SELECT_COLUMNS;
use crate::error::AppError;
use super::models::{
    CreateNoteRequest, CreateSnapshotRequest, KnowledgeNote, KnowledgeNoteSnapshot,
    ListNotesQuery, UpdateNoteRequest,
};

/// Create a new knowledge note owned by the given user.
///
/// If `household_id` is provided, verifies the user is a member of that
/// household before creating the note. If `initiative_id` is provided,
/// verifies the initiative exists and the user has access to it.
pub async fn create_note(
    pool: &PgPool,
    user_id: Uuid,
    req: CreateNoteRequest,
) -> Result<KnowledgeNote, AppError> {
    // If household_id is provided, verify the user is a member
    if let Some(household_id) = req.household_id {
        let is_member =
            crate::core::households::service::is_member(pool, household_id, user_id).await?;
        if !is_member {
            return Err(AppError::Forbidden(
                "You are not a member of this household".to_string(),
            ));
        }
    }

    // If initiative_id is provided, verify user has access to it
    if let Some(initiative_id) = req.initiative_id {
        crate::core::initiatives::service::get_initiative(pool, initiative_id, user_id).await?;
    }

    let content_type = req.content_type.unwrap_or(ContentType::Markdown);
    let is_pinned = req.is_pinned.unwrap_or(false);

    sqlx::query_as::<_, KnowledgeNote>(
        r#"INSERT INTO knowledge_notes (user_id, household_id, initiative_id, title, content, content_type, is_pinned)
           VALUES ($1, $2, $3, $4, $5, $6, $7)
           RETURNING id, user_id, household_id, initiative_id, title, content, content_type, is_pinned, created_at, updated_at"#,
    )
    .bind(user_id)
    .bind(req.household_id)
    .bind(req.initiative_id)
    .bind(&req.title)
    .bind(&req.content)
    .bind(content_type.as_str())
    .bind(is_pinned)
    .fetch_one(pool)
    .await
    .map_err(|e| match &e {
        sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
            AppError::Conflict("Knowledge note already exists".to_string())
        }
        sqlx::Error::Database(db_err) if db_err.is_foreign_key_violation() => {
            AppError::BadRequest("Referenced household or initiative does not exist".to_string())
        }
        sqlx::Error::Database(db_err) if db_err.is_check_violation() => {
            AppError::BadRequest("Invalid field value".to_string())
        }
        _ => AppError::Database(e),
    })
}

/// List knowledge notes owned by the user, or belonging to a household the
/// user is a member of.
///
/// When `household_id` is provided, verifies membership and returns notes
/// scoped to that household. Otherwise returns notes owned by the user.
/// Optional filters narrow by initiative_id and is_pinned.
pub async fn list_notes(
    pool: &PgPool,
    user_id: Uuid,
    query: ListNotesQuery,
) -> Result<Vec<KnowledgeNote>, AppError> {
    let mut qb: QueryBuilder<sqlx::Postgres> = QueryBuilder::new(
        "SELECT id, user_id, household_id, initiative_id, title, content, content_type, is_pinned, created_at, updated_at FROM knowledge_notes WHERE ",
    );

    if let Some(hid) = query.household_id {
        // Verify user is a member of the household
        let is_member =
            crate::core::households::service::is_member(pool, hid, user_id).await?;
        if !is_member {
            return Err(AppError::Forbidden(
                "You are not a member of this household".to_string(),
            ));
        }
        qb.push("household_id = ");
        qb.push_bind(hid);
    } else {
        qb.push("user_id = ");
        qb.push_bind(user_id);
    }

    if let Some(initiative_id) = query.initiative_id {
        qb.push(" AND initiative_id = ");
        qb.push_bind(initiative_id);
    }

    if let Some(is_pinned) = query.is_pinned {
        qb.push(" AND is_pinned = ");
        qb.push_bind(is_pinned);
    }

    qb.push(" ORDER BY updated_at DESC");

    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);
    qb.push(" LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);

    qb.build_query_as::<KnowledgeNote>()
        .fetch_all(pool)
        .await
        .map_err(AppError::Database)
}

/// Get a single knowledge note by ID.
///
/// The user must either own the note directly or be a member of the
/// household the note belongs to.
pub async fn get_note(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
) -> Result<KnowledgeNote, AppError> {
    let note = sqlx::query_as::<_, KnowledgeNote>(
        r#"SELECT id, user_id, household_id, initiative_id, title, content, content_type, is_pinned, created_at, updated_at
           FROM knowledge_notes
           WHERE id = $1"#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?
    .ok_or_else(|| AppError::NotFound("Knowledge note not found".to_string()))?;

    // Check access: user owns it directly or is a member of its household
    if note.user_id == user_id {
        return Ok(note);
    }

    if let Some(household_id) = note.household_id {
        let is_member =
            crate::core::households::service::is_member(pool, household_id, user_id).await?;
        if is_member {
            return Ok(note);
        }
    }

    Err(AppError::Forbidden(
        "You do not have access to this note".to_string(),
    ))
}

/// Update a knowledge note's fields with true partial-update semantics.
///
/// Only the note owner can update it. Each field follows these rules:
/// - `title`: `None` leaves it unchanged, `Some(val)` sets it to `val`
/// - `content`: `None` leaves it unchanged, `Some(None)` sets it to NULL,
///   `Some(Some(val))` sets it to `val`
/// - `household_id` / `initiative_id`: same triple-state as content
/// - `content_type` / `is_pinned`: `None` leaves unchanged, `Some(val)` sets it
pub async fn update_note(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    req: UpdateNoteRequest,
) -> Result<KnowledgeNote, AppError> {
    // Validate membership for FK changes before building the query
    if let Some(Some(household_id)) = req.household_id {
        let is_member =
            crate::core::households::service::is_member(pool, household_id, user_id).await?;
        if !is_member {
            return Err(AppError::Forbidden(
                "You are not a member of this household".to_string(),
            ));
        }
    }
    if let Some(Some(initiative_id)) = req.initiative_id {
        crate::core::initiatives::service::get_initiative(pool, initiative_id, user_id).await?;
    }

    let mut qb: QueryBuilder<sqlx::Postgres> =
        QueryBuilder::new("UPDATE knowledge_notes SET updated_at = now()");

    if let Some(ref title) = req.title {
        qb.push(", title = ");
        qb.push_bind(title.clone());
    }

    match &req.content {
        Some(None) => {
            qb.push(", content = NULL");
        }
        Some(Some(val)) => {
            qb.push(", content = ");
            qb.push_bind(val.clone());
        }
        None => {}
    }

    if let Some(ref content_type) = req.content_type {
        qb.push(", content_type = ");
        qb.push_bind(content_type.as_str().to_string());
    }

    match &req.household_id {
        Some(None) => {
            qb.push(", household_id = NULL");
        }
        Some(Some(val)) => {
            qb.push(", household_id = ");
            qb.push_bind(*val);
        }
        None => {}
    }

    match &req.initiative_id {
        Some(None) => {
            qb.push(", initiative_id = NULL");
        }
        Some(Some(val)) => {
            qb.push(", initiative_id = ");
            qb.push_bind(*val);
        }
        None => {}
    }

    if let Some(is_pinned) = req.is_pinned {
        qb.push(", is_pinned = ");
        qb.push_bind(is_pinned);
    }

    qb.push(" WHERE id = ");
    qb.push_bind(id);
    qb.push(" AND user_id = ");
    qb.push_bind(user_id);
    qb.push(" RETURNING id, user_id, household_id, initiative_id, title, content, content_type, is_pinned, created_at, updated_at");

    qb.build_query_as::<KnowledgeNote>()
        .fetch_optional(pool)
        .await
        .map_err(|e| match &e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                AppError::Conflict("Knowledge note already exists".to_string())
            }
            sqlx::Error::Database(db_err) if db_err.is_foreign_key_violation() => {
                AppError::BadRequest(
                    "Referenced household or initiative does not exist".to_string(),
                )
            }
            sqlx::Error::Database(db_err) if db_err.is_check_violation() => {
                AppError::BadRequest("Invalid field value".to_string())
            }
            _ => AppError::Database(e),
        })?
        .ok_or_else(|| {
            AppError::NotFound(
                "Knowledge note not found or you do not have permission".to_string(),
            )
        })
}

/// Hard-delete a knowledge note.
///
/// Only the note owner can delete it. Cascading deletes will remove
/// associated snapshots, tags, and attachments.
pub async fn delete_note(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
) -> Result<(), AppError> {
    let result = sqlx::query(
        r#"DELETE FROM knowledge_notes
           WHERE id = $1 AND user_id = $2"#,
    )
    .bind(id)
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(AppError::Database)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(
            "Knowledge note not found or you do not have permission".to_string(),
        ));
    }

    Ok(())
}

/// Create a snapshot of a knowledge note's current content.
///
/// Verifies the user has access to the note before creating the snapshot.
/// Returns an error if the note has no content to snapshot.
pub async fn create_snapshot(
    pool: &PgPool,
    note_id: Uuid,
    user_id: Uuid,
    req: CreateSnapshotRequest,
) -> Result<KnowledgeNoteSnapshot, AppError> {
    // Verify note access
    let note = get_note(pool, note_id, user_id).await?;

    let content = note.content.ok_or_else(|| {
        AppError::BadRequest("Cannot snapshot a note with no content".to_string())
    })?;

    sqlx::query_as::<_, KnowledgeNoteSnapshot>(
        r#"INSERT INTO knowledge_note_snapshots (note_id, content, created_by_process)
           VALUES ($1, $2, $3)
           RETURNING id, note_id, content, created_at, created_by_process"#,
    )
    .bind(note_id)
    .bind(&content)
    .bind(&req.created_by_process)
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)
}

/// List snapshots for a knowledge note, ordered by most recent first.
///
/// Verifies the user has access to the note before listing snapshots.
pub async fn list_snapshots(
    pool: &PgPool,
    note_id: Uuid,
    user_id: Uuid,
) -> Result<Vec<KnowledgeNoteSnapshot>, AppError> {
    // Verify note access
    get_note(pool, note_id, user_id).await?;

    sqlx::query_as::<_, KnowledgeNoteSnapshot>(
        r#"SELECT id, note_id, content, created_at, created_by_process
           FROM knowledge_note_snapshots
           WHERE note_id = $1
           ORDER BY created_at DESC"#,
    )
    .bind(note_id)
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)
}

/// Get all entity relations originating from a knowledge note.
///
/// Returns relations where from_entity_type='knowledge_note' and
/// from_entity_id matches the given note_id.
pub async fn get_note_relations(
    pool: &PgPool,
    note_id: Uuid,
    user_id: Uuid,
) -> Result<Vec<EntityRelation>, AppError> {
    // Verify note access
    get_note(pool, note_id, user_id).await?;

    sqlx::query_as::<_, EntityRelation>(&format!(
        "SELECT {RELATION_SELECT_COLUMNS} FROM entity_relations \
         WHERE from_entity_type = 'knowledge_note' AND from_entity_id = $1 \
         ORDER BY created_at DESC"
    ))
    .bind(note_id)
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)
}

/// Get all entity relations pointing to a knowledge note (backlinks).
///
/// Returns relations where to_entity_type='knowledge_note' and
/// to_entity_id matches the given note_id.
pub async fn get_note_backlinks(
    pool: &PgPool,
    note_id: Uuid,
    user_id: Uuid,
) -> Result<Vec<EntityRelation>, AppError> {
    // Verify note access
    get_note(pool, note_id, user_id).await?;

    sqlx::query_as::<_, EntityRelation>(&format!(
        "SELECT {RELATION_SELECT_COLUMNS} FROM entity_relations \
         WHERE to_entity_type = 'knowledge_note' AND to_entity_id = $1 \
         ORDER BY created_at DESC"
    ))
    .bind(note_id)
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)
}

/// Associate a tag with a knowledge note.
///
/// Verifies the user owns the note before adding the tag.
/// Handles duplicate associations gracefully with a conflict response.
pub async fn add_note_tag(
    pool: &PgPool,
    note_id: Uuid,
    tag_id: Uuid,
    user_id: Uuid,
) -> Result<(), AppError> {
    // Verify note ownership
    let note_exists = sqlx::query_as::<_, (Uuid,)>(
        "SELECT id FROM knowledge_notes WHERE id = $1 AND user_id = $2",
    )
    .bind(note_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?;

    if note_exists.is_none() {
        return Err(AppError::NotFound(
            "Knowledge note not found".to_string(),
        ));
    }

    sqlx::query("INSERT INTO note_tags (note_id, tag_id) VALUES ($1, $2)")
        .bind(note_id)
        .bind(tag_id)
        .execute(pool)
        .await
        .map_err(|e| match &e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                AppError::Conflict("Tag already associated with this note".to_string())
            }
            sqlx::Error::Database(db_err) if db_err.is_foreign_key_violation() => {
                AppError::NotFound("Referenced tag does not exist".to_string())
            }
            _ => AppError::Database(e),
        })?;

    Ok(())
}

/// Remove a tag association from a knowledge note.
///
/// Verifies the user owns the note before removing the tag.
pub async fn remove_note_tag(
    pool: &PgPool,
    note_id: Uuid,
    tag_id: Uuid,
    user_id: Uuid,
) -> Result<(), AppError> {
    // Verify note ownership
    let note_exists = sqlx::query_as::<_, (Uuid,)>(
        "SELECT id FROM knowledge_notes WHERE id = $1 AND user_id = $2",
    )
    .bind(note_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?;

    if note_exists.is_none() {
        return Err(AppError::NotFound(
            "Knowledge note not found".to_string(),
        ));
    }

    let result = sqlx::query("DELETE FROM note_tags WHERE note_id = $1 AND tag_id = $2")
        .bind(note_id)
        .bind(tag_id)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(
            "Tag association not found".to_string(),
        ));
    }

    Ok(())
}

/// Associate an attachment with a knowledge note.
///
/// Verifies the user owns the note before adding the attachment.
/// Handles duplicate associations gracefully with a conflict response.
pub async fn add_note_attachment(
    pool: &PgPool,
    note_id: Uuid,
    attachment_id: Uuid,
    user_id: Uuid,
) -> Result<(), AppError> {
    // Verify note ownership
    let note_exists = sqlx::query_as::<_, (Uuid,)>(
        "SELECT id FROM knowledge_notes WHERE id = $1 AND user_id = $2",
    )
    .bind(note_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?;

    if note_exists.is_none() {
        return Err(AppError::NotFound(
            "Knowledge note not found".to_string(),
        ));
    }

    sqlx::query("INSERT INTO note_attachments (note_id, attachment_id) VALUES ($1, $2)")
        .bind(note_id)
        .bind(attachment_id)
        .execute(pool)
        .await
        .map_err(|e| match &e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                AppError::Conflict("Attachment already associated with this note".to_string())
            }
            sqlx::Error::Database(db_err) if db_err.is_foreign_key_violation() => {
                AppError::NotFound("Referenced attachment does not exist".to_string())
            }
            _ => AppError::Database(e),
        })?;

    Ok(())
}

/// Remove an attachment association from a knowledge note.
///
/// Verifies the user owns the note before removing the attachment.
pub async fn remove_note_attachment(
    pool: &PgPool,
    note_id: Uuid,
    attachment_id: Uuid,
    user_id: Uuid,
) -> Result<(), AppError> {
    // Verify note ownership
    let note_exists = sqlx::query_as::<_, (Uuid,)>(
        "SELECT id FROM knowledge_notes WHERE id = $1 AND user_id = $2",
    )
    .bind(note_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?;

    if note_exists.is_none() {
        return Err(AppError::NotFound(
            "Knowledge note not found".to_string(),
        ));
    }

    let result =
        sqlx::query("DELETE FROM note_attachments WHERE note_id = $1 AND attachment_id = $2")
            .bind(note_id)
            .bind(attachment_id)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(
            "Attachment association not found".to_string(),
        ));
    }

    Ok(())
}
