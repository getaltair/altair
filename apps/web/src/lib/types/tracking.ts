/**
 * TypeScript type interfaces for the Tracking domain.
 *
 * These interfaces mirror the PowerSync local schema tables for tracking
 * items, events, locations, categories, and shopping lists. All ID fields
 * are `string` (UUID text). All timestamps are `string` (ISO 8601 text
 * from SQLite via PowerSync). Nullable fields become `T | null`.
 */

// ---------------------------------------------------------------------------
// Enum literal types
// ---------------------------------------------------------------------------

export type TrackingItemStatus = 'active' | 'archived';

export type ItemEventType = 'consumed' | 'restocked' | 'moved' | 'adjusted' | 'expired' | 'donated';

export type ShoppingListStatus = 'active' | 'completed' | 'archived';

// ---------------------------------------------------------------------------
// Entity interfaces
// ---------------------------------------------------------------------------

export interface TrackingItem {
	id: string;
	user_id: string;
	household_id: string;
	category_id: string | null;
	location_id: string | null;
	name: string;
	description: string | null;
	quantity: number;
	unit: string | null;
	min_quantity: number | null;
	barcode: string | null;
	status: TrackingItemStatus;
	created_at: string;
	updated_at: string;
}

export interface TrackingItemEvent {
	id: string;
	item_id: string;
	user_id: string;
	event_type: ItemEventType;
	quantity_change: number;
	notes: string | null;
	created_at: string;
}

export interface TrackingLocation {
	id: string;
	user_id: string;
	household_id: string;
	name: string;
	description: string | null;
	parent_location_id: string | null;
	created_at: string;
	updated_at: string;
}

export interface TrackingCategory {
	id: string;
	user_id: string;
	household_id: string;
	name: string;
	description: string | null;
	parent_category_id: string | null;
	created_at: string;
	updated_at: string;
}

export interface TrackingShoppingList {
	id: string;
	user_id: string;
	household_id: string;
	name: string;
	status: ShoppingListStatus;
	created_at: string;
	updated_at: string;
}

export interface TrackingShoppingListItem {
	id: string;
	shopping_list_id: string;
	item_id: string | null;
	name: string;
	quantity: number;
	unit: string | null;
	is_checked: number; // 0/1 from SQLite
	created_at: string;
}
