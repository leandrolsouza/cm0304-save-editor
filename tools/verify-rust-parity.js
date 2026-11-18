import { readFile, writeFile } from 'node:fs/promises';
import { execFileSync } from 'node:child_process';
import assert from 'node:assert/strict';
import { loadPlayers } from './game-db-cli.js';
import { readSave } from '../src/save-reader.js';

const results = [];
for (const [label, save] of [['inicial', 'save_inicial.cm4'], ['modificado', 'save_modificado.cm4']]) {
  const path = `game/user data/games/${save}`;
  const output = `reports/rust-players-${label}.json`;
  execFileSync('target/debug/cm0304-cli.exe', ['export', path, output]);
  const rust = JSON.parse(await readFile(output));
  const { players: js, database } = await loadPlayers(path);
  assert.deepStrictEqual(rust, js, `${label}: all player fields, attributes and offsets`);
  const container = JSON.parse(execFileSync('target/debug/cm0304-cli.exe', ['inspect', path], { maxBuffer: 4e6 }));
  const reference = readSave(await readFile(path));
  assert.deepStrictEqual(container.blocks, reference.blocks);
  assert.equal(container.sha256, reference.sha256);
  results.push({ save, sha256: container.sha256, players: rust.length, blocks: container.blocks.length,
    parsedTables: database.sections.length, allPlayerFieldsMatch: true, allBlockFieldsMatch: true });
}
await writeFile('reports/rust-parity.json', JSON.stringify({ results }, null, 2) + '\n');
console.log(JSON.stringify(results, null, 2));
