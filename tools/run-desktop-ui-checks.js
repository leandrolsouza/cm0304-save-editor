import { readFile } from 'node:fs/promises';
import { chromium } from 'playwright-core';
import checkDesktopUi from './check-staff-ui.playwright.js';

const assets = new Map(await Promise.all([
  ['/', 'index.html', 'text/html'],
  ['/main.js', 'main.js', 'text/javascript'],
  ['/i18n.js', 'i18n.js', 'text/javascript'],
  ['/en.js', 'en.js', 'text/javascript'],
  ['/style.css', 'style.css', 'text/css'],
].map(async ([route, filename, type]) => {
  const body = await readFile(new URL('../apps/desktop/dist/' + filename, import.meta.url));
  return [route, { type, body }];
})));

const origin = 'https://cm0304-ui.test';
const channel = process.env.CM0304_UI_BROWSER || 'msedge';
const browser = await chromium.launch({ channel, headless: true });
try {
  const context = await browser.newContext();
  await context.route(origin + '/**', async route => {
    const asset = assets.get(new URL(route.request().url()).pathname);
    await route.fulfill(asset
      ? { contentType: asset.type, body: asset.body }
      : { status: 404 });
  });
  const page = await context.newPage();
  page.setDefaultTimeout(8000);
  const result = await checkDesktopUi(page, origin + '/');
  for (const check of result.checks) console.log('OK ' + check);
  console.log(result.checks.length + ' cenários concluídos.');
} finally {
  await browser.close();
}
