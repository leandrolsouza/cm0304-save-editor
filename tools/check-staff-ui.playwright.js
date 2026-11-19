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
    const clubRows=Array.from({length:51},(_,i)=>({uid:200000+i,name:i===0?'Clube de São João':'Clube de teste '+i,shortName:i===0?'São João':'Teste '+i,players:1,staff:2}));
    window.__testCalls = [];
    let edits=[], header=null, clubBalances={}, clubTransferBudgets={}, clubTraining={}, clubYouthSetup={}, undo=[], redo=[]; const snapshot=()=>structuredClone({edits,header,clubBalances,clubTransferBudgets,clubTraining,clubYouthSetup});
    const original=e=>{const p=profiles.find(p=>p.uid===e.uid);return e.attribute.startsWith('personality.')?p.personality[e.attribute.slice(12)]:p.attributes[e.attribute];};
    const pending=()=>({clubTransferBudgetFieldCount:Object.values(clubTransferBudgets).reduce((n,b)=>n+Number(b.season!==118033)+Number(b.remaining!==121595),0),clubBalances,clubTransferBudgets,clubTraining,clubYouthSetup,edits:[],staffEdits:edits,staffHeaderEdits:header?[header]:[],healInjuries:[],clearSuspensions:[],undo:undo.length>0,redo:redo.length>0});
    window.__TAURI__ = { core: { invoke: async (command, args = {}) => {
      window.__testCalls.push({ command, args });
      if (command === 'open_save') return { name: 'controle_ui.cm4', path: 'controle_ui.cm4', players: 1, staff: 51, clubs:51, sha256: 'controlled-fixture' };
      if(command==='search_clubs'){const found=clubRows.filter(c=>args.query.startsWith('uid:')?c.uid===Number(args.query.slice(4)):key(c.name+' '+c.uid).includes(key(args.query)));return {total:found.length,rows:found.slice(args.page*50,args.page*50+50)};}
      if(command==='club_detail')return {expiringContracts:args.uid===200000?[{uid:600,index:1,name:player.name,expires:{year:2003,day:230},days:30,loan:true}]:[],info:{balanceEditable:args.uid===200000,balance:args.uid===200000?-685427:null,transferBudgetSeason:args.uid===200000?118033:null,transferBudgetRemaining:args.uid===200000?121595:null,trainingRaw:11,youthSetupRaw:10,averageAttendance:10500},profile:{...clubRows.find(c=>c.uid===args.uid),playerUids:[600],staffUids:[4950,600]},players:[{uid:600,name:player.name,ca:140,pa:150}],staff:[{uid:4950,name:'Rui Águas',role:'Treinador adjunto',isPlayer:false},{uid:600,name:'Pär Zetterberg',role:'Jogador',isPlayer:true}]};
      if (command === 'search_players') return { total: 1, rows: [{ uid: 600, name: player.name, club: player.club, age: 32, ca: 140, pa: 150 }] };
      if (command === 'search_staff') {
        const found = profiles.filter(p => key([p.uid, p.name, p.team, p.clubJob, p.nationalTeam, p.nationalJob].join(' ')).includes(key(args.query)));
        return { total: found.length, rows: found.slice(args.page * 50, args.page * 50 + 50).map(p => ({ uid: p.uid, name: p.name, team: p.team, role: p.clubJob, nationalTeam: p.nationalTeam, age: 43, ca: p.currentAbility, pa: p.potentialAbility, isPlayer: p.isPlayer })) };
      }
      if (command === 'staff_detail') {
        const profile=structuredClone(profiles.find(p=>p.uid===args.uid));
        for(const e of edits.filter(e=>e.uid===args.uid))if(e.attribute.startsWith('personality.'))profile.personality[e.attribute.slice(12)]=e.value;else profile.attributes[e.attribute]=e.value;
        if(header?.uid===args.uid){if(header.currentAbility!=null)profile.currentAbility=header.currentAbility;if(header.potentialAbility!=null)profile.potentialAbility=header.potentialAbility;if(header.reputation)profile.reputation=header.reputation;}return {profile,age:args.uid===600?32:43,pendingPlayerFields:0,pendingHeader:header?.uid===args.uid?header:null,editable:[...Object.keys(profile.attributes),'personality.adaptability']};
      }
      if(command==='stage_staff_edits'){undo.push(snapshot());redo=[];if(args.header)header=args.header;for(const e of args.edits){edits=edits.filter(x=>x.uid!==e.uid||x.attribute!==e.attribute);if(e.value!==original(e))edits.push(e);}return pending();}
      if(command==='pending_edits')return pending();
      if(command==='history_action'){if(args.action==='undo'){redo.push(snapshot());({edits,header,clubBalances,clubTransferBudgets,clubTraining,clubYouthSetup}=undo.pop());}else if(args.action==='redo'){undo.push(snapshot());({edits,header,clubBalances,clubTransferBudgets,clubTraining,clubYouthSetup}=redo.pop());}else {undo.push(snapshot());edits=[];header=null;clubBalances={};clubTransferBudgets={};clubTraining={};clubYouthSetup={};}return pending();}
      if(command==='stage_club_balance'){undo.push(snapshot());redo=[];if(args.value!==-685427)clubBalances[args.uid]=args.value;else delete clubBalances[args.uid];if(args.budgets)clubTransferBudgets[args.uid]=args.budgets;return pending();}
      if(command==='stage_club_infrastructure'){
        undo.push(snapshot());redo=[];
        if(args.training!==null)clubTraining[args.uid]=args.training;
        if(args.youth!==null)clubYouthSetup[args.uid]=args.youth;
        return pending();
      }
      const clubChanges=Object.entries(clubBalances).map(([uid,after])=>({uid:Number(uid),name:'Clube de São João',attribute:'club.balance',before:-685427,after}));
      for(const [values,attribute,before] of [[clubTraining,'club.training',11],[clubYouthSetup,'club.youthSetup',10]]){
        for(const [uid,after] of Object.entries(values))clubChanges.push({uid:Number(uid),name:'Clube de São João',attribute,before,after});
      }
      if(command==='save_copy')return {path:'copia_teste.cm4',report:{changes:clubChanges,staffChanges:[],removals:[],fieldChangedBytes:3}};
      if(command==='preview_edits')return {changes:header?[{uid:header.uid,name:profiles.find(p=>p.uid===header.uid).name,attribute:'currentAbility',before:70,after:header.currentAbility??70,derived:false,context:'Staff'}]:clubChanges,staffChanges:edits.map(e=>({...e,before:original(e),after:e.value})),removals:[],fieldChangedBytes:edits.length,removedBytes:0};
      if (command === 'player_detail') return { player, effective: player, age: 32, contracts: [{index:0,kind:0,kindLabel:'Completo',entity:'Outro clube',started:{year:2002,day:0}},{index:1,kind:1,kindLabel:'Empréstimo',entity:'Clube de São João',started:{year:2003,day:0}}], editable: ['passing'], pending: [], nations: ['Suécia'], injuries: [], healPending: false, suspensionCount: 0, suspensionPending: false };
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
  assert(await page.locator('#staff-physiotherapy').inputValue() === '7', 'Staff ratings must show the converted value, not the raw byte');
  assert(await page.locator('#staff-profile [data-staff-field]:not([data-staff-field^="personality."])').count() === 11, 'Only the 11 professional fields may be editable');
  await page.locator('#staff-coachingOutfieldPlayers').fill('20');
  await page.locator('#staff-stage').click();
  await page.locator('#status').filter({hasText:'Atributos de staff adicionados'}).waitFor();
  assert(await page.locator('#pending-count').innerText()==='1','Staff edit must enable review and save');
  await page.locator('#review').click();
  await page.locator('#changes-list').filter({hasText:'12 → 20'}).waitFor();
  assert((await page.locator('#changes-list').innerText()).includes('Treino de jogadores de linha'),'Review must label staff fields');
  await page.locator('#undo').click();
  await page.locator('#status').filter({hasText:'Plano de alterações atualizado'}).waitFor();
  assert(await page.locator('#save-copy').isDisabled(),'Undo must remove the staff edit');
  await page.locator('#redo').click();
  await page.locator('#status').filter({hasText:'Plano de alterações atualizado'}).waitFor();
  assert(await page.locator('#pending-count').innerText()==='1','Redo must restore the staff edit');
  await page.locator('#clear').click();
  await page.locator('#status').filter({hasText:'Plano de alterações atualizado'}).waitFor();
  await page.locator('#nav-staff').click();
  await page.locator('#staff-coachingOutfieldPlayers').fill('19');
  page.once('dialog',dialog=>dialog.dismiss());
  await page.locator('#nav-players').click();
  assert(await page.locator('#staff-view').isVisible(),'Cancel must keep staff draft');
  page.once('dialog',dialog=>dialog.accept());
  await page.locator('#nav-players').click();
  assert(await page.locator('#staff-coachingOutfieldPlayers').inputValue()==='12','Discard must reset staff draft');
  await page.locator('#nav-staff').click();
  await page.locator('#staff-currentAbility').fill('100');
  await page.locator('#staff-stage').click();
  await page.locator('#status').filter({hasText:'Escolha uma política'}).waitFor();
  assert(await page.locator('#pending-count').innerText()==='0','Missing CA policy must not stage');
  await page.locator('#staff-ca-policy').selectOption('preserveRelative');
  await page.locator('#staff-potentialAbility').fill('180');
  await page.locator('#staff-reputation-0').fill('8000');
  await page.locator('#staff-stage').click();
  await page.locator('#status').filter({hasText:'Atributos de staff adicionados'}).waitFor();
  assert(await page.locator('#staff-currentAbility').inputValue()==='100','Projected CA must be displayed');
  const submitted=await page.evaluate(()=>window.__testCalls.filter(c=>c.command==='stage_staff_edits').at(-1).args.header);
  assert(submitted.currentAbility===100&&submitted.potentialAbility===180&&submitted.reputation[0]===8000&&submitted.policy==='preserveRelative','Header payload must carry values and policy');
  await page.locator('#review').click();
  await page.locator('#changes-list').filter({hasText:'70 → 100'}).waitFor();
  await page.locator('#undo').click();
  await page.locator('#status').filter({hasText:'Plano de alterações atualizado'}).waitFor();
  assert(await page.locator('#staff-currentAbility').inputValue()==='70','Undo must restore header');
  await page.locator('#nav-staff').click();
  await page.locator('[data-staff-field="personality.adaptability"]').fill('20');
  await page.locator('#staff-stage').click();
  await page.locator('#status').filter({hasText:'Atributos de staff adicionados'}).waitFor();
  assert(await page.locator('[data-staff-field="personality.adaptability"]').inputValue()==='20','Personality projection must update');
  await page.locator('#review').click();
  await page.locator('#changes-list').filter({hasText:'11 → 20'}).waitFor();
  assert((await page.locator('#changes-list').innerText()).includes('Adaptação'),'Personality review must use a readable label');
  await page.locator('#undo').click();
  await page.locator('#status').filter({hasText:'Plano de alterações atualizado'}).waitFor();
  assert(await page.locator('[data-staff-field="personality.adaptability"]').inputValue()==='11','Personality undo must restore source');
  await page.locator('#nav-staff').click();
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
  await page.locator('#nav-clubs').click();
  await page.locator('#club-results button').first().waitFor();
  assert(await page.locator('#club-results button').count()===50,'Club first page must contain 50');
  await page.locator('#club-next').click();
  await page.locator('#club-page-label').filter({hasText:'2 / 2'}).waitFor();
  assert(await page.locator('#club-results button').count()===1,'Club last page must contain one');
  await page.locator('#club-search').fill('sao joao');
  await page.locator('#club-results button').filter({hasText:'São João'}).waitFor();
  assert(await page.locator('#club-results button').count()===1,'Club search must ignore accents');
  await page.locator('#club-results button').click();
  await page.locator('#club-profile h2').filter({hasText:'Clube de São João'}).waitFor();
  assert(await page.locator('[data-club-player="600"]:not([data-club-contract])').count()===1&&await page.locator('[data-club-staff="600"]').count()===1,'Dual role may appear in both club groups');
  const clubInfo = page.locator('#club-profile dl');
  assert((await clubInfo.innerText()).includes('-685.427'), 'Club balance must preserve sign and use local formatting');
  assert(await clubInfo.locator('dd').allTextContents().then(v=>v.join('|')) === '-685.427|118.033|121.595|11|10|10.500', 'Club details must render the save values');
  await page.locator('#club-search').fill('uid:200001');
  await page.locator('[data-club-uid="200001"]').click();
  await clubInfo.locator('dd').first().filter({hasText:'Indisponível'}).waitFor();
  await page.locator('#club-search').fill('sao joao');
  await page.locator('[data-club-uid="200000"]').click();
  await clubInfo.locator('dd').first().filter({hasText:'-685.427'}).waitFor();
  await page.locator('#club-balance').fill('2500000');
  await page.locator('#club-balance-form button').click();
  await page.locator('#status').filter({hasText:'Plano de balanço atualizado'}).waitFor();
  await page.locator('#review').click();
  await page.locator('#changes-list').filter({hasText:'-685427 → 2500000'}).waitFor();
  await page.locator('#undo').click();
  await page.locator('#changes-list').filter({hasText:'Nenhuma alteração pendente'}).waitFor();
  await page.locator('#redo').click();
  await page.locator('#changes-list').filter({hasText:'-685427 → 2500000'}).waitFor();
  await page.locator('#save-copy').click();
  await page.locator('#saved-path').filter({hasText:'copia_teste.cm4'}).waitFor();
  await page.locator('#undo').click();
  await page.locator('#changes-list').filter({hasText:'Nenhuma alteração pendente'}).waitFor();
  await page.locator('#nav-clubs').click();
  await page.locator('#club-budget-remaining').fill('700000');
  await page.locator('#club-balance-form button').click();
  await page.locator('#pending-count').filter({hasText:/^1$/}).waitFor();
  await page.locator('#club-budget-season').fill('900000');
  await page.locator('#club-balance-form button').click();
  await page.locator('#pending-count').filter({hasText:/^2$/}).waitFor();
  await page.locator('#nav-changes').click();
  await page.locator('#clear').click();
  await page.locator('#pending-count').filter({hasText:/^0$/}).waitFor();
  await page.locator('#nav-clubs').click();
  await page.locator('#club-training').fill('20');
  await page.locator('#club-youth').fill('19');
  await page.locator('#club-training-form button').click();
  await page.locator('#status').filter({hasText:'Plano de infraestrutura atualizado'}).waitFor();
  await page.locator('#review').click();
  await page.locator('#changes-list .change-item').filter({hasText:'11 → 20'}).waitFor();
  await page.locator('#changes-list .change-item').filter({hasText:'10 → 19'}).waitFor();
  await page.locator('#undo').click();
  await page.locator('#changes-list').filter({hasText:'Nenhuma alteração pendente'}).waitFor();
  await page.locator('#nav-clubs').click();
  assert(await page.locator('#club-training').inputValue()==='11' && await page.locator('#club-youth').inputValue()==='10','Undo must restore both infrastructure fields');
  await page.screenshot({path:'reports/v06-clubs-ui.png'});
  await page.locator('[data-club-player="600"]:not([data-club-contract])').click();
  await page.locator('#profile h2').filter({hasText:'Pär Zetterberg'}).waitFor();
  assert(await page.locator('#players-view').isVisible(),'Club member must open player');
  await page.locator('[data-field="passing"]').fill('20');
  page.once('dialog',dialog=>dialog.dismiss());await page.locator('#nav-clubs').click();
  assert(await page.locator('#players-view').isVisible(),'Cancel must retain player draft before club navigation');
  page.once('dialog',dialog=>dialog.accept());await page.locator('#nav-clubs').click();
  await page.locator('#club-results button').first().waitFor();
  assert(await page.locator('[data-field="passing"]').inputValue()==='15','Discard on club navigation must reset player draft');
  await page.locator('[data-club-staff="4950"]').click();
  await page.locator('#staff-profile h2').filter({hasText:'Rui Águas'}).waitFor();
  assert(await page.locator('#staff-view').isVisible(),'Club staff must open professional profile');
  await page.locator('#nav-clubs').click();
  await page.locator('#open-save').click();
  await page.locator('#status').filter({hasText:'Save carregado'}).waitFor();
  await page.locator('#nav-clubs').click();
  await page.locator('#club-results button').first().waitFor();
  assert(await page.locator('#club-search').inputValue()===''&&await page.locator('#club-page-label').innerText()==='1 / 2','New save must reset club search and page');
  assert((await page.locator('#club-profile').innerText()).includes('Escolha um clube'),'New save must clear selected club');
  await page.locator('[data-club-uid="200000"]').click();
  await page.locator('#club-profile summary').filter({hasText:'próximos 90 dias'}).click();
  await page.locator('[data-club-contract="1"]').click();
  await page.locator('#panel-contracts').waitFor();
  assert(await page.locator('#contract-card-1').evaluate(card=>card===document.activeElement),'Expiry shortcut must focus the loan');
  assert(errors.length===0,'Frontend errors: '+errors.join(', '));
  const result = { passed: true, checks: ['pagination', 'accentless search', 'role search', '11 staff controls', 'staff staging and review', 'staff undo/redo/clear', 'staff draft cancel/discard', 'dual-role navigation', 'player draft cancel/discard', 'save session reset', 'staff header policy/payload/projection/review/undo', 'staff personality staging/review/undo', 'club pagination/search', 'club player/staff navigation', 'club draft guard', 'club session reset', 'club finance and facilities', 'club balance review/history/save', 'club infrastructure atomic review and undo', 'club transfer budget field count', 'club expiry opens exact loan'], frontendErrors: errors };
  await page.evaluate(value => { window.__staffUiResult = value; }, result);
  return result;
}
