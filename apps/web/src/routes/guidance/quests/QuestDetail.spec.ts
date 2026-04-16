import { describe, it, expect, vi } from 'vitest';
import { render } from '@testing-library/svelte';
import type { QuestStatus } from '$lib/utils/quest-transitions';

// ============================================================
// Mocks — hoisted before any import
// ============================================================

vi.mock('$lib/sync', () => ({
  getSyncClient: () => ({
    watch: async function* () {},
    execute: vi.fn(),
  }),
}));

const mockQuestById = vi.fn();

vi.mock('$lib/repositories/quest.svelte', () => ({
  questById: (...args: unknown[]) => mockQuestById(...args),
}));

// Import component ONCE — mocks are already in place
import QuestDetailPage from './[id]/+page.svelte';

const QUEST_DATA = { id: 'quest-123' };

function makeQuest(status: QuestStatus) {
  return {
    id: 'quest-123',
    title: 'Test Quest',
    description: null,
    status,
    priority: null,
    due_date: null,
    epic_id: null,
    initiative_id: null,
    routine_id: null,
    user_id: 'user-1',
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
    deleted_at: null,
  };
}

// ============================================================
// FA-005: Quest status transitions shown in the UI are
// restricted to valid moves per the state machine.
// ============================================================

describe('Quest detail — status transition buttons (FA-005)', () => {
  it('in_progress: shows Deferred, Complete, Cancel buttons — not Not Started', () => {
    mockQuestById.mockReturnValue(makeQuest('in_progress'));
    const { getByText, queryByText } = render(QuestDetailPage, { props: { data: QUEST_DATA } });

    expect(getByText('Deferred')).toBeTruthy();
    expect(getByText('Complete')).toBeTruthy();
    expect(getByText('Cancel')).toBeTruthy();
    expect(queryByText('Not Started')).toBeNull();
  });

  it('completed: renders no transition buttons', () => {
    mockQuestById.mockReturnValue(makeQuest('completed'));
    const { queryByText } = render(QuestDetailPage, { props: { data: QUEST_DATA } });

    expect(queryByText('Not Started')).toBeNull();
    expect(queryByText('In Progress')).toBeNull();
    expect(queryByText('Deferred')).toBeNull();
    expect(queryByText('Complete')).toBeNull();
    expect(queryByText('Cancel')).toBeNull();
  });

  it('not_started: shows In Progress and Cancel buttons only', () => {
    mockQuestById.mockReturnValue(makeQuest('not_started'));
    const { getByText, queryByText } = render(QuestDetailPage, { props: { data: QUEST_DATA } });

    expect(getByText('In Progress')).toBeTruthy();
    expect(getByText('Cancel')).toBeTruthy();
    expect(queryByText('Deferred')).toBeNull();
    expect(queryByText('Complete')).toBeNull();
  });

  it('cancelled: renders no transition buttons', () => {
    mockQuestById.mockReturnValue(makeQuest('cancelled'));
    const { queryByText } = render(QuestDetailPage, { props: { data: QUEST_DATA } });

    expect(queryByText('In Progress')).toBeNull();
    expect(queryByText('Complete')).toBeNull();
    expect(queryByText('Cancel')).toBeNull();
  });

  it('deferred: shows Not Started and Cancel buttons', () => {
    mockQuestById.mockReturnValue(makeQuest('deferred'));
    const { getByText, queryByText } = render(QuestDetailPage, { props: { data: QUEST_DATA } });

    expect(getByText('Not Started')).toBeTruthy();
    expect(getByText('Cancel')).toBeTruthy();
    expect(queryByText('In Progress')).toBeNull();
    expect(queryByText('Complete')).toBeNull();
  });
});
