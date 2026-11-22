import assert from 'node:assert/strict';
import { execFileSync, spawn } from 'node:child_process';
import { createHash, randomBytes } from 'node:crypto';
import { createReadStream } from 'node:fs';
import { copyFile, mkdir, mkdtemp, readFile, stat, writeFile } from 'node:fs/promises';
import { createServer } from 'node:http';
import { join, resolve } from 'node:path';
import { setTimeout as delay } from 'node:timers/promises';
import { chromium } from 'playwright-core';

const sample = resolve(process.argv[2] ?? 'game/user data/games/save_inicial.cm4');
const hash = async file => createHash('sha256').update(await readFile(file)).digest('hex');
const sourceHash = await hash(sample);
await mkdir('target', { recursive: true });
const staging = await mkdtemp(resolve('target/updater-test-'));
const sampleCopy = join(staging, 'carreira.cm4');
await copyFile(sample, sampleCopy);

const product = 'CM0304 Updater Test';
const binary = 'cm0304-updater-test';
const { version: currentVersion } = JSON.parse(await readFile('apps/desktop/src-tauri/tauri.conf.json', 'utf8'));
const [major, minor, patch] = currentVersion.split('.').map(Number);
const nextVersion = `${major}.${minor}.${patch + 1}`;
const installDir = join(staging, 'installed');
const app = join(installDir, binary + '.exe');
const key = join(staging, 'test.key');
const password = randomBytes(24).toString('base64');
const tauriCli = resolve('node_modules/@tauri-apps/cli/tauri.js');
function tauri(args, env = process.env, stdio = 'inherit') {
  execFileSync(process.execPath, [tauriCli, ...args], { cwd: 'apps/desktop', env, stdio });
}

function launch(browserArgs) {
  const processHandle = spawn(app, [], { stdio: 'ignore', env: { ...process.env, WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS: browserArgs } });
  processHandle.unref();
}

let manifest;
let releaseManifest;
let downloaded = 0;
let installer;
const server = createServer(async (request, response) => {
  if (request.url === '/latest.json') {
    if (releaseManifest) await releaseManifest;
    response.writeHead(manifest ? 200 : 204, { 'Content-Type': 'application/json' });
    response.end(manifest);
  } else if (request.url === '/setup.exe' && installer) {
    response.writeHead(200, { 'Content-Length': (await stat(installer)).size });
    response.once('finish', () => downloaded++);
    createReadStream(installer).pipe(response);
  } else {
    response.writeHead(404).end();
  }
});
const listen = server => new Promise(resolve => server.listen(0, '127.0.0.1', () => resolve(server.address().port)));
const endpoint = `http://127.0.0.1:${await listen(server)}`;
const cdpProbe = createServer();
const cdp = `http://127.0.0.1:${await listen(cdpProbe)}`;
await new Promise(resolve => cdpProbe.close(resolve));

async function waitFor(check, description, timeout = 45_000) {
  const deadline = Date.now() + timeout;
  while (Date.now() < deadline) {
    const result = await check().catch(() => null);
    if (result) return result;
    await delay(200);
  }
  throw new Error(`Tempo esgotado: ${description}`);
}

async function connect() {
  await waitFor(async () => (await (await fetch(cdp + '/json/list')).json()).some(target => target.type === 'page'), 'WebView2');
  const browser = await chromium.connectOverCDP(cdp);
  const page = browser.contexts().flatMap(context => context.pages())[0];
  assert(page, 'A janela principal deve estar disponível');
  return { browser, page };
}

try {
  tauri(['signer', 'generate', '--ci', '--write-keys', key, '--password', password], process.env, 'ignore');
  const pubkey = (await readFile(key + '.pub', 'utf8')).trim();
  execFileSync(process.execPath, ['tools/build-ui.js'], { stdio: 'inherit' });
  const env = { ...process.env, TAURI_SIGNING_PRIVATE_KEY: key, TAURI_SIGNING_PRIVATE_KEY_PASSWORD: password };
  for (const version of [currentVersion, nextVersion]) {
    const config = join(staging, `tauri-${version}.json`);
    await writeFile(config, JSON.stringify({
      version, productName: product, identifier: 'com.cm0304.updater.test', mainBinaryName: binary,
      bundle: { createUpdaterArtifacts: true },
      plugins: { updater: { pubkey, endpoints: [endpoint + '/latest.json'], dangerousInsecureTransportProtocol: true } },
    }));
    tauri(['build', '--bundles', 'nsis', '--ci', '--config', config, '--', '--locked', '--offline'], env);
  }
  const bundles = resolve('target/release/bundle/nsis');
  execFileSync(join(bundles, `${product}_${currentVersion}_x64-setup.exe`), ['/S', `/D=${installDir}`], { stdio: 'inherit' });
  const browserArgs = `--remote-debugging-port=${new URL(cdp).port}`;
  launch(browserArgs);
  const initial = await connect();
  await initial.page.locator('#editor-version').getByText(`Editor ${currentVersion}`).waitFor();
  await initial.page.locator('#theme-select').selectOption('retro');
  await initial.page.evaluate(path => localStorage.setItem('cm0304.recent-saves.v1', JSON.stringify([{ path, name: 'carreira.cm4' }])), sampleCopy);

  installer = join(bundles, `${product}_${nextVersion}_x64-setup.exe`);
  const signature = (await readFile(installer + '.sig', 'utf8')).trim();
  let respond;
  releaseManifest = new Promise(resolve => { respond = resolve; });
  manifest = JSON.stringify({ version: nextVersion, platforms: { 'windows-x86_64': { signature, url: endpoint + '/setup.exe' } } });
  await initial.page.reload();
  await initial.page.locator('#recent-save-list button').click();
  await initial.page.waitForFunction(() => !document.getElementById('nav-players').disabled);
  respond();
  await waitFor(async () => downloaded >= 1, 'Download com save aberto');
  await delay(700);
  assert.equal(await initial.page.locator('#editor-version').textContent(), ` · Editor ${currentVersion}`);
  console.log('OK instalação adiada com save aberto');

  await initial.page.locator('#window-close').click();
  await waitFor(async () => !(await fetch(cdp + '/json/list', { signal: AbortSignal.timeout(1000) }).then(() => true, () => false)), 'Fechamento do WebView2');
  launch(browserArgs);
  const updated = await waitFor(async () => {
    const connection = await connect();
    if (await connection.page.locator('#editor-version').textContent({ timeout: 3000 }) === ` · Editor ${nextVersion}`) return connection;
    return null;
  }, `Reabertura automática na versão ${nextVersion}`, 60_000);
  assert.equal(await updated.page.locator('#theme-select').inputValue(), 'retro');
  await updated.page.locator('#recent-save-list button').getByText('carreira.cm4').waitFor();
  assert.equal(await hash(sampleCopy), sourceHash);
  assert.equal(await hash(sample), sourceHash);
  await updated.page.locator('#window-close').click();
  await waitFor(async () => !(await fetch(cdp + '/json/list', { signal: AbortSignal.timeout(1000) }).then(() => true, () => false)), 'Fechamento após atualização');
  execFileSync(join(installDir, 'uninstall.exe'), ['/S'], { stdio: 'inherit' });
  console.log(`OK atualização assinada ${currentVersion} → ${nextVersion}, hash dos saves e preferências preservados`);
} finally {
  server.closeAllConnections();
  server.close();
}
