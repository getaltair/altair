import { test, expect } from '@playwright/test';

// ---------------------------------------------------------------------------
// Admin guard E2E test — FA-002
// Verifies that a non-admin user navigating to /admin is redirected to /.
// ---------------------------------------------------------------------------

/** Generate a unique email for each test run to avoid registration conflicts. */
function uniqueEmail(): string {
  return `e2e-${Date.now()}-${Math.floor(Math.random() * 10000)}@altair.test`;
}

// Newly registered users are non-admin by default. The admin guard in
// apps/web/src/routes/admin/+layout.server.ts fetches /api/auth/me and
// redirects to / when is_admin is falsy.
test('non-admin user navigating to /admin is redirected to /', async ({ page }) => {
  const email = uniqueEmail();
  const password = 'TestPass123!';
  const displayName = 'E2E NonAdmin';

  // Step 1 — Register a fresh non-admin account
  await page.goto('/auth/register');
  await page.waitForSelector('form');

  await page.fill('input[name="email"]', email);
  await page.fill('input[name="display_name"]', displayName);
  await page.fill('input[name="password"]', password);

  await page.click('button[type="submit"]');

  // Step 2 — Confirm successful login (redirects to /)
  await page.waitForURL('/');

  // Step 3 — Attempt to navigate to the admin area
  await page.goto('/admin');

  // Step 4 — Assert the guard redirected away from /admin back to /
  // The server load issues a 303 redirect; Playwright follows it automatically.
  await page.waitForURL('/', { timeout: 10000 });
  expect(page.url()).not.toContain('/admin');
});
