import { describe, it, expect } from 'vitest';
import { contentTypeLabel, truncateContent } from './knowledge-format.js';

describe('contentTypeLabel', () => {
	it.each([
		['markdown', 'Markdown'],
		['plain', 'Plain Text'],
		['rich_text', 'Rich Text'],
		['code', 'Code']
	])('returns "%s" label for %s', (type, expected) => {
		expect(contentTypeLabel(type)).toBe(expected);
	});

	it('capitalizes unknown content types', () => {
		expect(contentTypeLabel('html')).toBe('Html');
	});
});

describe('truncateContent', () => {
	it('returns content unchanged when shorter than limit', () => {
		expect(truncateContent('short text', 120)).toBe('short text');
	});

	it('truncates and appends ellipsis when content exceeds limit', () => {
		const longContent = 'a'.repeat(200);
		const result = truncateContent(longContent, 120);
		expect(result.length).toBeLessThanOrEqual(121); // 120 + ellipsis char
		expect(result).toContain('\u2026');
	});

	it('uses default limit of 120', () => {
		const exactly120 = 'b'.repeat(120);
		expect(truncateContent(exactly120)).toBe(exactly120);

		const over120 = 'c'.repeat(121);
		expect(truncateContent(over120)).toContain('\u2026');
	});
});
