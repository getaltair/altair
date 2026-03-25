// AUTO-GENERATED from registry/sync-streams.json — do not edit

export enum SyncStream {
  MY_PROFILE = "my_profile",
  MY_MEMBERSHIPS = "my_memberships",
  MY_PERSONAL_DATA = "my_personal_data",
  MY_HOUSEHOLD_DATA = "my_household_data",
  MY_RELATIONS = "my_relations",
  MY_ATTACHMENT_METADATA = "my_attachment_metadata",
  INITIATIVE_DETAIL = "initiative_detail",
  NOTE_DETAIL = "note_detail",
  ITEM_HISTORY = "item_history",
  QUEST_DETAIL = "quest_detail",
}

export const AUTO_SUBSCRIBED_STREAMS: readonly SyncStream[] = [
  SyncStream.MY_PROFILE,
  SyncStream.MY_MEMBERSHIPS,
  SyncStream.MY_PERSONAL_DATA,
  SyncStream.MY_HOUSEHOLD_DATA,
  SyncStream.MY_RELATIONS,
  SyncStream.MY_ATTACHMENT_METADATA,
];

export const ON_DEMAND_STREAMS: readonly SyncStream[] = [
  SyncStream.INITIATIVE_DETAIL,
  SyncStream.NOTE_DETAIL,
  SyncStream.ITEM_HISTORY,
  SyncStream.QUEST_DETAIL,
];
