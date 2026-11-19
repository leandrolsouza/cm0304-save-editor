interface Edit {uid:number;attribute:string;value:number;policy?:'preserveValues'|'preserveRelative'}
interface Pending {edits:Edit[];healInjuries:number[];clearSuspensions:number[];undo:boolean;redo:boolean}
interface Field {value:number;min:number;max:number;width:number;offset:number}
interface Player {uid:number;name:string;firstName:string;surname:string;club:string|null;nation:string|null;currentAbility:number;potentialAbility:number;attributes:Record<string,number>;rawAttributes:Record<string,number>;fields:Record<string,Field>}
interface ContractView {index:number;kind:number;kindLabel:string;entityType:number;entity:string|null;started:{year:number;day:number};monthly:boolean}
interface Detail {player:Player;effective:Player;age:number|null;contracts:ContractView[];editable:string[];pending:Edit[];nations:string[];injuries:{name:string;recordedOn:{year:number;day:number}|null}[];healPending:boolean;suspensionCount:number;suspensionPending:boolean}
interface Change {uid:number;name:string;attribute:string;before:number;after:number;rawBefore:number;rawAfter:number;derived:boolean;context:string|null;displayBefore:string|null;displayAfter:string|null}
interface Report {changes:Change[];fieldChangedBytes:number;removedBytes:number;removals:{uid:number;name:string;kind:'injuries'|'suspensions';count:number;removedBytes:number}[];sourceSha256:string;copySha256:string}
interface Page {total:number;rows:{uid:number;name:string;club:string|null;age:number|null;ca:number;pa:number}[]}
interface Summary {name:string;path:string;players:number;sha256:string}
declare global {interface Window {__TAURI__:{core:{invoke:<T>(command:string,args?:Record<string,unknown>)=>Promise<T>}}}}
const $=<T extends HTMLElement=HTMLElement>(id:string)=>document.getElementById(id) as T;
const invoke=<T>(command:string,args?:Record<string,unknown>)=>window.__TAURI__.core.invoke<T>(command,args);
const escape=(text:unknown)=>String(text??'').replace(/[&<>"']/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]!));
const labels:Record<string,string>={goalkeeper:'Goleiro',sweeper:'Líbero',defender:'Defensor',defensiveMidfielder:'Volante',midfielder:'Meio-campista',attackingMidfielder:'Meia ofensivo',attacker:'Atacante',wingBack:'Ala',freeRole:'Função livre',rightSide:'Direita',leftSide:'Esquerda',central:'Centro',crossing:'Cruzamentos',dribbling:'Drible',finishing:'Finalização',heading:'Cabeceio',longShots:'Chutes de longe',marking:'Marcação',offTheBall:'Sem bola',passing:'Passe',penalties:'Pênaltis',tackling:'Desarme',creativity:'Criatividade',handling:'Jogo de mãos',aerialAbility:'Jogo aéreo',commandOfArea:'Comando de área',communication:'Comunicação',kicking:'Pontapé',throwing:'Lançamento',anticipation:'Antecipação',decisions:'Decisões',oneOnOnes:'Um contra um',positioning:'Posicionamento',reflexes:'Reflexos',throwIns:'Laterais',firstTouch:'Primeiro toque',technique:'Técnica',leftFoot:'Pé esquerdo',rightFoot:'Pé direito',flair:'Instinto',corners:'Escanteios',teamwork:'Trabalho em equipe',workRate:'Índice de trabalho',longThrows:'Lançamentos longos',eccentricity:'Excentricidade',rushingOut:'Saídas',tendencyToPunch:'Sair de soco',acceleration:'Aceleração',setPieces:'Bolas paradas',strength:'Força',stamina:'Resistência',pace:'Velocidade',jumping:'Impulsão',influence:'Influência',dirtiness:'Jogo sujo',balance:'Equilíbrio',bravery:'Coragem',consistency:'Consistência',aggression:'Agressividade',agility:'Agilidade',importantMatches:'Jogos importantes',injuryProneness:'Propensão a lesões',versatility:'Versatilidade',naturalFitness:'Aptidão natural',determination:'Determinação'};
const groups:[string,string[]][]=[['Técnicos',['crossing','dribbling','finishing','heading','longShots','marking','passing','penalties','tackling','firstTouch','technique','setPieces','corners','longThrows']],['Mentais',['aggression','anticipation','bravery','creativity','decisions','determination','influence','flair','positioning','offTheBall','teamwork','workRate']],['Físicos',['acceleration','agility','balance','strength','jumping','stamina','pace','naturalFitness']]];
const keeper=['handling','aerialAbility','commandOfArea','communication','kicking','throwing','reflexes','rushingOut','tendencyToPunch','oneOnOnes','eccentricity'];
let summary:Summary|null=null,detail:Detail|null=null,plan:Pending={edits:[],healInjuries:[],clearSuspensions:[],undo:false,redo:false},page=0,total=0,busy=false,searchGeneration=0;
let timer:ReturnType<typeof setTimeout>;
let profileTab='attributes';
function status(text:string,kind=''){$('status').textContent=text;$('status').className=kind;}
async function task(action:()=>Promise<void>){if(busy)return;busy=true;document.body.setAttribute('aria-busy','true');for(const b of document.querySelectorAll<HTMLButtonElement>('button'))b.disabled=true;try{await action();}catch(e){status(String(e),'error');}finally{busy=false;document.body.removeAttribute('aria-busy');for(const b of document.querySelectorAll<HTMLButtonElement>('button'))b.disabled=false;syncPlan();syncPages();}}
function inputValue(el:HTMLInputElement|HTMLSelectElement):number{const scale=Number(el.dataset.scale??1);return scale===1?Number(el.value):Math.round(Number(el.value)*scale);}
function drafts():Edit[]{
  if(!detail)return[];
  const d=detail;
  const policy=$<HTMLSelectElement>('ability-policy')?.value as Edit['policy'];
  const edits:Array<Edit>=[];
  for(const el of document.querySelectorAll<HTMLInputElement|HTMLSelectElement>('#profile [data-field]')){
    const key=el.dataset.field!,value=inputValue(el),pending=d.pending.find(p=>p.attribute===key);
    const dependency=key==='currentAbility'||key==='position.goalkeeper';
    if(value!==Number(el.dataset.initial)||(dependency&&pending&&policy!==pending.policy))edits.push({uid:d.player.uid,attribute:key,value,...(dependency&&policy?{policy}: {})});
  }
  for(const el of document.querySelectorAll<HTMLInputElement>('#profile [data-date]')){
    const prefix=el.dataset.date!,monthly=document.querySelector<HTMLInputElement>('[data-monthly="'+prefix+'"]');
    const isMonthly=monthly?.checked??false;
    if(el.value===el.dataset.initial&&isMonthly===(el.dataset.initialMonthly==='true'))continue;
    const millis=Date.parse(el.value+'T00:00:00Z'),date=new Date(millis);
    const year=isMonthly?1900:date.getUTCFullYear(),day=isMonthly?1:Math.round((millis-Date.UTC(year,0,1))/86400000);
    edits.push({uid:d.player.uid,attribute:prefix+'.year',value:year},{uid:d.player.uid,attribute:prefix+'.day',value:day});
  }
  return edits;
}
function healingDraft():{uid:number;enabled:boolean}|null {const checkbox=$<HTMLInputElement>('remove-injuries');return detail&&checkbox&&checkbox.checked!==detail.healPending?{uid:detail.player.uid,enabled:checkbox.checked}:null;}
function suspensionDraft():{uid:number;enabled:boolean}|null {const checkbox=$<HTMLInputElement>('remove-suspensions');return detail&&checkbox&&checkbox.checked!==detail.suspensionPending?{uid:detail.player.uid,enabled:checkbox.checked}:null;}
function planCount(){return plan.edits.length+plan.healInjuries.length+plan.clearSuspensions.length;}
function canLeave(){return (drafts().length===0&&!healingDraft()&&!suspensionDraft())||window.confirm('Descartar os campos ainda não adicionados à revisão?');}
function view(name:string){for(const v of ['players','changes','diagnostics']){$(`${v}-view`).hidden=v!==name;$(`nav-${v}`).classList.toggle('active',v===name);}}
function syncPlan(){const count=planCount();$('pending-count').textContent=String(count);$('review-count').textContent=String(count);$<HTMLButtonElement>('review').disabled=busy||!count;$<HTMLButtonElement>('save-copy').disabled=busy||!count;$<HTMLButtonElement>('undo').disabled=busy||!plan.undo;$<HTMLButtonElement>('redo').disabled=busy||!plan.redo;$<HTMLButtonElement>('clear').disabled=busy||!count;$('footer-state').textContent=count?`${count} alteração(ões) preparada(s). A origem continua intacta.`:'Seu save original permanece intacto.';}
function syncPages(){$<HTMLButtonElement>('previous').disabled=busy||page===0;$<HTMLButtonElement>('next').disabled=busy||(page+1)*50>=total;$('page-label').textContent=`${page+1} / ${Math.max(1,Math.ceil(total/50))}`;}
async function search(){if(!summary)return;const generation=++searchGeneration;const data=await invoke<Page>('search_players',{query:$<HTMLInputElement>('search').value,page});if(generation!==searchGeneration)return;total=data.total;$('result-count').textContent=`${total.toLocaleString('pt-BR')} resultados`;$('results').innerHTML=data.rows.map(p=>`<button class="player-row ${detail?.player.uid===p.uid?'selected':''}" data-uid="${p.uid}"><span><strong>${escape(p.name)}</strong><small>${escape(p.club??'Sem clube')}${p.age===null?'':` · ${p.age} anos`}</small></span><span class="abilities">${p.ca} <span>·</span> ${p.pa}</span></button>`).join('')||'<p class="profile-meta">Nenhum jogador encontrado.</p>';syncPages();}
function fieldLabel(key:string):string {
  const extra:Record<string,string>={currentAbility:'Habilidade atual (CA)',potentialAbility:'Potencial (PA)',height:'Altura (cm)',weight:'Peso (kg)',nationality:'Nacionalidade principal','birth.year':'Ano de nascimento','birth.day':'Dia ordinal de nascimento','reputation.home':'Reputação nacional','reputation.current':'Reputação atual','reputation.world':'Reputação mundial',adaptability:'Adaptação',ambition:'Ambição',loyalty:'Lealdade',pressure:'Pressão',professionalism:'Profissionalismo',sportsmanship:'Espírito esportivo',temperament:'Temperamento',controversy:'Controvérsia'};
  const career:Record<string,string>={'state.condition':'Condição física (%)','state.fitness':'Ritmo de jogo (%)','state.jadedness':'Cansaço acumulado','state.morale':'Moral (0–20)','state.estimatedValue':'Valor estimado (£)','state.saleValue':'Valor de venda (£; -1 automático)',weeklyWage:'Salário semanal (£)',squadStatus:'Situação no elenco',squadNumber:'Camisa (-1 sem número)','expires.year':'Término','expires.day':'Término'};
  if(key.startsWith('contract.')){const [,i,...parts]=key.split('.');return 'Contrato '+(Number(i)+1)+' · '+(career[parts.join('.')]??parts.join('.'));}
  const simple=key.replace(/^position\.|^personality\./,'');return career[key]??extra[key]??extra[simple]??labels[simple]??key;
}
function renderProfile(){
  if(!detail)return;
  const d=detail,p=d.player;
  const value=(key:string)=>d.effective.fields[key]?.value;
  const field=(key:string)=>{
    const meta=p.fields[key];if(!meta)return '';
    const v=value(key),scale=(key==='state.condition'||key==='state.fitness')?100:1;
    if(key.endsWith('.squadStatus')){
      const options=['Não especificado','Não especificado','Indispensável','Importante','Rodízio','Apoio à equipe principal','Grande promessa','Jovem de qualidade','Dispensável'];
      return '<label class="attribute"><span>'+escape(fieldLabel(key))+'</span><select aria-label="'+escape(fieldLabel(key))+'" data-field="'+key+'" data-initial="'+v+'">'+options.map((name,i)=>'<option value="'+(i-1)+'" '+(i-1===v?'selected':'')+'>'+name+'</option>').join('')+'</select><small class="original-value">Original: '+escape(options[meta.value+1]??meta.value)+'</small></label>';
    }
    return '<label class="attribute '+(meta.max>200?'wide-field':'')+'"><span>'+escape(fieldLabel(key))+'</span><input aria-label="'+escape(fieldLabel(key))+'" title="Original: '+meta.value/scale+'" data-field="'+key+'" data-scale="'+scale+'" data-initial="'+v+'" class="'+(v!==meta.value?'changed':'')+'" type="number" required min="'+meta.min/scale+'" max="'+meta.max/scale+'" step="'+(scale===100?'0.01':'1')+'" value="'+v/scale+'"><small class="original-value">Original: '+meta.value/scale+'</small></label>';
  };
  const dateControl=(prefix:string,label:string,allowMonthly=false)=>{
    const year=value(prefix+'.year'),day=value(prefix+'.day'),monthly=prefix!=='birth'&&year===1900&&day===1;
    const date=new Date(Date.UTC(year,0,1)+day*86400000),iso=monthly?'':Number.isFinite(date.getTime())?date.toISOString().slice(0,10):'';
    return '<div class="date-control"><label class="attribute"><span>'+escape(label)+'</span><input aria-label="'+escape(label)+'" type="date" data-date="'+prefix+'" data-initial="'+iso+'" data-initial-monthly="'+monthly+'" value="'+iso+'" '+(monthly?'disabled':'required')+'></label>'+(allowMonthly?'<label><input type="checkbox" data-monthly="'+prefix+'" '+(monthly?'checked':'')+'> Contrato mensal</label>':'')+'</div>';
  };
  const group=(title:string,keys:string[])=>'<div class="attribute-group"><h3>'+title+'</h3>'+keys.map(field).join('')+'</div>';
  const section=(id:string,title:string,body:string)=>'<section role="tabpanel" id="panel-'+id+'" aria-labelledby="tab-'+id+'" '+(id===profileTab?'':'hidden')+'><h3 class="tab-heading">'+title+'</h3><div class="attribute-grid">'+body+'</div></section>';
  const tabs:[string,string][]=[['attributes','Atributos'],['ability','CA / PA'],['positions','Posições'],['personal','Dados pessoais'],['personality','Personalidade'],['hidden','Ocultos'],['keeper','Goleiro'],['state','Estado / valores'],['contracts','Contratos']];
  const tabbar='<div class="profile-tabs" role="tablist" aria-label="Perfil do jogador">'+tabs.map(([id,title])=>'<button type="button" role="tab" id="tab-'+id+'" data-tab="'+id+'" aria-controls="panel-'+id+'" aria-selected="'+(id===profileTab)+'" tabindex="'+(id===profileTab?0:-1)+'">'+title+'</button>').join('')+'</div>';
  const policy=d.pending.find(e=>e.policy)?.policy??'';
  const selectedNation=value('nationality');
  const nationOptions=(selectedNation===-1?'<option value="-1" selected disabled>Não informada</option>':'')+d.nations.map((name,index)=>'<option value="'+index+'" '+(index===selectedNation?'selected':'')+'>'+escape(name)+'</option>').join('');
  const personal='<div class="attribute-group"><h3>Identificação</h3>'+dateControl('birth','Data de nascimento')+'<label class="attribute"><span>Nacionalidade</span><select aria-label="Nacionalidade principal" data-field="nationality" data-initial="'+selectedNation+'">'+nationOptions+'</select></label></div>'+group('Medidas',['height','weight'])+'<p class="profile-meta">Zero nas medidas significa não especificado. A altura é guardada em polegadas inteiras; valores em cm sem representação exata são recusados.</p>';
  const injuryBody='<div class="contract-card"><h3>Lesões registradas</h3>'+(d.injuries.length?'<ul>'+d.injuries.map(injury=>'<li>'+escape(injury.name)+(injury.recordedOn?' · registrada em '+escape(new Date(Date.UTC(injury.recordedOn.year,0,1)+injury.recordedOn.day*86400000).toLocaleDateString('pt-BR',{timeZone:'UTC'})):'')+'</li>').join('')+'</ul><label><input type="checkbox" id="remove-injuries" '+(d.healPending?'checked':'')+'> Remover as lesões deste jogador</label><p class="profile-meta">'+(d.healPending?'Remoção preparada na revisão. Desmarque e adicione novamente para restaurar.':'Adicione à revisão para preparar a remoção. Condição e ritmo podem ser ajustados separadamente.')+'</p>':'<p>Nenhuma lesão registrada.</p>')+'</div>';
  const suspensionBody='<div class="contract-card"><h3>Suspensões registradas</h3>'+(d.suspensionCount?'<p>'+d.suspensionCount+' registro(s) de suspensão.</p><label><input type="checkbox" id="remove-suspensions" '+(d.suspensionPending?'checked':'')+'> Remover as suspensões deste jogador</label><p class="profile-meta">'+(d.suspensionPending?'Remoção preparada na revisão. Desmarque e adicione novamente para restaurar.':'Remove as suspensões registradas. A contagem de cartões é preservada; uma nova punição pode ocorrer em jogos futuros.')+'</p>':'<p>Nenhuma suspensão registrada.</p>')+'</div>';
  const stateBody=group('Preparação',['state.condition','state.fitness','state.jadedness','state.morale'])+group('Valores',['state.estimatedValue','state.saleValue'])+injuryBody+suspensionBody+'<div class="career-help"><p>Condição e ritmo são independentes. Menor índice de cansaço representa mais descanso. Os valores financeiros podem ser recalculados pelo jogo.</p><button type="button" id="recover-player">Recuperar condição e ritmo</button><button type="button" id="rest-player">Descansar</button></div>';
  const contractsBody=d.contracts.map(c=>{
    const prefix='contract.'+c.index,role=c.entityType===1?'Seleção':c.kind===0?'Clube proprietário':c.kind===1?'Equipe de empréstimo':'Equipe vinculada';
    const start=new Date(Date.UTC(c.started.year,0,1)+c.started.day*86400000);
    return '<div class="contract-card"><h3>Contrato '+(c.index+1)+' · '+escape(c.kindLabel)+'</h3><p>'+escape(role)+': <strong>'+escape(c.entity??'Não informada')+'</strong></p><p>Início: '+escape(Number.isFinite(start.getTime())?start.toLocaleDateString('pt-BR',{timeZone:'UTC'}):'Não informado')+'</p>'+field(prefix+'.weeklyWage')+dateControl(prefix+'.expires','Término do contrato '+(c.index+1),c.kind===0)+field(prefix+'.squadStatus')+field(prefix+'.squadNumber')+'</div>';
  }).join('')||'<p class="profile-meta">Sem contrato ativo para editar.</p>';
  const loanHelp=d.contracts.some(c=>c.kind===1)?'<p class="profile-meta">No empréstimo, o salário do clube proprietário é o total do vínculo; o salário da equipe de empréstimo é a contribuição desse clube. O jogo pode mostrar separadamente a parcela que continua com o proprietário.</p>':'';
  $('profile').innerHTML='<div class="profile-header"><div><div class="eyebrow">PERFIL DO JOGADOR · UID '+p.uid+'</div><h2>'+escape(p.name)+'</h2><div class="profile-meta">'+escape(p.club??'Sem clube')+' · '+escape(d.effective.nation??'—')+(d.age===null?'':' · '+d.age+' anos na data do save')+'</div></div><div class="profile-badges"><div class="ability-badge"><small>CA</small><strong>'+d.effective.currentAbility+'</strong></div><div class="ability-badge"><small>PA</small><strong>'+d.effective.potentialAbility+'</strong></div></div></div><form id="attributes-form" novalidate>'+tabbar+section('attributes','Atributos',groups.map(([title,keys])=>group(title,keys)).join(''))+section('ability','Habilidade, potencial e reputação',group('Habilidade',['currentAbility','potentialAbility'])+group('Reputação',['reputation.home','reputation.current','reputation.world'])+'<p class="profile-meta">PA: 1–200 para potencial fixo; -1/-2 preservam os valores especiais do formato. Zero não é aceito como novo alvo.</p>')+section('positions','Posições, lados e pés',group('Posições',['goalkeeper','sweeper','defender','defensiveMidfielder','midfielder','attackingMidfielder','attacker','wingBack','freeRole'].map(k=>'position.'+k))+group('Lados',['rightSide','leftSide','central'].map(k=>'position.'+k))+group('Pés',['leftFoot','rightFoot']))+section('personal','Dados pessoais',personal)+section('personality','Personalidade',group('Personalidade',['adaptability','ambition','loyalty','pressure','professionalism','sportsmanship','temperament','controversy'].map(k=>'personality.'+k)))+section('hidden','Atributos ocultos e complementares',group('Complementares',['dirtiness','consistency','importantMatches','injuryProneness','versatility','throwIns']))+section('keeper','Atributos de goleiro',group('Goleiro',keeper))+section('state','Estado e valores',stateBody)+section('contracts','Contratos existentes',contractsBody+loanHelp+'<p class="profile-meta">Salários e valores em libras (£); salário semanal. O jogo pode mostrar outra periodicidade e arredondar. Cada cartão edita apenas o contrato indicado.</p>')+'<div class="policy-box"><label for="ability-policy">Ao alterar CA ou a classificação de goleiro</label><select id="ability-policy"><option value="">Escolha uma política se alterar esses campos</option><option value="preserveValues">Preservar valores convertidos dos atributos</option><option value="preserveRelative">Manter bytes relativos e recalcular os atributos</option></select><small>Preservar valores usa os atributos da origem, exceto alvos definidos explicitamente. A revisão mostra os efeitos indiretos; valores impossíveis serão recusados.</small></div><div class="profile-actions"><small>Revise todas as alterações antes de gravar.</small><button type="submit" class="primary">Adicionar à revisão</button></div></form><details><summary>Dados brutos de diagnóstico</summary><pre>'+escape(JSON.stringify(p.rawAttributes,null,2))+'</pre></details>';
  $<HTMLSelectElement>('ability-policy').value=policy;
  for(const box of document.querySelectorAll<HTMLInputElement>('#profile [data-monthly]'))box.onchange=()=>{const date=document.querySelector<HTMLInputElement>('[data-date="'+box.dataset.monthly+'"]')!;date.disabled=box.checked;date.required=!box.checked;};
  const fill=(key:string,target:number)=>{const input=document.querySelector<HTMLInputElement>('[data-field="'+key+'"]')!;input.value=String(target/Number(input.dataset.scale??1));input.dispatchEvent(new Event('input'));};
  $('recover-player').onclick=()=>{fill('state.condition',10000);fill('state.fitness',10000);status('Condição e ritmo preparados; adicione à revisão para validar.');};
  $('rest-player').onclick=()=>{fill('state.jadedness',Math.min(value('state.jadedness'),-1000));status('Descanso preparado; adicione à revisão para validar.');};

  function selectTab(id:string,focus=false){
    profileTab=id;
    for(const tab of document.querySelectorAll<HTMLButtonElement>('#profile [data-tab]')){
      const active=tab.dataset.tab===id;tab.setAttribute('aria-selected',String(active));tab.tabIndex=active?0:-1;if(active&&focus)tab.focus();
    }
    for(const panel of document.querySelectorAll<HTMLElement>('#profile [role=tabpanel]'))panel.hidden=panel.id!=='panel-'+id;
  }
  for(const tab of document.querySelectorAll<HTMLButtonElement>('#profile [data-tab]')){
    tab.onclick=()=>selectTab(tab.dataset.tab!);
    tab.onkeydown=event=>{
      const i=tabs.findIndex(([id])=>id===profileTab);let next=i;
      if(event.key==='ArrowRight')next=(i+1)%tabs.length;else if(event.key==='ArrowLeft')next=(i+tabs.length-1)%tabs.length;else if(event.key==='Home')next=0;else if(event.key==='End')next=tabs.length-1;else return;
      event.preventDefault();selectTab(tabs[next][0],true);
    };
  }

  $<HTMLFormElement>('attributes-form').onsubmit=event=>{
    event.preventDefault();void task(async()=>{
      for(const el of document.querySelectorAll<HTMLInputElement|HTMLSelectElement>('#profile [data-field]'))if(Number(el.value)!==Number(el.dataset.initial)/Number(el.dataset.scale??1)&&(!el.checkValidity()||!Number.isInteger(inputValue(el))))throw Error('Verifique os limites e a precisão dos campos alterados.');
      const edits=drafts(),healing=healingDraft(),suspension=suspensionDraft();if(!edits.length&&!healing&&!suspension){status('Nenhum campo novo foi alterado.');return;}
      if(edits.some(e=>!Number.isFinite(e.value)))throw Error('Verifique as datas alteradas.');
      status('Validando campos e efeitos sobre os atributos…');
      plan=await invoke<Pending>('stage_edits',{edits,healing,suspension});detail=await invoke<Detail>('player_detail',{uid:p.uid});renderProfile();status('Alterações adicionadas à revisão. Nada foi gravado no save.','success');
    });
  };
  for(const el of document.querySelectorAll<HTMLInputElement|HTMLSelectElement>('#profile [data-field]'))el.oninput=()=>el.classList.toggle('changed',inputValue(el)!==p.fields[el.dataset.field!].value);
}
async function showPlayer(uid:number){if(!canLeave())return;detail=await invoke<Detail>('player_detail',{uid});renderProfile();for(const row of document.querySelectorAll<HTMLElement>('.player-row'))row.classList.toggle('selected',Number(row.dataset.uid)===uid);}
async function open(){if(!canLeave())return;if(planCount()&&!window.confirm('Abrir outro save e descartar as alterações pendentes?'))return;await task(async()=>{status('Selecione o save. A leitura pode levar alguns segundos…');const data=await invoke<Summary|null>('open_save');if(!data){status('Abertura cancelada.');return;}summary=data;detail=null;plan={edits:[],healInjuries:[],clearSuspensions:[],undo:false,redo:false};page=0;$<HTMLInputElement>('search').value='';$('file-name').textContent=data.name;$('file-info').textContent=`${data.players.toLocaleString('pt-BR')} jogadores · banco 147`;$('file-name').title=data.path;$('player-count').textContent=data.players.toLocaleString('pt-BR');$('diagnostics').textContent=JSON.stringify(data,null,2);$('empty').hidden=true;$('workspace').hidden=false;$('profile').innerHTML='<div class="profile-placeholder"><h2>Escolha um jogador</h2><p>Use a busca para encontrar um nome ou elenco.</p></div>';view('players');await search();status('Save carregado. Selecione um jogador para começar.','success');});}
function renderChanges(report:Report):string {
  const seen=new Set<string>();return report.changes.filter(c=>{
    if(c.attribute==='birth.day'||c.attribute==='birth.year'||/^contract\.\d+\.expires\.(day|year)$/.test(c.attribute)){
      const key=c.uid+':'+c.attribute.replace(/\.(day|year)$/,'');if(seen.has(key))return false;seen.add(key);
    }return true;
  }).map(c=>'<div class="change-item"><strong>'+escape(c.name)+'</strong><span>'+escape((c.attribute.startsWith('birth.')?'Nascimento':fieldLabel(c.attribute))+(c.derived?' (efeito indireto)':''))+(c.context?'<small class="change-context">'+escape(c.context)+'</small>':'')+'</span><b>'+escape(c.displayBefore??c.before)+' → '+escape(c.displayAfter??c.after)+'</b></div>').join('')+report.removals.map(r=>'<div class="change-item"><strong>'+escape(r.name)+'</strong><span>'+ (r.kind==='suspensions'?'Remover suspensões registradas':'Remover lesões registradas')+'</span><b>'+r.count+' → 0</b></div>').join('');
}
function reportSummary(report:Report){const fields=report.changes.length,records=report.removals.reduce((sum,r)=>sum+r.count,0);return report.removals.length?`${fields} campo(s) editado(s) e ${records} registro(s) removido(s).`:`${fields} campo(s) e ${report.fieldChangedBytes} bytes alterados.`;}
async function review(){if(!summary||!canLeave())return;view('changes');await task(async()=>{plan=await invoke<Pending>('pending_edits');if(!planCount()){$('changes-list').textContent='Nenhuma alteração pendente.';return;}status('Validando o plano de edição…');const report=await invoke<Report>('preview_edits');$('changes-list').innerHTML=renderChanges(report);status('Plano válido: '+reportSummary(report),'success');});}
$('open-save').onclick=()=>void open();$('empty-open').onclick=()=>void open();$('nav-players').onclick=()=>{if(summary&&canLeave())view('players');};$('nav-changes').onclick=()=>void review();$('review').onclick=()=>void review();$('nav-diagnostics').onclick=()=>{if(summary&&canLeave())view('diagnostics');};
$('results').onclick=e=>{const row=(e.target as HTMLElement).closest<HTMLElement>('[data-uid]');if(row)void task(()=>showPlayer(Number(row.dataset.uid)));};
$<HTMLInputElement>('search').oninput=()=>{clearTimeout(timer);timer=setTimeout(()=>{page=0;void search().catch(e=>status(String(e),'error'));},180);};
$('previous').onclick=()=>{page--;void search().catch(e=>status(String(e),'error'));};$('next').onclick=()=>{page++;void search().catch(e=>status(String(e),'error'));};
for(const action of ['undo','redo','clear'])$(action).onclick=()=>void task(async()=>{plan=await invoke<Pending>('history_action',{action});if(detail){detail=await invoke<Detail>('player_detail',{uid:detail.player.uid});renderProfile();}if(planCount()){const report=await invoke<Report>('preview_edits');$('changes-list').innerHTML=renderChanges(report);}else $('changes-list').textContent='Nenhuma alteração pendente.';status('Plano de alterações atualizado.');});
$('save-copy').onclick=()=>void task(async()=>{status('Escolha um nome novo para a cópia. O original não será sobrescrito.');const result=await invoke<{path:string;report:Report}|null>('save_copy');if(!result){status('Gravação cancelada.');return;}$('saved-path').textContent=`Cópia validada e salva: ${result.path}`;status('Cópia salva e verificada: '+reportSummary(result.report),'success');});
window.onbeforeunload=e=>{if(planCount()||drafts().length||healingDraft()||suspensionDraft()){e.preventDefault();}};
syncPlan();
export {};
