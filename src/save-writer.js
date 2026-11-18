import { readSave } from './save-reader.js';
import { readGameDatabase, listPlayers, PLAYING_FIELDS } from './game-db-reader.js';
import { relativeMode, encodeRelativeAttribute, RELATIVE_ATTRIBUTES } from './attribute-conversion.js';

// Direct fields; relative technical fields are encoded separately without resizing.
export const EDITABLE_ATTRIBUTES = ['flair', 'corners', 'teamwork', 'workRate', 'longThrows',
  'acceleration', 'setPieces', 'strength', 'stamina', 'pace', 'jumping', 'influence',
  'balance', 'bravery', 'aggression', 'agility', 'naturalFitness', 'determination'];

export function editDirectAttribute(input, uid, attribute, value) {
  if (!EDITABLE_ATTRIBUTES.includes(attribute)) throw new Error('Unsupported direct attribute');
  return editAttribute(input, uid, attribute, value);
}

export function editAttribute(input, uid, attribute, value) {
  if (!Number.isInteger(uid) || uid < 0) throw new Error('Invalid player UID');
  if (!EDITABLE_ATTRIBUTES.includes(attribute) && !RELATIVE_ATTRIBUTES.includes(attribute)) throw new Error('Unsupported attribute');
  if (!Number.isInteger(value) || value < 1 || value > 20) throw new Error('Attribute value must be an integer from 1 to 20');
  const blocks = readSave(input).blocks.filter(b => b.name === 'game_db' && b.extension === '.dat');
  if (blocks.length !== 1) throw new Error('Expected exactly one game_db.dat');
  const block = blocks[0];
  const parse = buffer => listPlayers(readGameDatabase(buffer.subarray(block.payloadOffset, block.payloadOffset + block.size)), block.payloadOffset);
  const players = parse(input), matches = players.filter(p => p.uid === uid);
  if (matches.length !== 1) throw new Error(`Expected one player with UID ${uid}`);
  const player = matches[0], offset = player.offsets.attributes + PLAYING_FIELDS.indexOf(attribute);
  // Shared playingData records would also change another player: reject them.
  if (players.filter(p => p.offsets.attributes === player.offsets.attributes).length !== 1) throw new Error('Playing data is shared by multiple players');
  const before = player.attributes[attribute];
  if (before === value) throw new Error('Requested attribute already has this value');
  const rawBefore = input.readInt8(offset);
  const mode = relativeMode(attribute, player.rawAttributes.goalkeeper);
  const rawAfter = mode === null ? value : encodeRelativeAttribute(value, player.currentAbility, mode, rawBefore);
  const output = Buffer.from(input);
  output.writeInt8(rawAfter, offset);
  const verified = parse(output).find(p => p.uid === uid);
  if (verified.attributes[attribute] !== value) throw new Error('Post-write verification failed');
  return { output, change: { uid, name: player.name, club: player.club, attribute, before, after: value,
    rawBefore, rawAfter, encoding: mode === null ? 'direct' : mode ? 'relative-low' : 'relative-high', currentAbility: player.currentAbility, offset } };
}
