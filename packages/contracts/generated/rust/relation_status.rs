// AUTO-GENERATED from registry/relation-statuses.json — do not edit
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationStatus {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "suggested")]
    Suggested,
    #[serde(rename = "dismissed")]
    Dismissed,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "expired")]
    Expired,
}

impl RelationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Suggested => "suggested",
            Self::Dismissed => "dismissed",
            Self::Rejected => "rejected",
            Self::Expired => "expired",
        }
    }
}
