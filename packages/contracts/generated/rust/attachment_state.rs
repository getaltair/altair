// AUTO-GENERATED from registry/attachment-states.json — do not edit
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttachmentState {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "uploaded")]
    Uploaded,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "deleted")]
    Deleted,
}

impl AttachmentState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Uploaded => "uploaded",
            Self::Processing => "processing",
            Self::Ready => "ready",
            Self::Failed => "failed",
            Self::Deleted => "deleted",
        }
    }
}
