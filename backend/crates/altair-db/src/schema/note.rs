//! Knowledge domain types - Personal knowledge management entities

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::enums::EntityStatus;

/// Note - Individual knowledge document with markdown content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: Option<Thing>,
    pub title: String,
    pub content: String, // markdown
    pub embedding: Option<Vec<f32>>, // 384-dimensional vector for semantic search
    pub is_daily: bool,
    pub version: i32,
    pub status: EntityStatus,
    pub owner: Thing,
    pub device_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Note {
    /// Check if note has embedding for semantic search
    pub fn has_embedding(&self) -> bool {
        self.embedding.is_some()
    }

    /// Get embedding dimension (should be 384 for all-MiniLM-L6-v2)
    pub fn embedding_dimension(&self) -> Option<usize> {
        self.embedding.as_ref().map(|e| e.len())
    }
}

/// Folder - Organizational container for notes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    pub id: Option<Thing>,
    pub name: String,
    pub color: Option<String>,
    pub status: EntityStatus,
    pub owner: Thing,
    pub device_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// DailyNote - Special note type for daily journaling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyNote {
    pub id: Option<Thing>,
    pub date: NaiveDate, // unique per owner
    pub note_id: Thing,  // reference to actual note
    pub auto_created: bool,
    pub owner: Thing,
    pub device_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_embedding_helpers() {
        let mut note = Note {
            id: None,
            title: "Test".to_string(),
            content: "Content".to_string(),
            embedding: None,
            is_daily: false,
            version: 1,
            status: EntityStatus::Active,
            owner: Thing::from(("user".to_string(), "test".to_string())),
            device_id: "device1".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert!(!note.has_embedding());
        assert_eq!(note.embedding_dimension(), None);

        note.embedding = Some(vec![0.0; 384]);
        assert!(note.has_embedding());
        assert_eq!(note.embedding_dimension(), Some(384));
    }
}
