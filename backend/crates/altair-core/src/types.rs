//! Common types used across Altair

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User identifier (SurrealDB record ID)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub String);

impl UserId {
    /// Create a new user ID
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Get the inner string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for UserId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for UserId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// Generic entity identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId(pub String);

impl EntityId {
    /// Create a new entity ID
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Generate a new UUID-based entity ID
    pub fn generate() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Get the inner string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for EntityId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for EntityId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// Timestamp type (UTC)
pub type Timestamp = DateTime<Utc>;

/// Energy cost for quests (ADHD-focused task management)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum EnergyCost {
    /// Low energy task (quick, easy)
    Low,
    /// Medium energy task (moderate effort)
    #[default]
    Medium,
    /// High energy task (challenging, draining)
    High,
}

/// Generic entity status (for soft deletes)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum EntityStatus {
    /// Active entity
    #[default]
    Active,
    /// Archived/soft-deleted entity
    Archived,
}

impl EntityStatus {
    /// Check if entity is active
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }

    /// Check if entity is archived
    pub fn is_archived(&self) -> bool {
        matches!(self, Self::Archived)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id_creation() {
        let id = UserId::new("user:123");
        assert_eq!(id.as_str(), "user:123");

        let id2 = UserId::from("user:456");
        assert_eq!(id2.as_str(), "user:456");
    }

    #[test]
    fn test_entity_id_generation() {
        let id1 = EntityId::generate();
        let id2 = EntityId::generate();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_energy_cost_default() {
        assert_eq!(EnergyCost::default(), EnergyCost::Medium);
    }

    #[test]
    fn test_entity_status() {
        let status = EntityStatus::Active;
        assert!(status.is_active());
        assert!(!status.is_archived());

        let archived = EntityStatus::Archived;
        assert!(!archived.is_active());
        assert!(archived.is_archived());
    }
}
