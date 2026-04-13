import { EntityType } from "./entityTypes.js";
import type { EntityTypeValue } from "./entityTypes.js";

/**
 * The subset of entity types that sync through PowerSync streams.
 * Source: packages/contracts/sync-streams.json stream definitions.
 */
export const SYNCED_ENTITY_TYPES: EntityTypeValue[] = [
  EntityType.Initiative,
  EntityType.GuidanceQuest,
  EntityType.GuidanceRoutine,
  EntityType.GuidanceEpic,
  EntityType.GuidanceFocusSession,
  EntityType.GuidanceDailyCheckin,
  EntityType.KnowledgeNote,
  EntityType.TrackingItem,
  EntityType.TrackingShoppingList,
];
