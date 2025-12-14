//! Attachment queries - Database operations for file attachments

use altair_core::{Error, Result};
use chrono::Utc;
use surrealdb::Surreal;
use surrealdb::sql::Thing;

use crate::schema::{Attachment, MediaType};

/// Create a new attachment record
#[allow(clippy::too_many_arguments)]
pub async fn create_attachment<C: surrealdb::Connection>(
    db: &Surreal<C>,
    filename: String,
    mime_type: String,
    size_bytes: i32,
    storage_key: String,
    checksum: String,
    media_type: MediaType,
    owner: Thing,
    device_id: String,
) -> Result<Attachment> {
    let attachment: Attachment = db
        .create("attachment")
        .content(Attachment {
            id: None,
            filename,
            mime_type,
            size_bytes,
            storage_key,
            checksum,
            media_type,
            duration: None,
            thumbnail_key: None,
            transcription: None,
            owner,
            device_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
        .await
        .map_err(|e| Error::Database(format!("Failed to create attachment: {}", e)))?
        .ok_or_else(|| Error::Database("Attachment creation returned no result".to_string()))?;

    Ok(attachment)
}

/// Get attachment by ID
pub async fn get_attachment_by_id<C: surrealdb::Connection>(
    db: &Surreal<C>,
    id: &Thing,
) -> Result<Attachment> {
    let mut result = db
        .query("SELECT * FROM $id")
        .bind(("id", id.clone()))
        .await
        .map_err(|e| Error::Database(format!("Failed to get attachment by ID: {}", e)))?;

    let attachments: Vec<Attachment> = result
        .take(0)
        .map_err(|e| Error::Database(format!("Failed to deserialize attachment: {}", e)))?;

    attachments
        .into_iter()
        .next()
        .ok_or_else(|| Error::NotFound {
            entity_type: "attachment".to_string(),
            id: format!("{:?}", id),
        })
}

/// Get attachment by storage key
pub async fn get_attachment_by_storage_key<C: surrealdb::Connection>(
    db: &Surreal<C>,
    storage_key: &str,
) -> Result<Option<Attachment>> {
    let mut result = db
        .query("SELECT * FROM attachment WHERE storage_key = $key")
        .bind(("key", storage_key.to_string()))
        .await
        .map_err(|e| {
            Error::Database(format!("Failed to query attachment by storage key: {}", e))
        })?;

    let attachments: Vec<Attachment> = result
        .take(0)
        .map_err(|e| Error::Database(format!("Failed to deserialize attachment: {}", e)))?;

    Ok(attachments.into_iter().next())
}

/// Update attachment with thumbnail key
pub async fn update_attachment_thumbnail<C: surrealdb::Connection>(
    db: &Surreal<C>,
    id: &Thing,
    thumbnail_key: String,
) -> Result<Attachment> {
    let mut result = db
        .query(
            "UPDATE $id SET thumbnail_key = $thumbnail_key, updated_at = time::now() RETURN AFTER",
        )
        .bind(("id", id.clone()))
        .bind(("thumbnail_key", thumbnail_key))
        .await
        .map_err(|e| Error::Database(format!("Failed to update attachment thumbnail: {}", e)))?;

    let attachments: Vec<Attachment> = result
        .take(0)
        .map_err(|e| Error::Database(format!("Failed to deserialize attachment: {}", e)))?;

    attachments
        .into_iter()
        .next()
        .ok_or_else(|| Error::NotFound {
            entity_type: "attachment".to_string(),
            id: format!("{:?}", id),
        })
}

/// Archive (soft delete) an attachment
///
/// Note: The attachment table uses hard delete in the migration schema,
/// but we'll mark it by adding to a soft-delete list or adding status field.
/// For now, this actually deletes the record since there's no status field.
pub async fn delete_attachment<C: surrealdb::Connection>(
    db: &Surreal<C>,
    id: &Thing,
) -> Result<()> {
    // Get the attachment first to return it for quota updates
    let _attachment = get_attachment_by_id(db, id).await?;

    // Delete the attachment record
    db.query("DELETE $id")
        .bind(("id", id.clone()))
        .await
        .map_err(|e| Error::Database(format!("Failed to delete attachment: {}", e)))?;

    Ok(())
}

/// Get all attachments for a user
pub async fn get_attachments_by_owner<C: surrealdb::Connection>(
    db: &Surreal<C>,
    owner: &Thing,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<Vec<Attachment>> {
    let limit_val = limit.unwrap_or(50);
    let offset_val = offset.unwrap_or(0);

    let mut result = db
        .query("SELECT * FROM attachment WHERE owner = $owner ORDER BY created_at DESC LIMIT $limit START $offset")
        .bind(("owner", owner.clone()))
        .bind(("limit", limit_val))
        .bind(("offset", offset_val))
        .await
        .map_err(|e| Error::Database(format!("Failed to query attachments by owner: {}", e)))?;

    let attachments: Vec<Attachment> = result
        .take(0)
        .map_err(|e| Error::Database(format!("Failed to deserialize attachments: {}", e)))?;

    Ok(attachments)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_type_serialization() {
        let media_type = MediaType::Photo;
        let json = serde_json::to_string(&media_type).unwrap();
        assert_eq!(json, "\"photo\"");

        let deserialized: MediaType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, MediaType::Photo);
    }
}
