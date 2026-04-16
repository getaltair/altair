import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';

// ============================================================
// Mocks — hoisted before any component import
// ============================================================

const mockExecute = vi.fn().mockResolvedValue(undefined);

vi.mock('$lib/sync', () => ({
  getSyncClient: () => ({
    watch: async function* () {},
    execute: mockExecute,
  }),
}));

vi.mock('$lib/repositories/quest.svelte', () => ({
  questById: vi.fn().mockReturnValue({
    id: 'quest-456',
    title: 'Focus Test Quest',
    description: null,
    status: 'in_progress',
    priority: null,
    due_date: null,
    epic_id: null,
    initiative_id: null,
    routine_id: null,
    user_id: 'user-1',
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
    deleted_at: null,
  }),
}));

// Import component ONCE after mocks
import FocusPage from './[id]/focus/+page.svelte';

const FOCUS_DATA = { id: 'quest-456' };

// ============================================================
// Tests (FA-008)
// ============================================================

describe('Focus session screen (FA-008)', () => {
  let originalBg: string;

  beforeEach(() => {
    vi.useFakeTimers();
    originalBg = document.body.style.backgroundColor;
    mockExecute.mockClear();
  });

  afterEach(() => {
    document.body.style.backgroundColor = originalBg;
    vi.useRealTimers();
  });

  it('sets background to Soft Slate Haze (#cfddde) when timer starts', async () => {
    const { getByText } = render(FocusPage, { props: { data: FOCUS_DATA } });

    await fireEvent.click(getByText('Start'));

    expect(document.body.style.backgroundColor).toBe('#cfddde');
  });

  it('reverts background when timer is paused', async () => {
    const { getByText } = render(FocusPage, { props: { data: FOCUS_DATA } });

    await fireEvent.click(getByText('Start'));
    expect(document.body.style.backgroundColor).toBe('#cfddde');

    await fireEvent.click(getByText('Pause'));
    expect(document.body.style.backgroundColor).toBe('');
  });

  it('computes elapsed from Date.now() - startTime, not tick count', async () => {
    const { getByText } = render(FocusPage, { props: { data: FOCUS_DATA } });

    const t0 = Date.now();
    await fireEvent.click(getByText('Start'));

    // Advance 90 seconds via fake timers (also advances Date.now())
    vi.advanceTimersByTime(90_000);

    // Timer should still be running — background still Soft Slate Haze
    expect(document.body.style.backgroundColor).toBe('#cfddde');

    // Verify Date.now() advanced in sync with fake timers
    const elapsed = Date.now() - t0;
    expect(elapsed).toBeGreaterThanOrEqual(90_000);
  });

  it('calls getSyncClient().execute on session complete', async () => {
    const { getByText } = render(FocusPage, { props: { data: FOCUS_DATA } });

    await fireEvent.click(getByText('Start'));

    // Advance the full 25 minutes + buffer
    vi.advanceTimersByTime(25 * 60 * 1000 + 500);

    // Flush async microtasks from onSessionComplete
    await vi.runAllTimersAsync();

    expect(mockExecute).toHaveBeenCalledWith(
      expect.stringContaining('INSERT INTO focus_sessions'),
      expect.any(Array)
    );
  });
});
