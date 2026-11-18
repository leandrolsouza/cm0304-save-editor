import { convertAttributes } from './attribute-conversion.js';
// Layout reference: Michael Nygreen's published CM Scout 3.10 source.
// See research/FORMAT.md for provenance and validation limits.
export class BinaryReader {
  constructor(buffer) { this.buffer = buffer; this.offset = 0; }
  skip(size) {
    if (!Number.isSafeInteger(size) || size < 0 || this.offset + size > this.buffer.length) throw new Error(`Invalid size ${size} at game_db offset ${this.offset}`);
    this.offset += size;
  }
  number(size, signed = false) {
    const p = this.offset; this.skip(size);
    return signed ? this.buffer.readIntLE(p, size) : this.buffer.readUIntLE(p, size);
  }
  u8() { return this.number(1); }
  u16() { return this.number(2); }
  i32() { return this.number(4, true); }
  bool() { const v = this.u8(); if (v > 1) throw new Error(`Invalid boolean at ${this.offset - 1}`); return v === 1; }
  array(width, countSize = 4) { const count = this.number(countSize); this.skip(count * width); }
  string() {
    const length = this.i32();
    if (length < 0 || length > 100000) throw new Error(`Invalid string length at ${this.offset - 4}`);
    const start = this.offset;
    if (length) { this.skip(length * 2); if (this.u16() !== 0) throw new Error(`Missing string terminator at ${this.offset - 2}`); }
    return this.buffer.subarray(start, start + length * 2).toString('utf16le');
  }
  identity() { return { id: this.i32(), uid: this.i32() }; }
  date() { return { day: this.u16(), year: this.u16() }; }
}

function checkType(type, allowed, offset) {
  if (!allowed.includes(type)) throw new Error(`Unsupported type ${type} at game_db offset ${offset}`);
}

