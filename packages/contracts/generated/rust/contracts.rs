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
