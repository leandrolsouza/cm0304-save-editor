import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { readSave } from '../src/save-reader.js';
import { BinaryReader, readGameDatabase, listPlayers, PLAYING_FIELDS } from '../src/game-db-reader.js';

test('binary reader rejects truncated strings, negative sizes and invalid booleans', () => {
  assert.throws(() => new BinaryReader(Buffer.alloc(4)).skip(-1), /Invalid size/);
  const truncated = Buffer.alloc(6); truncated.writeUInt32LE(3);
  assert.throws(() => new BinaryReader(truncated).string(), /Invalid size/);
  assert.throws(() => new BinaryReader(Buffer.from([2])).bool(), /Invalid boolean/);
  const version = Buffer.from('002e006400610074009100', 'hex');
  assert.throws(() => readGameDatabase(version), /Unsupported game_db version/);
});

test('both real saves resolve every full player and preserve input bytes', async () => {
  for (const name of ['save_inicial.cm4', 'save_modificado.cm4']) {
    const save = await readFile(new URL(`../game/user%20data/games/${name}`, import.meta.url));
    const digest = () => createHash('sha256').update(save).digest('hex');
    const beforeHash = digest();
    const block = readSave(save).blocks.find(b => b.name === 'game_db');
    const payload = save.subarray(block.payloadOffset, block.payloadOffset + block.size);
    const db = readGameDatabase(payload), players = listPlayers(db, block.payloadOffset);
    assert.equal(players.length, 22062);
    assert.equal(new Set(players.map(p => p.uid)).size, players.length);
    assert.equal(db.playingData.length, 22099);
    assert.equal(db.names.firstNames[0].value, 'Ole');
    let end = 37;
    for (const section of db.sections) { assert.equal(section.offset, end); end = section.end; }
    assert.equal(end + db.remainingBytes, payload.length);
    for (const player of players) {
      assert.equal(Object.keys(player.rawAttributes).length, 65);
      assert.ok(player.currentAbility >= 0 && player.currentAbility <= 200);
      assert.equal(save.readInt16LE(player.offsets.currentAbility), player.currentAbility);
      assert.equal(save.readInt16LE(player.offsets.potentialAbility), player.potentialAbility);
      for (const [index, field] of PLAYING_FIELDS.entries()) {
        assert.equal(save.readInt8(player.offsets.attributes + index), player.rawAttributes[field]);
      }
    }
    const zidane = players.find(p => p.uid === 8076);
    assert.equal(zidane.firstName, 'Zinedine');
    assert.equal(zidane.surname, 'Zidane');
    assert.equal(zidane.club, 'Real Madrid C.F.');
    assert.equal(zidane.rawAttributes.flair, 20);
    const henry = players.find(p => p.uid === 9888);
    assert.equal(henry.firstName, 'Thierry');
    assert.equal(henry.surname, 'Henry');
    assert.equal(henry.club, 'Arsenal');
    assert.equal(digest(), beforeHash);
    assert.throws(() => readGameDatabase(payload.subarray(0, end - 1)), /Invalid size/);
  }
});
