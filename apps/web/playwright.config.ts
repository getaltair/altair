import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './src',
  testMatch: ['src/**/*.e2e.ts'],
  use: {
    baseURL: process.env.BASE_URL || 'http://localhost:5173',
    headless: true,
    trace: 'on-first-retry',
  },
  projects: [{ name: 'chromium', use: { ...devices['Desktop Chrome'] } }],
  webServer: {
    command: 'bun dev',
    url: 'http://localhost:5173',
    reuseExistingServer: !process.env.CI,
  },
});
