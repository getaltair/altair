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
// QuestStatus
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}

impl fmt::Display for QuestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl QuestStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            QuestStatus::Pending => "pending",
            QuestStatus::InProgress => "in_progress",
            QuestStatus::Completed => "completed",
            QuestStatus::Cancelled => "cancelled",
        }
    }
}

// ---------------------------------------------------------------------------
// RoutineStatus
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoutineStatus {
    Active,
    Paused,
    Archived,
}

impl fmt::Display for RoutineStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl RoutineStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            RoutineStatus::Active => "active",
            RoutineStatus::Paused => "paused",
            RoutineStatus::Archived => "archived",
        }
    }
}

// ---------------------------------------------------------------------------
// RoutineFrequency
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoutineFrequency {
    Daily,
    Weekly,
    Biweekly,
    Monthly,
}

impl fmt::Display for RoutineFrequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl RoutineFrequency {
    pub fn as_str(&self) -> &'static str {
        match self {
            RoutineFrequency::Daily => "daily",
            RoutineFrequency::Weekly => "weekly",
            RoutineFrequency::Biweekly => "biweekly",
            RoutineFrequency::Monthly => "monthly",
        }
    }
}

// ---------------------------------------------------------------------------
// Priority
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Priority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Priority::Low => "low",
            Priority::Medium => "medium",
            Priority::High => "high",
            Priority::Critical => "critical",
        }
    }
}

// ---------------------------------------------------------------------------
// TrackingItemStatus
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrackingItemStatus {
    Active,
    Archived,
}

impl fmt::Display for TrackingItemStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TrackingItemStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TrackingItemStatus::Active => "active",
            TrackingItemStatus::Archived => "archived",
        }
    }
}

// ---------------------------------------------------------------------------
// ShoppingListStatus
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShoppingListStatus {
    Active,
    Completed,
    Archived,
}

impl fmt::Display for ShoppingListStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ShoppingListStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ShoppingListStatus::Active => "active",
            ShoppingListStatus::Completed => "completed",
            ShoppingListStatus::Archived => "archived",
        }
    }
}

// ---------------------------------------------------------------------------
// ItemEventType
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemEventType {
    Consumed,
    Restocked,
    Moved,
    Adjusted,
    Expired,
    Donated,
}

