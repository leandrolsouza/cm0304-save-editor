import test from 'node:test';
import assert from 'node:assert/strict';
import { convertAttributes, convertRelativeAttribute, encodeRelativeAttribute } from '../src/attribute-conversion.js';
import { PLAYING_FIELDS } from '../src/game-db-reader.js';
import { readFile } from 'node:fs/promises';
import { loadPlayers } from '../tools/game-db-cli.js';

test('relative values use CA, rounding and limits', () => {
  assert.equal(convertRelativeAttribute(-4, 191), 19);
  assert.equal(convertRelativeAttribute(-120, 191, true), 1);
  assert.equal(convertRelativeAttribute(127, 200), 20);
  assert.equal(convertRelativeAttribute(-128, 0), 1);
  assert.throws(() => convertRelativeAttribute(128, 100), /signed/);
  assert.throws(() => convertRelativeAttribute(0, 201), /CA/);
});

test('inverse round trips attainable targets and rejects impossible targets without changing CA', () => {
  for (let ca = 0; ca <= 200; ca++) for (const low of [false, true]) {
    const attainable = new Set(Array.from({ length: 256 }, (_, i) => convertRelativeAttribute(i - 128, ca, low)));
    for (let value = 1; value <= 20; value++) {
      if (!attainable.has(value)) {
        assert.throws(() => encodeRelativeAttribute(value, ca, low, -30), /not representable/);
        continue;
      }
      const encoded = encodeRelativeAttribute(value, ca, low, -30);
      assert.equal(convertRelativeAttribute(encoded, ca, low), value);
      if (convertRelativeAttribute(-30, ca, low) === value) assert.equal(encoded, -30);
    }
  }
});

test('converted values match independently transcribed own-team profiles in the game', async () => {
  const fixture = JSON.parse(await readFile(new URL('./fixtures/ui-attributes.json', import.meta.url)));
  const { players } = await loadPlayers(fixture.save);
  let checked = 0;
  for (const observation of fixture.observations.filter(o => o.visibility === 'own-team')) {
    const player = players.find(p => p.uid === observation.uid);
    for (const [field, expected] of Object.entries(observation.attributes)) {
      assert.equal(player.attributes[field], expected, `${observation.name}: ${field}`); checked++;
    }
  }
  assert.equal(checked, 61);
});

test('keeper threshold switches technical groups, leaves direct fields and input intact', () => {
  const raw = Object.fromEntries(PLAYING_FIELDS.map(f => [f, 0]));
  raw.goalkeeper = 14; raw.pace = 17;
  const before = structuredClone(raw), field = convertAttributes(raw, 100);
  assert.deepEqual(raw, before);
  assert.equal(field.pace, 17);
  assert.equal(field.defender, 0);
  raw.goalkeeper = 15;
  const keeper = convertAttributes(raw, 100);
  assert.ok(field.crossing > keeper.crossing);
  assert.equal(field.crossing, keeper.handling);
  assert.equal(field.handling, keeper.crossing);
  assert.equal(field.anticipation, keeper.anticipation);
});
