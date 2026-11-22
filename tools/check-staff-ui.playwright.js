// Run with npm run test:ui. The frontend uses controlled IPC responses.
export default async function checkDesktopUi(page, url) {
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
    const player = { uid: 600, name: 'Pär Zetterberg', firstName: 'Pär', surname: 'Zetterberg', club: 'RSC Anderlecht', nation: 'Suécia', currentAbility: 140, potentialAbility: 150, attributes: {}, rawAttributes: {}, fields: { 'birth.year': field(1970), 'birth.day': field(286), nationality: field(0), height: {...field(188), min: 0, max: 300}, 'state.estimatedValue': {...field(225348), min: 0, max: 1000000000}, 'state.saleValue': {...field(451287), min: -1, max: 1000000000}, currentAbility: {...field(140), min: 1, max: 200}, 'position.goalkeeper': {...field(1), min: 1, max: 20}, 'position.midfielder': {...field(16), min: 1, max: 20}, passing: { ...field(15), min: 1, max: 20 } } };
    const clubRows=Array.from({length:51},(_,i)=>({uid:200000+i,name:i===0?'Clube de São João':'Clube de teste '+i,shortName:i===0?'São João':'Teste '+i,players:1,staff:2}));
    let suspensionPending = false;
    let contractPreview=[], contractEdits=[], staffPreview=[];
    window.__testCalls = [];
    let edits=[], header=null, clubBalances={}, clubTransferBudgets={}, clubTraining={}, clubYouthSetup={}, clubReputation={}, batchEdits=[], preparedBatch=[], undo=[], redo=[]; const snapshot=()=>structuredClone({edits,header,clubBalances,clubTransferBudgets,clubTraining,clubYouthSetup,clubReputation,batchEdits,contractEdits});
    const original=e=>{const p=profiles.find(p=>p.uid===e.uid);return e.attribute.startsWith('personality.')?p.personality[e.attribute.slice(12)]:p.attributes[e.attribute];};
    const describeStaffHistory = other => {
      if (!other || JSON.stringify(other.header)!==JSON.stringify(header)) return null;
      const keys = list => new Map(list.map(edit => [edit.uid+':'+edit.attribute, edit]));
      const previous = keys(other.edits), current = keys(edits);
      const staffFields = [...new Set([...previous, ...current]
        .filter(([key]) => previous.get(key)?.value!==current.get(key)?.value)
        .map(([,edit]) => edit.attribute))];
      return staffFields.length ? { index: 0, domains: ['Comissão'], playerCount: 0, playerFields: [], staffFields } : null;
    };
    const pending=()=>({undoOperation:describeStaffHistory(undo.at(-1)),redoOperation:describeStaffHistory(redo.at(-1)),reviewClubs:{players:{600:{uid:200000,name:'Clube de São João'}},staff:{4950:{uid:200000,name:'Clube de São João'}}},clubTransferBudgetFieldCount:Object.values(clubTransferBudgets).reduce((n,b)=>n+Number(b.season!==118033)+Number(b.remaining!==121595),0),clubBalances,clubTransferBudgets,clubTraining,clubYouthSetup,clubReputation,edits:[...batchEdits,...contractEdits],staffEdits:edits,staffHeaderEdits:header?[header]:[],healInjuries:[],clearSuspensions:suspensionPending?[600]:[],undo:undo.length>0,redo:redo.length>0});
    let windowMaximized = false;
    let activeSummary = null;
    window.addEventListener('pagehide', () => {
      if (activeSummary && sessionStorage.getItem('cm0304.resume-after-language')) {
        sessionStorage.setItem('mock-native-session', JSON.stringify({
          activeSummary, ...snapshot(), undo, redo, suspensionPending,
        }));
      }
    });
    window.__TAURI__ = { core: { invoke: async (command, args = {}) => {
      window.__testCalls.push({ command, args });
      if(command==='plugin:window|is_maximized')return windowMaximized;
      if(command==='plugin:window|toggle_maximize'){windowMaximized=!windowMaximized;return null;}
      if(command==='plugin:window|minimize')return null;
      if(command==='plugin:window|close'){
        window.dispatchEvent(new Event(window.__holdSave?'save-close-blocked':'editor-close-requested'));
        return null;
      }
      if(command==='close_editor')return null;
      if(command==='resume_editor'){
        const saved=JSON.parse(sessionStorage.getItem('mock-native-session'));
        sessionStorage.removeItem('mock-native-session');
        if(!saved)return null;
        ({activeSummary,edits,header,clubBalances,clubTransferBudgets,clubTraining,clubYouthSetup,clubReputation,
          batchEdits,contractEdits,undo,redo,suspensionPending}=saved);
        return [activeSummary,pending()];
      }
      if(command==='pick_save_file'){
        if(window.__holdPicker)return new Promise(resolve=>{window.__finishPicker=resolve;});
        return 'C:/teste/controle_ui.cm4';
      }
      if (command === 'open_save') {
        if(window.__rejectOpen)throw 'Não foi possível abrir o save. Versão interna incompatível.';
        if(window.__holdOpen)return new Promise(resolve=>{window.__finishOpen=resolve;});
        if(window.__managedFixture){
          edits=[];header=null;undo=[];redo=[];contractEdits=[];batchEdits=[];clubBalances={};clubTransferBudgets={};
          const managedClubs=clubRows.slice(0,window.__managedFixture).map(({uid,name})=>({uid,name}));
          return activeSummary={name:'meu_clube.cm4',path:'C:/teste/meu_clube.cm4',players:1,staff:51,clubs:51,sha256:'e'.repeat(64),managedClubs};
        }
        if(window.__openAnother){
          edits=[];header=null;undo=[];redo=[];contractEdits=[];
          return activeSummary={name:'segunda.cm4',path:'C:/teste/segunda.cm4',players:1,staff:51,clubs:51,sha256:'d'.repeat(64)};
        }
        contractEdits=[];
        return activeSummary={ name: 'controle_ui.cm4', path: 'controle_ui.cm4', players: 1, staff: 51, clubs:51, sha256: 'a'.repeat(64) };
      }
      if(command==='cancel_open_save'){window.__holdOpen=false;window.__finishOpen?.(null);return null;}
      if(command==='my_club_detail'||command==='stage_my_club_action'){
        const detail = () => {
          const value=clubBalances[args.uid]??-685427;
          const budget=clubTransferBudgets[args.uid]?.remaining??121595;
          return {shortcuts:[
            {command:'cash',value,target:value+100000000,affected:0,inReview:Number(value!==-685427),unavailable:0,reason:null},
            {command:'transfers',value:budget,target:budget<200000000?200000000:null,affected:0,inReview:Number(budget!==121595),unavailable:0,reason:budget>=200000000?'A verba já alcança £200 milhões':null},
            {command:'suspensions',value:null,target:null,affected:0,inReview:0,unavailable:0,reason:null}
          ]};
        };
        if(command==='my_club_detail')return detail();
        const applied=detail().shortcuts.find(item=>item.command===args.command);
        if(!applied?.target||args.uid!==200000)throw Error('Unexpected managed-club shortcut');
        undo.push(snapshot());redo=[];
        if(args.command==='cash')clubBalances[args.uid]=applied.target;
        if(args.command==='transfers')clubTransferBudgets[args.uid]={season:118033,remaining:applied.target};
        return {plan:pending(),club:detail(),applied};
      }
      if(command==='search_clubs'){const found=clubRows.filter(c=>args.query.startsWith('uid:')?c.uid===Number(args.query.slice(4)):key(c.name+' '+c.uid).includes(key(args.query)));return {total:found.length,rows:found.slice(args.page*50,args.page*50+50)};}
      if(command==='club_detail'){
        const detail = {
        stadium: args.uid === 200001 ? { capacity: 12500, seatedCapacity: 12500, firstTeamClubUids: [200001, 200002, 200003] }
          : args.uid === 200002 ? { capacity: 30000, seatedCapacity: 20000, firstTeamClubUids: [200002] } : null,
        expiringContracts:args.uid===200000?[{uid:600,index:1,name:player.name,expires:{year:2003,day:230},days:30,loan:true},{uid:600,index:0,name:player.name,expires:{year:2003,day:196},days:-3,loan:false},{uid:601,index:0,name:"Quim",expires:{year:2003,day:320},days:120,loan:false}]:[],info:{transferInstallments:args.uid===200000?[{counterparty:'Braga',playerName:player.name,personId:12,amountRaw:330,remaining:17,outgoing:true}]:args.uid===200001?null:[],sellOnClauses:args.uid===200000?[{playerUid:600,playerName:player.name,beneficiary:"Braga",percent:20}]:[],balanceEditable:args.uid===200000,balance:args.uid===200000?-685427:args.uid===200002?1250000:null,transferBudgetSeason:args.uid===200000?118033:null,transferBudgetRemaining:args.uid===200000?121595:null,wageBudgetWeekly:args.uid===200000?0:null,currentWagesWeekly:args.uid===200000?1100:null,playerContractWagesWeekly:args.uid===200000?1200:null,loanWageOffsetWeekly:args.uid===200000?100:null,financeSummary:args.uid===200000?{currentMonth:{income:12500,expenses:6900},previousMonth:{income:0,expenses:2500},season:{income:12500,expenses:9400}}:null,reputation:5497,trainingRaw:11,youthSetupRaw:10,averageAttendance:10500},profile:{...clubRows.find(c=>c.uid===args.uid),playerUids:[600],staffUids:[4950,600]},wageContracts:[{uid:600,index:0,name:player.name,loan:false,weeklyWage:523,squadStatus:5},{uid:600,index:1,name:player.name,loan:true,weeklyWage:476,squadStatus:2,squadNumber:41},{uid:601,index:0,name:"Quim",loan:false,weeklyWage:0,squadNumber:1}],attributeAverages:{passing:15},players:[{uid:600,name:player.name,ca:140,pa:150,age:32,injuryCount:2,suspensionCount:1,positions:["midfielder","attackingMidfielder"]}],staff:[{uid:4950,name:'Rui Águas',role:'Treinador adjunto',isPlayer:false,attributes:{judgingPlayerAbility:16}},{uid:600,name:'Pär Zetterberg',role:'Jogador',isPlayer:true,attributes:{judgingPlayerAbility:12}}]};
        detail.analysisLoans = [];
        detail.analysisTactic = null;
        detail.analysisTacticUnsupported = false;
        detail.players = detail.players.map(row=>({...row, familiarity:{midfielder:16,central:20},condition:null}));
        if(window.__analysisFixture&&args.uid===200000){
          detail.players = [
            {...detail.players[0], familiarity:{defender:20,leftSide:20,rightSide:20},condition:9234},
            {uid:601,name:'Quim',positions:['goalkeeper'],familiarity:{goalkeeper:20},condition:0},
            {uid:602,name:'César',positions:['defender'],familiarity:{defender:20,rightSide:20},condition:null},
            {uid:603,name:'Davi',positions:['attackingMidfielder'],familiarity:{attackingMidfielder:16,leftSide:17},condition:10000},
            {uid:604,name:'Bruno',positions:['goalkeeper'],familiarity:{goalkeeper:18},condition:9800},
            {uid:605,name:'Edson',positions:['goalkeeper'],familiarity:{goalkeeper:20},condition:7000,injuryCount:1},
          ];
          detail.analysisLoans = [{uid:602,index:1,expires:{year:2003,day:230}}];
          detail.analysisTactic = {name:'Diablo',positions:[
            [0,2],[2,4],[2,0],[2,3],[2,1],[4,4],[4,0],[4,2],[3,2],[6,3],[6,1],
          ].map(([row,column])=>({row,column})),starters:[601,600,602,null,null,null,null,null,null,null,null]};
        }
        return detail;
      }
      if(command==='plugin:app|version')return '1.1.0';
      if(command==='download_update') {
        const updateMode = new URLSearchParams(location.search).get('update');
        if (updateMode === 'pending') return new Promise(resolve => { window.__finishUpdate = resolve; });
        return updateMode === 'ready';
      }
      if(command==='install_update') {
        if (!document.body.inert) throw Error('A instalação deve bloquear ações novas');
        throw Error('Instalação indisponível no teste');
      }
      if(command==='select_filtered_players')return Array.from({length:51},(_,i)=>({uid:800000+i,name:'Resultado '+(i+1)}));
      if (command === 'search_players') {
        if(args.query==='consulta lenta')await new Promise(resolve=>{window.__finishSlowSearch=resolve;});
        if(args.query==='ar zett'&&!key(player.name).includes(key(args.query)))return {total:0,rows:[]};
        if(args.query==='uid:abc')throw 'UID de jogador inválido';
        if(args.filters?.watchUids&&!args.filters.watchUids.includes(player.uid))return {total:0,rows:[]};
        return { total: 1, rows: [{ uid: 600, name: player.name, club: player.club, age: 32, ca: 140, pa: 150 }] };
      }
      if (command === 'cancel_player_search') return null;
      if (command === 'search_staff') {
        const found = profiles.filter(p => key([p.uid, p.name, p.team, p.clubJob, p.nationalTeam, p.nationalJob].join(' ')).includes(key(args.query)));
        return { total: found.length, rows: found.slice(args.page * 50, args.page * 50 + 50).map(p => ({ uid: p.uid, name: p.name, team: p.team, role: p.clubJob, nationalTeam: p.nationalTeam, age: 43, ca: p.currentAbility, pa: p.potentialAbility, isPlayer: p.isPlayer })) };
      }
      if (command === 'staff_detail') {
        const recorded=profiles.find(p=>p.uid===args.uid);
        const profile=structuredClone(recorded);
        for(const e of edits.filter(e=>e.uid===args.uid))if(e.attribute.startsWith('personality.'))profile.personality[e.attribute.slice(12)]=e.value;else profile.attributes[e.attribute]=e.value;
        if(header?.uid===args.uid){if(header.currentAbility!=null)profile.currentAbility=header.currentAbility;if(header.potentialAbility!=null)profile.potentialAbility=header.potentialAbility;if(header.reputation)profile.reputation=header.reputation;}return {profile,recorded,age:args.uid===600?32:43,pendingPlayerFields:0,pendingHeader:header?.uid===args.uid?header:null,editable:[...Object.keys(profile.attributes),'personality.adaptability']};
      }
      if(command==='stage_staff_edits'){undo.push(snapshot());redo=[];if(args.header)header=args.header;for(const e of args.edits){edits=edits.filter(x=>x.uid!==e.uid||x.attribute!==e.attribute);if(e.value!==original(e))edits.push(e);}return pending();}
      if(command==='pending_edits')return pending();
      if(command==='export_session_plan'){
        localStorage.setItem('test-session-plan',JSON.stringify(snapshot()));
        return 'C:/teste/cm0304-plano.json';
      }
      if(command==='import_session_plan'){
        if(window.__changedPlanSource)throw 'O save de origem mudou no disco. Reabra o arquivo antes de retomar o plano.';
        const saved=JSON.parse(localStorage.getItem('test-session-plan'));
        undo.push(snapshot());redo=[];
        ({edits,header,clubBalances,clubTransferBudgets,clubTraining,clubYouthSetup,clubReputation,batchEdits,contractEdits}=saved);
        return pending();
      }
      if(command==='history_action'){if(args.action==='undo'){redo.push(snapshot());({edits,header,clubBalances,clubTransferBudgets,clubTraining,clubYouthSetup,clubReputation,batchEdits,contractEdits}=undo.pop());}else if(args.action==='redo'){undo.push(snapshot());({edits,header,clubBalances,clubTransferBudgets,clubTraining,clubYouthSetup,clubReputation,batchEdits,contractEdits}=redo.pop());}else {undo.push(snapshot());edits=[];header=null;clubBalances={};clubTransferBudgets={};clubTraining={};clubYouthSetup={};clubReputation={};batchEdits=[];contractEdits=[];}return pending();}
      if(command==='preview_player_batch'){
        const {uids,attribute,operation}=args.request;
        if(attribute==='personality.professionalism'){
          return uids.map(uid=>({uid,name:'Jogador '+uid,attribute,before:10,after:12,limited:false,unavailable:uid===800050}));
        }
        if(operation.kind==='adjust'){
          return uids.flatMap(uid=>operation.value.map(adjustment=>({uid,name:player.name,attribute:adjustment.attribute,
            before:15,after:adjustment.operation.value,limited:false})));
        }
        if(operation.kind==='copy'){
          if(operation.value.sourceUid!==600||!operation.value.attributes.includes('passing')||operation.value.attributes.includes('technique'))throw new Error('Unexpected copy selection');
          return [{uid:600,name:player.name,attribute:'passing',before:15,after:15,limited:false}];
        }
        if(attribute==='state.condition'&&operation.kind==='set'){
          if(operation.value!==7001)throw new Error('Condition must reach IPC in exact hundredths');
          return uids.map(uid=>({uid,name:player.name,attribute,before:8000,after:7001,limited:false}));
        }
        if(attribute!=='passing'||operation.kind!=='add'||operation.value!==20)throw new Error('Unexpected batch request');
        preparedBatch=uids.map(uid=>({uid,attribute,value:20}));
        return uids.map(uid=>({uid,name:player.name,before:15,after:20,limited:true}));
      }
      if(command==='preview_health_batch')return [
        ...(args.recover?[{uid:600,name:player.name,attribute:'state.condition',before:8000,after:10000,limited:false}]:[]),
        {uid:600,name:player.name,attribute:'healInjuries',before:2,after:0,limited:false},
        {uid:600,name:player.name,attribute:'clearSuspensions',before:1,after:0,limited:false}
      ];
      if(command==='stage_player_batch'){undo.push(snapshot());redo=[];batchEdits=preparedBatch;preparedBatch=[];return pending();}
      if(command==='remove_player_from_plan'){undo.push(snapshot());redo=[];batchEdits=batchEdits.filter(edit=>edit.uid!==args.uid||(args.field&&edit.attribute!==args.field));return pending();}
      if(command==='stage_club_balance'){undo.push(snapshot());redo=[];if(args.value!==-685427)clubBalances[args.uid]=args.value;else delete clubBalances[args.uid];if(args.budgets)clubTransferBudgets[args.uid]=args.budgets;return pending();}
      if(command==='stage_club_infrastructure'){
        undo.push(snapshot());redo=[];
        if(args.training!==null)clubTraining[args.uid]=args.training;
        if(args.youth!==null)clubYouthSetup[args.uid]=args.youth;
        if(args.reputation!=null)clubReputation[args.uid]=args.reputation;
        return pending();
      }
      const clubChanges=Object.entries(clubBalances).map(([uid,after])=>({uid:Number(uid),name:'Clube de São João',attribute:'club.balance',before:-685427,after}));
      for(const [values,attribute,before] of [[clubTraining,'club.training',11],[clubYouthSetup,'club.youthSetup',10],[clubReputation,'club.reputation',5497]]){
        for(const [uid,after] of Object.entries(values))clubChanges.push({uid:Number(uid),name:'Clube de São João',attribute,before,after});
      }
      for(const [uid,budgets] of Object.entries(clubTransferBudgets)){
        for(const [field,before,after] of [['Season',118033,budgets.season],['Remaining',121595,budgets.remaining]]){
          if(before!==after)clubChanges.push({uid:Number(uid),name:'Clube de São João',attribute:'club.transferBudget'+field,before,after});
        }
      }
      for(const edit of batchEdits)clubChanges.push({...edit,name:edit.uid===600?player.name:'Jogador '+edit.uid,before:15,after:edit.value});
      for(const edit of contractEdits)clubChanges.push({...edit,context:'Empréstimo · Clube de São João',
        ...(edit.attribute.includes('.expires.')?{displayBefore:'31/10/2003',displayAfter:'29/02/2004'}:{})});
      if(command==='compare_player_save'){
        if(window.__holdPlayerComparison){
          window.__holdPlayerComparison=false;
          await new Promise(resolve=>{window.__resolvePlayerComparison=resolve;});
        }
        if(window.__missingComparisonPlayer){
          window.__missingComparisonPlayer=false;
          throw 'Pär Zetterberg (UID 600) não foi encontrado em outra-carreira.cm4.';
        }
        const contract={index:0,kind:0,kindLabel:'Completo',entityType:0,entity:'RSC Anderlecht',started:{year:2002,day:0},monthly:false,transferStatus:7};
        const current={...player,fields:{...player.fields,'contract.0.weeklyWage':field(800),'contract.0.expires.year':field(2005),'contract.0.expires.day':field(180),'contract.0.squadNumber':field(-1),'contract.0.squadStatus':field(4)}};
        const other={...current,fields:{...current.fields,passing:field(18),'contract.0.weeklyWage':field(950),'contract.0.squadNumber':field(99),'contract.0.squadStatus':field(1),'contract.0.appearanceBonus':field(0),'contract.0.goalBonus':field(600),'contract.0.assistBonus':field(300),'contract.0.releaseClause':field(8000000)}};
        const unknown={...contract,index:1,transferStatus:255};
        return {current,other,currentName:'inicial.cm4',otherName:'avancado.cm4',currentDate:{year:2003,day:199},otherDate:{year:2003,day:227},currentContracts:[contract,unknown],otherContracts:[{...contract,transferStatus:68},unknown]};
      }
      if(command==='export_player_comparison')return 'comparacao.csv';
      if(command==='export_player_comparison_json')return 'comparacao.json';
      if(command==='cancel_save_comparison'){
        window.__resolveRosterComparison?.();
        window.__resolvePlayerComparison?.();
        return;
      }
      if(command==='compare_club_save'){
        if(window.__holdRosterComparison){
          window.__holdRosterComparison=false;
          await new Promise(resolve=>{window.__resolveRosterComparison=resolve;});
        }
        return {sourceName:'inicial.cm4',otherName:'avancado.cm4',report:{
        version:1,clubUid:args.uid,beforeClubName:'Clube de teste',afterClubName:'Clube de teste',
        beforeDate:{year:2003,day:199},afterDate:{year:2003,day:227},
        joined:[{uid:601,name:'Reforço'}],left:[{uid:602,name:'Ex-jogador'}],unchanged:4,
        changed:[{uid:600,name:player.name,attributes:[{field:'passing',before:15,after:18}],
          contractsBefore:[],contractsAfter:[{kind:0,entity:'Clube de teste',weeklyWageGbp:950,monthly:false,expires:[2004,90],transferStatus:7},
            {kind:1,entity:'Outro clube',weeklyWageGbp:200,monthly:true,expires:null}]}]
        }};
      }
      if(command==='export_club_comparison')return 'comparacao-elenco.json';
      if(command==='export_club_comparison_csv')return 'comparacao-elenco.csv';
      if(command==='preview_wage_batch')return contractPreview=[{uid:600,name:player.name,attribute:'contract.1.weeklyWage',before:476,after:571,limited:false}];
      if(command==='preview_expiry_batch')return contractPreview=[
        {uid:600,name:player.name,attribute:'contract.1.expires.year',before:2003,after:args.request.year,limited:false},
        {uid:600,name:player.name,attribute:'contract.1.expires.day',before:303,after:args.request.day,limited:false}
      ];
      if(command==='preview_squad_status_batch')return contractPreview=args.request.contracts.map(({uid,index})=>({uid,name:player.name,attribute:'contract.'+index+'.squadStatus',before:index===1?2:5,after:args.request.status,limited:false}));
      if(command==='stage_wage_batch'){
        undo.push(snapshot());redo=[];
        contractEdits.push(...contractPreview.map(target=>({...target,value:target.after})));
        return pending();
      }
      if(command==='preview_staff_batch')return staffPreview=args.uids.flatMap(uid=>Object.entries(args.values).map(([attribute,after])=>({uid,name:profiles.find(p=>p.uid===uid).name,attribute,before:original({uid,attribute}),after,limited:false})));
      if(command==='stage_staff_batch'){
        undo.push(snapshot());redo=[];
        edits.push(...staffPreview.map(target=>({...target,value:target.after})));
        return pending();
      }
      if(command==='export_batch_presets')return 'cm0304-presets.json';
      if(command==='export_saved_report')return 'cm0304-gravacao.json';
      if(command==='save_copy'){
        if(window.__holdSave)return new Promise(resolve=>{window.__finishSave=resolve;});
        return {path:window.__variantB?'C:/teste/copia_b.cm4':'C:/teste/copia_teste.cm4',report:{changes:clubChanges,staffChanges:[],removals:[],fieldChangedBytes:3,sourceSha256:'a'.repeat(64),copySha256:(window.__variantB?'c':'b').repeat(64)}};
      }
      if(command==='cancel_save_copy'){window.__holdSave=false;window.__finishSave?.(null);return null;}
      if(command==='open_copy_folder')return null;
      if(command==='preview_edits'){
        if(window.__failReview)throw 'A validação simulada falhou.';
        const market=contractEdits.find(edit=>edit.attribute==='contract.0.transferStatus');
        if(market)return {changes:[{...market,name:player.name,before:4,after:market.value,context:'Clube de São João'}],staffChanges:[],removals:[],fieldChangedBytes:1,removedBytes:0,marketJournalEntriesAdded:1,marketJournalBytesAdded:65};
      }
      if(command==='preview_edits')return {changes:header?[{uid:header.uid,name:profiles.find(p=>p.uid===header.uid).name,attribute:'currentAbility',before:70,after:header.currentAbility??70,derived:false,context:'Staff'}]:clubChanges,staffChanges:edits.map(e=>({...e,before:original(e),after:e.value})),removals:[],fieldChangedBytes:edits.length,removedBytes:0};
      if(command==='stage_edits'&&args.edits.some(edit=>edit.attribute==='contract.0.transferStatus')){contractEdits=contractEdits.filter(edit=>edit.attribute!=='contract.0.transferStatus');contractEdits.push(...args.edits.filter(edit=>edit.value!==4));return pending();}
      if(command==='stage_edits'&&args.edits.length===2&&args.edits.every(edit=>edit.attribute.endsWith('.squadNumber')))return pending();
      if (command === 'stage_edits' && args.suspension?.uid === 600) {
        suspensionPending = args.suspension.enabled;
        return pending();
      }
      if (command === 'player_detail') return { player, effective: player, age: 32, contracts: [{index:0,kind:0,kindLabel:'Completo',entity:'Outro clube',entityType:0,transferStatus:4,started:{year:2002,day:0}},{index:1,kind:1,kindLabel:'Empréstimo',entity:'Clube de São João',started:{year:2003,day:0}}], marketTargets:window.__marketJournalBlocked?[]:[4,6,68], marketJournalSupported:!window.__marketJournalBlocked, editable: ['passing'], pending: [], nations: ['Suécia', 'Argélia', 'Angola', 'Áustria'], injuries: [], healPending: false, suspensionCount: 1, bookingRecordCount: 1, suspensionPending };
      throw new Error('Unexpected IPC command: ' + command);
    } } };
  });
  const assert = (value, message) => { if (!value) throw new Error(message); };
  const openMenu = async selector => {
    const menu = page.locator(selector);
    if (await menu.getAttribute('open') === null) await menu.locator('summary').click();
  };
  await page.goto(url);
  await page.getByRole('heading', {name:'Open a career', exact:true}).waitFor();
  assert(await page.locator('html').getAttribute('lang') === 'en-GB', 'A fresh start must default to English');
  await page.locator('#language-select').selectOption('pt-BR');
  await page.locator('html[lang="pt-BR"] #open-save').waitFor();
  await page.goto(new URL('?update=ready', url).href);
  await page.getByRole('alert').filter({hasText:'Não foi possível instalar a atualização'}).waitFor();
  assert(await page.evaluate(()=>!document.body.inert && window.__testCalls.some(call=>call.command==='install_update')),
    'Failed installation must restore the editor');
  await page.goto(new URL('?update=pending', url).href);
  await page.locator('#open-save').click();
  await page.locator('#nav-players').waitFor({state:'visible'});
  await page.waitForFunction(()=>!document.getElementById('nav-players').disabled);
  await page.evaluate(()=>window.__finishUpdate(true));
  assert(await page.evaluate(()=>!window.__testCalls.some(call=>call.command==='install_update')),
    'A downloaded update must wait when a save is open');
  await page.evaluate(()=>localStorage.removeItem('cm0304.recent-saves.v1'));
  await page.goto(url);
  const windowControls = page.getByRole('group',{name:'Controles da janela'});
  await windowControls.getByRole('button',{name:'Minimizar janela'}).press('Enter');
  await page.waitForFunction(()=>window.__testCalls.some(call=>call.command==='plugin:window|minimize'));
  await windowControls.getByRole('button',{name:'Maximizar janela'}).click();
  await windowControls.getByRole('button',{name:'Restaurar janela'}).click();
  await windowControls.getByRole('button',{name:'Maximizar janela'}).waitFor();
  const themePicker = page.getByLabel('Tema', { exact: true });
  const root = page.locator('html');
  const themeBackground = () => root.evaluate(el => getComputedStyle(el).backgroundColor);
  assert(await themePicker.inputValue() === 'green', 'The original theme must be the default');
  const defaultBackground = await themeBackground();
  await themePicker.selectOption('retro');
  const retroBackground = await themeBackground();
  assert(retroBackground !== defaultBackground, 'The retro theme must apply its palette immediately');
  await page.reload();
  await page.locator('html[data-theme=retro]').waitFor();
  assert(await themePicker.inputValue() === 'retro' && await themeBackground() === retroBackground,
    'The theme and selector must be restored after reloading');
  await page.locator('#editor-version').getByText('Editor 1.1.0').waitFor();
  assert(await page.locator('#empty #open-save').isVisible(),'Opening must be available inside the start screen');
  assert(await page.locator('#nav-staff').isDisabled()&&await page.locator('#nav-clubs').isDisabled(),
    'Career sections must communicate their unavailable state before opening a save');
  await page.locator('#open-save').click();
  const appToast = page.locator('body > .toast-region .toast');
  const toastBelowHeader = locator => locator.evaluate(el => {
    const toast = el.getBoundingClientRect();
    const header = document.querySelector('main > header').getBoundingClientRect();
    return toast.top >= header.bottom && toast.top <= header.bottom + 12 && Math.abs(innerWidth - toast.right - 16) < 1;
  });
  await appToast.filter({ hasText: 'Save carregado' }).waitFor();
  assert(await toastBelowHeader(appToast), 'The app toast must sit below the header at the right edge');
  assert(await page.locator('#nav-players').getAttribute('aria-current')==='page'
    && await page.locator('#search').evaluate(el=>el===document.activeElement),
    'Opening must announce the selected section and focus its search');
  const openFile = await page.locator('#file-name').textContent();
  await themePicker.selectOption('light');
  assert(await themeBackground() !== retroBackground
    && await root.evaluate(el => getComputedStyle(el).colorScheme) === 'light'
    && await page.locator('#file-name').textContent() === openFile
    && await page.locator('#search').isVisible(), 'Changing theme must preserve the active career and search');
  await themePicker.selectOption('green');
  const divider = page.locator('#players-view .pane-divider');
  const roster = page.locator('#players-view .roster');
  assert(await page.locator('#results').evaluate(el => el.scrollWidth === el.clientWidth), 'Default player results must fit without horizontal scrolling');
  const originalWidth = (await roster.boundingBox()).width;
  await divider.press('ArrowRight');
  assert((await roster.boundingBox()).width > originalWidth, 'Keyboard resizing must widen the results pane');
  await divider.press('ArrowLeft');
  const exportMenu = page.locator('#players-view .export-menu');
  assert(!await page.locator('#export-filtered-players-json').isVisible(), 'Export formats must start collapsed');
  await exportMenu.locator('summary').click();
  await page.locator('#export-filtered-players-json').waitFor();
  await exportMenu.locator('summary').click();
  const unloadBlocked = () => page.evaluate(() =>
    !window.dispatchEvent(new Event('beforeunload', { cancelable: true })));
  assert(!await unloadBlocked(), 'An idle editor without edits must allow unload');
  await page.evaluate(()=>{window.__holdOpen=true;window.__holdPicker=true;});
  await page.locator('#open-save').click();
  await page.waitForFunction(()=>!!window.__finishPicker);
  assert(await page.locator('#open-save-progress').isHidden(),'Picker must not show loading progress');
  await page.evaluate(()=>{window.__holdPicker=false;window.__finishPicker('C:/teste/controle_ui.cm4');});
  await page.locator('#open-save-progress progress').waitFor();
  await page.getByRole('heading',{name:'Abrindo controle_ui.cm4'}).waitFor();
  await page.locator('body > .toast-region .toast[data-notice="info"]').getByText('Lendo o save e preparando a busca.').waitFor();
  assert(await unloadBlocked(), 'An active save load must prevent unload even without pending edits');
  await page.locator('#cancel-open-save').click();
  await page.locator('body > .toast-region .toast').filter({hasText:'Abertura cancelada.'}).waitFor();
  assert(!await unloadBlocked(), 'Canceling the load must allow unload again');
  await page.locator('body > .toast-region .toast').waitFor({state:'hidden'});
  if(await page.locator('#file-name').innerText()!=='controle_ui.cm4')throw Error('Canceling open must preserve the active save');
  await page.locator('#nav-diagnostics').click();
  const technicalDetails = page.locator('#player-diagnostics');
  await technicalDetails.locator('summary').click();
  await technicalDetails.getByText('Selecione um jogador', {exact:false}).waitFor();
  await page.locator('#nav-players').click();
  const playerSearch = page.locator('#search');
  await playerSearch.fill('ar zett');
  await playerSearch.press('ArrowDown');
  const playerMatch = page.locator('#results button').filter({hasText:'Pär Zetterberg'});
  await playerMatch.waitFor();
  assert(await playerMatch.evaluate(el => el === document.activeElement), 'ArrowDown must focus the player result');
  await page.keyboard.press('Escape');
  assert(await playerSearch.inputValue() === '' && await playerSearch.evaluate(el => el === document.activeElement),
    'Escape must clear the search and focus its input');
  await page.waitForFunction(() => !document.querySelector('#results').hasAttribute('aria-busy') &&
    document.querySelectorAll('#results button').length === 1);
  await playerSearch.fill('ar zett');
  await playerSearch.press('ArrowDown');
  await playerMatch.waitFor();
  await page.keyboard.press('ArrowUp');
  assert(await playerSearch.evaluate(el => el === document.activeElement), 'ArrowUp must return to player search');
  await page.keyboard.press('ArrowDown');
  await page.keyboard.press('Enter');
  await page.locator('#profile h2').getByText('Pär Zetterberg', {exact:true}).waitFor();
  await playerSearch.fill('');
  assert(await page.locator('#profile').getByText('Dados brutos de diagnóstico').count() === 0,
    'Raw data must not appear in the player profile');
  await page.locator('#nav-diagnostics').click();
  assert(await technicalDetails.getAttribute('open') === null, 'Player diagnostics must be collapsed');
  await technicalDetails.getByText('Dados técnicos do jogador selecionado', {exact:true}).click();
  await technicalDetails.getByText('Pär Zetterberg · UID 600', {exact:true}).waitFor();
  await page.locator('#player-diagnostics-raw').waitFor();
  await page.locator('#nav-players').click();
  await page.locator('#tab-personal').click();
  const nationality = page.locator('#profile [data-field="nationality"]');
  const names = await nationality.locator('option').allTextContents();
  assert(names.join(', ') === 'Angola, Argélia, Áustria, Suécia', 'Nationality labels must use Portuguese alphabetical order');
  assert(await nationality.inputValue() === '0', 'Sorting must preserve the selected nationality ID');
  const countryPicker = page.locator('#nation-trigger');
  const nationSearch = page.locator('#nation-search');
  const visibleCountries = page.locator('#nation-results button');
  assert(await nationSearch.isHidden(), 'Search must be inside the closed selector');
  await countryPicker.click();
  assert(await nationSearch.evaluate(el => el === document.activeElement), 'Opening must focus search');
  await nationSearch.fill('stria');
  assert((await visibleCountries.allTextContents()).join() === 'Áustria', 'Partial search must match accented names');
  await nationSearch.fill('austria');
  assert(await nationality.inputValue() === '0', 'Searching must preserve nationality until selection');
  await nationSearch.fill('xyzzz');
  await page.locator('#nation-search-feedback').getByText('Nenhum país encontrado.').waitFor();
  await nationSearch.fill('');
  assert(await visibleCountries.count() === 4, 'Clearing search must restore all countries');
  await nationSearch.fill('austria');
  await nationSearch.press('ArrowDown');
  await visibleCountries.first().press('Enter');
  assert(await nationality.inputValue() === '3' && await nationSearch.isHidden(), 'Keyboard choice must update and close the selector');
  await countryPicker.click();
  await nationSearch.fill('suecia');
  await nationSearch.press('Enter');
  assert(await nationality.inputValue() === '0', 'Original country must be restorable');
  const abilityChoice = page.locator('#player-ability-choice');
  for (const [tab, key, original, changed] of [
    ['ability', 'currentAbility', '140', '145'],
    ['positions', 'position.goalkeeper', '1', '20'],
  ]) {
    await page.locator('#tab-'+tab).click();
    assert(await abilityChoice.isHidden(), 'Ability choice must start hidden');
    const field = page.locator('#profile [data-field="'+key+'"]');
    await field.fill(changed);
    await abilityChoice.waitFor();
    assert(await field.evaluate(el => el.closest('label').nextElementSibling.id === 'player-ability-choice'),
      'Ability choice must sit next to the edited field');
    await page.locator('#ability-policy').selectOption({label:'Manter os atributos atuais'});
    await field.fill(original);
    await abilityChoice.waitFor({state:'hidden'});
  }
  assert(await page.locator('#stage-player').isDisabled(), 'An unchanged player must not be staged');
  await page.locator('#tab-attributes').click();
  const passingField = page.locator('#profile [data-field="passing"]');
  await passingField.fill('99');
  await page.locator('#profile').getByText('Passe deve estar entre 1 e 20.', {exact:true}).waitFor();
  assert(await passingField.inputValue() === '99' && await passingField.getAttribute('aria-invalid') === 'true', 'Invalid value must be kept and marked');
  assert(await page.locator('#stage-player').isDisabled(), 'Invalid drafts must block staging');
  await passingField.fill('16');
  assert(await passingField.getAttribute('aria-invalid') === null && !await page.locator('#stage-player').isDisabled(), 'Valid correction must unlock staging');
  await passingField.fill('15');
  await page.locator('#tab-market').click();
  await page.locator('#panel-market [data-notice="info"]').getByText('O jogo pode recalcular estes valores.').waitFor();
  await page.locator('#tab-personal').click();
  const heightField=page.locator('#profile [data-field="height"]');
  await heightField.fill('189');
  await page.locator('#tab-market').click();
  await page.getByText('Corrija 1 campo em Dados pessoais.', {exact:true}).waitFor();
  await page.getByRole('button',{name:'Ir ao campo'}).click();
  assert(await heightField.evaluate(el=>el===document.activeElement), 'Error action must open the tab and focus the field');
  await page.locator('#numeric-error-height').getByText('Essa altura não pode ser salva. Use 188 ou 191 cm.', {exact:true}).waitFor();
  await page.locator('body > .toast-region .toast[data-notice="error"]').getByText('Essa altura não pode ser salva. Use 188 ou 191 cm.').waitFor();
  await heightField.fill('188');
  await page.locator('#tab-market').click();
  const askingPrice = page.locator('#panel-market [data-field="state.saleValue"]');
  const noPrice = page.locator('#no-asking-price');
  await noPrice.check();
  assert(await askingPrice.inputValue() === '-1' && await askingPrice.isHidden(), 'Unspecified price must hide the stored marker');
  await noPrice.uncheck();
  assert(await askingPrice.inputValue() === '451287', 'Unchecking must restore the previous price');
  await page.locator('#tab-state').click();
  await page.getByText('Sem lesões', {exact:true}).waitFor();
  const suspensionDetails = page.locator('#panel-state details').filter({has:page.locator('#remove-suspensions')});
  assert(await suspensionDetails.getAttribute('open') === null, 'Suspensions must start collapsed');
  await suspensionDetails.locator('summary').click();
  const bookingRecord = page.getByText('1 registro de cartões no save.', { exact: true });
  await bookingRecord.waitFor();
  await page.getByRole('tab',{name:'Contratos'}).click();
  await page.evaluate(()=>{window.__marketJournalBlocked=true;});
  await page.locator('#results button').first().click();
  await page.getByRole('tab',{name:'Contratos'}).click();
  await page.locator('#contract-card-0 summary').click();
  await page.getByText('O diário deste save permite apenas consultar a situação de mercado.',{exact:true}).waitFor();
  assert(await page.locator('#player-market-status').count()===0,'Unsupported journal must keep the market selector unavailable');
  await page.locator('#contract-card-0 summary').click();
  await page.evaluate(()=>{window.__marketJournalBlocked=false;});
  await page.locator('#results button').first().click();
  await page.getByRole('tab',{name:'Contratos'}).click();
  const firstContract=page.locator('#contract-card-0');
  const secondContract=page.locator('#contract-card-1');
  assert(await firstContract.getAttribute('open')===null && await secondContract.getAttribute('open')===null, 'Contracts must start collapsed');
  await secondContract.locator('summary').click();
  await firstContract.locator('summary').click();
  assert(await secondContract.getAttribute('open')===null, 'Opening a contract must collapse the other');
  const market=page.locator('#profile').getByLabel('Situação de mercado');
  await market.selectOption('6');
  await firstContract.getByText('Rascunho',{exact:true}).waitFor();
  await page.locator('#selected-market-status').getByText('Transferência sem situação definida · Disponível para empréstimo',{exact:true}).waitFor();
  await page.getByRole('button',{name:'Adicionar à revisão',exact:true}).click();
  await page.locator('body > .toast-region .toast').filter({hasText:'Alterações adicionadas à revisão'}).waitFor();
  assert(await market.inputValue()==='6','Projection must show staged market status');
  await firstContract.getByText('Na revisão',{exact:true}).waitFor();
  const statusEdit=await page.evaluate(()=>window.__testCalls.findLast(call=>call.command==='stage_edits').args.edits[0]);
  assert(statusEdit.uid===600&&statusEdit.attribute==='contract.0.transferStatus'&&statusEdit.value===6,'Market target must use the exact player and contract');
  await page.locator('#review').click();
  await page.locator('body > .toast-region .toast').filter({hasText:'Diário de mercado com 1 nova(s) entrada(s) (65 bytes acrescentados)'}).waitFor();
  const marketReview=page.locator('#changes-list');
  await marketReview.getByText('Clube de São João',{exact:true}).first().waitFor();
  await marketReview.getByText(/Transferência e empréstimo sem situação definida.*Disponível para empréstimo/).waitFor();
  const reviewSearch=page.locator('#review-query');
  await reviewSearch.fill('ausente');
  await marketReview.getByText('0 de 1 alteração').waitFor();
  await reviewSearch.fill('mercado sao joao');
  await marketReview.getByText('1 de 1 alteração').waitFor();
  await marketReview.getByRole('button',{name:/Ajustar.*Situação de mercado/}).click();
  assert(await market.evaluate(el=>el===document.activeElement),'Review must focus the exact contract field');
  await page.locator('#review').click();
  assert(await reviewSearch.inputValue()==='mercado sao joao','Reopening review must keep its filter');
  await reviewSearch.fill('');
  await marketReview.getByRole('button',{name:/Ajustar.*Situação de mercado/}).click();
  await market.selectOption('4');
  await page.getByRole('button',{name:'Adicionar à revisão',exact:true}).click();
  await page.locator('body > .toast-region .toast').filter({hasText:'Alterações adicionadas à revisão'}).waitFor();
  assert(await market.inputValue()==='4','Reverting to source must restore the original selection');
  await page.getByRole('tab',{name:'Estado',exact:true}).click();
  await suspensionDetails.locator('summary').click();
  await page.locator('#remove-suspensions').check();
  await page.getByRole('button', { name: 'Adicionar à revisão' }).click();
  await page.getByText('Remoção preparada na revisão.', { exact: false }).waitFor();
  await bookingRecord.waitFor();
  await page.locator('#remove-suspensions').uncheck();
  await page.getByRole('button', { name: 'Adicionar à revisão' }).click();
  await suspensionDetails.locator('summary').click();
  await page.getByText('A contagem de cartões é preservada', { exact: false }).waitFor();
  await page.locator('#tab-attributes').click();
  await page.locator('#nav-staff').click();
  await page.locator('#staff-results button').first().waitFor();

  assert(await page.locator('#staff-results button').count() === 50, 'First page must contain 50 staff');
  assert(await page.locator('#staff-previous').isDisabled(), 'Previous must be disabled on page one');
  await page.locator('#staff-next').click();
  await page.locator('#staff-page-label').filter({ hasText: '2 / 2' }).waitFor();
  assert(await page.locator('#staff-results button').count() === 1, 'Last page must contain one staff');
  assert(await page.locator('#staff-next').isDisabled(), 'Next must be disabled on last page');
  await page.locator('#staff-search').fill('ui agu');
  await page.locator('#staff-search').press('ArrowDown');
  const staffMatch = page.locator('#staff-results button').filter({hasText:'Rui Águas'});
  await staffMatch.waitFor();
  assert(await page.locator('#staff-results button').count() === 1, 'Partial search must match names without accents');
  assert(await staffMatch.evaluate(el=>el===document.activeElement), 'ArrowDown must focus the filtered staff result');
  await page.keyboard.press('Escape');
  assert(await page.locator('#staff-search').inputValue() === '' && await page.locator('#staff-search').evaluate(el => el === document.activeElement),
    'Escape must clear the search and focus its input');
  await page.waitForFunction(() => !document.querySelector('#staff-results').hasAttribute('aria-busy') &&
    document.querySelectorAll('#staff-results button').length === 50);
  await page.locator('#staff-search').fill('ui agu');
  await page.locator('#staff-search').press('ArrowDown');
  await staffMatch.waitFor();
  await page.keyboard.press('ArrowUp');
  assert(await page.locator('#staff-search').evaluate(el=>el===document.activeElement), 'ArrowUp must return to the search');
  await page.keyboard.press('ArrowDown');
  await page.keyboard.press('Enter');
  await page.locator('#staff-profile h2').filter({ hasText: 'Rui Águas' }).waitFor();
  assert((await page.locator('#staff-profile').innerText()).includes('Treinador adjunto'), 'Staff role must be visible');
  assert(await page.locator('#staff-physiotherapy').inputValue() === '7', 'Staff ratings must show the converted value, not the raw byte');
  assert(await page.locator('#staff-profile [data-staff-field]:not([data-staff-field^="personality."])').count() === 11, 'Only the 11 professional fields may be editable');
  await page.locator('#staff-coachingOutfieldPlayers').fill('20');
  await page.locator('#staff-panel-ratings .attribute-difference').getByText('12 → 20', {exact:true}).waitFor();
  await page.locator('#staff-stage').press('Enter');
  await page.locator('body > .toast-region .toast').filter({hasText:'Atributos de staff adicionados'}).waitFor();
  assert(await page.locator('#staff-profile h2').evaluate(el=>el===document.activeElement),
    'Staff staging must restore a meaningful focus target after rerendering');
  assert(await page.locator('#pending-count').innerText()==='1','Staff edit must enable review and save');
  await page.locator('#staff-physiotherapy').fill('8');
  const closeEditor=page.getByRole('dialog',{name:'Fechar o editor?'});
  await windowControls.getByRole('button',{name:'Fechar janela'}).click();
  await closeEditor.getByRole('button',{name:'Voltar'}).click();
  assert(await page.locator('#staff-physiotherapy').inputValue()==='8'
    && !await page.evaluate(()=>window.__testCalls.some(call=>call.command==='close_editor')),
    'Declining close must preserve the draft and keep the editor open');
  await page.evaluate(async()=>{
    await new Promise(resolve=>requestAnimationFrame(()=>requestAnimationFrame(resolve)));
    window.dispatchEvent(new Event('editor-close-requested'));
  });
  await closeEditor.getByRole('button',{name:'Fechar editor'}).click();
  await page.waitForFunction(()=>window.__testCalls.some(call=>call.command==='close_editor'));
  await page.evaluate(()=>{window.__rejectOpen=true;});
  await page.locator('#open-save').click();
  const confirmOpen=page.getByRole('dialog',{name:'Abrir outro save?'});
  await confirmOpen.getByRole('button',{name:'Voltar'}).click();
  assert(await page.locator('#open-save-progress').isHidden(),'Declining must leave the file picker closed');
  await page.locator('#open-save').click();
  await confirmOpen.getByRole('button',{name:'Continuar'}).click();
  await page.locator('body > .toast-region .toast[data-notice="error"]').getByText(/Versão interna incompatível\. O save atual continua aberto, com as alterações na revisão\./).waitFor();
  assert(await page.locator('#file-name').innerText()==='controle_ui.cm4'
    && await page.locator('#pending-count').innerText()==='1','A rejected save must preserve the active save and its edits');
  assert(await page.locator('#staff-physiotherapy').inputValue()==='8','A rejected save must preserve a staff draft');
  await page.evaluate(()=>{window.__rejectOpen=false;window.__holdOpen=true;});
  await page.locator('#open-save').click();
  await confirmOpen.getByRole('button',{name:'Continuar'}).click();
  await page.locator('#cancel-open-save').click();
  await page.locator('body > .toast-region .toast').filter({hasText:'Abertura cancelada.'}).waitFor();
  assert(await page.locator('#staff-physiotherapy').inputValue()==='8'
    && await page.locator('#pending-count').innerText()==='1','Canceling open must preserve the draft and pending edit');
  await page.locator('#staff-physiotherapy').fill('7');
  await page.locator('#review').click();
  await page.locator('#changes-list').filter({hasText:'12 → 20'}).waitFor();
  assert((await page.locator('#changes-list').innerText()).includes('Treino de jogadores de linha'),'Review must label staff fields');
  await page.getByRole('button',{name:'Desfazer edição de Treino de jogadores de linha',exact:true}).waitFor();
  await page.evaluate(()=>{window.__failReview=true;});
  await page.locator('#nav-changes').click();
  await page.locator('body > .toast-region .toast[data-notice="error"]').getByText('Não foi possível validar o plano. A validação simulada falhou.').waitFor();
  assert(await page.locator('#save-copy').isDisabled()&&await page.locator('#save-variant').isDisabled(),
    'Failed validation must disable both save routes');
  assert(await page.locator('#changes-list .change-item').count()===0,'An earlier report must not survive validation failure');
  const saveCallsBefore=await page.evaluate(()=>window.__testCalls.filter(call=>call.command==='save_copy').length);
  await page.keyboard.press('Control+Shift+s');
  await page.locator('#save-variant-form').evaluate(form=>{
    form.querySelector('input').value='variante';
    form.dispatchEvent(new Event('submit',{cancelable:true}));
  });
  assert(saveCallsBefore===await page.evaluate(()=>window.__testCalls.filter(call=>call.command==='save_copy').length),
    'Neither the shortcut nor form submission may bypass validation');
  await page.evaluate(()=>{window.__failReview=false;});
  await page.locator('#retry-review').click();
  await page.locator('#save-copy:enabled').waitFor();
  assert(await page.locator('#review-error').isHidden(),'A successful retry must clear the validation error');
  const variantForm=page.locator('details').filter({has:page.locator('#save-variant-form')});
  await variantForm.locator('summary').click();
  const variantName=variantForm.locator('#save-variant-name');
  await variantName.fill('');
  await variantForm.locator('#save-variant').click();
  await page.locator('body > .toast-region .toast[data-notice="error"]').getByText('Preencha “Nome da variante”.').waitFor();
  assert(await variantName.evaluate(el=>el===document.activeElement),'Native validation must focus the invalid field');
  assert(saveCallsBefore===await page.evaluate(()=>window.__testCalls.filter(call=>call.command==='save_copy').length),
    'Native validation must keep a blank variant name from reaching the save command');
  await variantForm.locator('summary').click();
  await page.evaluate(()=>{window.__holdSave=true;});
  await page.locator('#save-copy').click();
  await page.locator('#save-copy-progress progress').waitFor();
  const closeCallsBefore = await page.evaluate(()=>window.__testCalls.filter(call=>call.command==='close_editor').length);
  await windowControls.getByRole('button',{name:'Fechar janela'}).click();
  await page.evaluate(()=>window.dispatchEvent(new Event('editor-close-requested')));
  assert(closeCallsBefore === await page.evaluate(()=>window.__testCalls.filter(call=>call.command==='close_editor').length),
    'A close request during a save must never invoke close_editor');
  assert(await page.getByRole('dialog').count() === 0,
    'A close request during a save must not display the discard confirmation');
  await page.locator('body > .toast-region .toast[data-notice="warning"]').getByText(/Aguarde a gravação da cópia/).waitFor();
  await page.locator('#cancel-save-copy').click();
  await page.locator('body > .toast-region .toast').filter({hasText:'Gravação cancelada.'}).waitFor();
  assert(await page.locator('#pending-count').innerText()==='1','Canceling save must preserve the review');
  await page.locator('#undo').click();
  await page.locator('body > .toast-region .toast').filter({hasText:'Plano de alterações atualizado'}).waitFor();
  assert(await page.locator('#save-copy').isDisabled(),'Undo must remove the staff edit');
  await page.getByRole('button',{name:'Refazer edição de Treino de jogadores de linha',exact:true}).waitFor();
  await page.locator('#redo').click();
  await page.locator('body > .toast-region .toast').filter({hasText:'Plano de alterações atualizado'}).waitFor();
  assert(await page.locator('#pending-count').innerText()==='1','Redo must restore the staff edit');
  await page.locator('#clear').click();
  await page.locator('body > .toast-region .toast').filter({hasText:'Plano de alterações atualizado'}).waitFor();
  await page.locator('#nav-staff').click();
  await page.locator('#staff-coachingOutfieldPlayers').fill('19');
  await page.locator('#nav-players').click();
  const confirmDiscard=page.getByRole('dialog',{name:'Descartar rascunhos?'});
  await confirmDiscard.getByRole('button',{name:'Voltar'}).click();
  assert(await page.locator('#staff-view').isVisible(),'Cancel must keep staff draft');
  await page.locator('#nav-players').click();
  await confirmDiscard.getByRole('button',{name:'Descartar'}).click();
  await page.locator('#players-view').waitFor();
  assert(await page.locator('#staff-coachingOutfieldPlayers').inputValue()==='12','Discard must reset staff draft');
  await page.locator('#nav-staff').click();
  await page.locator('#staff-tab-ratings').press('ArrowRight');
  assert(await page.locator('#staff-tab-ability').getAttribute('aria-selected')==='true', 'ArrowRight must activate the next Staff tab');
  assert(await page.locator('#staff-policy-box').isHidden(), 'CA choice must be hidden before editing');
  const staffCA = page.locator('#staff-currentAbility');
  await staffCA.fill('201');
  await page.locator('#staff-profile').getByText('CA deve estar entre 0 e 200.', {exact:true}).waitFor();
  assert(await staffCA.inputValue()==='201' && await staffCA.getAttribute('aria-invalid')==='true', 'Invalid staff CA must stay visible');
  assert(await page.locator('#staff-stage').isDisabled(), 'Invalid staff CA must block staging');
  await page.locator('#staff-tab-ratings').click();
  await page.locator('#go-to-staff-error').click();
  assert(await staffCA.evaluate(el=>el===document.activeElement), 'Staff error action must focus the field across tabs');
  await staffCA.fill('100');
  assert(!await page.locator('#staff-stage').isDisabled(), 'Correcting staff CA must enable staging');
  await page.locator('#staff-stage').click();
  await page.locator('body > .toast-region .toast').filter({hasText:'Escolha uma política'}).waitFor();
  assert(await page.locator('#pending-count').innerText()==='0','Missing CA policy must not stage');
  await page.locator('#staff-policy-box').waitFor();
  await page.locator('#staff-ca-policy').selectOption('preserveRelative');
  await page.locator('#staff-potentialAbility').fill('180');
  await page.locator('#staff-reputation-0').fill('8000');
  await page.locator('#staff-stage').click();
  await page.locator('body > .toast-region .toast').filter({hasText:'Atributos de staff adicionados'}).waitFor();
  assert(await page.locator('#staff-currentAbility').inputValue()==='100','Projected CA must be displayed');
  const submitted=await page.evaluate(()=>window.__testCalls.filter(c=>c.command==='stage_staff_edits').at(-1).args.header);
  assert(submitted.currentAbility===100&&submitted.potentialAbility===180&&submitted.reputation[0]===8000&&submitted.policy==='preserveRelative','Header payload must carry values and policy');
  await page.locator('#review').click();
  await page.locator('#changes-list').filter({hasText:'70 → 100'}).waitFor();
  await page.locator('#changes-list .review-columns').first().getByText('Original', {exact:true}).waitFor();
  await page.locator('#review-summary').filter({hasText:'alterações pendentes'}).waitFor();
  await page.locator('#save-copy').waitFor();
  await page.locator('#undo').click();
  await page.locator('body > .toast-region .toast').filter({hasText:'Plano de alterações atualizado'}).waitFor();
  assert(await page.locator('#staff-currentAbility').inputValue()==='70','Undo must restore header');
  await page.locator('#nav-staff').click();
  await page.locator('#staff-tab-personality').click();
  await page.locator('[data-staff-field="personality.adaptability"]').fill('20');
  await page.locator('#staff-stage').click();
  await page.locator('body > .toast-region .toast').filter({hasText:'Atributos de staff adicionados'}).waitFor();
  assert(await page.locator('[data-staff-field="personality.adaptability"]').inputValue()==='20','Personality projection must update');
  await page.locator('#review').click();
  await page.locator('#changes-list').filter({hasText:'11 → 20'}).waitFor();
  assert((await page.locator('#changes-list').innerText()).includes('Adaptação'),'Personality review must use a readable label');
  await page.locator('#undo').click();
  await page.locator('body > .toast-region .toast').filter({hasText:'Plano de alterações atualizado'}).waitFor();
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
  assert(await page.locator('#search').inputValue()==='uid:600','Dual-role navigation must search by the exact UID');
  assert(await page.locator('#staff-results [data-staff-uid="600"]').evaluate(row=>row.classList.contains('selected')),
    'Opening the player profile must preserve the selected staff row');
  await page.locator('[data-field="passing"]').fill('20');
  await page.locator('#nav-staff').click();
  await confirmDiscard.getByRole('button',{name:'Voltar'}).click();
  assert(await page.locator('#players-view').isVisible(), 'Canceling draft discard must retain player view');
  await page.locator('#nav-staff').click();
  await confirmDiscard.getByRole('button',{name:'Descartar'}).click();
  await page.locator('#staff-view').waitFor();
  await page.locator('body > .toast-region .toast').filter({ hasText: 'Perfil de staff carregado' }).waitFor();
  assert(await page.locator('#staff-view').isVisible(), 'Accepting discard must open staff');
  assert(await page.locator('[data-field="passing"]').inputValue() === '15', 'Discarded player draft must be reset');
  await page.locator('#nav-players').click();
  await page.locator('[data-field="passing"]').fill('20');
  await page.locator('#nav-changes').click();
  await confirmDiscard.getByRole('button',{name:'Descartar'}).click();
  await page.locator('#changes-view').waitFor();
  assert(await page.locator('[data-field="passing"]').inputValue()==='15',
    'Opening review after discard must clear the hidden player draft');
  assert(errors.length === 0, 'Frontend errors: ' + errors.join('; '));
  await page.locator('#open-save').click();
  await page.locator('body > .toast-region .toast').filter({ hasText: 'Save carregado' }).waitFor();
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
  await page.locator('#club-search').fill('ao jo');
  await page.locator('#club-search').press('ArrowDown');
  const clubMatch = page.locator('#club-results button').filter({hasText:'São João'});
  await clubMatch.waitFor();
  assert(await page.locator('#club-results button').count()===1,'Club search must match partial names without accents');
  assert(await clubMatch.evaluate(el=>el===document.activeElement),'ArrowDown must focus the filtered club');
  await page.keyboard.press('Escape');
  assert(await page.locator('#club-search').inputValue() === '' && await page.locator('#club-search').evaluate(el => el === document.activeElement),
    'Escape must clear the search and focus its input');
  await page.waitForFunction(() => !document.querySelector('#club-results').hasAttribute('aria-busy') &&
    document.querySelectorAll('#club-results button').length === 50);
  await page.locator('#club-search').fill('ao jo');
  await page.locator('#club-search').press('ArrowDown');
  await clubMatch.waitFor();
  await page.keyboard.press('Enter');
  await page.locator('#club-profile h2').filter({hasText:'Clube de São João'}).waitFor();
  assert(await page.locator('[data-club-player="600"]:not([data-club-contract])').count()===1&&await page.locator('[data-club-staff="600"]').count()===1,'Dual role may appear in both club groups');
  const clubInfo=page.locator('#club-finance-indicators');
  const editClubFinances=page.getByRole('button',{name:'Editar finanças',exact:true});
  const financeForm=page.locator('#club-balance-form');
  const financeStage=page.locator('button[form="club-balance-form"]');
  const balanceField=page.locator('#club-balance');
  assert((await clubInfo.innerText()).includes('-685.427'), 'Club balance must preserve sign and use local formatting');
  assert(await clubInfo.locator('dt').count()===3,'Finance overview must have three indicators');
  assert(await clubInfo.locator('dd:nth-child(2)').allTextContents().then(v=>v.join('|'))==='-685.427|£ 121.595|£ 0',
    'Indicators must render the registered balance, remaining budget and weekly wage budget, including zero');
  await clubInfo.getByText(/Temporada · £\s118\.033/).waitFor();
  const movement=page.getByRole('table',{name:'Movimentação financeira registrada (£)'});
  assert((await movement.boundingBox()).y>=(await clubInfo.boundingBox()).y+(await clubInfo.boundingBox()).height,
    'Movement table must follow the indicators');
  assert((await movement.getByRole('row',{name:/Resultado/}).getByRole('cell').allTextContents()).join('|')==='5.600|-2.500|3.100',
    'Movement must show the original period totals and derived results');
  await page.getByText('Detalhes da folha',{exact:true}).click();
  await page.locator('.finance-details').getByText('Folha estimada após contribuições de empréstimos',{exact:true}).waitFor();
  await page.getByText('Detalhes da folha',{exact:true}).click();
  assert(await financeForm.isHidden(),'Financial editing must start collapsed');
  const financeViewport=page.viewportSize();
  await page.setViewportSize({width:1700,height:900});
  await editClubFinances.press('Enter');
  const financeEditor=page.locator('#club-finance-editor');
  assert(await editClubFinances.getAttribute('aria-expanded')==='true'
    && await balanceField.evaluate(el=>el===document.activeElement),'Opening the editor must expose its state and focus the first field');
  assert((await financeEditor.boundingBox()).x>(await clubInfo.boundingBox()).x,'A wide profile must place the editor to the right');
  assert(await financeStage.isDisabled(),'No-op finance submissions must be disabled');
  await balanceField.fill('-1');
  await financeEditor.getByRole('button',{name:'Recolher'}).click();
  assert(await financeForm.isHidden()&&await editClubFinances.evaluate(el=>el===document.activeElement),
    'Collapsing the editor must return focus to its trigger');
  await page.getByText('Rascunho de finanças',{exact:true}).waitFor();
  await editClubFinances.click();
  assert(await balanceField.inputValue()==='-1','Collapsing must preserve the draft');
  await balanceField.fill('-685427');
  await financeEditor.getByRole('button',{name:'Recolher'}).click();
  await page.setViewportSize(financeViewport);
  await page.getByText('Cláusulas de revenda',{exact:true}).click();
  await page.getByText('Pär Zetterberg · 20% para Braga',{exact:true}).waitFor();
  await page.getByText('Prestações de transferências',{exact:true}).click();
  await page.getByRole('row').filter({hasText:/Pär Zetterberg.*Braga.*A pagar.*330.*17/}).waitFor();
  await page.locator('#club-search').fill('uid:200001');
  await page.locator('[data-club-uid="200001"]').click();
  await page.locator('#club-tab-stadium').click();
  await page.getByText('Estádio compartilhado disponível para consulta.',{exact:true}).waitFor();
  await page.getByText('Clubes que usam o estádio com a equipe principal',{exact:true}).waitFor();
  assert(await page.locator('#club-stadium-form').count()===0,'Shared stadium must remain read-only');
  await page.locator('#club-tab-finance').click();
  await page.getByText('Prestações de transferências',{exact:true}).click();
  await page.getByText('Formato de prestações indisponível neste save.',{exact:true}).waitFor();
  await clubInfo.locator('dd').first().filter({hasText:'Indisponível'}).waitFor();
  await page.getByText('Este registro não tem dados de saldo ou orçamento.').waitFor();
  assert(await editClubFinances.count()===0,'Missing balance must remain read-only');
  await page.locator('#club-search').fill('uid:200002');
  await page.locator('[data-club-uid="200002"]').click();
  await page.getByText('Prestações de transferências',{exact:true}).click();
  await page.getByText('Nenhuma prestação registrada para este clube.',{exact:true}).waitFor();
  await clubInfo.locator('dd').first().filter({hasText:'1.250.000'}).waitFor();
  await page.getByText('Registro financeiro básico. O saldo bruto está disponível para consulta.').waitFor();
  assert(await editClubFinances.count()===0,'Basic balance must remain read-only');
  await page.locator('#club-tab-stadium').click();
  const stadiumForm=page.locator('#club-stadium-form');
  const stadiumStage=stadiumForm.getByRole('button',{name:'Adicionar à revisão'});
  await stadiumForm.getByText('Registrado · 30.000',{exact:true}).waitFor();
  assert(await stadiumStage.isDisabled(),'The stadium form must start with its local review action disabled');
  const seats=stadiumForm.getByRole('spinbutton',{name:'Lugares sentados'});
  await seats.fill('31000');
  await stadiumForm.getByText('Os lugares sentados devem caber na lotação total.',{exact:true}).waitFor();
  assert(await stadiumStage.isDisabled(),'The stadium form must block an invalid seated capacity');
  await seats.fill('26000');
  assert(await stadiumStage.isEnabled(),'Correcting the seated capacity must enable review');
  await seats.fill('20000');
  await page.locator('#club-tab-finance').click();
  await page.locator('#club-search').fill('sao joao');
  await page.locator('[data-club-uid="200000"]').click();
  await clubInfo.locator('dd').first().filter({hasText:'-685.427'}).waitFor();
  await editClubFinances.click();
  await page.locator('#club-balance').fill('2500000');
  await page.locator('button[form="club-balance-form"]').click();
  await page.locator('body > .toast-region .toast').filter({hasText:'Plano de balanço atualizado'}).waitFor();
  await clubInfo.getByText('Na revisão · 2.500.000',{exact:true}).waitFor();
  assert(await clubInfo.locator('dd').first().textContent()==='-685.427','Staging must keep the registered value visible');
  await page.locator('#review').click();
  await page.locator('#changes-list').filter({hasText:'-685427 → 2500000'}).waitFor();
  await page.keyboard.press('Control+z');
  await page.locator('#changes-list').filter({hasText:'Nenhuma alteração pendente'}).waitFor();
  await page.keyboard.press('Control+y');
  await page.locator('#changes-list').filter({hasText:'-685427 → 2500000'}).waitFor();
  await page.locator('#export-session-plan').click();
  await page.locator('body > .toast-region .toast').filter({hasText:'Plano guardado'}).waitFor();
  await page.evaluate(()=>{window.__changedPlanSource=true;});
  await page.locator('#import-session-plan').click();
  const confirmImport=page.getByRole('dialog',{name:'Substituir alterações pendentes?'});
  await confirmImport.getByRole('button',{name:'Voltar'}).click();
  assert(!await page.evaluate(()=>window.__testCalls.some(call=>call.command==='import_session_plan')),
    'Declining plan replacement must leave the file picker closed');
  await page.locator('#import-session-plan').click();
  await confirmImport.getByRole('button',{name:'Substituir'}).click();
  await page.locator('body > .toast-region .toast[data-notice="error"]').filter({hasText:'O save de origem mudou no disco'}).waitFor();
  await page.locator('#changes-list').getByText('-685427 → 2500000',{exact:true}).waitFor();
  assert(await page.locator('#pending-count').innerText()==='1','Rejected import must preserve the pending plan');
  assert(await page.evaluate(()=>window.__testCalls.findLast(call=>call.command==='import_session_plan').args.replaceExisting),
    'Replacing an existing plan must require confirmation');
  await page.evaluate(()=>{window.__changedPlanSource=false;});
  await page.locator('#clear').click();
  await page.locator('#changes-list').filter({hasText:'Nenhuma alteração pendente'}).waitFor();
  await page.locator('#import-session-plan').click();
  await page.locator('body > .toast-region .toast').filter({hasText:'Plano restaurado e validado'}).waitFor();
  await page.locator('#changes-list').getByText('-685427 → 2500000',{exact:true}).waitFor();
  await page.keyboard.press('Control+Shift+s');
  await page.locator('#saved-path').filter({hasText:'copia_teste.cm4'}).waitFor();
  await page.locator('#open-copy-folder').click();
  assert(await page.evaluate(()=>window.__testCalls.some(call=>call.command==='open_copy_folder')),'Copy confirmation must open the folder through IPC');
  await page.locator('#open-copy-history').click();
  await page.locator('.saved-copy-entry').filter({hasText:'copia_teste.cm4'}).getByText('b'.repeat(64),{exact:true}).waitFor();
  await page.evaluate(()=>{window.__variantB=true;});
  await page.locator('#save-copy').click();
  await page.locator('#saved-path').filter({hasText:'copia_b.cm4'}).waitFor();
  await page.evaluate(()=>{
    const key='cm0304.saved-copies.v1';
    const copies=JSON.parse(localStorage.getItem(key));
    copies.push({...copies[0],path:'C:/teste/outra-origem.cm4',sourceSha256:'f'.repeat(64),reportId:undefined});
    localStorage.setItem(key,JSON.stringify(copies));
  });
  const historyOrigin=page.locator('#saved-copy-current-origin');
  await historyOrigin.check();
  assert(await page.locator('.saved-copy-entry').count()===2,'Origin filter must keep both variants of the same save');
  await historyOrigin.uncheck();
  await page.locator('.saved-copy-entry').filter({hasText:'outra-origem.cm4'}).waitFor();
  await page.locator('#undo').click();
  await page.locator('#changes-list').filter({hasText:'Nenhuma alteração pendente'}).waitFor();
  await page.locator('#nav-clubs').click();
  await page.locator('#club-tab-finance').click();
  await editClubFinances.click();
  await page.locator('#club-budget-remaining').fill('700000');
  await page.locator('button[form="club-balance-form"]').click();
  await page.locator('#pending-count').filter({hasText:/^1$/}).waitFor();
  await page.locator('#club-budget-season').fill('900000');
  await page.locator('button[form="club-balance-form"]').click();
  await page.locator('#pending-count').filter({hasText:/^2$/}).waitFor();
  await page.locator('#nav-changes').click();
  await page.locator('#clear').click();
  await page.locator('#pending-count').filter({hasText:/^0$/}).waitFor();
  await page.locator('#nav-clubs').click();
  await page.locator('#club-tab-infrastructure').click();
  const trainingLevel = page.locator('#club-training');
  const infraForm=page.locator('#club-training-form');
  await infraForm.getByText('Registrado · 11',{exact:true}).waitFor();
  assert(await infraForm.getByRole('button',{name:'Adicionar à revisão'}).isDisabled(),
    'Infrastructure must show the registered value and start with its local review action disabled');
  await trainingLevel.fill('99');
  assert(await trainingLevel.inputValue()==='99' && await trainingLevel.getAttribute('aria-invalid')==='true', 'Invalid club level must stay visible');
  assert(await page.locator('button[form="club-training-form"]').isDisabled(), 'Invalid infrastructure must block its review action');
  await page.locator('#club-tab-finance').click();
  await editClubFinances.click();
  await balanceField.fill('-1');
  assert(!await financeStage.isDisabled(), 'Invalid infrastructure must not disable independent finance edits');
  await balanceField.fill('-685427');
  await page.locator('#club-tab-infrastructure').click();
  await trainingLevel.fill('20');
  assert(!await page.locator('button[form="club-training-form"]').isDisabled(), 'Correcting club level must enable staging');
  await page.locator('#club-youth').fill('19');
  await page.locator('#club-reputation').fill('10000');
  await page.locator('button[form="club-training-form"]').click();
  await page.locator('body > .toast-region .toast').filter({hasText:'Plano de infraestrutura atualizado'}).waitFor();
  await page.locator('#review').click();
  await page.locator('#changes-list .change-item').filter({hasText:'11 → 20'}).waitFor();
  await page.locator('#changes-list .change-item').filter({hasText:'10 → 19'}).waitFor();
  await page.locator('#changes-list .change-item').filter({hasText:'5497 → 10000'}).waitFor();
  const clubReview=page.locator('#changes-list .review-entity').filter({hasText:'Instalações de treino'});
  assert(await clubReview.count()===1&&await clubReview.locator('.change-item').count()===3,'Club edits must appear under one entity');
  await page.locator('#undo').click();
  await page.locator('#changes-list').filter({hasText:'Nenhuma alteração pendente'}).waitFor();
  await page.locator('#nav-clubs').click();
  assert(await page.locator('#club-training').inputValue()==='11' && await page.locator('#club-youth').inputValue()==='10' && await page.locator('#club-reputation').inputValue()==='5497','Undo must restore training, youth and reputation');
  await page.locator('#workspace[aria-busy]').waitFor({state:'detached'});
  await page.evaluate(()=>{window.__holdRosterComparison=true;});
  await page.locator('#club-tab-squad').click();
  const openClubTool=async id=>{
    await page.locator('#club-tools > summary').click();
    await page.locator('#'+id).click();
  };
  const rosterSearch=page.locator('#club-roster-search');
  await rosterSearch.fill('ar zett');
  await rosterSearch.press('ArrowDown');
  assert(await page.locator('#club-roster-player-list [data-club-player="600"]').evaluate(el=>el===document.activeElement),'Elenco search must focus an accentless partial match');
  await page.keyboard.press('Escape');
  assert(await rosterSearch.inputValue()==='', 'Escape must clear the roster filter');
  await openClubTool('compare-club');
  await page.locator('#cancel-save-comparison').click();
  await page.locator('body > .toast-region .toast').filter({hasText:'Comparação cancelada.'}).waitFor();
  assert(await page.locator('#club-comparison').isHidden(),'A late cancelled result must not open the comparison');
  assert(await page.locator('#cancel-save-comparison').isHidden(),'Cancel must be hidden after the worker returns');
  await openClubTool('compare-club');
  const rosterDiff=page.locator('#club-comparison');
  await rosterDiff.getByText('Reforço · UID 601').waitFor();
  await rosterDiff.getByText('Ex-jogador · UID 602').waitFor();
  await rosterDiff.locator('summary').first().click();
  await rosterDiff.getByText('15 → 18',{exact:true}).waitFor();
  await rosterDiff.getByText('Empréstimo',{exact:true}).waitFor();
  await rosterDiff.locator('li').filter({hasText:'Listado para transferência · Disponível para empréstimo'}).waitFor();
  await rosterDiff.locator('#export-club-comparison').click();
  await rosterDiff.locator('.toast').filter({hasText:'comparacao-elenco.json'}).waitFor();
  const rosterJson=await page.evaluate(()=>JSON.parse(window.__testCalls.findLast(call=>call.command==='export_club_comparison').args.json));
  assert(rosterJson.clubUid===200000&&rosterJson.changed[0].contractsAfter[0].weeklyWageGbp===950,'Roster export must preserve club UID and contracts');
  assert(rosterJson.changed[0].contractsAfter[0].transferStatus===7,'Roster JSON must preserve the market code');
  await rosterDiff.locator('#export-club-comparison-csv').click();
  await rosterDiff.locator('.toast').filter({hasText:'comparacao-elenco.csv'}).waitFor();
  const rosterCsv=await page.evaluate(()=>window.__testCalls.findLast(call=>call.command==='export_club_comparison_csv').args.csv);
  assert(rosterCsv.startsWith('\uFEFF')&&rosterCsv.includes('"Entrada";601;"Reforço"')
    &&rosterCsv.includes('"Saída";602;"Ex-jogador"')&&rosterCsv.includes('"passing";15;18')
    &&rosterCsv.includes('"Completo";"Clube de teste";950;')
    &&rosterCsv.includes('"Empréstimo";"Outro clube";200;"Mensal"'),'Roster CSV must include membership, attributes and contract types');
  assert(rosterCsv.includes('"Situação de mercado"')&&rosterCsv.includes('"Listado para transferência · Disponível para empréstimo"'),'Roster CSV must include the market description');
  await rosterDiff.getByRole('button',{name:'Fechar',exact:true}).click();
  await openClubTool('club-open-shirts');
  const swap=page.locator('form').filter({hasText:'Trocar números de camisa'});
  await swap.getByLabel('Primeiro jogador').selectOption({label:'41 · Pär Zetterberg (empréstimo)'});
  await swap.getByLabel('Segundo jogador').selectOption({label:'1 · Quim'});
  await swap.getByRole('button',{name:'Adicionar troca à revisão'}).click();
  await page.locator('body > .toast-region .toast').filter({hasText:'Troca de camisas adicionada à revisão'}).waitFor();
  const swapEdits=await page.evaluate(()=>window.__testCalls.findLast(call=>call.command==='stage_edits').args.edits);
  assert(swapEdits.length===2&&swapEdits[0].uid===600&&swapEdits[0].attribute==='contract.1.squadNumber'
    &&swapEdits[0].value===1&&swapEdits[1].uid===601&&swapEdits[1].attribute==='contract.0.squadNumber'
    &&swapEdits[1].value===41,'Shirt swap must send both exact club contract edits in one IPC call');
  await openClubTool('club-open-contract-batch');
  const wageForm=page.locator('#wage-batch-form');
  await wageForm.locator('[name="contract"][value="1"]').check();
  await wageForm.locator('[name="percent"]').fill('20');
  await wageForm.locator('[name="ceiling"]').fill('600');
  await wageForm.getByRole('button',{name:'Prévia salarial'}).click();
  await wageForm.locator('#wage-batch-preview').filter({hasText:'Empréstimos · £476 → £571'}).waitFor();
  const wageRequest=await page.evaluate(()=>window.__testCalls.findLast(call=>call.command==='preview_wage_batch').args);
  assert(wageRequest.clubUid===200000&&wageRequest.request.contracts[0].index===1&&wageRequest.request.ceiling===600,'Wage preview must use the selected contract and ceiling');
  await wageForm.locator('#stage-wage-batch').click();
  await page.locator('body > .toast-region .toast').filter({hasText:'Lote salarial adicionado'}).waitFor();
  await openClubTool('club-open-contract-batch');
  await wageForm.locator('[name="contract"][value="1"]').check();
  await wageForm.locator('[name="expiry"]').fill('2004-02-29');
  await wageForm.locator('#preview-expiry-batch').click();
  await wageForm.locator('#wage-batch-preview').filter({hasText:'31/10/2003 → 29/02/2004'}).waitFor();
  const expiryRequest=await page.evaluate(()=>window.__testCalls.findLast(call=>call.command==='preview_expiry_batch').args);
  assert(expiryRequest.request.day===59&&expiryRequest.request.contracts[0].index===1,'Expiry preview must use the correct leap-year date and contract');
  await wageForm.locator('#stage-wage-batch').click();
  await page.locator('body > .toast-region .toast').filter({hasText:'Lote de vencimentos adicionado'}).waitFor();
  await openClubTool('club-open-contract-batch');
  await wageForm.locator('[name="contract"][value="0"]').check();
  await wageForm.locator('[name="contract"][value="1"]').check();
  await wageForm.locator('[name="squadStatus"]').selectOption('1');
  await wageForm.locator('#preview-squad-status-batch').click();
  await wageForm.locator('#wage-batch-preview').filter({hasText:'Empréstimo · Importante → Indispensável'}).waitFor();
  await wageForm.getByRole('button',{name:'Retirar Pär Zetterberg · contrato do clube da prévia'}).click();
  await wageForm.locator('#stage-wage-batch').waitFor({state:'hidden'});
  assert(!await wageForm.locator('[name="contract"][value="0"]').isChecked()
    && await wageForm.locator('[name="contract"][value="1"]').isChecked(),'Exclusion must keep the loan selected');
  await wageForm.locator('#preview-squad-status-batch').click();
  await wageForm.locator('#stage-wage-batch').waitFor();
  const squadRequest=await page.evaluate(()=>window.__testCalls.findLast(call=>call.command==='preview_squad_status_batch').args);
  assert(squadRequest.request.status===1&&squadRequest.request.contracts.length===1&&squadRequest.request.contracts[0].index===1,'Squad status must target only the remaining loan');
  await wageForm.locator('#stage-wage-batch').click();
  await page.locator('body > .toast-region .toast').filter({hasText:'Lote de situação no elenco adicionado'}).waitFor();
  const squadSummary=page.locator('#club-squad-summary');
  await squadSummary.locator(':scope > summary').evaluate(summary=>summary.click());
  await squadSummary.getByText('31 ou mais · 1',{exact:true}).evaluate(summary=>summary.click());
  assert(await squadSummary.getByText('Meio-campista · 1',{exact:true}).isVisible(),'Squad must show position counts');
  assert(await squadSummary.getByText('15,0',{exact:true}).isVisible(),'Squad must show localized averages');
  await squadSummary.locator('details[open] [data-squad-player="600"]').click();
  await page.locator('#profile h2').filter({hasText:'Pär Zetterberg'}).waitFor();
  assert(await page.locator('#players-view').isVisible(),'Club member must open player');
  await page.locator('[data-field="passing"]').fill('20');
  await page.locator('#nav-clubs').click();
  await confirmDiscard.getByRole('button',{name:'Voltar'}).click();
  assert(await page.locator('#players-view').isVisible(),'Cancel must retain player draft before club navigation');
  await page.locator('#nav-clubs').click();
  await confirmDiscard.getByRole('button',{name:'Descartar'}).click();
  await page.locator('#clubs-view').waitFor();
  await page.locator('#club-results button').first().waitFor();
  assert(await page.locator('[data-field="passing"]').inputValue()==='15','Discard on club navigation must reset player draft');
  await page.locator('#club-roster-staff').click();
  const staffRanking=page.locator('#club-staff-comparison');
  await staffRanking.locator(':scope > summary').click();
  await staffRanking.locator('#staff-comparison-attribute').selectOption('judgingPlayerAbility');
  assert(await staffRanking.locator('[data-club-ranked-staff]').first().getAttribute('data-club-ranked-staff')==='4950','Staff comparison must rank by selected attribute');
  await staffRanking.locator('#staff-comparison-role').selectOption('Treinador adjunto');
  assert(await staffRanking.locator('[data-club-ranked-staff]').count()===1,'Role filter must retain the selected staff');
  await staffRanking.getByText('Editar comissão em lote',{exact:true}).click();
  const staffBatch=staffRanking.locator('#staff-batch-form');
  await staffBatch.locator('[name="staffUid"][value="4950"]').check();
  await staffBatch.locator('[name="judgingPlayerAbility"]').fill('20');
  await staffBatch.locator('#staff-batch-preset-name').fill('Olheiros');
  await staffBatch.locator('#staff-batch-save-preset').click();
  const staffPresets=await page.evaluate(()=>JSON.parse(localStorage.getItem('cm0304.staff-presets.v1')));
  assert(staffPresets[0].name==='Olheiros'&&staffPresets[0].values.judgingPlayerAbility===20&&!('uids' in staffPresets[0]),'Staff presets must persist values without member selections');
  await staffBatch.locator('[name="judgingPlayerAbility"]').fill('10');
  await staffBatch.locator('#staff-batch-load-preset').click();
  assert(await staffBatch.locator('[name="judgingPlayerAbility"]').inputValue()==='20','Loading a staff preset must restore its target values');
  await staffBatch.getByRole('button',{name:'Prévia da comissão'}).click();
  await staffBatch.locator('#staff-batch-preview').filter({hasText:'12 → 20'}).waitFor();
  const staffBatchRequest=await page.evaluate(()=>window.__testCalls.findLast(call=>call.command==='preview_staff_batch').args);
  assert(staffBatchRequest.uids.join(',')==='4950'&&staffBatchRequest.values.judgingPlayerAbility===20,'Staff batch must use checked members and professional values');
  await staffBatch.locator('#staff-batch-stage').click();
  await page.locator('body > .toast-region .toast').filter({hasText:'Lote da comissão adicionado'}).waitFor();
  const stagedStaffBatch=await page.evaluate(()=>window.__testCalls.findLast(call=>call.command==='stage_staff_batch').args);
  assert(Number.isSafeInteger(staffBatchRequest.previewId)&&stagedStaffBatch.previewId===staffBatchRequest.previewId,'Staging must identify the exact staff preview');
  assert(new Set([wageRequest.previewId,expiryRequest.previewId,squadRequest.previewId,staffBatchRequest.previewId]).size===4,'Each batch preview must have its own identifier');
  await staffRanking.locator('[data-club-ranked-staff="4950"]').click();
  await page.locator('#staff-profile h2').filter({hasText:'Rui Águas'}).waitFor();
  assert(await page.locator('#staff-view').isVisible(),'Club staff must open professional profile');
  await page.locator('#nav-clubs').click();
  await page.locator('#club-tab-finance').click();
  await editClubFinances.click();
  await page.locator('#club-budget-remaining').fill('700000');
  await page.locator('button[form="club-balance-form"]').click();
  await page.locator('body > .toast-region .toast').filter({hasText:'Plano de balanço atualizado'}).waitFor();
  await page.locator('#nav-changes').click();
  const mixedReview=page.locator('#changes-list .review-club').filter({hasText:'Clube de São João'});
  await mixedReview.locator('.review-entity').filter({hasText:'Pär Zetterberg'}).getByText('476 → 571',{exact:true}).waitFor();
  await mixedReview.getByText('31/10/2003 → 29/02/2004',{exact:true}).waitFor();
  await mixedReview.locator('.review-entity').filter({hasText:'Rui Águas'}).getByText('12 → 20',{exact:true}).waitFor();
  await mixedReview.getByText('121595 → 700000',{exact:true}).waitFor();
  await page.locator('#clear').click();
  await page.locator('#nav-clubs').click();
  await page.locator('#open-save').click();
  await page.locator('body > .toast-region .toast').filter({hasText:'Save carregado'}).waitFor();
  await page.locator('#nav-clubs').click();
  await page.locator('#club-results button').first().waitFor();
  assert(await page.locator('#club-search').inputValue()===''&&await page.locator('#club-page-label').innerText()==='1 / 2','New save must reset club search and page');
  assert((await page.locator('#club-profile').innerText()).includes('Escolha um clube'),'New save must clear selected club');
  await page.evaluate(()=>{window.__analysisFixture=true;});
  await page.locator('[data-club-uid="200000"]').click();
  await page.locator('#club-tab-finance').press('End');
  const analysis=page.locator('#club-panel-analysis');
  await analysis.getByRole('heading',{name:'Mapa do elenco'}).waitFor();
  const formation=analysis.getByRole('combobox',{name:'Formação',exact:true});
  const scenario=analysis.getByRole('combobox',{name:'Simulação',exact:true});
  const coverage=analysis.locator('#club-analysis-summary');
  const candidates=analysis.locator('#club-analysis-slot');
  const slot=id=>analysis.locator('[data-analysis-slot="'+id+'"]');
  assert(await formation.inputValue()==='save'&&await formation.locator('option:checked').textContent()==='Tática do save · Diablo',
    'The saved tactic must take precedence over the manual formations');
  assert(await slot('save-3-2').getByText('VOL',{exact:true}).isVisible()
    &&await slot('save-4-2').getByText('MC',{exact:true}).isVisible(),
    'The saved tactic must display separate defensive and central midfield lines');
  assert(await analysis.locator('[data-analysis-slot]').count()===11,'The pitch must show all eleven vacancies');
  await coverage.filter({hasText:'3 de 11 titulares escalados no jogo.'}).waitFor();
  await slot('save-2-4').getByText('Pär Zetterberg',{exact:true}).waitFor();
  await slot('save-2-0').getByText('César',{exact:true}).waitFor();
  await slot('save-2-1').getByText('Sem titular',{exact:true}).waitFor();
  const callsBeforeSavedScenario=await page.evaluate(()=>window.__testCalls.length);
  await scenario.selectOption('absence:601');
  await slot('save-0-2').getByText('Fora na simulação',{exact:true}).waitFor();
  assert(await slot('save-0-2').getAttribute('aria-pressed')==='true','The absence must open its own slot');
  await candidates.getByRole('heading',{name:'Quem pode entrar · 1'}).waitFor();
  await candidates.getByRole('button',{name:/Bruno/}).waitFor();
  assert(!await candidates.getByRole('button',{name:/Edson/}).isVisible(),'A player with a health restriction must appear in the separate group');
  await candidates.getByText('Com lesão ou suspensão registrada (1)',{exact:true}).click();
  await candidates.getByRole('button',{name:/Edson/}).waitFor();
  await slot('save-2-0').getByText('César',{exact:true}).waitFor();
  await scenario.selectOption('loan:602:1');
  await slot('save-2-0').getByText('Fora na simulação',{exact:true}).waitFor();
  await candidates.getByText('Nenhuma opção livre para esta vaga.',{exact:true}).waitFor();
  await slot('save-2-4').getByText('Pär Zetterberg',{exact:true}).waitFor();
  await scenario.selectOption('');
  await slot('save-0-2').getByText('Escalado no jogo',{exact:true}).waitFor();
  assert(await page.evaluate(()=>window.__testCalls.length)===callsBeforeSavedScenario,'The saved lineup simulation must remain local');
  await formation.selectOption('4-4-2');
  await slot('LD').click();
  await candidates.getByText('Sugestão do mapa',{exact:true}).waitFor();
  assert(!await candidates.getByRole('button',{name:/Pär Zetterberg/}).isVisible(),
    'Players in other slots must stay in a collapsed group');
  await candidates.getByText('Jogadores em outras vagas (1)',{exact:true}).click();
  await candidates.getByText(/92,34% de condição.*Lateral esquerdo/).waitFor();
  await candidates.getByText('Lesão registrada · Suspensão registrada',{exact:true}).waitFor();
  await candidates.getByText(/Condição indisponível/).waitFor();
  await slot('GOL').click();
  await candidates.getByRole('button',{name:/Quim.* · 0% de condição/}).waitFor();
  await formation.selectOption('4-2-3-1');
  await coverage.filter({hasText:'4 de 11'}).waitFor();
  await slot('MOE').click();
  await candidates.getByText(/Adaptado.*100% de condição/).waitFor();
  const advanced=candidates.getByText(/Davi.*Posição 16\/20.*Lado 17\/20/);
  assert(!await advanced.isVisible(),'Raw familiarity must start collapsed');
  await candidates.getByText('Detalhes da posição',{exact:true}).click();
  await advanced.waitFor();
  const callsBeforeSimulation=await page.evaluate(()=>window.__testCalls.length);
  const pendingBeforeSimulation=await page.locator('#pending-count').innerText();
  await scenario.selectOption('absence:600');
  await coverage.filter({hasText:'3 de 11 vagas preenchidas com a simulação · 4 antes.'}).waitFor();
  assert(await slot('LE').getAttribute('data-coverage')==='empty','An absence must remove the player from every slot');
  assert(await scenario.locator('optgroup[label="Fim de empréstimo"] option').allTextContents().then(labels=>labels.join())==='César · 19/08/2003',
    'The loan selector must use the exact receiving link and its recorded date');
  await scenario.selectOption('loan:602:1');
  await coverage.filter({hasText:'3 de 11 vagas preenchidas com a simulação · 4 antes.'}).waitFor();
  assert(await slot('LD').getAttribute('data-coverage')==='empty' && await slot('LD').getByText('Opções em outras vagas').isVisible(),
    'One player must not fill both sides after the loan ends');
  await slot('LE').click();
  await candidates.getByRole('button',{name:/Pär Zetterberg/}).waitFor();
  await scenario.selectOption('');
  await coverage.filter({hasText:'4 de 11 vagas preenchidas na distribuição do elenco.'}).waitFor();
  assert(await page.evaluate(()=>window.__testCalls.length)===callsBeforeSimulation
    && await page.locator('#pending-count').innerText()===pendingBeforeSimulation,
    'Running and clearing scenarios must leave the save and review untouched');
  await candidates.getByRole('button',{name:/Pär Zetterberg/}).click();
  await page.locator('#profile h2').getByText('Pär Zetterberg',{exact:true}).waitFor();
  await page.locator('#nav-clubs').click();
  await analysis.waitFor();
  assert(await formation.inputValue()==='4-2-3-1'&&await slot('LE').getAttribute('aria-pressed')==='true',
    'Returning from a player must restore the analysis selection');
  await page.evaluate(()=>{window.__analysisFixture=false;});
  await page.locator('[data-club-uid="200000"]').click();
  await analysis.getByText('Tática do save indisponível para este clube. Escolha uma formação manual.',{exact:true}).waitFor();
  await page.locator('#club-tab-squad').click();
  await page.locator('#club-open-expiries').click();
  const expiryResults=page.locator('#club-contract-results');
  await page.locator('#club-contract-period').selectOption('30');
  assert(await expiryResults.getByRole('button').count()===1,'The 30-day horizon must include its boundary');
  await expiryResults.getByText('1 vínculo · Soma dos salários exibidos: £476/semana',{exact:true}).waitFor();
  await page.locator('#club-contract-period').selectOption('180');
  assert(await expiryResults.getByRole('button').count()===2
    &&await expiryResults.getByText('Quim',{exact:true}).isVisible(),'The 180-day horizon must include a contract beyond 90 days and exclude expired contracts');
  await page.locator('#club-contract-period').selectOption('all');
  await expiryResults.getByText('3 vínculos · Soma dos salários exibidos: £999/semana',{exact:true}).waitFor();
  await page.locator('#club-contract-status').selectOption('2');
  assert(await page.locator('#club-contract-results button').count()===1
    &&await page.locator('#club-contract-results button').getAttribute('data-club-contract')==='1','Squad status must filter the exact contract');
  await page.locator('#club-contract-status').selectOption('all');
  await page.locator('#club-contract-order').selectOption('wage');
  assert(await page.locator('#club-contract-results button').first().getAttribute('data-club-contract')==='0','Contract view must sort by weekly wage');
  await page.locator('#club-contract-period').selectOption('expired');
  assert(await page.locator('#club-contract-results').getByText('Vencido',{exact:true}).isVisible(),'Expired contract must be discoverable');
  await page.locator('#club-contract-period').selectOption('60');
  await page.locator('#club-contract-kind').selectOption('loan');
  await page.locator('[data-club-contract="1"]').click();
  await page.locator('#panel-contracts').waitFor();
  assert(await page.locator('#contract-card-1').evaluate(card=>card===document.activeElement),'Expiry shortcut must focus the loan');
  await page.locator('#nav-clubs').click();
  await expiryResults.getByText('1 vínculo · Soma dos salários exibidos: £476/semana',{exact:true}).waitFor();
  assert(await page.locator('#club-contract-period').inputValue()==='60'
    && await page.locator('#club-contract-kind').inputValue()==='loan'
    && await page.locator('#club-contract-order').inputValue()==='wage','Returning from a contract must restore the club filters');
  await openClubTool('club-open-player-batch');
  const clubBatchSelection=page.locator('#batch-selection-details > summary');
  const clubBatchPlayer=page.locator('#batch-player-list input[name="batch-player"]').first();
  await clubBatchSelection.click();
  await page.getByRole('tab',{name:'Recuperação e ocorrências'}).click();
  await clubBatchPlayer.check();
  await page.locator('#batch-heal-injuries').check();
  await page.locator('#batch-clear-suspensions').check();
  await clubBatchPlayer.uncheck();
  await page.locator('#batch-player-list').getByText(/Lesões: 2 · Suspensões: 1/).waitFor();
  await page.locator('#batch-select-health').click();
  await page.locator('#batch-selection').getByText('Pär Zetterberg · 1 jogador selecionado',{exact:true}).waitFor();
  await page.locator('#batch-health').click();
  await page.getByText(/Lesões: 2 → 0/).waitFor();
  await page.getByText(/Suspensões: 1 → 0/).waitFor();
  const healthRequest=await page.evaluate(()=>window.__testCalls.findLast(call=>call.command==='preview_health_batch').args);
  assert(healthRequest.uids.join(',')==='600'&&healthRequest.healInjuries&&healthRequest.clearSuspensions,'Health preview must target only the selected players and removals');
  await page.locator('#batch-recover').click();
  await page.getByText(/Condição física.*80.*100/).waitFor();
  await page.getByText(/Suspensões: 1 → 0/).waitFor();
  assert(await page.evaluate(()=>window.__testCalls.findLast(call=>call.command==='preview_health_batch').args.recover),'Recovery preview must include the selected removals');
  await page.locator('#batch-clear-suspensions').uncheck();
  assert(await page.locator('#batch-stage').isHidden(),'Changing removals must invalidate the preview');
  await page.getByRole('tab',{name:'Ajustar atributos'}).click();
  await page.locator('#batch-attribute').selectOption('passing');
  await page.locator('#batch-operation').selectOption('add');
  await page.locator('#batch-value').fill('20');
  await page.locator('#batch-queue-adjustment').click();
  await page.locator('#batch-attribute').selectOption('state.morale');
  await page.locator('#batch-operation').selectOption('set');
  await page.locator('#batch-value').fill('19');
  await page.locator('#batch-queue-adjustment').click();
  await page.locator('#batch-adjustment-preview').click();
  await page.locator('#batch-preview-summary').filter({hasText:'1 de 1 jogador com alterações'}).waitFor();
  const adjustments=await page.evaluate(()=>window.__testCalls.findLast(call=>call.command==='preview_player_batch').args.request.operation.value);
  assert(adjustments.length===2&&adjustments[0].attribute==='passing'&&adjustments[1].attribute==='state.morale','Combined preview must submit both adjustments');
  await page.getByRole('button',{name:'Remover ajuste de Moral'}).click();
  assert(await page.locator('#batch-stage').isHidden(),'Removing a queued adjustment must invalidate the preview');
  await page.locator('#batch-attribute').selectOption('passing');
  await page.locator('#batch-operation').selectOption('add');
  await page.locator('#batch-value').fill('20');
  const previewButton=page.locator('#preview-player-adjustment');
  await previewButton.click();
  await page.locator('#batch-preview').filter({hasText:'15 → 20 (limitado)'}).waitFor();
  await clubBatchPlayer.uncheck();
  assert(await page.locator('#batch-stage').isHidden(),'Selection changes must invalidate the preview');
  await clubBatchPlayer.check();
  await previewButton.click();
  await page.locator('#batch-stage').click();
  await page.locator('body > .toast-region .toast').filter({hasText:'Lote adicionado à revisão'}).waitFor();
  await page.locator('#review').click();
  await page.locator('#changes-list').filter({hasText:'15 → 20'}).waitFor();
  const playerClubGroup=page.locator('.review-club').filter({hasText:'Clube de São João'});
  assert(await playerClubGroup.locator('.review-entity').filter({hasText:'Pär Zetterberg'}).count()===1,
    'Review must place the player under the linked club');
  await page.locator('[data-remove-plan-field="passing"]').click();
  await page.locator('#changes-list').filter({hasText:'Nenhuma alteração pendente'}).waitFor();
  await page.locator('#undo').click();
  await page.locator('#changes-list').filter({hasText:'15 → 20'}).waitFor();
  await page.locator('#undo').click();
  await page.locator('#changes-list').filter({hasText:'Nenhuma alteração pendente'}).waitFor();
  await page.locator('#nav-clubs').click();
  await openClubTool('club-open-player-batch');
  await clubBatchSelection.click();
  await clubBatchPlayer.check();
  await page.getByRole('tab',{name:'Copiar atributos'}).click();
  await page.locator('#batch-copy-source').selectOption('600');
  await page.getByRole('button',{name:'Técnicos',exact:true}).click();
  const pass=page.locator('[name="batch-copy-field"][value="passing"]');
  assert(await pass.isChecked(),'Technical group must select passing');
  await page.locator('[name="batch-copy-field"][value="technique"]').uncheck();
  await page.locator('#batch-copy').click();
  await page.locator('#batch-preview').filter({hasText:'Passe: 15 → 15'}).waitFor();
  assert(await page.locator('#batch-stage').isHidden(),'A no-op copy must not be staged');
  await page.getByRole('tab',{name:'Ajustar atributos'}).click();
  await page.locator('#club-batch-form summary').filter({hasText:'Gerenciar presets'}).click();
  await page.locator('#batch-attribute').selectOption('passing');
  await page.locator('#batch-operation').selectOption('floor');
  await page.locator('#batch-value').fill('12');
  await page.locator('#batch-preset-name').fill('Passe mínimo');
  await page.locator('#batch-preset-save').click();
  await page.locator('[data-club-uid="200000"]').click();
  await openClubTool('club-open-player-batch');
  await clubBatchSelection.click();
  await page.locator('#club-batch-form summary').filter({hasText:'Gerenciar presets'}).click();
  await page.locator('#batch-preset').selectOption('Passe mínimo');
  await page.locator('#batch-preset-load').click();
  assert(await page.locator('#batch-attribute').inputValue()==='passing'
    && await page.locator('#batch-operation').inputValue()==='floor'
    && await page.locator('#batch-value').inputValue()==='12'
    && await clubBatchPlayer.isChecked()===false,'Preset must restore settings without selecting players');
  const importField=page.locator('#batch-preset-import');
  await page.locator('#batch-preset-name').fill('Passe alternativo');
  await page.locator('#batch-preset-duplicate').click();
  assert(await page.locator('#batch-preset').inputValue()==='Passe alternativo','Duplicate must select the copy');
  await page.locator('#batch-preset-name').fill('Passe de treino');
  await page.locator('#batch-preset-rename').click();
  assert(await page.locator('#batch-preset').inputValue()==='Passe de treino','Rename must select the new name');
  const importPresets=presets=>importField.setInputFiles({
    name:'presets.json',mimeType:'application/json',buffer:Buffer.from(JSON.stringify({version:1,presets}))
  });
  const valid={name:'Moral alta',attribute:'state.morale',kind:'set',value:20};
  await importPresets([valid,{...valid,name:'Inválido',attribute:'identity'}]);
  await page.locator('body > .toast-region .toast').filter({hasText:'preset inválido'}).waitFor();
  assert(await page.locator('#batch-preset option').filter({hasText:'Moral alta'}).count()===0,'Invalid import must be atomic');
  await importPresets([valid]);
  await page.locator('#batch-preset option').filter({hasText:'Moral alta'}).waitFor({state:'attached'});
  await importPresets([{...valid,value:19}]);
  const confirmPresetImport=page.getByRole('dialog',{name:'Substituir presets?'});
  await confirmPresetImport.getByRole('button',{name:'Voltar'}).click();
  await page.locator('#batch-preset-export').click();
  await page.locator('body > .toast-region .toast').filter({hasText:'Presets exportados'}).waitFor();
  const exported=await page.evaluate(()=>JSON.parse(window.__testCalls.findLast(call=>call.command==='export_batch_presets').args.json));
  for (const name of ['Passe mínimo','Passe de treino']) {
    assert(exported.presets.some(p=>p.name===name&&p.adjustments.some(a=>a.attribute==='passing'&&a.kind==='floor'&&a.value===12)),'Named presets must preserve their settings');
  }
  assert(!exported.presets.some(p=>p.name==='Passe alternativo'),'Rename must remove the old name');
  assert(exported.presets.some(p=>p.name==='Goleiros · Reflexos mínimos'),'Initial presets must be exportable');
  assert(exported.version===2&&exported.presets.some(p=>p.name==='Moral alta'&&p.adjustments[0].value===20),'Export must include validated legacy preset');
  await page.locator('#batch-attribute').selectOption('state.condition');
  await page.locator('#batch-operation').selectOption('set');
  await page.locator('#batch-preset-name').fill('Condição a 70,01%');
  await page.locator('#batch-value').fill('70.011');
  await page.locator('#batch-preset-save').click();
  await page.locator('body > .toast-region .toast').filter({hasText:'valor válido para o preset'}).waitFor();
  await page.locator('#batch-value').fill('70.01');
  await page.locator('#batch-preset-save').click();
  await page.locator('#batch-preset').selectOption('Condição a 70,01%');
  await page.locator('#batch-preset-load').click();
  assert(await page.locator('#batch-value').inputValue()==='70.01','Preset must restore fractional condition');
  await clubBatchPlayer.check();
  await page.getByRole('button',{name:'Gerar prévia',exact:true}).click();
  await page.locator('#batch-preview').filter({hasText:'70,01%'}).waitFor();
  await clubBatchPlayer.uncheck();
  await page.locator('#batch-preset').selectOption('Elenco · Recuperação');
  await page.locator('#batch-preset-load').click();
  assert(await page.locator('#batch-adjustment-list li').count()===4,'Recovery preset must load all four settings');
  assert(!await clubBatchPlayer.isChecked(),'A preset must leave player selection to the user');
  await page.locator('#nav-players').click();
  await page.locator('#player-filters > summary').click();
  const attributeFilters=page.locator('#player-filter-attributes');
  const contractFilters=page.locator('#player-filter-contract');
  const savedFilters=page.locator('#player-filter-saved');
  assert(await attributeFilters.locator('[name="minCa"]').isHidden()
    && await contractFilters.locator('[name="contract"]').isHidden()
    && await savedFilters.locator('#saved-player-search').isHidden(),
    'Advanced and saved filters must start collapsed');
  await page.locator('#search').fill('Anderlecht');
  await page.locator('[name="minAge"]').fill('40');
  await page.locator('[name="maxAge"]').fill('34');
  await page.getByText('O máximo deve ser maior ou igual ao mínimo.', {exact:true}).waitFor();
  assert(await page.getByRole('button',{name:'Aplicar filtros'}).isDisabled(), 'Inverted age range must block applying filters');
  await page.locator('[name="minAge"]').fill('');
  await attributeFilters.locator(':scope > summary').click();
  await page.locator('[name="minCa"]').fill('125');
  await page.locator('[name="minPa"]').fill('140');
  await page.locator('[name="nation"]').fill('Suécia');
  await page.locator('[name="position"]').selectOption('midfielder');
  await page.locator('[name="attribute"]').selectOption('passing');
  await page.getByRole('button',{name:'Aplicar filtros'}).click();
  await attributeFilters.getByText('Escolha um atributo e informe seu valor mínimo.',{exact:true}).waitFor();
  await page.locator('body > .toast-region .toast[data-notice="error"]').getByText('Escolha um atributo e informe seu valor mínimo.').waitFor();
  assert(await attributeFilters.locator('[name="minAttribute"]').evaluate(el=>el===document.activeElement),
    'An incomplete attribute filter must explain the missing value and focus its field');
  await page.locator('[name="minAttribute"]').fill('15');
  await contractFilters.getByText('Pé e vínculo',{exact:true}).click();
  await page.locator('[name="foot"]').selectOption('both');
  await page.locator('#player-filter-form [name="contract"]').selectOption('full');
  await page.locator('[name="order"]').selectOption('pa');
  await page.getByRole('button',{name:'Aplicar filtros'}).click();
  await page.locator('#active-filters').getByText('Idade máxima · 34', {exact:true}).waitFor();
  const expectedFilters={maxAge:34,minCa:125,minPa:140,nation:'Suécia',order:'pa',position:'midfielder',attribute:'passing',minAttribute:15,foot:'both',contract:'full'};
  const readSearch=()=>page.evaluate(()=>window.__testCalls.findLast(call=>call.command==='search_players').args);
  let search=await readSearch();
  assert(search.page===0&&search.query==='Anderlecht'&&Object.entries(expectedFilters).every(([key,value])=>search.filters[key]===value),'Combined filters must be sent before pagination');
  await savedFilters.getByText('Buscas salvas',{exact:true}).click();
  await page.locator('#player-search-name').fill('Meias da Suécia');
  await page.locator('#save-player-search').click();
  await page.getByRole('button',{name:'Limpar filtros'}).click();
  await page.locator('#search').fill('');
  await page.locator('#load-player-search').click();
  search=await readSearch();
  assert(search.page===0&&search.query==='Anderlecht'&&Object.entries(expectedFilters).every(([key,value])=>search.filters[key]===value),'Saved search must restore text, filters and order');
  await page.locator('#batch-filtered').click();
  const filtered=page.getByRole('dialog',{name:'Editar jogadores em lote'});
  const filterSelection=filtered.locator('#batch-selection-details > summary');
  assert(await filtered.locator('#batch-player-list').isHidden(),'The filtered dialog must start with a collapsed selection');
  assert(await filtered.locator('#batch-preset-name').isHidden(),'Preset management must start collapsed');
  await filtered.getByRole('tab',{name:'Ajustar atributos'}).press('ArrowRight');
  assert(await filtered.getByRole('tab',{name:'Recuperação e ocorrências'}).getAttribute('aria-selected')==='true','Arrow keys must change the operation tab');
  await filtered.locator('#batch-recover').click();
  const selectionError=filtered.locator('.toast[data-notice="error"]').getByText('Selecione pelo menos um jogador.',{exact:true});
  await selectionError.waitFor();
  assert(await toastBelowHeader(filtered.locator('.toast')), 'Modal toasts must use the same top-right position');
  assert(await filtered.locator('#batch-selection').evaluate(el=>el===document.activeElement),'Batch selection summary must receive focus after an error');
  const dismissToast=filtered.getByRole('button',{name:'Dispensar notificação'});
  await dismissToast.focus();
  await dismissToast.press('Enter');
  assert(await selectionError.isHidden(),'The modal toast must dismiss by keyboard');
  assert(await filtered.locator('#batch-selection').evaluate(el=>el===document.activeElement),'Dismissing must return focus to the selection');
  await filtered.locator('#batch-recover').click();
  await selectionError.waitFor();
  await filterSelection.click();
  await filtered.getByRole('tab',{name:'Ajustar atributos'}).click();
  await filtered.locator('#batch-all').check();
  assert(await selectionError.isHidden(),'Changing the selection must clear its error');
  await filtered.locator('#batch-next-players').click();
  const last=filtered.locator('[name="batch-player"][value="800050"]');
  assert(await last.isChecked(),'Select all must include players beyond the first page');
  await last.uncheck();
  await filtered.locator('#batch-prev-players').click();
  assert(await filtered.locator('[name="batch-player"]:checked').count()===50,'Selection must survive pagination');
  await filtered.getByText('Grupos salvos',{exact:true}).click();
  await filtered.locator('#batch-group-name').fill('Grupo com exclusão');
  await filtered.locator('#batch-save-group').click();
  await filtered.locator('#filtered-batch-close').click();
  await page.locator('#batch-filtered').click();
  await filterSelection.click();
  await filtered.getByText('Grupos salvos',{exact:true}).click();
  await filtered.locator('#batch-saved-group').selectOption('Grupo com exclusão');
  await filtered.locator('#batch-delete-group').click();
  await page.getByRole('dialog',{name:'Excluir grupo?'}).getByRole('button',{name:'Voltar'}).click();
  assert(await filtered.locator('#batch-saved-group').inputValue()==='Grupo com exclusão',
    'Canceling a nested confirmation must preserve the group and keep the batch open');
  await filtered.locator('#batch-load-group').click();
  assert(await filtered.locator('[name="batch-player"]:checked').count()===50,'Saved group must restore the previous selection');
  await filtered.locator('#batch-attribute').selectOption('passing');
  await filtered.locator('#batch-operation').selectOption('add');
  await filtered.locator('#batch-value').fill('20');
  await filtered.getByRole('button',{name:'Gerar prévia',exact:true}).click();
  await filtered.locator('#batch-preview-summary').filter({hasText:'50 de 50 jogadores com alterações'}).waitFor();
  const selection=await page.evaluate(()=>window.__testCalls.findLast(call=>call.command==='preview_player_batch').args.request.uids);
  assert(selection.length===50&&!selection.includes(800050),'Preview must exclude the unchecked result');
  await filtered.getByRole('tab',{name:'Copiar atributos'}).click();
  assert(await filtered.locator('#batch-stage').isHidden(),'Switching operation must invalidate the previous preview');
  await filtered.getByRole('tab',{name:'Ajustar atributos'}).click();
  assert(await filtered.locator('#batch-value').inputValue()==='20','Switching operation must preserve the adjustment input');
  await filtered.locator('#batch-all').check();
  await filtered.locator('#batch-attribute').selectOption('personality.professionalism');
  await filtered.getByRole('button',{name:'Gerar prévia',exact:true}).click();
  await filtered.locator('#batch-exclude-unavailable').waitFor();
  await filtered.locator('.toast[data-notice="warning"]').getByText(/campos indisponíveis/).waitFor();
  assert(await filtered.locator('#batch-stage').isHidden(),'Unavailable fields must prevent staging');
  assert(await filtered.locator('#batch-preview-rows li').count()===50,'Preview must limit the first page to 50 rows');
  await filtered.locator('#batch-next-preview').click();
  assert(await filtered.locator('#batch-preview-page').textContent()==='2 / 2'
    && await filtered.locator('#batch-preview-rows li').count()===1,
    'Last preview page must retain the remaining result');
  await filtered.locator('#batch-preview-rows').filter({hasText:'Jogador 800050 · Profissionalismo: indisponível'}).waitFor();
  await filtered.locator('#batch-only-unavailable').check();
  assert(await filtered.locator('#batch-preview-rows li').count()===1,'Attention filter must isolate unavailable fields');
  await filtered.locator('[data-batch-exclude="800050"]').click();
  assert(await filtered.locator('#batch-selection').textContent()==='50 jogadores selecionados','Exclusion must preserve all compatible players');
  assert(await filtered.locator('#batch-stage').isHidden(),'Exclusion must invalidate the previous preview');
  await filtered.getByRole('button',{name:'Gerar prévia',exact:true}).click();
  await filtered.locator('#batch-stage').waitFor();
  await filtered.locator('#batch-all').check();
  await filtered.locator('#batch-attribute').selectOption('passing');
  await filtered.getByRole('button',{name:'Gerar prévia',exact:true}).click();
  await filtered.locator('#batch-stage').click();
  await filtered.locator('#filtered-batch-close').click();
  await page.locator('#nav-changes').click();
  const batchReview=page.locator('#changes-list');
  await batchReview.locator('[data-review-page]').filter({hasText:'1 / 2 · 51 entidades'}).waitFor();
  assert(await batchReview.locator('.change-item').count()===50,'Review must render at most 50 entities on page one');
  await batchReview.locator('[data-review-next]').click();
  await batchReview.getByText('Jogador 800050',{exact:true}).waitFor();
  assert(await batchReview.locator('.change-item').count()===1,'Last review page must preserve the remaining entity');
  await batchReview.locator('[data-remove-plan-player="800050"][data-remove-plan-field="passing"]').click();
  await page.locator('#pending-count').filter({hasText:/^50$/}).waitFor();
  await page.locator('#undo').click();
  await batchReview.locator('[data-review-page]').filter({hasText:'1 / 2 · 51 entidades'}).waitFor();
  await page.locator('#clear').click();
  await page.locator('#pending-count').filter({hasText:/^0$/}).waitFor();
  await page.locator('#nav-players').click();
  await openMenu('#profile .player-compare-menu');
  await page.locator('#compare-save').click();
  const comparison=page.locator('#player-comparison');
  await comparison.locator('.comparison-identities').getByText('inicial.cm4 · 19/07/2003',{exact:true}).waitFor();
  await comparison.locator('.comparison-identities').getByText('avancado.cm4 · 16/08/2003',{exact:true}).waitFor();
  const overviewTab=comparison.getByRole('tab',{name:'Visão geral'});
  assert(await overviewTab.getAttribute('aria-selected')==='true'
    && await comparison.locator('#comparison-panel-contracts').isHidden(),'The comparison must open on the overview');
  await comparison.getByText('Diferença: Outro save − Save ativo',{exact:true}).waitFor();
  await overviewTab.press('ArrowRight');
  assert(await comparison.getByRole('tab',{name:'Posições'}).getAttribute('aria-selected')==='true','Arrow keys must navigate the comparison tabs');
  await comparison.getByRole('tab',{name:'Atributos'}).click();
  const passing=await comparison.getByRole('row').filter({hasText:'Passe'}).locator('td').allTextContents();
  assert(passing.join('|')==='15|18|+3','Save comparison must show both attributes and the difference');
  await comparison.getByLabel('Somente diferenças').check();
  await comparison.getByRole('tab',{name:'Posições'}).click();
  await comparison.getByText('Nenhuma diferença nesta seção.',{exact:true}).waitFor();
  await comparison.getByRole('tab',{name:'Contratos'}).click();
  await comparison.locator('.compare-contracts').filter({hasText:'£950/semana'}).waitFor();
  await comparison.getByText(/Presença.*Gol.*Assistência.*Rescisão/).waitFor();
  await comparison.getByText(/Camisa: 99.*Indispensável/).waitFor();
  await comparison.locator('.comparison-export > summary').click();
  await comparison.locator('#export-player-comparison').click();
  await comparison.locator('.toast').filter({hasText:'Comparação exportada'}).waitFor();
  const csv=await page.evaluate(()=>window.__testCalls.findLast(call=>call.command==='export_player_comparison').args.csv);
  assert(csv.includes('"Passe";"15";"18";"+3"'),'Export must preserve the comparison values');
  assert(csv.includes('"Contrato 1 · Presença (£)";"";"0";""'),'CSV must distinguish zero from unavailable terms');
  assert(csv.includes('"Contrato 1 · Rescisão (£)";"";"8000000";""'),'CSV must export the clause amount');
  assert(csv.includes('"Contrato 1 · Camisa";"Sem número";"99";""'),'CSV must export both squad numbers');
  assert(csv.includes('"Contrato 1 · Situação de mercado";"Listado para transferência · Disponível para empréstimo";"Transferência sem situação definida · Clube não deseja emprestar";""'),'CSV must export both mapped market descriptions');
  assert(csv.includes('"Contrato 2 · Situação de mercado";"";"";""'),'CSV must leave unknown market descriptions blank');
  await comparison.locator('.comparison-export > summary').click();
  await comparison.locator('#export-player-comparison-json').click();
  await comparison.locator('.toast').filter({hasText:'comparacao.json'}).waitFor();
  const json=await page.evaluate(()=>JSON.parse(window.__testCalls.findLast(call=>call.command==='export_player_comparison_json').args.json));
  assert(json.version===1&&json.left.save.name==='inicial.cm4'&&json.right.save.name==='avancado.cm4','JSON must identify both saves');
  assert(json.right.contracts.some(contract=>contract.weeklyWageGbp===950),'JSON must preserve weekly wages');
  const terms=json.right.contracts[0];
  assert(terms.squadNumber===99&&terms.squadStatus===1,'JSON must preserve squad number and status');
  assert(json.left.contracts[0].transferStatus===7&&terms.transferStatus===68,'JSON must preserve both market codes');
  assert(json.left.contracts[0].marketStatus==='Listado para transferência · Disponível para empréstimo'
    && terms.marketStatus==='Transferência sem situação definida · Clube não deseja emprestar','JSON must export both mapped market descriptions');
  assert(json.right.contracts[1].transferStatus===255&&json.right.contracts[1].marketStatus===null,'JSON must preserve unknown codes without guessing the description');
  assert(terms.appearanceBonusGbp===0&&terms.goalBonusGbp===600&&terms.assistBonusGbp===300&&terms.releaseClauseGbp===8000000,'JSON must preserve all mapped terms, including zero');
  assert(json.left.contracts[0].releaseClauseGbp===null,'Unmapped terms must remain null');
  assert(json.fields.some(field=>field.key==='passing'&&field.left===15&&field.right===18&&field.difference===3),'JSON must preserve numeric differences');
  assert(await page.locator('#file-name').innerText()==='controle_ui.cm4','Comparison must keep the active session');
  await comparison.getByRole('button',{name:'Fechar',exact:true}).click();
  await page.waitForFunction(()=>document.querySelector('.player-compare-menu > summary')===document.activeElement);
  const originalViewport=page.viewportSize();
  await page.setViewportSize({width:960,height:650});
  await page.locator('#player-columns summary').click();
  await page.locator('#player-column-options').getByLabel('UID', {exact:true}).check();
  await page.locator('#player-column-options').getByLabel('PA', {exact:true}).uncheck();
  await page.locator('.player-result-header').getByText('UID', {exact:true}).waitFor();
  assert(await page.locator('.player-result-header').getByText('PA', {exact:true}).count()===0,'Hidden column must leave the header');
  await page.locator('#results [data-uid="600"]').click();
  await page.locator('#player-columns summary').click();
  await page.setViewportSize(originalViewport);
  await page.evaluate(()=>localStorage.setItem('cm0304.watchlist.v1','[601]'));
  await page.reload();
  await page.locator('#open-save').click();
  await page.locator('#results [data-uid="600"]').click();
  await openMenu('#watchlist-tools');
  assert(await page.locator('#watch-count').innerText()==='(1)','Legacy list must migrate');
  await page.locator('#watchlist-name').fill('Reforços');
  await page.getByRole('button',{name:'Criar lista'}).click();
  assert(await page.locator('#watch-count').innerText()==='(0)','New list must start empty');
  await page.locator('#watch-player').getByText('Adicionar à observação',{exact:true}).click();
  await page.locator('#watchlist-select').selectOption('Observação');
  await page.locator('#watch-player').getByText('Adicionar à observação',{exact:true}).waitFor();
  await page.locator('#watchlist-select').selectOption('Reforços');
  await page.reload();
  assert(await page.locator('#watchlist-select').inputValue()==='Reforços','Active list must survive reload');
  await page.locator('#nav-changes').click();
  await page.locator('#history-heading').waitFor();
  assert(!await page.locator('#active-save-review').isVisible(),'History without an active save must hide editing controls');
  const startupCopy=page.locator('.saved-copy-entry').filter({hasText:'copia_teste.cm4'});
  await startupCopy.getByRole('button',{name:'Ver relatório'}).click();
  await startupCopy.locator('[id^="archived-report-"] [data-remove-plan-player]').waitFor({state:'detached'});
  await startupCopy.locator('[id^="archived-report-"][data-loaded="true"]').waitFor();
  await startupCopy.getByRole('button',{name:'Exportar relatório JSON'}).click();
  await page.locator('body > .toast-region .toast').filter({hasText:'Relatório de gravação exportado: cm0304-gravacao.json'}).waitFor();
  const exportedReport=await page.evaluate(()=>JSON.parse(window.__testCalls.findLast(call=>call.command==='export_saved_report').args.json));
  assert(exportedReport.sourceSha256==='a'.repeat(64)&&exportedReport.copySha256==='b'.repeat(64),
    'Export must contain the exact archived source and copy hashes');
  await page.locator('#saved-copy-history > summary').click();
  await page.locator('#back-to-start').click();
  await page.locator('#empty').getByRole('heading',{name:'Abra uma carreira'}).waitFor();
  await page.locator('#open-save').click();
  await page.locator('#results [data-uid="600"]').waitFor();
  await page.locator('.player-result-header').getByText('UID', {exact:true}).waitFor();
  assert(await page.locator('.player-result-header').getByText('PA', {exact:true}).count()===0,'Column preferences must survive reload');
  await openMenu('#watchlist-tools');
  await page.locator('#only-watchlist').check();
  await page.waitForFunction(()=>window.__testCalls.some(call=>call.command==='search_players'&&call.args.filters?.watchUids?.includes(600)));
  await page.locator('#results [data-uid="600"]').click();
  await page.locator('#watch-player').getByText('Remover da observação',{exact:true}).waitFor();
  await page.evaluate(()=>{window.__missingComparisonPlayer=true;});
  await openMenu('#profile .player-compare-menu');
  await page.locator('#compare-save').click();
  await page.locator('body > .toast-region .toast[data-notice="error"]').getByText('Pär Zetterberg (UID 600) não foi encontrado em outra-carreira.cm4.',{exact:true}).waitFor();
  assert(await page.locator('#player-comparison').isHidden(),'Missing UID must leave the comparison closed');
  assert(await page.locator('#file-name').innerText()==='controle_ui.cm4','Missing UID must preserve the open save');
  await page.evaluate(()=>{window.__holdPlayerComparison=true;});
  await openMenu('#profile .player-compare-menu');
  await page.locator('#compare-save').click();
  await page.waitForFunction(()=>typeof window.__resolvePlayerComparison==='function');
  await page.locator('#cancel-save-comparison').click();
  await page.locator('body > .toast-region .toast').getByText('Comparação cancelada.',{exact:true}).waitFor();
  assert(await page.locator('#player-comparison').isHidden(),'Canceled comparison must discard a late result');
  await openMenu('#profile .player-compare-menu');
  await page.locator('#compare-save').click();
  await page.getByText('avancado.cm4',{exact:false}).first().waitFor();
  assert(await page.evaluate(()=>window.__testCalls.findLast(call=>call.command==='compare_player_save').args.uid)===600,
    'Comparison after reopening must use the watched UID');
  const comparisonDialog=page.locator('#player-comparison');
  await comparisonDialog.getByRole('button',{name:'Fechar',exact:true}).click();
  await page.locator('#watch-player').click();
  await page.locator('#results [data-uid="600"]').waitFor({state:'detached'});
  await page.locator('#only-watchlist').uncheck();
  await openMenu('#watchlist-tools');
  const listSelect=page.getByLabel('Lista ativa',{exact:true});
  await page.getByRole('button',{name:'Excluir lista'}).click();
  const confirmDelete=page.getByRole('dialog',{name:'Excluir lista de observação?'});
  await confirmDelete.getByRole('button',{name:'Voltar'}).click();
  assert(await listSelect.inputValue()==='Reforços','Canceling deletion must preserve the active list');
  await page.getByRole('button',{name:'Excluir lista'}).click();
  await confirmDelete.getByRole('button',{name:'Excluir',exact:true}).click();
  await listSelect.locator('option').filter({hasText:'Reforços'}).waitFor({state:'detached'});
  assert(await listSelect.inputValue()==='Observação','Deletion must select the default list');
  assert(await listSelect.locator('option').count()===1,'Only the default list must remain');
  assert(await page.locator('#watch-count').innerText()==='(1)','Deleting a list must preserve the legacy list');
  assert(await page.getByRole('button',{name:'Excluir lista'}).isDisabled(),'The default list must be protected');
  await page.locator('#results [data-uid="600"]').click();
  const tabs=page.getByRole('tablist',{name:'Perfil do jogador'});
  await tabs.waitFor();
  assert(await page.getByRole('button',{name:'Excluir lista'}).isDisabled(),'Loading a profile must preserve the default list protection');
  await tabs.getByRole('tab',{name:'Atributos',exact:true}).focus();
  await page.keyboard.press('End');
  const contractsTab=tabs.getByRole('tab',{name:'Contratos',exact:true});
  assert(await contractsTab.getAttribute('aria-selected')==='true','End must activate the last profile tab');
  assert(await contractsTab.evaluate(tab=>tab===document.activeElement),'The last tab must receive keyboard focus');
  await page.keyboard.press('ArrowRight');
  assert(await tabs.getByRole('tab',{name:'Atributos',exact:true}).getAttribute('aria-selected')==='true','Right Arrow must wrap to the first profile tab');
  await page.locator('#nav-changes').click();
  await page.locator('#saved-copy-history > summary').click();
  await page.locator('.saved-copy-entry').filter({hasText:'copia_teste.cm4'}).getByText('a'.repeat(64),{exact:true}).waitFor();
  const archivedCopy=page.locator('.saved-copy-entry').filter({hasText:'copia_teste.cm4'});
  await archivedCopy.getByRole('button',{name:'Ver relatório'}).click();
  await archivedCopy.locator('.change-item').filter({hasText:'-685427 → 2500000'}).waitFor();
  assert(await archivedCopy.locator('[data-remove-plan-player]').count()===0,'Archived report must be read-only');
  await archivedCopy.getByRole('button',{name:'Abrir cópia'}).click();
  assert(await page.evaluate(()=>window.__testCalls.findLast(call=>call.command==='open_save')?.args?.recentPath)==='C:/teste/copia_teste.cm4','History must reopen the saved copy');
  await page.locator('#nav-changes').click();
  await archivedCopy.getByRole('button',{name:'Abrir origem'}).click();
  assert(await page.evaluate(()=>window.__testCalls.findLast(call=>call.command==='open_save')?.args?.recentPath)==='controle_ui.cm4','History must reopen the recorded source');
  await page.locator('#search').fill('consulta lenta');
  assert(await page.locator('#batch-filtered').isDisabled(),'A new query must disable batch actions during debounce');
  await page.waitForFunction(()=>typeof window.__finishSlowSearch==='function');
  assert(await page.locator('#export-filtered-players-json').isDisabled(),'Pending search must disable exports');
  await page.locator('#cancel-player-search').click();
  await page.evaluate(()=>window.__finishSlowSearch());
  assert(await page.locator('#result-count').innerText()==='Busca cancelada','Late response must not replace canceled state');
  await page.locator('#search').fill('nova consulta');
  await page.locator('#results [data-uid="600"]').waitFor();
  await page.locator('#search').fill('uid:abc');
  await page.locator('#player-search-error').filter({hasText:'UID de jogador inválido'}).waitFor();
  assert(await page.locator('#search').getAttribute('aria-invalid')==='true'
    && await page.locator('#search').getAttribute('aria-describedby')==='player-search-error',
  'Invalid UID must explain the problem next to the search field');
  assert(!await page.locator('#results').getByText('Nenhum jogador encontrado.').count(),
    'A failed query must not look like an empty result');
  assert(await page.locator('#results [data-uid]').count()===0,'Invalid UID must remove stale search results');
  for(const id of ['batch-filtered','export-filtered-players','export-filtered-players-json']){
    assert(await page.locator('#'+id).isDisabled(),'Invalid UID must disable actions on results');
  }
  await page.locator('#search').fill('nova consulta');
  await page.locator('#results [data-uid="600"]').waitFor();
  await page.locator('#batch-filtered:enabled').waitFor();
  await page.locator('#player-filters > summary').click();
  await page.locator('#player-filter-form [name="order"]').selectOption('ca');
  await page.locator('#player-filter-form button[type="submit"]').click();
  await page.locator('#result-count').getByText('1 resultado',{exact:true}).waitFor();
  await page.reload();
  await page.locator('#open-save').click();
  await page.locator('#results [data-uid="600"]').waitFor();
  const restored=await page.evaluate(()=>window.__testCalls.findLast(call=>call.command==='search_players').args);
  assert(restored.query==='nova consulta'&&restored.filters.order==='ca','Last query and ordering must be restored after reload');

  await page.setViewportSize({width:768,height:520});
  const compactPlayer=page.locator('#profile');
  assert(await compactPlayer.isHidden(),'Compact layout must start with the results list');
  await page.locator('#search').press('Enter');
  await compactPlayer.getByRole('heading',{name:'Pär Zetterberg'}).waitFor();
  const profileFacts=compactPlayer.locator('.player-profile-facts');
  assert(await profileFacts.locator('dd').allTextContents().then(fields=>fields.join('|'))==='RSC Anderlecht|Meio-campista|Suécia|32 anos',
    'The compact header must show club, strong positions, nationality and age at the save date');
  assert(await page.locator('#players-view .roster').isHidden(),'Opening a compact profile must free its list space');
  const inViewport = locator => locator.evaluate(el=>{
    const {left,right,top,bottom}=el.getBoundingClientRect();
    return left>=0&&right<=innerWidth&&top>=0&&bottom<=innerHeight;
  });
  assert(await inViewport(page.locator('#stage-player')),'Player staging must fit the compact viewport');
  assert(await compactPlayer.locator('[data-field="passing"]').evaluate(field=>{
    const {left,top,width,height}=field.getBoundingClientRect();
    return field===document.elementFromPoint(left+width/2,top+height/2);
  }),'Compact profile must show an editable field above the sticky actions');
  const profilePresets=compactPlayer.getByRole('button',{name:'Presets de edição'});
  await profilePresets.click();
  const presetDialog=page.getByRole('dialog',{name:'Editar jogadores em lote'});
  await presetDialog.getByText('Pär Zetterberg · 1 jogador selecionado',{exact:true}).waitFor();
  assert(await presetDialog.locator('#batch-all').isHidden()&&await presetDialog.locator('#batch-player-page').isHidden(),
    'Individual preset entry must omit select all and pagination');
  const footer=presetDialog.locator('.batch-footer');
  await presetDialog.getByRole('tab',{name:'Copiar atributos'}).click();
  await presetDialog.getByText('Alterar seleção',{exact:true}).click();
  await presetDialog.getByText('Grupos salvos',{exact:true}).click();
  const body=presetDialog.locator('.batch-body');
  const beforeScroll=await footer.boundingBox();
  const scrolled=await body.evaluate(el=>{el.scrollTop=el.scrollHeight;return el.scrollTop;});
  const afterScroll=await footer.boundingBox();
  assert(scrolled>0&&beforeScroll.y===afterScroll.y&&await inViewport(footer),
    'The compact modal footer must remain visible while the operations scroll');
  await presetDialog.getByRole('tab',{name:'Ajustar atributos'}).click();
  await presetDialog.locator('#batch-preset').selectOption('Dois pés');
  await footer.getByRole('button',{name:'Prévia do preset'}).click();
  await presetDialog.locator('#batch-preview-rows li').last().waitFor();
  assert(await inViewport(footer.getByRole('button',{name:'Adicionar lote à revisão'})),
    'A usable preview must expose the staging action in the fixed footer');
  const presetRequest=await page.evaluate(()=>window.__testCalls.findLast(call=>call.command==='preview_player_batch').args.request);
  assert(presetRequest.uids.join() === '600'
    && presetRequest.operation.value.map(entry=>entry.attribute).join() === 'leftFoot,rightFoot',
    'The individual preset must preview both feet for the open player only');
  await presetDialog.getByRole('button',{name:'Fechar',exact:true}).click();
  assert(await profilePresets.evaluate(el=>el===document.activeElement),'Closing presets must return focus to the profile action');
  await compactPlayer.getByRole('button',{name:'Voltar à lista'}).click();
  assert(await page.locator('#results [data-uid="600"]').evaluate(el=>el===document.activeElement),
    'Returning to a compact list must focus the selected row');
  await page.locator('#nav-clubs').click();
  await page.locator('[data-club-uid="200000"]').click();
  await page.locator('#club-profile').getByRole('button',{name:'Voltar à lista'}).waitFor();
  assert(await financeForm.isHidden(),'The compact finance editor must also start collapsed');
  await editClubFinances.click();
  assert((await financeEditor.boundingBox()).y>(await movement.boundingBox()).y,
    'The compact finance editor must open below the overview and movement');
  assert(await inViewport(balanceField),'Opening finance editing must bring the first field into the compact viewport');
  await financeStage.scrollIntoViewIfNeeded();
  assert(await inViewport(financeStage),'Club staging must fit the compact viewport');
  await page.locator('#nav-staff').click();
  await page.locator('[data-staff-uid="4950"]').click();
  await page.locator('#staff-profile').getByRole('button',{name:'Voltar à lista'}).waitFor();
  assert(await inViewport(page.locator('#staff-stage')),'Staff staging must fit the compact viewport');
  await page.locator('#staff-physiotherapy').fill('8');
  const compactStaff=page.locator('#staff-profile');
  await compactStaff.evaluate(el=>{el.scrollTop=180;});
  await page.locator('#staff-stage').evaluate(el=>el.addEventListener('click',()=>{
    window.__staffStageScroll=el.closest('.profile').scrollTop;
  },{once:true,capture:true}));
  await page.locator('#staff-stage').click();
  await appToast.filter({hasText:'Atributos de staff adicionados à revisão'}).waitFor();
  assert(await toastBelowHeader(appToast), 'The toast must stay below the header in compact windows');
  await compactStaff.locator('.field-edit-status').getByText('Na revisão',{exact:true}).waitFor();
  const [beforeStaging,afterStaging]=await compactStaff.evaluate(el=>[window.__staffStageScroll,el.scrollTop]);
  assert(beforeStaging>0&&Math.abs(afterStaging-beforeStaging)<24,
    'Staging staff changes must keep the profile scrolled near the edited fields ('+beforeStaging+' → '+afterStaging+')');
  await page.locator('#review:enabled').click();
  await page.locator('#save-copy:enabled').waitFor();
  assert(await inViewport(page.locator('#save-copy')),'Saving must fit the compact viewport');
  await page.locator('#save-copy').click();
  await page.locator('#saved-path').filter({hasText:'copia_teste.cm4'}).waitFor();
  await page.evaluate(()=>{window.__openAnother=true;});
  await page.locator('#open-save').click();
  await page.getByRole('dialog',{name:'Abrir outro save?'}).getByRole('button',{name:'Continuar'}).click();
  await page.locator('#file-name').getByText('segunda.cm4',{exact:true}).waitFor();
  await page.locator('#nav-changes').click();
  await page.locator('#changes-list').getByText('Nenhuma alteração pendente.').waitFor();
  assert(await page.locator('#saved-path').textContent()==='','Switching careers must clear the last copy confirmation');
  await page.locator('#saved-copy-history > summary').click();
  await page.locator('.saved-copy-entry').filter({hasText:'copia_teste.cm4'}).getByText('controle_ui.cm4',{exact:true}).waitFor();
  await page.setViewportSize(originalViewport);
  await page.evaluate(()=>{window.__managedFixture=1;});
  await page.locator('#open-save').click();
  await page.locator('#my-club-title').getByText('Clube de São João',{exact:true}).waitFor();
  assert(await page.locator('#nav-my-club').getAttribute('aria-current')==='page','A managed club must be the initial view');
  assert(await page.locator('#my-club-select').isHidden(),'One controlled club needs no picker');
  const cashShortcut=page.locator('[data-my-club-action=cash]');
  const emptyShortcut=page.locator('[data-my-club-action=suspensions]');
  assert(await emptyShortcut.getAttribute('aria-disabled')==='true','An action without targets must be unavailable');
  await cashShortcut.click();
  await cashShortcut.getByText(/Na revisão.*99\.314\.573/).waitFor();
  assert(await page.locator('#my-club-view').isVisible(),'Staging stays on the quick-action panel');
  await page.locator('#my-club-review').click();
  await page.locator('#save-copy:enabled').waitFor();
  await page.locator('#nav-my-club').click();
  await page.locator('#my-club-undo').click();
  await cashShortcut.getByText(/Saldo.*685\.427/).waitFor();
  const transferShortcut=page.locator('[data-my-club-action=transfers]');
  await transferShortcut.getByText('Definir £200 mi para contratar',{exact:true}).click();
  await transferShortcut.getByText(/Na revisão.*200\.000\.000/).waitFor();
  assert(await transferShortcut.getAttribute('aria-disabled')==='true','The preset must stop at £200 million');
  await page.locator('#my-club-undo').click();
  await transferShortcut.getByText(/Verba restante.*121\.595/).waitFor();
  await page.evaluate(()=>{window.__managedFixture=2;});
  await page.locator('#open-save').click();
  const managedPicker=page.locator('#my-club-select');
  await managedPicker.waitFor();
  assert(await managedPicker.inputValue()===''&&await cashShortcut.isHidden(),'A new save with two managed clubs must request a choice before staging');
  await managedPicker.selectOption('200001');
  await page.locator('#my-club-title').getByText('Clube de teste 1',{exact:true}).waitFor();
  await page.locator('#nav-players').click();
  await page.locator('#results [data-uid="600"]').waitFor();
  await checkLanguageLayouts(page, assert);
  assert(errors.length===0,'Frontend errors: '+errors.join(', '));
  const result = { passed: true, checks: ['startup update and active-save deferral', 'pagination', 'accentless search', 'role search', '11 staff controls', 'staff staging and review', 'staff undo/redo/clear', 'staff draft cancel/discard', 'dual-role navigation', 'player draft cancel/discard', 'save session reset', 'staff header policy/payload/projection/review/undo', 'staff personality staging/review/undo', 'club pagination/search', 'club player/staff navigation', 'club draft guard', 'club session reset', 'club finance and facilities', 'club balance review/history/save', 'session plan export, rejection and restore', 'club infrastructure and reputation atomic review and undo', 'club transfer budget field count', 'club expiry opens exact loan', 'shirt swap stages two exact contracts atomically', 'market status staging and restoration', 'mixed review keeps contract, staff and club budget edits', 'club batch preview invalidation and review', 'club copy group and field exclusions', 'club batch preset persistence', 'preset import validation and export', 'combined player filters and saved search', 'filtered batch cross-page selection', 'save comparison contracts and CSV', 'column and watchlist persistence across reload', 'watchlist deletion and profile keyboard navigation', 'toast types, timeout, modal dismissal and focus', 'review retry gates copy and variant', 'copy confirmation scoped to career', 'compact profiles and keyboard focus', 'club finance dashboard and responsive disclosure'], frontendErrors: errors };
  result.checks.push('club formation coverage and read-only scenarios');
  result.checks.push('managed-club opening, one-click staging, review, undo and multiple managers');
  result.checks.push('theme switching, persistence and active career');
  result.checks.push('integrated window controls and guarded close');
  result.checks.push('language preference, draft protection and bilingual layouts');
  await page.evaluate(value => { window.__staffUiResult = value; }, result);
  return result;
}