impl fmt::Display for ItemEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ItemEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ItemEventType::Consumed => "consumed",
            ItemEventType::Restocked => "restocked",
            ItemEventType::Moved => "moved",
            ItemEventType::Adjusted => "adjusted",
            ItemEventType::Expired => "expired",
            ItemEventType::Donated => "donated",
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

    // -- QuestStatus --------------------------------------------------------

    #[test]
    fn quest_status_serde_roundtrip() {
        let variants = [
            QuestStatus::Pending,
            QuestStatus::InProgress,
            QuestStatus::Completed,
            QuestStatus::Cancelled,
        ];
        for variant in &variants {
            let json = serde_json::to_string(variant).expect("serialize");
            let back: QuestStatus = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(&back, variant);
        }
    }

    #[test]
    fn quest_status_unknown_string_fails() {
        let result = serde_json::from_str::<QuestStatus>("\"unknown\"");
        assert!(result.is_err());
    }

    #[test]
    fn quest_status_as_str() {
        assert_eq!(QuestStatus::Pending.as_str(), "pending");
        assert_eq!(QuestStatus::InProgress.as_str(), "in_progress");
        assert_eq!(QuestStatus::Completed.as_str(), "completed");
        assert_eq!(QuestStatus::Cancelled.as_str(), "cancelled");
    }

    // -- RoutineStatus ------------------------------------------------------

    #[test]
    fn routine_status_serde_roundtrip() {
        let variants = [
            RoutineStatus::Active,
            RoutineStatus::Paused,
            RoutineStatus::Archived,
        ];
        for variant in &variants {
            let json = serde_json::to_string(variant).expect("serialize");
            let back: RoutineStatus = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(&back, variant);
        }
    }

    #[test]
    fn routine_status_unknown_string_fails() {
        let result = serde_json::from_str::<RoutineStatus>("\"unknown\"");
        assert!(result.is_err());
    }

    #[test]
    fn routine_status_as_str() {
        assert_eq!(RoutineStatus::Active.as_str(), "active");
        assert_eq!(RoutineStatus::Paused.as_str(), "paused");
        assert_eq!(RoutineStatus::Archived.as_str(), "archived");
    }

    // -- RoutineFrequency ---------------------------------------------------

    #[test]
    fn routine_frequency_serde_roundtrip() {
        let variants = [
            RoutineFrequency::Daily,
            RoutineFrequency::Weekly,
            RoutineFrequency::Biweekly,
            RoutineFrequency::Monthly,
        ];
        for variant in &variants {
            let json = serde_json::to_string(variant).expect("serialize");
            let back: RoutineFrequency = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(&back, variant);
        }
    }

    #[test]
    fn routine_frequency_unknown_string_fails() {
        let result = serde_json::from_str::<RoutineFrequency>("\"unknown\"");
        assert!(result.is_err());
    }

    #[test]
    fn routine_frequency_as_str() {
        assert_eq!(RoutineFrequency::Daily.as_str(), "daily");
        assert_eq!(RoutineFrequency::Weekly.as_str(), "weekly");
        assert_eq!(RoutineFrequency::Biweekly.as_str(), "biweekly");
        assert_eq!(RoutineFrequency::Monthly.as_str(), "monthly");
    }

    // -- Priority -----------------------------------------------------------

    #[test]
    fn priority_serde_roundtrip() {
        let variants = [
            Priority::Low,
            Priority::Medium,
            Priority::High,
            Priority::Critical,
        ];
        for variant in &variants {
            let json = serde_json::to_string(variant).expect("serialize");
            let back: Priority = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(&back, variant);
        }
    }

    #[test]
    fn priority_unknown_string_fails() {
        let result = serde_json::from_str::<Priority>("\"unknown\"");
        assert!(result.is_err());
    }

    #[test]
    fn priority_as_str() {
        assert_eq!(Priority::Low.as_str(), "low");
        assert_eq!(Priority::Medium.as_str(), "medium");
        assert_eq!(Priority::High.as_str(), "high");
        assert_eq!(Priority::Critical.as_str(), "critical");
    }

    // -- TrackingItemStatus -------------------------------------------------

    #[test]
    fn tracking_item_status_serde_roundtrip() {
        let variants = [
            TrackingItemStatus::Active,
            TrackingItemStatus::Archived,
        ];
        for variant in &variants {
            let json = serde_json::to_string(variant).expect("serialize");
            let back: TrackingItemStatus = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(&back, variant);
        }
    }

    #[test]
    fn tracking_item_status_unknown_string_fails() {
        let result = serde_json::from_str::<TrackingItemStatus>("\"deleted\"");
        assert!(result.is_err());
    }

    #[test]
    fn tracking_item_status_as_str() {
        assert_eq!(TrackingItemStatus::Active.as_str(), "active");
        assert_eq!(TrackingItemStatus::Archived.as_str(), "archived");
    }

    // -- ShoppingListStatus -------------------------------------------------

    #[test]
    fn shopping_list_status_serde_roundtrip() {
        let variants = [
            ShoppingListStatus::Active,
            ShoppingListStatus::Completed,
            ShoppingListStatus::Archived,
        ];
        for variant in &variants {
            let json = serde_json::to_string(variant).expect("serialize");
            let back: ShoppingListStatus = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(&back, variant);
        }
    }

    #[test]
    fn shopping_list_status_unknown_string_fails() {
        let result = serde_json::from_str::<ShoppingListStatus>("\"cancelled\"");
        assert!(result.is_err());
    }

    #[test]
    fn shopping_list_status_as_str() {
        assert_eq!(ShoppingListStatus::Active.as_str(), "active");
        assert_eq!(ShoppingListStatus::Completed.as_str(), "completed");
        assert_eq!(ShoppingListStatus::Archived.as_str(), "archived");
    }

    // -- ItemEventType ------------------------------------------------------

    #[test]
    fn item_event_type_serde_roundtrip() {
        let variants = [
            ItemEventType::Consumed,
            ItemEventType::Restocked,
            ItemEventType::Moved,
            ItemEventType::Adjusted,
            ItemEventType::Expired,
            ItemEventType::Donated,
        ];
        for variant in &variants {
            let json = serde_json::to_string(variant).expect("serialize");
            let back: ItemEventType = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(&back, variant);
        }
    }

    #[test]
    fn item_event_type_unknown_string_fails() {
        let result = serde_json::from_str::<ItemEventType>("\"purchased\"");
        assert!(result.is_err());
    }

    #[test]
    fn item_event_type_as_str() {
        assert_eq!(ItemEventType::Consumed.as_str(), "consumed");
        assert_eq!(ItemEventType::Restocked.as_str(), "restocked");
        assert_eq!(ItemEventType::Moved.as_str(), "moved");
        assert_eq!(ItemEventType::Adjusted.as_str(), "adjusted");
        assert_eq!(ItemEventType::Expired.as_str(), "expired");
        assert_eq!(ItemEventType::Donated.as_str(), "donated");
    }
}
