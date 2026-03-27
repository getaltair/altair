import type { TrackingItemStatus, ItemEventType } from '$lib/types/tracking.js';

/**
 * Format an ISO date string into a human-readable form.
 */
export function formatDate(dateStr: string): string {
	const d = new Date(dateStr);
	return d.toLocaleDateString('en-US', {
		month: 'short',
		day: 'numeric',
		year: 'numeric'
	});
}

/**
 * Map an item status to Tailwind badge classes (bg + text).
 */
export function itemStatusColor(status: TrackingItemStatus): string {
	switch (status) {
		case 'active':
			return 'bg-[#a8c5a0]/30 text-[#5a8a52]';
		case 'consumed':
			return 'bg-[#c7e7fa]/50 text-[#446273]';
		case 'disposed':
		case 'lost':
			return 'bg-[#e8eef0] text-[#566162]';
		default:
			return 'bg-surface-low text-on-surface-muted';
	}
}

/**
 * Human-readable label for an event type.
 */
export function eventTypeLabel(type: ItemEventType): string {
	switch (type) {
		case 'added':
			return 'Added';
		case 'removed':
			return 'Removed';
		case 'consumed':
			return 'Consumed';
		case 'restocked':
			return 'Restocked';
		case 'adjusted':
			return 'Adjusted';
		case 'moved':
			return 'Moved';
		case 'expired':
			return 'Expired';
		default:
			return String(type).charAt(0).toUpperCase() + String(type).slice(1);
	}
}

/**
 * Icon character for an event type.
 */
export function eventTypeIcon(type: ItemEventType): string {
	switch (type) {
		case 'added':
			return 'add_circle';
		case 'removed':
			return 'remove_circle';
		case 'consumed':
			return 'restaurant';
		case 'restocked':
			return 'inventory';
		case 'adjusted':
			return 'tune';
		case 'moved':
			return 'move_item';
		case 'expired':
			return 'event_busy';
		default:
			return 'circle';
	}
}

/**
 * CSS class for stock-level visual indicator.
 */
export function stockLevelClass(quantity: number, minQuantity?: number | null): string {
	if (quantity <= 0) return 'out-of-stock';
	if (minQuantity != null && quantity < minQuantity) return 'low-stock';
	return '';
}

/**
 * Format a quantity with optional unit: "3 kg", "5", etc.
 */
export function formatQuantity(quantity: number, unit?: string | null): string {
	if (unit) return `${quantity} ${unit}`;
	return String(quantity);
}
