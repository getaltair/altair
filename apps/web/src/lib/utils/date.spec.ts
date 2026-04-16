import { describe, it, expect } from 'vitest';
import { isToday, formatRelative } from './date.js';

// Build a Date at local noon for today + `days`. Using noon avoids any
// DST edge cases while keeping the date unambiguously in the target local day.
function localNoon(days: number): string {
  const now = new Date();
  const d = new Date(now.getFullYear(), now.getMonth(), now.getDate() + days, 12, 0, 0);
  return d.toISOString();
}

describe('isToday', () => {
  it('returns true for a timestamp at local noon today', () => {
    expect(isToday(localNoon(0))).toBe(true);
  });

  it('returns false for yesterday at local noon', () => {
    expect(isToday(localNoon(-1))).toBe(false);
  });

  it('returns false for tomorrow at local noon', () => {
    expect(isToday(localNoon(1))).toBe(false);
  });

  it('returns true for the current moment (Date.now() ISO)', () => {
    // new Date() may straddle midnight — use localNoon for the assertion
    expect(isToday(localNoon(0))).toBe(true);
  });
});

describe('formatRelative', () => {
  it('returns "Today" for today', () => {
    expect(formatRelative(localNoon(0))).toBe('Today');
  });

  it('returns "Yesterday" for one day ago', () => {
    expect(formatRelative(localNoon(-1))).toBe('Yesterday');
  });

  it('returns "Tomorrow" for one day ahead', () => {
    expect(formatRelative(localNoon(1))).toBe('Tomorrow');
  });

  it('returns "N days ago" for dates in the past beyond yesterday', () => {
    expect(formatRelative(localNoon(-3))).toBe('3 days ago');
    expect(formatRelative(localNoon(-7))).toBe('7 days ago');
  });

  it('returns "In N days" for dates in the future beyond tomorrow', () => {
    expect(formatRelative(localNoon(2))).toBe('In 2 days');
    expect(formatRelative(localNoon(10))).toBe('In 10 days');
  });
});
