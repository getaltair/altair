import { describe, expect, it } from 'vitest';
import { validNextStatuses } from './quest-transitions.js';

describe('validNextStatuses', () => {
  it('returns [in_progress, cancelled] from not_started', () => {
    expect(validNextStatuses('not_started')).toEqual(['in_progress', 'cancelled']);
  });

  it('returns [completed, deferred, cancelled] from in_progress (any order)', () => {
    const result = validNextStatuses('in_progress');
    expect(result).toHaveLength(3);
    expect(result).toContain('completed');
    expect(result).toContain('deferred');
    expect(result).toContain('cancelled');
  });

  it('returns [not_started, cancelled] from deferred (any order)', () => {
    const result = validNextStatuses('deferred');
    expect(result).toHaveLength(2);
    expect(result).toContain('not_started');
    expect(result).toContain('cancelled');
  });

  it('returns [] from completed (terminal state)', () => {
    expect(validNextStatuses('completed')).toEqual([]);
  });

  it('returns [] from cancelled (terminal state)', () => {
    expect(validNextStatuses('cancelled')).toEqual([]);
  });
});
