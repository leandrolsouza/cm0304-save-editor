import { mkdir, copyFile } from 'node:fs/promises';
import { execFileSync } from 'node:child_process';
await mkdir('apps/desktop/dist', { recursive: true });
execFileSync(process.execPath, ['node_modules/typescript/bin/tsc', '-p', 'apps/desktop/tsconfig.json'], { stdio: 'inherit' });
for (const file of ['index.html', 'style.css']) await copyFile(`apps/desktop/ui/${file}`, `apps/desktop/dist/${file}`);
