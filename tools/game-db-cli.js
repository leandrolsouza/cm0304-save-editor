import { readFile, writeFile, mkdir } from 'node:fs/promises';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { readSave } from '../src/save-reader.js';
import { readGameDatabase, listPlayers, PLAYING_FIELDS } from '../src/game-db-reader.js';

const defaults = ['game/user data/games/save_inicial.cm4', 'game/user data/games/save_modificado.cm4'];
const normalize = value => value.normalize('NFD').replace(/\p{M}/gu, '').toLowerCase();

export async function loadPlayers(path) {
  const save = await readFile(path);
  const blocks = readSave(save).blocks.filter(b => b.name === 'game_db' && b.extension === '.dat');
  if (blocks.length !== 1) throw new Error(`Expected one game_db.dat, found ${blocks.length}`);
  const block = blocks[0], database = readGameDatabase(save.subarray(block.payloadOffset, block.payloadOffset + block.size));
  return { database, block, players: listPlayers(database, block.payloadOffset) };
}

function comparePlayers(before, after) {
  const index = list => {
    const map = new Map();
    for (const p of list) { if (map.has(p.uid)) throw new Error(`Duplicate player UID ${p.uid}`); map.set(p.uid, p); }
    return map;
  };
  const left = index(before), right = index(after), changes = [];
  let added = 0, removed = 0, unchanged = 0;
  for (const p of before) {
    const q = right.get(p.uid);
    if (!q) { removed++; continue; }
    const fields = {};
    for (const key of ['currentAbility', 'potentialAbility', 'club']) if (p[key] !== q[key]) fields[key] = [p[key], q[key]];
    for (const group of ['rawAttributes', 'personality']) {
      for (const key of new Set([...Object.keys(p[group] ?? {}), ...Object.keys(q[group] ?? {})])) {
        if (p[group]?.[key] !== q[group]?.[key]) fields[`${group}.${key}`] = [p[group]?.[key] ?? null, q[group]?.[key] ?? null];
      }
    }
    if (Object.keys(fields).length) changes.push({ uid: p.uid, name: p.name, club: q.club, fields });
    else unchanged++;
  }
  for (const p of after) if (!left.has(p.uid)) added++;
  return { added, removed, changed: changes.length, unchanged, changes };
}

function csv(players) {
  const columns = ['uid', 'name', 'firstName', 'surname', 'club', 'nation', 'currentAbility', 'potentialAbility', ...PLAYING_FIELDS, ...PLAYING_FIELDS.map(f => `${f}_raw`)];
  const escape = v => `"${String(v ?? '').replaceAll('"', '""')}"`;
  return '\ufeff' + [columns, ...players.map(p => columns.map(c => c.endsWith('_raw') ? p.rawAttributes[c.slice(0, -4)] : p.attributes[c] ?? p[c]))]
    .map(row => row.map(escape).join(',')).join('\r\n') + '\r\n';
}

async function main() {
  const [command = 'analyze', ...args] = process.argv.slice(2);
  if (command === 'players' && args.length >= 1 && args.length <= 2) {
    const { players } = await loadPlayers(args[0]);
    const query = normalize(args[1] ?? '');
    const matches = players.filter(p => normalize(`${p.uid} ${p.name} ${p.firstName} ${p.surname} ${p.club ?? ''}`).includes(query));
    console.log(JSON.stringify({ count: matches.length, attributeEncoding: 'attributes: CM Scout 3.10 conversion; rawAttributes: original signed bytes. Absent positions/direct zero values are preserved.', players: matches }, null, 2));
    return;
  }
  if (command !== 'analyze' || ![0, 2, 3].includes(args.length)) throw new Error('Usage: node tools/game-db-cli.js analyze [<before> <after> [output-directory]] | players <save> [name, club or UID]');
  const output = resolve(args[2] ?? 'reports');
  const before = await loadPlayers(args[0] ?? defaults[0]);
  const after = await loadPlayers(args[1] ?? defaults[1]);
  const comparison = comparePlayers(before.players, after.players);
  const summary = (data) => ({ gameDate: data.database.gameDate, version: data.database.version,
    people: data.database.people.length, players: data.players.length, sections: data.database.sections,
    parsedUntil: data.database.parsedUntil, remainingBytes: data.database.remainingBytes,
    personTypes: Object.fromEntries([...new Set(data.database.people.map(p => p.type))].map(t => [t, data.database.people.filter(p => p.type === t).length])) });
  const examples = before.players.filter(p => [8076, 9888].includes(p.uid) || p.club?.includes('Fortaleza'));
  const report = { before: summary(before), after: summary(after), comparison, examples };
  await mkdir(output, { recursive: true });
  await writeFile(resolve(output, 'game-db-analysis.json'), JSON.stringify(report, null, 2) + '\n');
  await writeFile(resolve(output, 'players-inicial.csv'), csv(before.players));
  await writeFile(resolve(output, 'players-modificado.csv'), csv(after.players));
  const lines = ['# Investigação de game_db.dat', '',
    `Versão interna: ${before.database.version}. Jogadores com dados completos: ${before.players.length} → ${after.players.length}.`,
    `Pessoas/entidades: ${before.database.people.length} → ${after.database.people.length}. Inclui registros virtuais, aposentados, árbitros e comissão técnica.`, '',
    `Entre os jogadores com dados completos: ${comparison.changed} mudaram nos campos comparados; ${comparison.unchanged} permaneceram iguais; ${comparison.added} entraram; ${comparison.removed} saíram.`,
    'Campos comparados: CA, PA, clube, 65 bytes de atributos e oito atributos de personalidade. Igualdade aqui não cobre contratos, lesões ou todos os dados da pessoa.', '',
    'Os CSVs exportam atributos convertidos pelo modelo CM Scout 3.10 e valores brutos com sufixo `_raw`. Posições ausentes preservam zero. A conferência visual está em attribute-validation.md.', '',
    '## Exemplos extraídos do save inicial', '', '| UID | Nome | Clube | CA | PA | Aceleração (raw) | Velocidade (raw) |', '|---:|---|---|---:|---:|---:|---:|',
    ...examples.map(p => `| ${p.uid} | ${p.name} | ${p.club ?? '—'} | ${p.currentAbility} | ${p.potentialAbility} | ${p.rawAttributes.acceleration} | ${p.rawAttributes.pace} |`), '',
    '## Tabelas', '', '| Tabela | Contagem inicial | Contagem final | Offset inicial no bloco |', '|---|---:|---:|---:|',
    ...before.database.sections.map(s => `| ${s.name} | ${s.count} | ${after.database.sections.find(t => t.name === s.name)?.count ?? '—'} | ${s.offset} |`), '',
    `A leitura termina após playingData. Restam ${before.database.remainingBytes} e ${after.database.remainingBytes} bytes, respectivamente, ainda não interpretados.`, '',
    'Referência de estrutura: [código-fonte público do CM Scout 3.10, Michael Nygreen](https://sourceforge.net/projects/cmeditors/files/CM%20Scout/CM%20Scout%203.10/). Identidades e atributos ainda precisam de conferência visual no jogo antes de implementar escrita.', ''];
  await writeFile(resolve(output, 'game-db-analysis.md'), lines.join('\n'));
  console.log(JSON.stringify({ before: { players: before.players.length, people: before.database.people.length },
    after: { players: after.players.length, people: after.database.people.length }, comparison: { ...comparison, changes: undefined }, output }, null, 2));
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  main().catch(error => { console.error(error.message); process.exitCode = 1; });
}
