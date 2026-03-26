//! Canonical domain enums for Altair entity types, relation types, source types,
//! and status values. These enums are the single source of truth used across
//! the system, replacing the previous `const &[&str]` arrays.

use serde::{Deserialize, Serialize};
use std::fmt;

// ---------------------------------------------------------------------------
// InitiativeStatus
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InitiativeStatus {
    Active,
    Paused,
    Completed,
    Archived,
}

impl fmt::Display for InitiativeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl InitiativeStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            InitiativeStatus::Active => "active",
            InitiativeStatus::Paused => "paused",
            InitiativeStatus::Completed => "completed",
            InitiativeStatus::Archived => "archived",
        }
    }
}

// ---------------------------------------------------------------------------
// ContentType
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    Markdown,
    Plain,
}

impl ContentType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ContentType::Markdown => "markdown",
            ContentType::Plain => "plain",
        }
    }
}

impl std::fmt::Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

// ---------------------------------------------------------------------------
// EntityType
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    User,
    Household,
    Initiative,
    Tag,
    Attachment,
    GuidanceEpic,
    GuidanceQuest,
    GuidanceRoutine,
    GuidanceFocusSession,
    GuidanceDailyCheckin,
    KnowledgeNote,
    KnowledgeNoteSnapshot,
    TrackingLocation,
    TrackingCategory,
    TrackingItem,
    TrackingItemEvent,
    TrackingShoppingList,
    TrackingShoppingListItem,
}

impl fmt::Display for EntityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl EntityType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EntityType::User => "user",
            EntityType::Household => "household",
            EntityType::Initiative => "initiative",
            EntityType::Tag => "tag",
            EntityType::Attachment => "attachment",
            EntityType::GuidanceEpic => "guidance_epic",
            EntityType::GuidanceQuest => "guidance_quest",
            EntityType::GuidanceRoutine => "guidance_routine",
            EntityType::GuidanceFocusSession => "guidance_focus_session",
            EntityType::GuidanceDailyCheckin => "guidance_daily_checkin",
            EntityType::KnowledgeNote => "knowledge_note",
            EntityType::KnowledgeNoteSnapshot => "knowledge_note_snapshot",
            EntityType::TrackingLocation => "tracking_location",
            EntityType::TrackingCategory => "tracking_category",
            EntityType::TrackingItem => "tracking_item",
            EntityType::TrackingItemEvent => "tracking_item_event",
            EntityType::TrackingShoppingList => "tracking_shopping_list",
            EntityType::TrackingShoppingListItem => "tracking_shopping_list_item",
        }
    }
}

// ---------------------------------------------------------------------------
// RelationType
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationType {
    References,
    Supports,
    Requires,
    RelatedTo,
    DependsOn,
    Duplicates,
    SimilarTo,
    GeneratedFrom,
}

impl fmt::Display for RelationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl RelationType {
    pub fn as_str(&self) -> &'static str {
        match self {
            RelationType::References => "references",
            RelationType::Supports => "supports",
            RelationType::Requires => "requires",
            RelationType::RelatedTo => "related_to",
            RelationType::DependsOn => "depends_on",
            RelationType::Duplicates => "duplicates",
            RelationType::SimilarTo => "similar_to",
            RelationType::GeneratedFrom => "generated_from",
        }
    }
}

// ---------------------------------------------------------------------------
// SourceType
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceType {
    User,
    Ai,
    Import,
    Rule,
    Migration,
    System,
}

impl fmt::Display for SourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl SourceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SourceType::User => "user",
            SourceType::Ai => "ai",
            SourceType::Import => "import",
            SourceType::Rule => "rule",
            SourceType::Migration => "migration",
            SourceType::System => "system",
        }
    }
}

// ---------------------------------------------------------------------------
// RelationStatus
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationStatus {
    Accepted,
    Suggested,
    Dismissed,
    Rejected,
    Expired,
}

impl fmt::Display for RelationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl RelationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            RelationStatus::Accepted => "accepted",
            RelationStatus::Suggested => "suggested",
            RelationStatus::Dismissed => "dismissed",
            RelationStatus::Rejected => "rejected",
            RelationStatus::Expired => "expired",
        }
    }
}

// ---------------------------------------------------------------------------
// ProcessingState
// ---------------------------------------------------------------------------

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProcessingState {
    Pending,
    Processing,
    Ready,
    Failed,
}

