// AUTO-GENERATED from registry/sync-streams.json — do not edit
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SyncStream {
    #[serde(rename = "my_profile")]
    MyProfile,
    #[serde(rename = "my_memberships")]
    MyMemberships,
    #[serde(rename = "my_personal_data")]
    MyPersonalData,
    #[serde(rename = "my_household_data")]
    MyHouseholdData,
    #[serde(rename = "my_relations")]
    MyRelations,
    #[serde(rename = "my_attachment_metadata")]
    MyAttachmentMetadata,
    #[serde(rename = "initiative_detail")]
    InitiativeDetail,
    #[serde(rename = "note_detail")]
    NoteDetail,
    #[serde(rename = "item_history")]
    ItemHistory,
    #[serde(rename = "quest_detail")]
    QuestDetail,
}

impl SyncStream {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MyProfile => "my_profile",
            Self::MyMemberships => "my_memberships",
            Self::MyPersonalData => "my_personal_data",
            Self::MyHouseholdData => "my_household_data",
            Self::MyRelations => "my_relations",
            Self::MyAttachmentMetadata => "my_attachment_metadata",
            Self::InitiativeDetail => "initiative_detail",
            Self::NoteDetail => "note_detail",
            Self::ItemHistory => "item_history",
            Self::QuestDetail => "quest_detail",
        }
    }

    pub fn is_auto_subscribed(&self) -> bool {
        matches!(self, Self::MyProfile | Self::MyMemberships | Self::MyPersonalData | Self::MyHouseholdData | Self::MyRelations | Self::MyAttachmentMetadata)
    }
}
