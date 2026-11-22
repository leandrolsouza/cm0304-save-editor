import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readdirSync, readFileSync } from 'node:fs';
import { dirname, join, relative } from 'node:path';

const target = 'x86_64-pc-windows-msvc';
const [format = '--inventory', supplements, ...extra] = process.argv.slice(2);
const release = format === '--release-notices';
if (extra.length || !['--inventory', '--notices', '--release-notices'].includes(format)
    || Boolean(supplements) !== release) {
  throw new Error('Uso: node tools/windows-dependency-licenses.js [--inventory | --notices | --release-notices <pasta>]');
}

const supplementalCrates = {
  'alloc-stdlib@0.3.0': 'alloc-stdlib.txt',
  'selectors@0.38.0': 'selectors.txt',
  'webview2-com@0.39.1': 'webview2.txt',
  'webview2-com-macros@0.8.1': 'webview2-macros.txt',
  'webview2-com-sys@0.39.1': 'webview2.txt',
};
const supplementalHashes = {
  'alloc-stdlib.txt': 'c0c56f26d9c051cac4d200c34c84e7ae9aaa853e01a982a1df08b09931e518ae',
  'selectors.txt': 'fab3dd6bdab226f1c08630b1dd917e11fcb4ec5e1e020e2c16f83a0a13863e85',
  'webview2.txt': '0dcf41516e608bbcb6cdc5229feb7b86fe4a643b85e7df251133c93408fdac73',
  'webview2-macros.txt': '0dcf41516e608bbcb6cdc5229feb7b86fe4a643b85e7df251133c93408fdac73',
  'nsis-copying.txt': '388357c1215ff403c5ebde3a5ecd273e68f8b79a579996775245d1ee65442aba',
  'nsis-tauri-utils-mit.txt': '1c1020fa10a6bf318717e82c911bcc54ebdfb9bb280460ae332bcb2f82f57fbe',
  'nsis-tauri-utils-apache.txt': '0d542e0c8804e39aa7f37eb00da5a762149dc682d7829451287e11b938e94594',
};

function supplementalNotice(filename) {
  const bytes = readFileSync(join(supplements, filename));
  const hash = createHash('sha256').update(bytes).digest('hex');
  if (hash !== supplementalHashes[filename]) throw new Error(`SHA-256 divergente: ${filename}`);
  return bytes.toString('utf8').trimEnd();
}
const metadata = JSON.parse(execFileSync('cargo', [
  'metadata', '--offline', '--locked', '--format-version', '1', '--filter-platform', target,
], { encoding: 'utf8', maxBuffer: 16 * 1024 * 1024 }));
const packages = new Map(metadata.packages.map(pkg => [pkg.id, pkg]));
const nodes = new Map(metadata.resolve.nodes.map(node => [node.id, node]));
const desktop = metadata.packages.find(pkg => pkg.name === 'cm0304-desktop' && !pkg.source);
if (!desktop) throw new Error('Pacote cm0304-desktop ausente no workspace');

const seen = new Set();
const pending = [desktop.id];
while (pending.length) {
  const id = pending.pop();
  if (seen.has(id)) continue;
  seen.add(id);
  for (const dep of nodes.get(id).deps) {
    if (dep.dep_kinds.some(kind => kind.kind === null)) pending.push(dep.pkg);
  }
}

const dependencies = [...seen].map(id => packages.get(id)).filter(pkg => pkg.source)
  .sort((a, b) => a.name.localeCompare(b.name, 'en') || a.version.localeCompare(b.version, 'en'))
  .map(pkg => {
    const root = dirname(pkg.manifest_path);
    const notices = readdirSync(root, { withFileTypes: true, recursive: true })
      .filter(entry => entry.isFile() && /^(licen[cs]e|licen[cs]ing|copying|notice|copyright)([._-]|$)/i.test(entry.name))
      .map(entry => relative(root, join(entry.parentPath, entry.name)).replaceAll('\\', '/'));
    if (pkg.license_file) notices.push(pkg.license_file);
    return {
      root,
      name: pkg.name,
      version: pkg.version,
      source: pkg.source,
      repository: pkg.repository ?? null,
      declaredLicense: pkg.license ?? null,
      noticeFiles: [...new Set(notices)].sort(),
    };
  });

if (format === '--inventory') {
  const inventory = dependencies.map(({ root, ...entry }) => entry);
  console.log(JSON.stringify({ target, root: desktop.name, dependencies: inventory }, null, 2));
} else {
  const sections = [
    release ? '# Third-party notices for Windows x64' : '# Rust dependency notices for Windows x64',
    'Runtime dependencies of the desktop application, as locked in Cargo.lock.',
  ];
  for (const dep of dependencies) {
    sections.push(`## ${dep.name} ${dep.version}`,
      `Declared licence — ${dep.declaredLicense ?? 'unavailable'}\nSource — ${dep.repository ?? dep.source}`);
    if (!dep.noticeFiles.length) {
      if (release) {
        const key = `${dep.name}@${dep.version}`;
        const filename = supplementalCrates[key];
        if (!filename) throw new Error(`Arquivo de aviso ausente para ${key}`);
        sections.push(supplementalNotice(filename));
      } else {
        sections.push('Notice file pending review.');
      }
    }
    for (const notice of dep.noticeFiles) {
      const contents = readFileSync(join(dep.root, notice), 'utf8').trimEnd();
      sections.push(`### ${notice}\n\n${contents}`);
    }
  }
  if (release) {
    sections.push('## NSIS 3.11', supplementalNotice('nsis-copying.txt'),
      '## nsis-tauri-utils 0.5.2', supplementalNotice('nsis-tauri-utils-mit.txt'),
      supplementalNotice('nsis-tauri-utils-apache.txt'));
  }
  console.log(sections.join('\n\n'));
}