export function readGameDatabase(buffer) {
  const r = new BinaryReader(buffer), sections = [];
  if (buffer.subarray(0, 9).toString('hex') !== '002e00640061007400') throw new Error('Invalid game_db signature');
  r.skip(9);
  const version = r.u16();
  if (version !== 147) throw new Error(`Unsupported game_db version ${version}; expected 147 (4.1.5 samples)`);
  r.skip(6); const gameDate = r.date(); r.skip(16);
  const section = (name, fn) => {
    const offset = r.offset, count = r.i32();
    if (count < 0 || count > 1000000) throw new Error(`Invalid ${name} count at ${offset}`);
    const result = [];
    for (let index = 0; index < count; index++) {
      const start = r.offset;
      try { result.push({ index, offset: start, ...fn(r.u8(), start), size: r.offset - start }); }
      catch (error) { throw new Error(`${name}[${index}] at ${start}: ${error.message}`); }
    }
    sections.push({ name, offset, count, end: r.offset });
    return result;
  };
  const fixed = (name, width) => {
    const offset = r.offset, count = r.i32();
    if (count < 0 || count > 1000000) throw new Error(`Invalid ${name} count`);
    r.skip(count * width); sections.push({ name, offset, count, end: r.offset });
  };
  const teamContainer = () => {
    const containerType = r.u8(); checkType(containerType, [0, 1], r.offset - 1);
    r.skip(2); const name = r.string(), shortName = r.string();
    r.array(6, 1); r.array(4, 1); r.array(8, 1);
    return { containerType, name, shortName, ...r.identity() };
  };
  fixed('awards', 45);
  section('cities', () => { r.skip(18); r.string(); r.skip(11); });
  const clubs = section('clubs', type => {
    checkType(type, [0, 12], r.offset - 1);
    if (type === 12) r.skip(9);
    const nationIndex = r.i32(); r.skip(1); r.string(); r.skip(1); r.string();
    r.skip(23); r.array(4, 1); r.array(4, 1);
    const finance = r.u8(); checkType(finance, [0, 1, 2], r.offset - 1); r.skip(finance === 0 ? 4 : finance === 1 ? 438 : 0);
    if (r.i32() !== 0) r.skip(13);
    r.array(4, 2);
    const transfers = r.u16();
    for (let i = 0; i < transfers; i++) {
      const kind = r.u8(), sizes = { 0: 5, 1: 1, 2: 5, 4: 16, 5: 10, 6: 15, 7: 12, 8: 1 };
      if (kind === 3) { if (r.bool()) r.array(4, 1); r.skip(5); }
      else if (sizes[kind] !== undefined) r.skip(sizes[kind]);
      else throw new Error(`Unknown transfer info ${kind}`);
    }
    r.array(6, 1); r.skip(12); r.array(4, 2); r.array(5, 1); r.skip(5);
    return { nationIndex, ...teamContainer() };
  });
  section('competitions', () => { r.skip(11); const nameType = r.u8(); r.skip(1); if (nameType === 4) { r.string(); r.string(); } r.skip(29); });
  fixed('continents', 17);
  section('injuries', () => { r.string(); r.skip(2); r.string(); r.skip(17); });
  section('media', () => { r.string(); r.skip(1); r.string(); r.skip(2); r.array(4, 1); r.skip(14); });
  section('languages', () => { r.string(); r.skip(21); });
  const nations = section('nations', type => {
    checkType(type, [0], r.offset - 1);
    r.skip(51); r.array(2, 1); r.array(4, 1); r.array(4, 1); r.array(4, 2);
    for (let i = 0; i < 8; i++) { r.skip(2); r.array(4, 1); }
    const important = r.u16();
    if (important === 1) { r.skip(13); for (let i = 0; i < 4; i++) r.string(); }
    else if (important !== 0) throw new Error(`Unknown important-player flag ${important}`);
    r.skip(21); r.array(4, 1); r.skip(2);
    return teamContainer();
  });
  const names = {};
  for (const name of ['firstNames', 'surnames', 'commonNames']) {
    names[name] = section(name, type => {
      checkType(type, [0], r.offset - 1);
      const value = r.string(); r.skip(6); return { value, ...r.identity() };
    });
  }
  fixed('regions', 9);
  section('sponsors', () => { r.string(); r.skip(1); r.string(); r.skip(10); });
  fixed('stadiums', 48); fixed('stadiumChanges', 38);
  section('stages', () => { r.string(); r.skip(1); r.string(); r.skip(1); r.string(); r.skip(9); });
  const teams = section('teams', type => {
    checkType(type, [10, 11], r.offset - 1);
    if (type === 11) r.skip(14);
    r.skip(42); r.array(4, 1); r.array(4, 2); r.array(4, 1); r.array(4, 1); r.skip(1);
    r.array(12, 2); r.skip(12);
    const squadType = r.u8(), containerType = r.u8(); r.skip(1); const containerIndex = r.i32();
    return { squadType, containerType, containerIndex, ...r.identity() };
  });
  section('weather', () => { r.string(); r.skip(87); });
  fixed('playerTypes', 10);
  const peopleOffset = r.offset;
  const people = section('people', type => {
    checkType(type, [1, 2, 3, 4, 5, 6, 7, 8, 9], r.offset - 1);
    if (type === 9) skipHuman(r);
    if (type === 6 || type === 7) { r.skip(type === 6 ? 1 : 6); return { type, ...r.identity() }; }
    if (type === 5) { const firstName = r.string(), surname = r.string(); r.skip(2); return { type, firstName, surname, ...r.identity() }; }
    let playerData;
    if (type === 2 || type === 3) playerData = readPlayerData(r);
    if ([1, 3, 6, 9].includes(type)) r.skip(15);
    if (type === 4) r.skip(24);
    const firstNameIndex = r.i32(), surnameIndex = r.i32(), commonNameIndex = r.i32();
    const dateOfBirth = r.date(), nationIndex = r.i32();
    r.skip(13); const teamIndex = r.i32(); r.skip(6);
    const personDataIndex = r.i32(); r.skip(4); r.array(4, 1); r.skip(8);
    const contracts = r.u8(); for (let i = 0; i < contracts; i++) skipContract(r, r.u8());
    const offers = r.u8();
    for (let i = 0; i < offers; i++) { const ct = r.u8(); r.skip(6); if (ct !== 255) skipContract(r, ct); }
    if (r.bool()) r.array(8, 1);
    r.array(4, 2);
    if (r.bool()) { r.string(); r.skip(4); }
    r.skip(1); const identity = r.identity();
    if (type === 3) r.skip(2);
    return { type, firstNameIndex, surnameIndex, commonNameIndex, dateOfBirth, nationIndex, teamIndex, personDataIndex, playerData, ...identity };
  });
  const personalityFields = ['adaptability', 'ambition', 'loyalty', 'pressure', 'professionalism', 'sportsmanship', 'temperament', 'controversy'];
  const personalities = section('personData', type => {
    checkType(type, [0], r.offset - 1);
    const attributes = Object.fromEntries(personalityFields.map(name => [name, r.u8()]));
    return { attributes, ...r.identity() };
  });
  const playingData = section('playingData', type => {
    checkType(type, [0], r.offset - 1);
    const attributesOffset = r.offset;
    const rawAttributes = Object.fromEntries(PLAYING_FIELDS.map(name => [name, r.number(1, true)]));
    r.skip(1); return { attributesOffset, rawAttributes, ...r.identity() };
  });
  return { version, gameDate, sections, clubs, nations, teams, names, peopleOffset, people, personalities, playingData,
    parsedUntil: r.offset, remainingBytes: buffer.length - r.offset };
}

