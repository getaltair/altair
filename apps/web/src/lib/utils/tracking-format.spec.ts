import { describe, it, expect } from 'vitest';
import {
	itemStatusColor,
	eventTypeLabel,
	eventTypeIcon,
	stockLevelClass,
	formatQuantity
} from './tracking-format.js';

describe('itemStatusColor', () => {
	it('returns green-toned classes for active status', () => {
		const result = itemStatusColor('active');
		expect(result).toContain('#a8c5a0');
		expect(result).toContain('#5a8a52');
	});

	it('returns muted classes for archived status', () => {
		const result = itemStatusColor('archived');
		expect(result).toContain('#e8eef0');
		expect(result).toContain('#566162');
	});
});

describe('eventTypeLabel', () => {
	it.each([
		['consumed', 'Consumed'],
		['restocked', 'Restocked'],
		['moved', 'Moved'],
		['adjusted', 'Adjusted'],
		['expired', 'Expired'],
		['donated', 'Donated']
	] as const)('returns "%s" label for %s', (type, expected) => {
		expect(eventTypeLabel(type)).toBe(expected);
	});
});

describe('eventTypeIcon', () => {
	it.each([
		['consumed', 'restaurant'],
		['restocked', 'inventory'],
		['moved', 'move_item'],
		['adjusted', 'tune'],
		['expired', 'event_busy'],
		['donated', 'volunteer_activism']
	] as const)('returns "%s" icon for %s', (type, expected) => {
		expect(eventTypeIcon(type)).toBe(expected);
	});
});

describe('stockLevelClass', () => {
	it('returns "out-of-stock" when quantity is 0', () => {
		expect(stockLevelClass(0)).toBe('out-of-stock');
	});

	it('returns "low-stock" when quantity is below min_quantity', () => {
		expect(stockLevelClass(2, 5)).toBe('low-stock');
	});

	it('returns empty string when quantity is at or above min_quantity', () => {
		expect(stockLevelClass(10, 5)).toBe('');
	});

	it('returns empty string when no min_quantity is provided', () => {
		expect(stockLevelClass(3)).toBe('');
	});
});

describe('formatQuantity', () => {
	it('returns quantity with unit when unit is provided', () => {
		expect(formatQuantity(3, 'kg')).toBe('3 kg');
	});

	it('returns quantity as string when no unit is provided', () => {
		expect(formatQuantity(5)).toBe('5');
	});
});
