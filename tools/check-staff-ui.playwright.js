// Run using playwright-cli run-code --filename=tools/check-staff-ui.playwright.js.
// This tests the frontend contract with controlled IPC responses, not native Tauri IPC.
async page => {
  const errors = [];
  page.on('pageerror', error => errors.push(error.message));
  await page.addInitScript(() => {
    const profiles = Array.from({ length: 51 }, (_, i) => ({
      uid: 100000 + i, name: 'Staff de teste ' + i, isPlayer: false, isHuman: false,
      team: 'Clube de teste', clubJob: 'Preparador', nationalTeam: null, nationalJob: 'Não informado',
      nation: 'Portugal', dateOfBirth: { year: 1960, day: 118 }, currentAbility: 70,
      potentialAbility: 140, reputation: [2500, 2500, 1000], rawAttributes: { physiotherapy: -32 },
      attributes: { coachingGoalkeepers:6, coachingOutfieldPlayers:12, physiotherapy:7, tacticalKnowledge:8, workingWithYoungsters:17, manManagement:17, judgingPlayerAbility:12, judgingPlayerPotential:12, motivating:14, discipline:7, determination:12 }, personality: { adaptability:11 },
    }));
    Object.assign(profiles[0], { uid: 4950, name: 'Rui Águas', team: 'Sporting Clube de Braga', clubJob: 'Treinador adjunto' });
    Object.assign(profiles[1], { uid: 600, name: 'Pär Zetterberg', team: 'RSC Anderlecht', isPlayer: true, nationalTeam: 'Suécia', nationalJob: 'Jogador', nation: 'Suécia', dateOfBirth: { year: 1970, day: 286 }, currentAbility: 150, potentialAbility: 150 });
    Object.assign(profiles[2], { uid: 7004959, name: 'Francisco Miranda', clubJob: 'Fisioterapeuta' });
    const key = value => value.normalize('NFD').replace(/\p{M}/gu, '').toLowerCase();
    const field = value => ({ value, min: 0, max: 2003, width: 2, offset: 0 });
    const player = { uid: 600, name: 'Pär Zetterberg', firstName: 'Pär', surname: 'Zetterberg', club: 'RSC Anderlecht', nation: 'Suécia', currentAbility: 140, potentialAbility: 150, attributes: {}, rawAttributes: {}, fields: { 'birth.year': field(1970), 'birth.day': field(286), nationality: field(0), passing: { ...field(15), min: 1, max: 20 } } };
    window.__testCalls = [];
    window.__TAURI__ = { core: { invoke: async (command, args = {}) => {
      window.__testCalls.push({ command, args });
      if (command === 'open_save') return { name: 'controle_ui.cm4', path: 'controle_ui.cm4', players: 1, staff: 51, sha256: 'controlled-fixture' };
      if (command === 'search_players') return { total: 1, rows: [{ uid: 600, name: player.name, club: player.club, age: 32, ca: 140, pa: 150 }] };
      if (command === 'search_staff') {
        const found = profiles.filter(p => key([p.uid, p.name, p.team, p.clubJob, p.nationalTeam, p.nationalJob].join(' ')).includes(key(args.query)));
        return { total: found.length, rows: found.slice(args.page * 50, args.page * 50 + 50).map(p => ({ uid: p.uid, name: p.name, team: p.team, role: p.clubJob, nationalTeam: p.nationalTeam, age: 43, ca: p.currentAbility, pa: p.potentialAbility, isPlayer: p.isPlayer })) };
      }
      if (command === 'staff_detail') return { profile: profiles.find(p => p.uid === args.uid), age: args.uid === 600 ? 32 : 43, pendingPlayerFields: 0 };
      if (command === 'player_detail') return { player, effective: player, age: 32, contracts: [], editable: ['passing'], pending: [], nations: ['Suécia'], injuries: [], healPending: false, suspensionCount: 0, suspensionPending: false };
      throw new Error('Unexpected IPC command: ' + command);
    } } };
  });
  await page.goto('http://127.0.0.1:4173');
  await page.locator('#open-save').click();
  await page.locator('#status').filter({ hasText: 'Save carregado' }).waitFor();
  await page.locator('#nav-staff').click();
  await page.locator('#staff-results button').first().waitFor();
  const assert = (value, message) => { if (!value) throw new Error(message); };
  assert(await page.locator('#staff-results button').count() === 50, 'First page must contain 50 staff');
  assert(await page.locator('#staff-previous').isDisabled(), 'Previous must be disabled on page one');
  await page.locator('#staff-next').click();
  await page.locator('#staff-page-label').filter({ hasText: '2 / 2' }).waitFor();
  assert(await page.locator('#staff-results button').count() === 1, 'Last page must contain one staff');
  assert(await page.locator('#staff-next').isDisabled(), 'Next must be disabled on last page');
  await page.locator('#staff-search').fill('rui aguas');
  await page.locator('#staff-results button').filter({ hasText: 'Rui Águas' }).waitFor();
  assert(await page.locator('#staff-results button').count() === 1, 'Accentless search must isolate Rui Águas');
  await page.locator('#staff-results button').click();
  await page.locator('#staff-profile h2').filter({ hasText: 'Rui Águas' }).waitFor();
  assert((await page.locator('#staff-profile').innerText()).includes('Treinador adjunto'), 'Staff role must be visible');
  assert(await page.locator('#staff-profile dt').filter({ hasText: 'Fisioterapia' }).locator('..').locator('dd').innerText() === '7', 'Staff ratings must show the converted value, not the raw byte');
  assert(await page.locator('#staff-profile input').count() === 0, 'Unvalidated staff fields must not have editable inputs');
  await page.locator('#staff-search').fill('fisioterapeuta');
  await page.locator('#staff-results button').filter({ hasText: 'Francisco Miranda' }).waitFor();
  await page.locator('#staff-search').fill('par zetterberg');
  await page.locator('#staff-results button').filter({ hasText: 'Pär Zetterberg' }).waitFor();
  await page.locator('#staff-results button').click();
  await page.locator('#staff-open-player').waitFor();
  await page.locator('#staff-open-player').click();
  await page.locator('#profile h2').filter({ hasText: 'Pär Zetterberg' }).waitFor();
  assert(await page.locator('#players-view').isVisible(), 'Dual role must navigate to player profile');
  await page.locator('[data-field="passing"]').fill('20');
  page.once('dialog', dialog => dialog.dismiss());
  await page.locator('#nav-staff').click();
  assert(await page.locator('#players-view').isVisible(), 'Canceling draft discard must retain player view');
  page.once('dialog', dialog => dialog.accept());
  await page.locator('#nav-staff').click();
  await page.locator('#status').filter({ hasText: 'Perfil de staff carregado' }).waitFor();
  assert(await page.locator('#staff-view').isVisible(), 'Accepting discard must open staff');
  assert(await page.locator('[data-field="passing"]').inputValue() === '15', 'Discarded player draft must be reset');
  assert(errors.length === 0, 'Frontend errors: ' + errors.join('; '));
  await page.screenshot({ path: 'reports/v06-staff-ui.png' });
  await page.locator('#open-save').click();
  await page.locator('#status').filter({ hasText: 'Save carregado' }).waitFor();
  await page.locator('#nav-staff').click();
  await page.locator('#staff-results button').first().waitFor();
  assert(await page.locator('#staff-search').inputValue() === '', 'Opening a save must reset staff query');
  assert(await page.locator('#staff-page-label').innerText() === '1 / 2', 'Opening a save must reset pagination');
  assert((await page.locator('#staff-profile').innerText()).includes('Escolha um membro do staff'), 'Opening a save must clear previous staff profile');
  const result = { passed: true, checks: ['pagination', 'accentless search', 'role search', 'readonly staff', 'dual-role navigation', 'draft cancel/discard', 'save session reset'], frontendErrors: errors };
  await page.evaluate(value => { window.__staffUiResult = value; }, result);
  return result;
}
