// AUTO-GENERATED from registry/relation-sources.json — do not edit
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationSource {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "ai")]
    Ai,
    #[serde(rename = "import")]
    Import,
    #[serde(rename = "rule")]
    Rule,
    #[serde(rename = "migration")]
    Migration,
    #[serde(rename = "system")]
    System,
}

impl RelationSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Ai => "ai",
            Self::Import => "import",
            Self::Rule => "rule",
            Self::Migration => "migration",
            Self::System => "system",
        }
    }
}
