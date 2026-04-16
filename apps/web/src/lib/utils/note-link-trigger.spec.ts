import { describe, expect, it } from 'vitest';
import { detectLinkTrigger } from './note-link-trigger.js';

describe('detectLinkTrigger', () => {
  it('detects a single character query', () => {
    expect(detectLinkTrigger('hello [[w', 9)).toEqual({ query: 'w', triggerStart: 6 });
  });

  it('detects a multi-character query', () => {
    expect(detectLinkTrigger('hello [[wo', 10)).toEqual({ query: 'wo', triggerStart: 6 });
  });

  it('returns null when query contains whitespace', () => {
    expect(detectLinkTrigger('hello [[wo rld', 13)).toBeNull();
  });

  it('returns null when no trigger is present', () => {
    expect(detectLinkTrigger('hello world', 11)).toBeNull();
  });

  it('returns null when cursor is in plain text with no [[', () => {
    expect(detectLinkTrigger('hello world', 5)).toBeNull();
  });

  it('detects empty query when cursor is immediately after [[', () => {
    expect(detectLinkTrigger('test[[', 6)).toEqual({ query: '', triggerStart: 4 });
  });

  it('returns null when cursor is before any [[ in the string', () => {
    // cursor at position 1 (inside 'ab') — no [[ before it
    expect(detectLinkTrigger('ab [[word', 1)).toBeNull();
  });

  it('detects an empty query at cursor immediately after [[', () => {
    expect(detectLinkTrigger('[[', 2)).toEqual({ query: '', triggerStart: 0 });
  });

  it('detects the last trigger when multiple [[ exist', () => {
    expect(detectLinkTrigger('first [[a second [[b', 19)).toEqual({
      query: 'b',
      triggerStart: 17,
    });
  });
});
