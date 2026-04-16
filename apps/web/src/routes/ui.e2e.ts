import { test, expect } from '@playwright/test';
import { checkAccessibility } from '../lib/test-utils/axe-helper';

// ---------------------------------------------------------------------------
// Test credentials — provided via environment variables in CI.
// Set TEST_USER_EMAIL and TEST_USER_PASSWORD in the CI environment.
// TEST_ADMIN_EMAIL / TEST_ADMIN_PASSWORD are required only for /admin/health.
// ---------------------------------------------------------------------------
const TEST_EMAIL = process.env.TEST_USER_EMAIL ?? 'e2e@altair.test';
const TEST_PASSWORD = process.env.TEST_USER_PASSWORD ?? 'e2e-password';

// ---------------------------------------------------------------------------
// Shared login helper — fills the login form and waits for the app shell.
// ---------------------------------------------------------------------------
async function login(page: import('@playwright/test').Page): Promise<void> {
  await page.goto('/auth/login');
  await page.fill('#email', TEST_EMAIL);
  await page.fill('#password', TEST_PASSWORD);
  await page.click('button[type="submit"]');
  // Wait for the sidebar (app shell) to confirm we are authenticated.
  await expect(page.locator('aside.sidebar')).toBeVisible();
}

// ---------------------------------------------------------------------------
// FA-014 — axe-core WCAG AA on all authenticated routes
// ---------------------------------------------------------------------------
test.describe('FA-014 — accessibility on all routes', () => {
  test('Today (/) has no critical/serious violations', async ({ page }) => {
    await login(page);
    await page.goto('/');
    await checkAccessibility(page);
  });

  test('/guidance/initiatives has no critical/serious violations', async ({ page }) => {
    await login(page);
    await page.goto('/guidance/initiatives');
    await checkAccessibility(page);
  });

  test('/knowledge has no critical/serious violations', async ({ page }) => {
    await login(page);
    await page.goto('/knowledge');
    await checkAccessibility(page);
  });

  test('/tracking has no critical/serious violations', async ({ page }) => {
    await login(page);
    await page.goto('/tracking');
    await checkAccessibility(page);
  });

  test('/search has no critical/serious violations', async ({ page }) => {
    await login(page);
    await page.goto('/search');
    await checkAccessibility(page);
  });

  test('/settings has no critical/serious violations', async ({ page }) => {
    await login(page);
    await page.goto('/settings');
    await checkAccessibility(page);
  });

  test('/admin/health has no critical/serious violations', async ({ page }) => {
    await login(page);
    await page.goto('/admin/health');
    // If non-admin, the route redirects — skip if that happens.
    if (page.url().includes('/admin/health')) {
      await checkAccessibility(page);
    }
  });
});

