// Generated from registry JSON. Do not edit by hand.

export const ENTITY_TYPES = {
  USER: "user",
  HOUSEHOLD: "household",
  INITIATIVE: "initiative",
  TAG: "tag",
  ATTACHMENT: "attachment",
  GUIDANCE_EPIC: "guidance_epic",
  GUIDANCE_QUEST: "guidance_quest",
  GUIDANCE_ROUTINE: "guidance_routine",
  GUIDANCE_FOCUS_SESSION: "guidance_focus_session",
  GUIDANCE_DAILY_CHECKIN: "guidance_daily_checkin",
  KNOWLEDGE_NOTE: "knowledge_note",
  KNOWLEDGE_NOTE_SNAPSHOT: "knowledge_note_snapshot",
  TRACKING_LOCATION: "tracking_location",
  TRACKING_CATEGORY: "tracking_category",
  TRACKING_ITEM: "tracking_item",
  TRACKING_ITEM_EVENT: "tracking_item_event",
  TRACKING_SHOPPING_LIST: "tracking_shopping_list",
  TRACKING_SHOPPING_LIST_ITEM: "tracking_shopping_list_item",
} as const;
export type EntityTypesValue = typeof ENTITY_TYPES[keyof typeof ENTITY_TYPES];

export const RELATION_TYPES = {
  REFERENCES: "references",
  SUPPORTS: "supports",
  REQUIRES: "requires",
  RELATED_TO: "related_to",
  DEPENDS_ON: "depends_on",
  DUPLICATES: "duplicates",
  SIMILAR_TO: "similar_to",
  GENERATED_FROM: "generated_from",
} as const;
export type RelationTypesValue = typeof RELATION_TYPES[keyof typeof RELATION_TYPES];

export const RELATION_SOURCE_TYPES = {
  USER: "user",
  AI: "ai",
  IMPORT: "import",
  RULE: "rule",
  MIGRATION: "migration",
  SYSTEM: "system",
} as const;
export type RelationSourceTypesValue = typeof RELATION_SOURCE_TYPES[keyof typeof RELATION_SOURCE_TYPES];

export const RELATION_STATUS_TYPES = {
  ACCEPTED: "accepted",
  SUGGESTED: "suggested",
  DISMISSED: "dismissed",
  REJECTED: "rejected",
  EXPIRED: "expired",
} as const;
export type RelationStatusTypesValue = typeof RELATION_STATUS_TYPES[keyof typeof RELATION_STATUS_TYPES];

export const SYNC_STREAMS = {
  MY_PROFILE: "my_profile",
  MY_MEMBERSHIPS: "my_memberships",
  MY_PERSONAL_DATA: "my_personal_data",
  MY_HOUSEHOLD_DATA: "my_household_data",
  MY_RELATIONS: "my_relations",
  MY_ATTACHMENT_METADATA: "my_attachment_metadata",
  INITIATIVE_DETAIL: "initiative_detail",
  NOTE_DETAIL: "note_detail",
  ITEM_HISTORY: "item_history",
  QUEST_DETAIL: "quest_detail",
} as const;
export type SyncStreamsValue = typeof SYNC_STREAMS[keyof typeof SYNC_STREAMS];