impl fmt::Display for ProcessingState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ProcessingState {
    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        match self {
            ProcessingState::Pending => "pending",
            ProcessingState::Processing => "processing",
            ProcessingState::Ready => "ready",
            ProcessingState::Failed => "failed",
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- InitiativeStatus ---------------------------------------------------

    #[test]
    fn initiative_status_serde_roundtrip() {
        let variants = [
            InitiativeStatus::Active,
            InitiativeStatus::Paused,
            InitiativeStatus::Completed,
            InitiativeStatus::Archived,
        ];
        for variant in &variants {
            let json = serde_json::to_string(variant).expect("serialize");
            let back: InitiativeStatus = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(&back, variant);
        }
    }

    #[test]
    fn initiative_status_unknown_string_fails() {
        let result = serde_json::from_str::<InitiativeStatus>("\"unknown\"");
        assert!(result.is_err());
    }

    #[test]
    fn initiative_status_as_str() {
        assert_eq!(InitiativeStatus::Active.as_str(), "active");
        assert_eq!(InitiativeStatus::Paused.as_str(), "paused");
        assert_eq!(InitiativeStatus::Completed.as_str(), "completed");
        assert_eq!(InitiativeStatus::Archived.as_str(), "archived");
    }

    // -- EntityType ---------------------------------------------------------

    #[test]
    fn entity_type_serde_roundtrip() {
        let variants = [
            EntityType::User,
            EntityType::Household,
            EntityType::Initiative,
            EntityType::Tag,
            EntityType::Attachment,
            EntityType::GuidanceEpic,
            EntityType::GuidanceQuest,
            EntityType::GuidanceRoutine,
            EntityType::GuidanceFocusSession,
            EntityType::GuidanceDailyCheckin,
            EntityType::KnowledgeNote,
            EntityType::KnowledgeNoteSnapshot,
            EntityType::TrackingLocation,
            EntityType::TrackingCategory,
            EntityType::TrackingItem,
            EntityType::TrackingItemEvent,
            EntityType::TrackingShoppingList,
            EntityType::TrackingShoppingListItem,
        ];
        for variant in &variants {
            let json = serde_json::to_string(variant).expect("serialize");
            let back: EntityType = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(&back, variant);
        }
    }

    #[test]
    fn entity_type_unknown_string_fails() {
        let result = serde_json::from_str::<EntityType>("\"unknown_entity\"");
        assert!(result.is_err());
    }

    #[test]
    fn entity_type_as_str() {
        assert_eq!(EntityType::User.as_str(), "user");
        assert_eq!(EntityType::KnowledgeNote.as_str(), "knowledge_note");
        assert_eq!(
            EntityType::TrackingShoppingListItem.as_str(),
            "tracking_shopping_list_item"
        );
    }

    // -- RelationType -------------------------------------------------------

    #[test]
    fn relation_type_serde_roundtrip() {
        let variants = [
            RelationType::References,
            RelationType::Supports,
            RelationType::Requires,
            RelationType::RelatedTo,
            RelationType::DependsOn,
            RelationType::Duplicates,
            RelationType::SimilarTo,
            RelationType::GeneratedFrom,
        ];
        for variant in &variants {
            let json = serde_json::to_string(variant).expect("serialize");
            let back: RelationType = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(&back, variant);
        }
    }

    #[test]
    fn relation_type_unknown_string_fails() {
        let result = serde_json::from_str::<RelationType>("\"links_to\"");
        assert!(result.is_err());
    }

    #[test]
    fn relation_type_as_str() {
        assert_eq!(RelationType::References.as_str(), "references");
        assert_eq!(RelationType::GeneratedFrom.as_str(), "generated_from");
    }

    // -- SourceType ---------------------------------------------------------

    #[test]
    fn source_type_serde_roundtrip() {
        let variants = [
            SourceType::User,
            SourceType::Ai,
            SourceType::Import,
            SourceType::Rule,
            SourceType::Migration,
            SourceType::System,
        ];
        for variant in &variants {
            let json = serde_json::to_string(variant).expect("serialize");
            let back: SourceType = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(&back, variant);
        }
    }

    #[test]
    fn source_type_unknown_string_fails() {
        let result = serde_json::from_str::<SourceType>("\"robot\"");
        assert!(result.is_err());
    }

    #[test]
    fn source_type_as_str() {
        assert_eq!(SourceType::User.as_str(), "user");
        assert_eq!(SourceType::Ai.as_str(), "ai");
        assert_eq!(SourceType::System.as_str(), "system");
    }

    // -- RelationStatus -----------------------------------------------------

    #[test]
    fn relation_status_serde_roundtrip() {
        let variants = [
            RelationStatus::Accepted,
            RelationStatus::Suggested,
            RelationStatus::Dismissed,
            RelationStatus::Rejected,
            RelationStatus::Expired,
        ];
        for variant in &variants {
            let json = serde_json::to_string(variant).expect("serialize");
            let back: RelationStatus = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(&back, variant);
        }
    }

    #[test]
    fn relation_status_unknown_string_fails() {
        let result = serde_json::from_str::<RelationStatus>("\"pending\"");
        assert!(result.is_err());
    }

    #[test]
    fn relation_status_as_str() {
        assert_eq!(RelationStatus::Accepted.as_str(), "accepted");
        assert_eq!(RelationStatus::Expired.as_str(), "expired");
    }

    // -- ProcessingState ----------------------------------------------------

    #[test]
    fn processing_state_serde_roundtrip() {
        let variants = [
            ProcessingState::Pending,
            ProcessingState::Processing,
            ProcessingState::Ready,
            ProcessingState::Failed,
        ];
        for variant in &variants {
            let json = serde_json::to_string(variant).expect("serialize");
            let back: ProcessingState = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(&back, variant);
        }
    }

    #[test]
    fn processing_state_unknown_string_fails() {
        let result = serde_json::from_str::<ProcessingState>("\"uploading\"");
        assert!(result.is_err());
    }

    #[test]
    fn processing_state_as_str() {
        assert_eq!(ProcessingState::Pending.as_str(), "pending");
        assert_eq!(ProcessingState::Ready.as_str(), "ready");
    }
}