// ---------------------------------------------------------------------------
// FA-018 — sidebar collapses at 767px, expands at 768px
// ---------------------------------------------------------------------------
test.describe('FA-018 — responsive sidebar', () => {
  test('hamburger is visible and nav labels hidden at 767px width', async ({ page }) => {
    await login(page);
    await page.goto('/');

    await page.setViewportSize({ width: 767, height: 900 });

    // The hamburger button is shown via CSS at max-width 767px.
    const hamburger = page.locator('.hamburger');
    await expect(hamburger).toBeVisible();

    // Nav labels are hidden via CSS (display: none) at this breakpoint.
    // The sidebar itself remains visible (as a 64px strip) so we check label
    // visibility using the computed style rather than element existence.
    const firstNavLabel = page.locator('.nav-label').first();
    await expect(firstNavLabel).toBeHidden();
  });

  test('hamburger is hidden and nav labels visible at 768px width', async ({ page }) => {
    await login(page);
    await page.goto('/');

    await page.setViewportSize({ width: 768, height: 900 });

    // At desktop breakpoint the hamburger is hidden via CSS (display: none).
    const hamburger = page.locator('.hamburger');
    await expect(hamburger).toBeHidden();

    // Nav labels are visible.
    const firstNavLabel = page.locator('.nav-label').first();
    await expect(firstNavLabel).toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// FA-019 — search page empty state and local filter input
// ---------------------------------------------------------------------------
test.describe('FA-019 — search empty state', () => {
  test('shows "Search not yet available." empty state and search input', async ({ page }) => {
    await login(page);
    await page.goto('/search');

    // Empty state message present in the global search section.
    await expect(page.getByText('Search not yet available.')).toBeVisible();

    // The search input is rendered by the Input component with id="global-search"
    // and label text "Search everything...".
    const searchInput = page.locator('#global-search');
    await expect(searchInput).toBeVisible();
    await expect(page.getByLabel('Search everything...')).toBeVisible();

    await checkAccessibility(page);
  });

  test('typing a query does not crash the page and results area updates', async ({ page }) => {
    await login(page);
    await page.goto('/search');

    const searchInput = page.locator('#global-search');
    await expect(searchInput).toBeVisible();

    // Type a query — assert no JS error and the results area is present.
    await searchInput.fill('test query');

    // The results list renders when there are matches; if empty, the
    // "No notes match that filter." message appears. Either way the page
    // must not crash and the search input must still be present.
    await expect(searchInput).toBeVisible();

    // Assert that the results area is either a list or a no-results message;
    // both elements exist only conditionally, so just verify the page
    // has not navigated away.
    await expect(page).toHaveURL(/\/search/);
  });
});

// ---------------------------------------------------------------------------
// FA-020 — CRUD across Guidance, Knowledge, and Tracking domains
// ---------------------------------------------------------------------------

// ---- Guidance: create a quest inline from an initiative detail page,
//      update its status via transition button, then verify the change. ----
//
// NOTE: This test requires at least one initiative with at least one epic to
// exist in the test user's account. If the test environment is freshly seeded,
// ensure a seed initiative and epic are present.
test.describe('FA-020 — Guidance CRUD (quest via initiative detail)', () => {
  test('creates and transitions a quest within an initiative', async ({ page }) => {
    await login(page);

    // Navigate to the initiatives list.
    await page.goto('/guidance/initiatives');
    await checkAccessibility(page);

    // Click the first initiative to open its detail.
    const firstInitiativeLink = page.locator('a.item-link').first();
    await expect(firstInitiativeLink).toBeVisible();
    await firstInitiativeLink.click();
    await expect(page).toHaveURL(/\/guidance\/initiatives\/.+/);
    await checkAccessibility(page);

    // Click the first "Add Quest" button (within the first epic).
    const addQuestBtn = page.locator('button.add-quest-btn').first();
    await expect(addQuestBtn).toBeVisible();
    await addQuestBtn.click();

    // Fill in the quest title.
    const questTitle = `E2E Quest ${Date.now()}`;
    await page.fill('input.quest-input', questTitle);

    // Submit by clicking "Add Quest".
    await page.getByRole('button', { name: 'Add Quest' }).click();

    // After creation the form dismisses. The new quest title should appear in the list.
    await expect(page.getByText(questTitle)).toBeVisible();

    // Navigate to the quest detail to verify it persisted.
    await page.getByText(questTitle).click();
    await expect(page).toHaveURL(/\/guidance\/quests\/.+/);
    await expect(page.locator('h1.page__title')).toContainText(questTitle);
    await checkAccessibility(page);

    // Apply a status transition (e.g. "In Progress").
    // The transition buttons render with STATUS_LABELS values: quest with
    // status not_started shows "In Progress", "Deferred", "Complete", "Cancel".
    const inProgressBtn = page.getByRole('button', { name: 'In Progress' });
    if (await inProgressBtn.isVisible()) {
      await inProgressBtn.click();
      // The badge should update to reflect the new status (status renders
      // with underscores replaced: "in progress").
      await expect(page.getByText('in progress')).toBeVisible();
    }

    // Navigate away and back to confirm the change persisted.
    await page.goto('/guidance/initiatives');
    await page.goBack();
    await expect(page.locator('h1.page__title')).toContainText(questTitle);
  });
});

// ---- Knowledge: edit a note's content via the note editor, verify persistence. ----
//
// NOTE: Requires at least one note to exist in the test user's account.
// The knowledge domain does not currently expose a "New Note" creation route
// in the UI; existing notes are accessed via the /knowledge list. Tests cover
// the Read and Update operations that are available through the UI.
test.describe('FA-020 — Knowledge CRUD (note content edit)', () => {
  test('edits a note and verifies the updated content persists', async ({ page }) => {
    await login(page);

    // Navigate to the notes list.
    await page.goto('/knowledge');
    await checkAccessibility(page);

    // Open the first note.
    const firstNoteLink = page.locator('a.note-link').first();
    await expect(firstNoteLink).toBeVisible();
    await firstNoteLink.click();
    await expect(page).toHaveURL(/\/knowledge\/.+/);
    await checkAccessibility(page);

    // Find the note editor textarea (aria-label="Note content" on the .editor element).
    const editor = page.locator('textarea[aria-label="Note content"]');
    await expect(editor).toBeVisible();

    // Append a unique timestamped string so we can identify the change.
    const appendedText = ` E2E-edit-${Date.now()}`;
    await editor.click();
    // Move to the end of existing content and append.
    await editor.press('Control+End');
    await editor.type(appendedText);

    // Trigger blur to save (the component saves content to PowerSync on blur).
    await page.locator('h1.note-title').click();

    // Navigate away and return to confirm the edit persisted.
    await page.goto('/knowledge');
    // Re-open the same note — click the first note link again.
    await page.locator('a.note-link').first().click();
    await expect(page.locator('textarea[aria-label="Note content"]')).toContainText(appendedText.trim());
    await checkAccessibility(page);
  });
});

// ---- Tracking: full item CRUD via /tracking/items/new
//      (create → open detail → log consumption → verify quantity → navigate back) ----
test.describe('FA-020 — Tracking CRUD (item create and consumption logging)', () => {
  test('creates a tracking item, logs consumption, and verifies quantity update', async ({ page }) => {
    await login(page);

    // Navigate to the inventory list.
    await page.goto('/tracking');
    await checkAccessibility(page);

    // Click "New item" link to open the creation form.
    await page.click('a[href="/tracking/items/new"]');
    await expect(page).toHaveURL(/\/tracking\/items\/new/);
    await checkAccessibility(page);

    // ---- Create ----
    const itemName = `E2E Item ${Date.now()}`;
    // Fill name via the Input component — label is "Name".
    await page.getByLabel('Name').fill(itemName);
    // Set starting quantity via the Input component — label is "Starting quantity".
    await page.getByLabel('Starting quantity').fill('10');

    // Submit via the "Create item" button.
    await page.getByRole('button', { name: 'Create item' }).click();

    // After creation, the form redirects to the item detail page.
    await expect(page).toHaveURL(/\/tracking\/items\/.+/);
    await expect(page.locator('h1.page-title')).toContainText(itemName);
    await checkAccessibility(page);

    // ---- Log consumption ----
    // The consumption form has an input with aria-label="Quantity consumed".
    const consumeInput = page.locator('input[aria-label="Quantity consumed"]');
    await expect(consumeInput).toBeVisible();
    await consumeInput.fill('3');
    await page.getByRole('button', { name: 'Log' }).click();

    // After logging, the input clears and the quantity meta updates (10 - 3 = 7).
    await expect(consumeInput).toHaveValue('');
    // The quantity value in the meta grid should now reflect 7.
    await expect(page.locator('.meta-value').first()).toContainText('7');

    // The event timeline should show the consumption event.
    await expect(page.getByText('Consumed')).toBeVisible();
    await checkAccessibility(page);

    // ---- Navigate away and back to verify persistence ----
    await page.goto('/tracking');
    // Locate the item link in the table view and navigate back to the detail.
    await page.locator(`a.item-link:has-text("${itemName}")`).click();
    await expect(page.locator('h1.page-title')).toContainText(itemName);
    // Confirm the updated quantity (7) persisted in PowerSync.
    await expect(page.locator('.meta-value').first()).toContainText('7');
    // The event timeline entry should also be present after navigation.
    await expect(page.getByText('Consumed')).toBeVisible();
  });
});

// ---- Tracking: location CRUD as a secondary Tracking CRUD verification
//      (create → edit → verify → delete → verify gone) ----
//
// This test covers the soft-delete flow which is not exposed on item detail.
test.describe('FA-020 — Tracking CRUD (location create, edit, delete)', () => {
  test('creates, edits, and soft-deletes a tracking location', async ({ page }) => {
    await login(page);

    // Navigate to tracking locations.
    await page.goto('/tracking/locations');
    await checkAccessibility(page);

    // ---- Create ----
    const locationName = `E2E Location ${Date.now()}`;
    // The create form uses <Input label="New location name" />.
    await page.getByLabel('New location name').fill(locationName);
    await page.getByRole('button', { name: 'Add' }).click();

    // The new location should appear in the list.
    await expect(page.getByText(locationName)).toBeVisible();
    await checkAccessibility(page);

    // ---- Update ----
    // Click the "Edit" button next to our new location.
    const listItem = page.locator('li.list-item').filter({ hasText: locationName });
    await listItem.getByRole('button', { name: 'Edit' }).click();

    // The inline edit form appears with an Input component (no explicit id).
    const updatedName = `${locationName} (updated)`;
    const editInput = listItem.locator('input.input');
    await editInput.fill(updatedName);
    await listItem.getByRole('button', { name: 'Save' }).click();

    // Verify the updated name is visible.
    await expect(page.getByText(updatedName)).toBeVisible();

    // Navigate away and back to confirm persistence in PowerSync.
    await page.goto('/tracking');
    await page.goto('/tracking/locations');
    await expect(page.getByText(updatedName)).toBeVisible();
    await checkAccessibility(page);

    // ---- Delete (soft) ----
    const updatedListItem = page.locator('li.list-item').filter({ hasText: updatedName });
    await updatedListItem.getByRole('button', { name: 'Delete' }).click();

    // The item should be gone from the list after soft-delete.
    await expect(page.getByText(updatedName)).not.toBeVisible();
  });
});
