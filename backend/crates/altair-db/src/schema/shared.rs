//! Shared types - User, attachments, and tags

use chrono::{DateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::enums::{MediaType, UserRole};

/// User - Account and preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Option<Thing>,
    pub email: String, // unique
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub role: UserRole,
    pub preferences: UserPreferences,
    pub device_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// User preferences for customization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub theme: String, // "light", "dark", "auto"
    pub energy_filter_default: Option<String>,
    pub gamification_enabled: bool,
    pub weekly_harvest_day: i32, // 0-6 (Sunday-Saturday)
    pub weekly_harvest_time: NaiveTime,
    pub focus_session_duration: i32,  // minutes
    pub pomodoro_break_duration: i32, // minutes
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            theme: "auto".to_string(),
            energy_filter_default: None,
            gamification_enabled: true,
            weekly_harvest_day: 0, // Sunday
            weekly_harvest_time: NaiveTime::from_hms_opt(18, 0, 0).unwrap(), // 6 PM
            focus_session_duration: 25, // Pomodoro default
            pomodoro_break_duration: 5, // Short break
        }
    }
}

/// Attachment - File attachments for any entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub id: Option<Thing>,
    pub filename: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub storage_key: String, // S3 key
    pub checksum: String,    // SHA-256
    pub media_type: MediaType,
    pub duration: Option<i32>, // seconds, for audio/video
    pub thumbnail_key: Option<String>,
    pub transcription: Option<String>, // for audio/video
    pub owner: Thing,
    pub device_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Attachment {
    /// Check if attachment is an image
    pub fn is_image(&self) -> bool {
        matches!(self.media_type, MediaType::Photo)
    }

    /// Check if attachment is audio or video
    pub fn is_media(&self) -> bool {
        matches!(self.media_type, MediaType::Audio | MediaType::Video)
    }

    /// Get human-readable file size
    pub fn human_readable_size(&self) -> String {
        let size = self.size_bytes as f64;
        if size < 1024.0 {
            format!("{} B", size)
        } else if size < 1024.0 * 1024.0 {
            format!("{:.2} KB", size / 1024.0)
        } else if size < 1024.0 * 1024.0 * 1024.0 {
            format!("{:.2} MB", size / (1024.0 * 1024.0))
        } else {
            format!("{:.2} GB", size / (1024.0 * 1024.0 * 1024.0))
        }
    }
}

/// Tag - Organizational label for entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: Option<Thing>,
    pub name: String,
    pub namespace: Option<String>, // for tag categorization
    pub color: Option<String>,
    pub owner: Thing,
    pub device_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Tag {
    /// Get fully qualified tag name (namespace::name or just name)
    pub fn qualified_name(&self) -> String {
        if let Some(ns) = &self.namespace {
            format!("{}::{}", ns, self.name)
        } else {
            self.name.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_preferences_default() {
        let prefs = UserPreferences::default();
        assert_eq!(prefs.theme, "auto");
        assert!(prefs.gamification_enabled);
        assert_eq!(prefs.focus_session_duration, 25);
    }

    #[test]
    fn test_attachment_helpers() {
        let attachment = Attachment {
            id: None,
            filename: "test.jpg".to_string(),
            mime_type: "image/jpeg".to_string(),
            size_bytes: 1024 * 1024 * 2, // 2 MB
            storage_key: "s3://bucket/key".to_string(),
            checksum: "abc123".to_string(),
            media_type: MediaType::Photo,
            duration: None,
            thumbnail_key: None,
            transcription: None,
            owner: Thing::from(("user".to_string(), "test".to_string())),
            device_id: "device1".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert!(attachment.is_image());
        assert!(!attachment.is_media());
        assert_eq!(attachment.human_readable_size(), "2.00 MB");
    }

    #[test]
    fn test_tag_qualified_name() {
        let tag_with_ns = Tag {
            id: None,
            name: "urgent".to_string(),
            namespace: Some("priority".to_string()),
            color: Some("#ff0000".to_string()),
            owner: Thing::from(("user".to_string(), "test".to_string())),
            device_id: "device1".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(tag_with_ns.qualified_name(), "priority::urgent");

        let tag_no_ns = Tag {
            namespace: None,
            ..tag_with_ns
        };

        assert_eq!(tag_no_ns.qualified_name(), "urgent");
    }
}