function skipContract(r, type) {
  if (![0, 1, 2, 5].includes(type)) throw new Error(`Unsupported contract type ${type} at ${r.offset - 1}`);
  if (type === 0) { r.skip(20); const count = r.u8(); r.skip(6 + count * 5); }
  if (type === 1) r.skip(6);
  if (type === 5) r.skip(7);
  if (type !== 2) r.skip(1);
  const container = r.u8(); checkType(container, [0, 1], r.offset - 1);
  r.skip(33);
}

function readPlayerData(r) {
  const offset = r.offset;
  const homeReputation = r.u16(), currentReputation = r.u16(), worldReputation = r.u16();
  const currentAbility = r.number(2, true), potentialAbility = r.number(2, true), playingDataIndex = r.i32();
  r.skip(4); const displayValue = r.i32(); r.skip(20); const saleValue = r.i32();
  if (r.bool()) r.skip(16);
  r.skip(4);
  if (r.bool()) {
    const injuries = r.u8();
    for (let i = 0; i < injuries; i++) {
      r.skip(10); const type = r.u8(), sizes = { 0: 1, 1: 3, 3: 5, 5: 5, 7: 7 };
      if (sizes[type] === undefined) throw new Error(`Unknown injury fixture type ${type}`);
      r.skip(sizes[type] + 26);
    }
    const bans = r.u8(); for (let i = 0; i < bans; i++) { r.skip(18); if (r.bool()) r.skip(39); }
    if (r.bool()) r.skip(26);
  }
  const bookings = r.u8();
  for (let i = 0; i < bookings; i++) {
    const kind = r.u8(), size = [3, 5, 5, 8, 4][kind];
    if (size === undefined) throw new Error(`Unknown booking count ${kind}`);
    r.skip(size);
  }
  r.skip(38);
  if (r.bool()) {
    const count = r.u8();
    for (let i = 0; i < count; i++) {
      r.skip(5); const subcount = r.u8();
      for (let j = 0; j < subcount; j++) { r.skip(10); if (r.bool()) r.skip(29); }
    }
  }
  return { offset, homeReputation, currentReputation, worldReputation, currentAbility, potentialAbility, playingDataIndex, displayValue, saleValue };
}

