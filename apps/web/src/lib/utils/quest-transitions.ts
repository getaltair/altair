// Quest status type mirrors the spec (docs/specs/06-state-machines.md)
// and the database storage values.
export type QuestStatus = 'not_started' | 'in_progress' | 'completed' | 'deferred' | 'cancelled';

const TRANSITIONS: Record<QuestStatus, QuestStatus[]> = {
  not_started: ['in_progress', 'cancelled'],
  in_progress: ['completed', 'deferred', 'cancelled'],
  deferred: ['not_started', 'cancelled'],
  completed: [],
  cancelled: [],
};

/**
 * Returns the valid next statuses from the given current status.
 * Returns [] for terminal states (completed, cancelled).
 */
export function validNextStatuses(current: QuestStatus): QuestStatus[] {
  return TRANSITIONS[current];
}
