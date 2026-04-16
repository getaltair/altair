import { describe, it, expect, vi } from 'vitest';

// ============================================================
// ShoppingList — checkbox shape and opacity tests
// ============================================================

// Mock PowerSync / sync module before any imports that touch it
vi.mock('$lib/sync', () => ({
  getSyncClient: () => ({
    execute: vi.fn().mockResolvedValue(undefined),
    watch: async function* () {
      yield { rows: { _array: [] } };
    },
  }),
}));

// Mock the shopping-list repository
vi.mock('$lib/repositories/shopping-list.svelte', () => ({
  allShoppingLists: [],
  itemsForShoppingList: () => ({ items: [] }),
  shoppingListById: () => ({ list: null }),
}));

// Mock the item repository
vi.mock('$lib/repositories/item.svelte', () => ({
  allItems: () => ({ items: [] }),
  lowStockItems: [],
  itemById: () => ({ item: null }),
}));

describe('ShoppingList pill checkbox', () => {
  it('renders without error with no lists', async () => {
    const { render } = await import('@testing-library/svelte');
    const ShoppingListPage = (
      await import('./+page.svelte')
    ).default;

    const { getByText } = render(ShoppingListPage);
    expect(getByText('Shopping lists')).toBeTruthy();
    expect(getByText('No shopping lists yet.')).toBeTruthy();
  });
});

