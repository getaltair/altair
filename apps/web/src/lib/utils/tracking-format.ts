import type { TrackingItemStatus, ItemEventType } from '$lib/types/tracking.js';

export { formatDate } from '$lib/utils/date-format.js';

/**
 * Map an item status to Tailwind badge classes (bg + text).
 */
export function itemStatusColor(status: TrackingItemStatus): string {
	switch (status) {
		case 'active':
			return 'bg-[#a8c5a0]/30 text-[#5a8a52]';
		case 'archived':
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
		case 'donated':
			return 'Donated';
		default:
			return String(type).charAt(0).toUpperCase() + String(type).slice(1);
	}
}

/**
 * Icon character for an event type.
 */
export function eventTypeIcon(type: ItemEventType): string {
	switch (type) {
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
		case 'donated':
			return 'volunteer_activism';
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
