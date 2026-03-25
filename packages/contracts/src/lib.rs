// Generated from registry JSON. Do not edit by hand.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum RelationSourceType {
	User,
	Ai,
	Import,
	Rule,
	Migration,
	System,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum RelationStatusType {
	Accepted,
	Suggested,
	Dismissed,
	Rejected,
	Expired,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SyncStream {
	MyProfile,
	MyMemberships,
	MyPersonalData,
	MyHouseholdData,
	MyRelations,
	MyAttachmentMetadata,
	InitiativeDetail,
	NoteDetail,
	ItemHistory,
	QuestDetail,
}

// DTO structs

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityRef {
	pub entity_type: EntityType,

	pub entity_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationRecord {
	pub id: String,

	pub from: EntityRef,

	pub to: EntityRef,

	pub relation_type: RelationType,

	pub source_type: RelationSourceType,

	pub status: RelationStatusType,

	pub confidence: Option<f64>,

	pub evidence: serde_json::Value,

	pub created_at: String,

	pub updated_at: String,
}
