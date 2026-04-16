import { expect } from '@playwright/test';
import type { Page } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

export async function checkAccessibility(page: Page): Promise<void> {
  const results = await new AxeBuilder({ page })
    .withTags(['wcag2a', 'wcag2aa'])
    .analyze();
  const criticalOrSerious = results.violations.filter(
    v => v.impact === 'critical' || v.impact === 'serious'
  );
  expect(criticalOrSerious, `Accessibility violations: ${JSON.stringify(criticalOrSerious, null, 2)}`).toHaveLength(0);
}
