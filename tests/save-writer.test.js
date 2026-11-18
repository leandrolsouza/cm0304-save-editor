import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { editDirectAttribute, editAttribute } from '../src/save-writer.js';
import { readSave } from '../src/save-reader.js';
import { readGameDatabase, listPlayers } from '../src/game-db-reader.js';

test('Clodoaldo edit changes exactly one byte and leaves the source buffer intact', async () => {
  const input = await readFile(new URL('../game/user%20data/games/save_inicial.cm4', import.meta.url));
  const snapshot = Buffer.from(input);
  const { output, change } = editDirectAttribute(input, 2001642, 'flair', 20);
  assert.equal(change.name, 'Clodoaldo');
  assert.equal(change.club, 'Fortaleza EC');
  assert.equal(change.before, 18);
  assert.equal(change.after, 20);
  assert.equal(input.equals(snapshot), true);
  assert.equal(output.length, input.length);
  const differences = [];
  for (let i = 0; i < input.length; i++) if (input[i] !== output[i]) differences.push(i);
  assert.deepEqual(differences, [change.offset]);
  assert.throws(() => editDirectAttribute(input, 2001642, 'passing', 20), /Unsupported/);
  assert.throws(() => editDirectAttribute(input, 2001642, 'flair', 21), /1 to 20/);
  assert.throws(() => editDirectAttribute(input, 2001642, 'flair', 18), /already/);
  assert.throws(() => editDirectAttribute(input, 999999999, 'flair', 20), /Expected one player/);
});

test('Clodoaldo passing edit round trips to 20 and preserves every other byte', async () => {
  const input = await readFile(new URL('../game/user%20data/games/save_inicial.cm4', import.meta.url));
  const { output, change } = editAttribute(input, 2001642, 'passing', 20);
  assert.equal(change.before, 13);
  assert.equal(change.encoding, 'relative-high');
  const differences = [];
  for (let i = 0; i < input.length; i++) if (input[i] !== output[i]) differences.push(i);
  assert.deepEqual(differences, [change.offset]);
  const block = readSave(output).blocks.find(b => b.name === 'game_db');
  const players = listPlayers(readGameDatabase(output.subarray(block.payloadOffset, block.payloadOffset + block.size)));
  const player = players.find(p => p.uid === 2001642);
  assert.equal(player.attributes.passing, 20);
  assert.equal(player.currentAbility, 142);
  assert.equal(player.rawAttributes.flair, 18);
  assert.equal(input.readInt8(change.offset), change.rawBefore);
  assert.throws(() => editAttribute(input, 2001642, 'goalkeeper', 20), /Unsupported/);
});
