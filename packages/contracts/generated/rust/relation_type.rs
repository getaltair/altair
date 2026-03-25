// AUTO-GENERATED from registry/relation-types.json — do not edit
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationType {
    #[serde(rename = "references")]
    References,
    #[serde(rename = "supports")]
    Supports,
    #[serde(rename = "requires")]
    Requires,
    #[serde(rename = "related_to")]
    RelatedTo,
    #[serde(rename = "depends_on")]
    DependsOn,
    #[serde(rename = "duplicates")]
    Duplicates,
    #[serde(rename = "similar_to")]
    SimilarTo,
    #[serde(rename = "generated_from")]
    GeneratedFrom,
}

impl RelationType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::References => "references",
            Self::Supports => "supports",
            Self::Requires => "requires",
            Self::RelatedTo => "related_to",
            Self::DependsOn => "depends_on",
            Self::Duplicates => "duplicates",
            Self::SimilarTo => "similar_to",
            Self::GeneratedFrom => "generated_from",
        }
    }
}
