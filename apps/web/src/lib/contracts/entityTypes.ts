// Source of truth: packages/contracts/entity-types.json
export const EntityType = {
  User:                     'user',
  Household:                'household',
  Initiative:               'initiative',
  Tag:                      'tag',
  Attachment:               'attachment',
  GuidanceEpic:             'guidance_epic',
  GuidanceQuest:            'guidance_quest',
  GuidanceRoutine:          'guidance_routine',
  GuidanceFocusSession:     'guidance_focus_session',
  GuidanceDailyCheckin:     'guidance_daily_checkin',
  KnowledgeNote:            'knowledge_note',
  KnowledgeNoteSnapshot:    'knowledge_note_snapshot',
  TrackingLocation:         'tracking_location',
  TrackingCategory:         'tracking_category',
  TrackingItem:             'tracking_item',
  TrackingItemEvent:        'tracking_item_event',
  TrackingShoppingList:     'tracking_shopping_list',
  TrackingShoppingListItem: 'tracking_shopping_list_item',
} as const;

export type EntityTypeValue = typeof EntityType[keyof typeof EntityType];
