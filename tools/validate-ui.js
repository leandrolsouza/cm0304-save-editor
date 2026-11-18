import { readFile, writeFile } from 'node:fs/promises';
import { loadPlayers } from './game-db-cli.js';

const fixture = JSON.parse(await readFile(new URL('../tests/fixtures/ui-attributes.json', import.meta.url)));
const { players } = await loadPlayers(fixture.save);
const rows = fixture.observations.flatMap(observation => {
  const player = players.find(p => p.uid === observation.uid);
  if (!player) throw new Error(`Missing observed player ${observation.uid}`);
  return Object.entries(observation.attributes).map(([field, observed]) => ({ uid: player.uid, name: player.name,
    visibility: observation.visibility, field, raw: player.rawAttributes[field], converted: player.attributes[field], observed,
    matches: player.attributes[field] === observed }));
});
const controls = rows.filter(r => r.visibility === 'own-team');
const text = ['# Conferência da conversão no CM0304 4.1.5', '',
  `Save inicial, data no jogo ${fixture.gameDate}. ${fixture.method}`, '',
  `Controle com o clube do treinador (Braga): ${controls.filter(r => r.matches).length}/${controls.length} atributos coincidiram, incluindo um jogador de linha (Wender) e um goleiro (Quim).`, '',
  'No Fortaleza, Clodoaldo e Erandir exibem atributos parcialmente ocultos; Richarlyson exibe todos os atributos do perfil como "-". Há coincidências e divergências entre os valores visíveis e os convertidos. A comparação do Fortaleza é inconclusiva para validar igualdade com os atributos reais. O conhecimento limitado do treinador é uma explicação provável, não foi isolado experimentalmente.', '',
  'Não foram alterados atributos, opções de mascaramento ou saves. A conversão permanece identificada como modelo CM Scout 3.10; a conferência cobre apenas os campos transcritos abaixo. Nenhum valor oculto foi tratado como zero ou como um acerto.', '',
  '| Jogador | Campo | Byte bruto | Convertido | Tela do jogo | Coincide |', '|---|---|---:|---:|---:|---|',
  ...rows.map(r => `| ${r.name} | ${r.field} | ${r.raw} | ${r.converted} | ${r.observed} | ${r.matches ? 'sim' : 'não'} |`), '',
  '## Conversão', '',
  'Para os atributos relativos: t = (2 × raw + CA × fator) × 0,05 + 10; resultado = trunc((0,1 × t² + t) / 3 + 0,5), limitado a 1–20. O fator é 1 no grupo principal e 0,1 no secundário. Goleiro >= 15 troca os grupos técnicos de linha e de goleiro. Oito atributos relativos comuns usam sempre o fator 1. Os demais campos são mantidos diretamente, inclusive zeros de posições ausentes.', '',
  'Os CSVs completos têm colunas convertidas e colunas `_raw`. A tabela `attributes` do JSON contém a conversão; `rawAttributes` preserva os bytes originais.', ''].join('\n');
await writeFile('reports/attribute-validation.md', text);
await writeFile('reports/attribute-validation.json', JSON.stringify({ ...fixture, rows }, null, 2) + '\n');
const fortaleza = players.filter(p => p.club?.includes('Fortaleza'));
await writeFile('reports/fortaleza-attributes.json', JSON.stringify({ save: fixture.save, conversion: 'CM Scout 3.10', players: fortaleza }, null, 2) + '\n');
console.log(`Own-team UI checks: ${controls.filter(r => r.matches).length}/${controls.length}; Fortaleza players exported: ${fortaleza.length}`);
if (controls.some(r => !r.matches)) process.exitCode = 1;
