import { test, expect } from '@playwright/test';

test('first screen states the job, audience, and sample action', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/');
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('Block agent writes outside your project');
  await expect(page.getByText(/For developers running local coding agents/)).toBeVisible();
  await expect(page.getByRole('link', { name: /Try it with sample data/ })).toBeVisible();
  await expect(page.getByText(/Opens a completed sample run/)).toBeVisible();
});

test('query demo entry opens the isolated sample with reset and exit controls', async ({ page }) => {
  await page.goto('/?demo=1');
  await expect(page).toHaveURL(/\/demo\/$/);
  await expect(page.getByText('Demo — sample data, nothing is saved')).toBeVisible();
  await expect(page.getByText('BLOCKED  ../blocked-agent.conf')).toBeVisible();
  expect(await page.evaluate(() => Object.keys(localStorage))).toEqual(['demo:awb:session']);
  await page.getByRole('button', { name: 'Reset demo', exact: true }).first().click();
  await expect(page.locator('#demo-reset-status')).toContainText('Demo reset');
  await expect(page.getByRole('link', { name: 'Start for real' }).first()).toHaveAttribute('href', '/#install');
});

test('reduced motion removes meaningful transition duration', async ({ page }) => {
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await page.goto('/');
  const duration = await page.locator('.button').first().evaluate((element) => getComputedStyle(element).transitionDuration);
  expect(Number.parseFloat(duration)).toBeLessThanOrEqual(0.001);
});