function skipHuman(r) {
  r.skip(16); r.string(); r.skip(68); r.array(8, 1); r.skip(96);
  const lists = r.i32();
  if (lists < 0 || lists > 10000) throw new Error('Invalid human filter count');
  const ascii = n => { const start = r.offset; r.skip(n); return r.buffer.subarray(start, start + n).toString('ascii'); };
  for (let i = 0; i < lists; i++) {
    const kind = ascii(8), count = r.i32();
    const expected = { slfttslf: 'daqstlif', tslttslf: 'daqstlif', tslntslf: 'swentlif', ltontslf: 'etontlif', tslhtslf: 'ntantlif' }[kind];
    if (!expected || count < 0 || count > 10000) throw new Error(`Unsupported human filter ${kind}`);
    for (let j = 0; j < count; j++) {
      const filter = ascii(8); if (filter !== expected) throw new Error(`Unsupported filter ${filter}`);
      r.string(); r.skip(5); const infos = r.i32();
      if (infos < 0 || infos > 10000) throw new Error('Invalid filter info count');
      for (let k = 0; k < infos; k++) {
        const info = ascii(4);
        if (kind === 'tslhtslf' && info === 'rftn') { r.skip(2); r.array(8); r.skip(8); }
        else {
          const size = { rfqs: 10, rfsp: 2, rfpv: 3, rfsn: 1, rfcn: 2, rftn: 9 }[info];
          if (size === undefined) throw new Error(`Unsupported filter info ${info}`);
          r.skip(size);
        }
      }
    }
    if (kind === 'slfttslf') r.skip(10);
    if (kind === 'tslntslf') r.skip(4);
  }
  r.skip(16); r.array(32, 1); r.skip(1);
}

export const PLAYING_FIELDS = [
  'goalkeeper', 'sweeper', 'defender', 'defensiveMidfielder', 'midfielder', 'attackingMidfielder', 'attacker', 'wingBack', 'freeRole', 'rightSide', 'leftSide', 'central',
  'crossing', 'dribbling', 'finishing', 'heading', 'longShots', 'marking', 'offTheBall', 'passing', 'penalties', 'tackling', 'creativity',
  'handling', 'aerialAbility', 'commandOfArea', 'communication', 'kicking', 'throwing', 'anticipation', 'decisions', 'oneOnOnes', 'positioning', 'reflexes',
  'throwIns', 'firstTouch', 'technique', 'leftFoot', 'rightFoot', 'flair', 'corners', 'teamwork', 'workRate', 'longThrows', 'eccentricity', 'rushingOut', 'tendencyToPunch',
  'acceleration', 'setPieces', 'strength', 'stamina', 'pace', 'jumping', 'influence', 'dirtiness', 'balance', 'bravery', 'consistency', 'aggression', 'agility',
  'importantMatches', 'injuryProneness', 'versatility', 'naturalFitness', 'determination',
];

export function listPlayers(database, payloadOffset = 0) {
  const reference = (table, index, label, optional = false) => {
    if (optional && index === -1) return null;
    if (!Number.isInteger(index) || index < 0 || index >= table.length) throw new Error(`Invalid ${label} reference ${index}`);
    return table[index];
  };
  return database.people.filter(p => p.playerData).map(person => {
    const data = person.playerData;
    const firstName = reference(database.names.firstNames, person.firstNameIndex, 'first name', true)?.value ?? '';
    const surname = reference(database.names.surnames, person.surnameIndex, 'surname', true)?.value ?? '';
    const commonName = reference(database.names.commonNames, person.commonNameIndex, 'common name', true)?.value ?? '';
    const nation = reference(database.nations, person.nationIndex, 'nation', true);
    const team = reference(database.teams, person.teamIndex, 'team', true);
    const container = team ? reference(team.containerType === 0 ? database.clubs : database.nations, team.containerIndex, 'team container') : null;
    const attributes = reference(database.playingData, data.playingDataIndex, 'playing data');
    const personality = reference(database.personalities, person.personDataIndex, 'personality', true);
    return { uid: person.uid, id: person.id, type: person.type, firstName, surname, commonName,
      name: commonName || `${firstName} ${surname}`.trim(), club: team?.containerType === 0 ? container.name : null,
      nation: nation?.name ?? null, dateOfBirth: person.dateOfBirth,
      currentAbility: data.currentAbility, potentialAbility: data.potentialAbility,
      rawAttributes: attributes.rawAttributes, attributes: convertAttributes(attributes.rawAttributes, data.currentAbility), personality: personality?.attributes ?? null,
      offsets: { person: payloadOffset + person.offset, currentAbility: payloadOffset + data.offset + 6,
        potentialAbility: payloadOffset + data.offset + 8, attributes: payloadOffset + attributes.attributesOffset },
    };
  });
}
