import { readFile, mkdir, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { readSave, compareSaves } from '../src/save-reader.js';

async function main() {
  const [command, ...args] = process.argv.slice(2);
  if (command === 'inspect' && args.length === 1) {
    console.log(JSON.stringify(readSave(await readFile(args[0])), null, 2));
    return;
  }
  if (command !== 'compare' || (args.length !== 0 && args.length !== 2 && args.length !== 3)) {
    throw new Error('Usage: node tools/save-cli.js inspect <save> | compare [<before> <after> [output-directory]]');
  }
  const beforePath = args[0] ?? 'game/user data/games/save_inicial.cm4';
  const afterPath = args[1] ?? 'game/user data/games/save_modificado.cm4';
  const output = resolve(args[2] ?? 'reports');
  const [beforeBuffer, afterBuffer] = await Promise.all([readFile(beforePath), readFile(afterPath)]);
  const before = readSave(beforeBuffer), after = readSave(afterBuffer);
  const comparison = compareSaves(before, after);
  const report = { beforePath: resolve(beforePath), afterPath: resolve(afterPath), before, after, comparison };
  const rows = [...comparison.blocks].sort((a, b) => Math.abs(b.delta) - Math.abs(a.delta));
  const markdown = ['# Comparação dos saves CM0304', '',
    `Inicial: ${before.size} bytes, ${before.blocks.length} blocos.`,
    `Após um mês: ${after.size} bytes, ${after.blocks.length} blocos.`,
    `Variação: ${comparison.delta} bytes.`, '',
    `Adicionados: ${comparison.counts.added}; removidos: ${comparison.counts.removed}; alterados: ${comparison.counts.changed}; iguais: ${comparison.counts.unchanged}.`, '',
    'Comparação por nome + extensão e SHA-256 do conteúdo completo de cada bloco. Offsets não são usados como identidade. Os conteúdos ainda não são interpretados.', '',
    '| Bloco | Estado | Bytes antes | Bytes depois | Variação |',
    '|---|---|---:|---:|---:|',
    ...rows.map(b => `| ${b.name} | ${b.status} | ${b.beforeSize ?? '—'} | ${b.afterSize ?? '—'} | ${b.delta} |`), ''].join('\n');
  await mkdir(output, { recursive: true });
  await writeFile(resolve(output, 'comparison.json'), JSON.stringify(report, null, 2) + '\n');
  await writeFile(resolve(output, 'comparison.md'), markdown);
  console.log(JSON.stringify({ ...comparison, blocks: undefined, output }, null, 2));
}
main().catch(error => { console.error(error.message); process.exitCode = 1; });
