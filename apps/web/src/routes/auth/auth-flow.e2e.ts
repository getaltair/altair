import { test, expect } from '@playwright/test';
import { checkAccessibility } from '../../lib/test-utils/axe-helper';

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/** Generate a unique email for each test run to avoid registration conflicts. */
function uniqueEmail(): string {
  return `e2e-${Date.now()}-${Math.floor(Math.random() * 10000)}@altair.test`;
}

const API_BASE = process.env.PUBLIC_API_BASE_URL ?? 'http://localhost:3000';

/**
 * Create an initiative via the API and return its id.
 */
async function createInitiativeViaApi(
  request: import('@playwright/test').APIRequestContext,
  accessToken: string,
  title: string
): Promise<string> {
  const res = await request.post(`${API_BASE}/api/initiatives`, {
    headers: { Authorization: `Bearer ${accessToken}` },
    data: { title, status: 'active' },
  });
  if (!res.ok()) {
    throw new Error(`Create initiative failed: ${res.status()} ${await res.text()}`);
  }
  const body = await res.json();
  return body.id as string;
}

/**
 * Create an epic under an initiative via the API and return its id.
 */
async function createEpicViaApi(
  request: import('@playwright/test').APIRequestContext,
  accessToken: string,
  initiativeId: string,
  title: string
): Promise<string> {
  const res = await request.post(`${API_BASE}/api/guidance/epics`, {
    headers: { Authorization: `Bearer ${accessToken}` },
    data: { title, initiative_id: initiativeId, status: 'active' },
  });
  if (!res.ok()) {
    throw new Error(`Create epic failed: ${res.status()} ${await res.text()}`);
  }
  const body = await res.json();
  return body.id as string;
}

// ---------------------------------------------------------------------------
// Critical-path E2E test — FA-013, FA-014
// ---------------------------------------------------------------------------

test('critical path: register → quest lifecycle → logout', async ({ page, request }) => {
  const email = uniqueEmail();
  const password = 'TestPass123!';
  const displayName = 'E2E Tester';

  // -------------------------------------------------------------------------
  // Step 1 — Navigate to /auth/register, fill the form, submit
  // -------------------------------------------------------------------------
  await page.goto('/auth/register');
  await page.waitForSelector('form');

  await page.fill('input[name="email"]', email);
  await page.fill('input[name="display_name"]', displayName);
  await page.fill('input[name="password"]', password);

  await page.click('button[type="submit"]');

  // -------------------------------------------------------------------------
  // Step 2 — Assert redirect to / (authenticated home)
  // -------------------------------------------------------------------------
  await page.waitForURL('/');
  expect(page.url()).toMatch(/\/$/);

  // -------------------------------------------------------------------------
  // Step 3 — Accessibility check on the home page (FA-014)
  // -------------------------------------------------------------------------
  await checkAccessibility(page);

  // -------------------------------------------------------------------------
  // Pre-seed: create an initiative + epic via the API so the UI has something
  // to work with. Log in via API with the registered credentials to get a
  // token; the browser session is already authenticated separately via cookies.
  // -------------------------------------------------------------------------
  const loginRes = await request.post(`${API_BASE}/api/auth/login`, {
    data: { email, password },
  });
  const loginBody = await loginRes.json();
  const apiToken = loginBody.access_token as string;

  const initiativeId = await createInitiativeViaApi(request, apiToken, 'E2E Initiative');
  await createEpicViaApi(request, apiToken, initiativeId, 'E2E Epic');

  // -------------------------------------------------------------------------
  // Step 4 — Navigate to /guidance/initiatives, accessibility check (FA-014)
  // -------------------------------------------------------------------------
  await page.goto('/guidance/initiatives');
  await page.waitForSelector('h1');
  await checkAccessibility(page);

  // -------------------------------------------------------------------------
  // Step 5 — Navigate to the initiative detail and create a quest via the
  // inline form inside the first epic
  // -------------------------------------------------------------------------
  await page.goto(`/guidance/initiatives/${initiativeId}`);

  // Wait for the page to render the epic section
  await page.waitForSelector('.epic', { timeout: 10000 });

  // Click the "+ Add Quest" button for the first epic
  await page.click('.add-quest-btn');

  // Fill the inline quest title input
  const questTitle = `E2E Quest ${Date.now()}`;
  await page.waitForSelector('.quest-input');
  await page.fill('.quest-input', questTitle);

  // Click "Add Quest" to submit
  await page.click('button:has-text("Add Quest")');

  // Wait for the inline form to close (creatingForEpicId resets to null)
  await page.waitForSelector('.add-quest-btn', { timeout: 10000 });

  // -------------------------------------------------------------------------
  // Step 6 — Navigate to the new quest's detail page
  // The quest should appear in the quest list under the epic. Click it.
  // -------------------------------------------------------------------------
  const questLink = page.locator('.quest-link', { hasText: questTitle });
  await questLink.waitFor({ timeout: 10000 });
  await questLink.click();

  // Wait for the quest detail page to load
  await page.waitForSelector('.page__title');

  // -------------------------------------------------------------------------
  // Step 7 — Assert quest status is visible
  // The Badge renders quest.status.replace(/_/g, ' '), so not_started → "not started"
  // -------------------------------------------------------------------------
  const statusBadge = page.locator('.page__title-group').getByText(/not started/i);
  await statusBadge.waitFor({ timeout: 5000 });
  await expect(statusBadge).toBeVisible();

  // -------------------------------------------------------------------------
  // Step 8 — Transition quest: not_started → in_progress → completed
  // The STATUS_LABELS map: in_progress → 'In Progress', completed → 'Complete'
  // Transition buttons are in .transition-buttons
  // -------------------------------------------------------------------------

  // First transition: not_started → in_progress
  const inProgressBtn = page.locator('.transition-buttons button', { hasText: 'In Progress' });
  await inProgressBtn.waitFor({ timeout: 5000 });
  await inProgressBtn.click();

  // Wait for the badge to reflect "in progress" before the next transition
  const inProgressBadge = page.locator('.page__title-group').getByText(/in progress/i);
  await inProgressBadge.waitFor({ timeout: 8000 });

  // Second transition: in_progress → completed
  const completeBtn = page.locator('.transition-buttons button', { hasText: 'Complete' });
  await completeBtn.waitFor({ timeout: 5000 });
  await completeBtn.click();

  // -------------------------------------------------------------------------
  // Step 9 — Assert status shown as "completed"
  // The Badge renders quest.status.replace(/_/g, ' '), completed → "completed"
  // -------------------------------------------------------------------------
  const completedBadge = page.locator('.page__title-group').getByText(/completed/i);
  await completedBadge.waitFor({ timeout: 8000 });
  await expect(completedBadge).toBeVisible();

  // Transition buttons section should be gone (no valid next statuses for completed)
  await expect(page.locator('.transitions')).not.toBeAttached();

  // -------------------------------------------------------------------------
  // Step 10 — Navigate to /settings and trigger logout
  // The logout form submits to ?/logout and the button text is "Sign Out"
  // -------------------------------------------------------------------------
  await page.goto('/settings');
  await page.waitForSelector('h1');

  const signOutBtn = page.locator('button', { hasText: 'Sign Out' });
  await signOutBtn.waitFor({ timeout: 5000 });
  await signOutBtn.click();

  // -------------------------------------------------------------------------
  // Step 11 — Assert redirect to /auth/login
  // -------------------------------------------------------------------------
  await page.waitForURL('**/auth/login', { timeout: 10000 });
  expect(page.url()).toContain('/auth/login');
});
