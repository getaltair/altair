import { describe, it, expect } from 'vitest';
import { formatDate } from './date-format.js';

describe('formatDate', () => {
	it('returns a formatted string for a valid ISO date', () => {
		const result = formatDate('2024-01-15T10:30:00Z');
		// toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' })
		expect(result).toContain('Jan');
		expect(result).toContain('15');
		expect(result).toContain('2024');
	});

	it('handles another date string correctly', () => {
		const result = formatDate('2023-12-25T12:00:00Z');
		expect(result).toContain('Dec');
		expect(result).toContain('25');
		expect(result).toContain('2023');
	});
});
