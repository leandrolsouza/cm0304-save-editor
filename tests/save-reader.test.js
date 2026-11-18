import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { readSave, compareSaves } from '../src/save-reader.js';

function fixture(payload = Buffer.from('15cd5b0701', 'hex')) {
  const header = Buffer.alloc(73);
  header.writeUInt32LE(123456789);
  header[4] = 2;
  header.write('example', 5, 'utf16le');
  header.write('.dat', 59, 'utf16le');
  header.writeUInt32LE(payload.length, 69);
  return Buffer.concat([Buffer.from('002e0063006d003400020000', 'hex'), header, payload]);
}

test('embedded markers in payload do not create false records', () => {
  const parsed = readSave(fixture());
  assert.equal(parsed.blocks.length, 1);
  assert.equal(parsed.blocks[0].name, 'example');
  assert.equal(parsed.blocks[0].extension, '.dat');
  assert.equal(parsed.blocks[0].size, 5);
});

test('rejects bad signature, truncated payload and unknown record types', () => {
  assert.throws(() => readSave(Buffer.alloc(12)), /signature/);
  assert.throws(() => readSave(fixture().subarray(0, -1)), /Truncated/);
  const bad = fixture(); bad[16] = 3;
  assert.throws(() => readSave(bad), /Unknown record type/);
});

test('detects equal-size payload edits and missing blocks', () => {
  const before = readSave(fixture(Buffer.from('abc')));
  const after = readSave(fixture(Buffer.from('abd')));
  assert.equal(compareSaves(before, after).counts.changed, 1);
  assert.equal(compareSaves(before, before).counts.unchanged, 1);
  assert.equal(compareSaves({ blocks: [], size: 12 }, after).counts.added, 1);
  assert.equal(compareSaves(before, { blocks: [], size: 12 }).counts.removed, 1);
});

test('both supplied saves have complete contiguous record coverage', async () => {
  for (const [name, expected] of [['save_inicial.cm4', 87], ['save_modificado.cm4', 267]]) {
    const buffer = await readFile(new URL(`../game/user%20data/games/${name}`, import.meta.url));
    const parsed = readSave(buffer);
    assert.equal(parsed.blocks.length, expected);
    let offset = 12;
    for (const record of parsed.records) {
      assert.equal(record.offset, offset);
      offset += record.size;
    }
    assert.equal(offset, buffer.length);
    assert.equal(compareSaves(parsed, parsed).counts.unchanged, expected);
  }
});
