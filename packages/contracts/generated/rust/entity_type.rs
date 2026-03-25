// AUTO-GENERATED from registry/entity-types.json — do not edit
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityType {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "household")]
    Household,
    #[serde(rename = "initiative")]
    Initiative,
    #[serde(rename = "tag")]
    Tag,
    #[serde(rename = "attachment")]
    Attachment,
    #[serde(rename = "guidance_epic")]
    GuidanceEpic,
    #[serde(rename = "guidance_quest")]
    GuidanceQuest,
    #[serde(rename = "guidance_routine")]
    GuidanceRoutine,
    #[serde(rename = "guidance_focus_session")]
    GuidanceFocusSession,
    #[serde(rename = "guidance_daily_checkin")]
    GuidanceDailyCheckin,
    #[serde(rename = "knowledge_note")]
    KnowledgeNote,
    #[serde(rename = "knowledge_note_snapshot")]
    KnowledgeNoteSnapshot,
    #[serde(rename = "tracking_location")]
    TrackingLocation,
    #[serde(rename = "tracking_category")]
    TrackingCategory,
    #[serde(rename = "tracking_item")]
    TrackingItem,
    #[serde(rename = "tracking_item_event")]
    TrackingItemEvent,
    #[serde(rename = "tracking_shopping_list")]
    TrackingShoppingList,
    #[serde(rename = "tracking_shopping_list_item")]
    TrackingShoppingListItem,
}

impl EntityType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Household => "household",
            Self::Initiative => "initiative",
            Self::Tag => "tag",
            Self::Attachment => "attachment",
            Self::GuidanceEpic => "guidance_epic",
            Self::GuidanceQuest => "guidance_quest",
            Self::GuidanceRoutine => "guidance_routine",
            Self::GuidanceFocusSession => "guidance_focus_session",
            Self::GuidanceDailyCheckin => "guidance_daily_checkin",
            Self::KnowledgeNote => "knowledge_note",
            Self::KnowledgeNoteSnapshot => "knowledge_note_snapshot",
            Self::TrackingLocation => "tracking_location",
            Self::TrackingCategory => "tracking_category",
            Self::TrackingItem => "tracking_item",
            Self::TrackingItemEvent => "tracking_item_event",
            Self::TrackingShoppingList => "tracking_shopping_list",
            Self::TrackingShoppingListItem => "tracking_shopping_list_item",
        }
    }
}
