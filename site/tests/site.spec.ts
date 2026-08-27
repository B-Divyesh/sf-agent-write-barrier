import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

test('homepage has a complete semantic shell and no serious accessibility issues', async ({ page }) => {
  const errors: string[] = [];
  page.on('console', (message) => {
    if (message.type() === 'error') errors.push(message.text());
  });
  await page.goto('/');
  await expect(page).toHaveTitle(/Agent Write Barrier/);
  await expect(page.locator('main')).toHaveCount(1);
  await expect(page.locator('h1')).toHaveCount(1);
  await expect(page.locator('img[alt]')).toHaveCount(1);
  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations.filter((issue) => ['serious', 'critical'].includes(issue.impact ?? ''))).toEqual([]);
  expect(errors).toEqual([]);
});

test('receipt demo exposes loading and final state to keyboard and assistive technology', async ({ page }) => {
  await page.goto('/#demo');
  const button = page.getByRole('button', { name: /Run simulation/ });
  await button.focus();
  await page.keyboard.press('Enter');
  await expect(page.getByText('Checking policy and snapshot…')).toBeVisible();
  await expect(page.locator('.result-status')).toHaveText('BLOCKED · operation not permitted');
  await expect(page.locator('#demo-status')).toHaveText('BLOCKED · operation not permitted');

  await page.getByText('Old kernel', { exact: true }).click();
  await button.click();
  await expect(page.locator('.result-status')).toHaveText('REFUSED · enforcement unavailable');
  await expect(page.locator('.terminal-mode')).toHaveText('FAILED CLOSED');
});

test('390px layout has no horizontal overflow and critical targets remain usable', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/');
  const overflow = await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth);
  expect(overflow).toBeLessThanOrEqual(1);
  const install = page.getByRole('link', { name: /Install the barrier/ });
  const box = await install.boundingBox();
  expect(box?.height).toBeGreaterThanOrEqual(44);
});

for (const path of ['/privacy/', '/terms/']) {
  test(`${path} has one h1 and passes serious accessibility checks`, async ({ page }) => {
    await page.goto(path);
    await expect(page.locator('h1')).toHaveCount(1);
    await expect(page.locator('main')).toHaveCount(1);
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations.filter((issue) => ['serious', 'critical'].includes(issue.impact ?? ''))).toEqual([]);
  });
}
