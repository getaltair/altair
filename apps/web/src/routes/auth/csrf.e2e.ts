import { test, expect } from '@playwright/test';

const BASE = 'http://localhost:5173';

// ---------------------------------------------------------------------------
// FA-015: SvelteKit's built-in checkOrigin rejects state-changing form actions
//         submitted from a mismatched Origin with HTTP 403 (invariant SEC-6,
//         ADR-012 — no custom CSRF token hook required).
// ---------------------------------------------------------------------------

test('FA-015: form action from mismatched Origin is rejected with 403', async ({ page }) => {
  // Use Playwright's request API to send a POST directly without loading the
  // page.  The 'Origin' header is explicitly set to a different domain so that
  // SvelteKit's checkOrigin middleware rejects it.
  const response = await page.request.post(`${BASE}/?/updateProfile`, {
    headers: {
      Origin: 'https://evil.example.com',
      'Content-Type': 'application/x-www-form-urlencoded',
    },
    data: 'displayName=hacked',
  });

  expect(response.status()).toBe(403);
});

test('FA-015: settings form action from mismatched Origin is rejected with 403', async ({
  page,
}) => {
  // Also verify a named settings route form action — same CSRF protection
  // applies to all SvelteKit form actions regardless of route.
  const response = await page.request.post(`${BASE}/settings?/updateProfile`, {
    headers: {
      Origin: 'https://evil.example.com',
      'Content-Type': 'application/x-www-form-urlencoded',
    },
    data: 'displayName=hacked',
  });

  expect(response.status()).toBe(403);
});

test('FA-015: login action from mismatched Origin is rejected with 403', async ({ page }) => {
  // Verify CSRF protection also covers the auth login action, which is a
  // state-changing POST.
  const response = await page.request.post(`${BASE}/auth/login`, {
    headers: {
      Origin: 'https://evil.example.com',
      'Content-Type': 'application/x-www-form-urlencoded',
    },
    data: 'email=victim@altair.test&password=password',
  });

  expect(response.status()).toBe(403);
});