async function checkLanguageLayouts(page, assert) {
  const language = page.locator('#language-select');
  const assertReadable = async (screen, width) => {
    const clipped = await page.evaluate(() => {
      const modal = document.querySelector('dialog[open]');
      const root = modal ?? document.body;
      const walker = document.createTreeWalker(root, NodeFilter.SHOW_TEXT);
      const range = document.createRange();
      const labels = new Set();
      while (walker.nextNode()) {
        const node = walker.currentNode;
        const text = node.textContent.trim();
        const element = node.parentElement;
        const closedDetails = element.closest('details:not([open])');
        if (!text || element.closest('script, select, pre, .toast-announcement, .player-fact dt, [hidden]') ||
            (closedDetails && !closedDetails.querySelector(':scope > summary')?.contains(element)) ||
            !element.getClientRects().length || getComputedStyle(element).visibility !== 'visible') continue;
        range.selectNodeContents(node);
        const rects = [...range.getClientRects()];
        for (let ancestor = element; ancestor; ancestor = ancestor.parentElement) {
          const {overflowX} = getComputedStyle(ancestor);
          if (overflowX === 'auto' || overflowX === 'scroll') break;
          if (overflowX !== 'hidden' && overflowX !== 'clip') continue;
          const bounds = ancestor.getBoundingClientRect();
          if (ancestor.clientWidth && rects.some(rect => rect.width > 0 &&
              (rect.right > bounds.left + ancestor.clientLeft + ancestor.clientWidth + 2 ||
               rect.left < bounds.left + ancestor.clientLeft - 2))) {
            labels.add(text);
            break;
          }
        }
      }
      return [...labels];
    });
    assert(clipped.length === 0, screen + ' at ' + width + 'px clips: ' + clipped.join(', '));
  };
  await page.locator('#results [data-uid="600"]').click();
  const passing = page.locator('#profile [data-field="passing"]');
  await passing.fill('18');
  await language.selectOption('en-GB');
  await page.getByText('Adicione os rascunhos à revisão antes de trocar o idioma.').first().waitFor();
  assert(await passing.inputValue() === '18', 'Changing language must preserve an open draft');
  await passing.fill('15');
  await page.locator('#nav-my-club').click();
  await page.locator('#my-club-select').selectOption('200000');
  await page.locator('#my-club-title').getByText('Clube de São João',{exact:true}).waitFor();
  await page.locator('[data-my-club-action="cash"]').click();
  await page.locator('#pending-count').filter({hasText:/^1$/}).waitFor();

  const languages = [
    {locale:'en-GB', heading:'Save editor', passing:'Passing', assistant:'Assistant manager',
      finances:'Finances', analysis:'Analysis', contract:'Contract 1 · Full contract', review:'No pending changes.',
      batch:'Batch edit players', preset:'Edit presets', comparison:'Player comparison', history:'Saved copy history', firstTeam:'First team'},
    {locale:'pt-BR', heading:'Editor de save', passing:'Passe', assistant:'Treinador adjunto',
      finances:'Finanças', analysis:'Análise', contract:'Contrato 1 · Completo', review:'Nenhuma alteração pendente.',
      batch:'Editar jogadores em lote', preset:'Presets de edição', comparison:'Comparação de jogadores', history:'Histórico de cópias gravadas', firstTeam:'Equipe principal'},
  ];
  for (const copy of languages) {
    await language.selectOption(copy.locale);
    const clubPicker = page.locator('html[lang="'+copy.locale+'"] #my-club-select');
    await clubPicker.selectOption('200000');
    await page.locator('#my-club-title').getByText('Clube de São João',{exact:true}).waitFor();
    assert(await language.inputValue() === copy.locale, 'Language must persist on reload');
    await page.locator('#nav-changes').click();
    if (copy.locale === 'en-GB') {
      await page.locator('#pending-count').filter({hasText:/^1$/}).waitFor();
      await page.locator('#undo').click();
    }
    await page.locator('#redo:enabled').waitFor();
    await page.locator('#pending-count').filter({hasText:/^0$/}).waitFor();
    await page.getByRole('heading', {name:copy.heading, exact:true}).waitFor();
    for (const width of [1280, 960, 760]) {
      const screen = label => assertReadable(copy.locale + ' ' + label, width);
      await page.setViewportSize({width,height:width===1280?860:650});
      assert(await page.locator('.editor-header').evaluate(header=>{
        const center = element => {
          const {top,height}=element.getBoundingClientRect();
          return top+height/2;
        };
        return Math.abs(center(header.querySelector('.window-controls'))-
          center(header.querySelector('.editor-heading'))) < 2;
      }), 'Window controls must stay in the title row at ' + width + 'px');
      await page.locator('#nav-my-club').click();
      await page.getByRole('heading',{name:copy.firstTeam,exact:true}).waitFor();
      await screen('My club');
      await page.locator('#nav-players').click();
      await page.locator('#results [data-uid="600"]').waitFor();
      await screen('Players');
      const exportMenu = page.locator('#players-view .export-menu > summary');
      await exportMenu.click();
      await screen('Results export menu');
      await exportMenu.click();
      await page.locator('#results [data-uid="600"]').click();
      await page.locator('[aria-controls="panel-attributes"]').click();
      await page.locator('#profile').getByLabel(copy.passing,{exact:true}).waitFor();
      await screen('Player attributes');
      await page.locator('[aria-controls="panel-contracts"]').click();
      await page.getByText(copy.contract,{exact:true}).waitFor();
      await screen('Player contracts');
      for (const tab of await page.locator('#profile [role="tab"]').all()) {
        const label = await tab.innerText();
        await tab.click();
        await screen('Player · ' + label);
      }
      await page.getByRole('button',{name:copy.preset,exact:true}).click();
      await page.getByRole('dialog',{name:copy.batch}).waitFor();
      await screen('Player batch');
      await page.keyboard.press('Escape');
      await page.locator('#profile .player-compare-menu > summary').click();
      await page.locator('#compare-save').click();
      await page.getByRole('dialog',{name:copy.comparison}).waitFor();
      await screen('Player comparison');
      await page.keyboard.press('Escape');
      await page.locator('#nav-staff').click();
      await page.locator('#staff-results [data-staff-uid="4950"]').click();
      await page.locator('#staff-profile').getByText(copy.assistant,{exact:false}).first().waitFor();
      await screen('Staff profile');
      for (const tab of await page.locator('#staff-profile [role="tab"]').all()) {
        const label = await tab.innerText();
        await tab.click();
        await screen('Staff · ' + label);
      }
      await page.locator('#nav-clubs').click();
      await page.locator('#club-results [data-club-uid="200000"]').click();
      await page.getByRole('tab',{name:copy.finances,exact:true}).waitFor();
      await screen('Club finances');
      await page.getByRole('tab',{name:copy.analysis,exact:true}).click();
      await screen('Squad analysis');
      for (const tab of await page.locator('#club-profile [role="tab"][id^="club-tab-"]').all()) {
        const label = await tab.innerText();
        await tab.click();
        await screen('Club · ' + label);
      }
      await page.locator('#nav-changes').click();
      await page.locator('#changes-list').getByText(copy.review).waitFor();
      await page.locator('#saved-copy-history > summary').getByText(copy.history).click();
      await screen('Review and copy history');
      await page.locator('#saved-copy-history > summary').click();
      await page.locator('#nav-diagnostics').click();
      await screen('Diagnostics');
    }
  }
  await page.reload();
  await page.locator('#empty').waitFor();
  await assertReadable('pt-BR welcome', 760);
  await language.selectOption('en-GB');
  await page.getByRole('heading',{name:'Open a career',exact:true}).waitFor();
  await assertReadable('en-GB welcome', 760);
}
