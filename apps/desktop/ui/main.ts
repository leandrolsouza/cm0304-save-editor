interface Edit {uid:number;attribute:string;value:number;policy?:'preserveValues'|'preserveRelative'}
interface Pending {edits:Edit[];undo:boolean;redo:boolean}
interface Field {value:number;min:number;max:number;width:number;offset:number}
interface Player {uid:number;name:string;firstName:string;surname:string;club:string|null;nation:string|null;currentAbility:number;potentialAbility:number;attributes:Record<string,number>;rawAttributes:Record<string,number>;fields:Record<string,Field>}
interface Detail {player:Player;effective:Player;editable:string[];pending:Edit[];nations:string[]}
interface Change {uid:number;name:string;attribute:string;before:number;after:number;rawBefore:number;rawAfter:number;derived:boolean}
interface Report {changes:Change[];changedBytes:number;sourceSha256:string;copySha256:string}
interface Page {total:number;rows:{uid:number;name:string;club:string|null;ca:number;pa:number}[]}
interface Summary {name:string;path:string;players:number;sha256:string}
declare global {interface Window {__TAURI__:{core:{invoke:<T>(command:string,args?:Record<string,unknown>)=>Promise<T>}}}}
const $=<T extends HTMLElement=HTMLElement>(id:string)=>document.getElementById(id) as T;
const invoke=<T>(command:string,args?:Record<string,unknown>)=>window.__TAURI__.core.invoke<T>(command,args);
const escape=(text:unknown)=>String(text??'').replace(/[&<>"']/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]!));
const labels:Record<string,string>={goalkeeper:'Goleiro',sweeper:'Líbero',defender:'Defensor',defensiveMidfielder:'Volante',midfielder:'Meio-campista',attackingMidfielder:'Meia ofensivo',attacker:'Atacante',wingBack:'Ala',freeRole:'Função livre',rightSide:'Direita',leftSide:'Esquerda',central:'Centro',crossing:'Cruzamentos',dribbling:'Drible',finishing:'Finalização',heading:'Cabeceio',longShots:'Chutes de longe',marking:'Marcação',offTheBall:'Sem bola',passing:'Passe',penalties:'Pênaltis',tackling:'Desarme',creativity:'Criatividade',handling:'Jogo de mãos',aerialAbility:'Jogo aéreo',commandOfArea:'Comando de área',communication:'Comunicação',kicking:'Pontapé',throwing:'Lançamento',anticipation:'Antecipação',decisions:'Decisões',oneOnOnes:'Um contra um',positioning:'Posicionamento',reflexes:'Reflexos',throwIns:'Laterais',firstTouch:'Primeiro toque',technique:'Técnica',leftFoot:'Pé esquerdo',rightFoot:'Pé direito',flair:'Instinto',corners:'Escanteios',teamwork:'Trabalho em equipe',workRate:'Índice de trabalho',longThrows:'Lançamentos longos',eccentricity:'Excentricidade',rushingOut:'Saídas',tendencyToPunch:'Sair de soco',acceleration:'Aceleração',setPieces:'Bolas paradas',strength:'Força',stamina:'Resistência',pace:'Velocidade',jumping:'Impulsão',influence:'Influência',dirtiness:'Jogo sujo',balance:'Equilíbrio',bravery:'Coragem',consistency:'Consistência',aggression:'Agressividade',agility:'Agilidade',importantMatches:'Jogos importantes',injuryProneness:'Propensão a lesões',versatility:'Versatilidade',naturalFitness:'Aptidão natural',determination:'Determinação'};
const groups:[string,string[]][]=[['Técnicos',['crossing','dribbling','finishing','heading','longShots','marking','passing','penalties','tackling','firstTouch','technique','setPieces','corners','longThrows']],['Mentais',['aggression','anticipation','bravery','creativity','decisions','determination','influence','flair','positioning','offTheBall','teamwork','workRate']],['Físicos',['acceleration','agility','balance','strength','jumping','stamina','pace','naturalFitness']]];
const keeper=['handling','aerialAbility','commandOfArea','communication','kicking','throwing','reflexes','rushingOut','tendencyToPunch','oneOnOnes','eccentricity'];
let summary:Summary|null=null,detail:Detail|null=null,plan:Pending={edits:[],undo:false,redo:false},page=0,total=0,busy=false,searchGeneration=0;
let timer:ReturnType<typeof setTimeout>;
function status(text:string,kind=''){$('status').textContent=text;$('status').className=kind;}
async function task(action:()=>Promise<void>){if(busy)return;busy=true;document.body.setAttribute('aria-busy','true');for(const b of document.querySelectorAll<HTMLButtonElement>('button'))b.disabled=true;try{await action();}catch(e){status(String(e),'error');}finally{busy=false;document.body.removeAttribute('aria-busy');for(const b of document.querySelectorAll<HTMLButtonElement>('button'))b.disabled=false;syncPlan();syncPages();}}
function drafts():Edit[]{
  if(!detail)return[];
  const d=detail;
  const policy=$<HTMLSelectElement>('ability-policy')?.value as Edit['policy'];
  const edits:Array<Edit>=[];
  for(const el of document.querySelectorAll<HTMLInputElement|HTMLSelectElement>('#profile [data-field]')){
    const key=el.dataset.field!,value=Number(el.value),pending=d.pending.find(p=>p.attribute===key);
    const dependency=key==='currentAbility'||key==='position.goalkeeper';
    if(value!==Number(el.dataset.initial)||(dependency&&pending&&policy!==pending.policy))edits.push({uid:d.player.uid,attribute:key,value,...(dependency&&policy?{policy}: {})});
  }
  const birthday=$<HTMLInputElement>('birthday');
  if(birthday&&birthday.value!==birthday.dataset.initial){
    const millis=Date.parse(birthday.value+'T00:00:00Z'),date=new Date(millis),year=date.getUTCFullYear();
    const day=Math.round((millis-Date.UTC(year,0,1))/86400000);
    edits.push({uid:d.player.uid,attribute:'birth.year',value:year},{uid:d.player.uid,attribute:'birth.day',value:day});
  }
  return edits;
}
function canLeave(){return drafts().length===0||window.confirm('Descartar os campos ainda não adicionados à revisão?');}
function view(name:string){for(const v of ['players','changes','diagnostics']){$(`${v}-view`).hidden=v!==name;$(`nav-${v}`).classList.toggle('active',v===name);}}
function syncPlan(){$('pending-count').textContent=String(plan.edits.length);$('review-count').textContent=String(plan.edits.length);$<HTMLButtonElement>('review').disabled=busy||!plan.edits.length;$<HTMLButtonElement>('save-copy').disabled=busy||!plan.edits.length;$<HTMLButtonElement>('undo').disabled=busy||!plan.undo;$<HTMLButtonElement>('redo').disabled=busy||!plan.redo;$<HTMLButtonElement>('clear').disabled=busy||!plan.edits.length;$('footer-state').textContent=plan.edits.length?`${plan.edits.length} alteração(ões) preparada(s). A origem continua intacta.`:'Seu save original permanece intacto.';}
function syncPages(){$<HTMLButtonElement>('previous').disabled=busy||page===0;$<HTMLButtonElement>('next').disabled=busy||(page+1)*50>=total;$('page-label').textContent=`${page+1} / ${Math.max(1,Math.ceil(total/50))}`;}
async function search(){if(!summary)return;const generation=++searchGeneration;const data=await invoke<Page>('search_players',{query:$<HTMLInputElement>('search').value,page});if(generation!==searchGeneration)return;total=data.total;$('result-count').textContent=`${total.toLocaleString('pt-BR')} resultados`;$('results').innerHTML=data.rows.map(p=>`<button class="player-row ${detail?.player.uid===p.uid?'selected':''}" data-uid="${p.uid}"><span><strong>${escape(p.name)}</strong><small>${escape(p.club??'Sem clube')}</small></span><span class="abilities">${p.ca} <span>·</span> ${p.pa}</span></button>`).join('')||'<p class="profile-meta">Nenhum jogador encontrado.</p>';syncPages();}
function fieldLabel(key:string):string {
  const extra:Record<string,string>={currentAbility:'Habilidade atual (CA)',potentialAbility:'Potencial (PA)',height:'Altura (cm)',weight:'Peso (kg)',nationality:'Nacionalidade principal','birth.year':'Ano de nascimento','birth.day':'Dia ordinal de nascimento','reputation.home':'Reputação nacional','reputation.current':'Reputação atual','reputation.world':'Reputação mundial',adaptability:'Adaptação',ambition:'Ambição',loyalty:'Lealdade',pressure:'Pressão',professionalism:'Profissionalismo',sportsmanship:'Espírito esportivo',temperament:'Temperamento',controversy:'Controvérsia'};
  const simple=key.replace(/^position\.|^personality\./,'');return extra[key]??extra[simple]??labels[simple]??key;
}
function renderProfile(){
  if(!detail)return;
  const d=detail,p=d.player,pending=new Map(d.pending.map(e=>[e.attribute,e.value]));
  const value=(key:string)=>d.effective.fields[key]?.value;
  const field=(key:string)=>{
    const meta=p.fields[key];if(!meta)return '';
    const v=value(key);return '<label class="attribute '+(meta.max>200?'wide-field':'')+'"><span>'+escape(fieldLabel(key))+'</span><input aria-label="'+escape(fieldLabel(key))+'" data-field="'+key+'" data-initial="'+v+'" class="'+(v!==meta.value?'changed':'')+'" type="number" min="'+meta.min+'" max="'+meta.max+'" step="1" value="'+v+'"></label>';
  };
  const group=(title:string,keys:string[])=>'<div class="attribute-group"><h3>'+title+'</h3>'+keys.map(field).join('')+'</div>';
  const section=(title:string,body:string)=>'<details class="extended-section"><summary>'+title+'</summary><div class="attribute-grid">'+body+'</div></details>';
  const policy=d.pending.find(e=>e.policy)?.policy??'';
  const birthdayDate=new Date(Date.UTC(value('birth.year'),0,1)+value('birth.day')*86400000);
  const birthday=Number.isFinite(birthdayDate.getTime())?birthdayDate.toISOString().slice(0,10):'';
  const selectedNation=value('nationality');
  const nationOptions=d.nations.map((name,index)=>'<option value="'+index+'" '+(index===selectedNation?'selected':'')+'>'+escape(name)+'</option>').join('');
  const personal='<div class="attribute-group"><h3>Identificação</h3><label class="attribute"><span>Nascimento</span><input id="birthday" aria-label="Data de nascimento" type="date" required data-initial="'+birthday+'" value="'+birthday+'"></label><label class="attribute"><span>Nacionalidade</span><select aria-label="Nacionalidade principal" data-field="nationality" data-initial="'+selectedNation+'">'+nationOptions+'</select></label></div>'+group('Medidas',['height','weight']);
  $('profile').innerHTML='<div class="profile-header"><div><div class="eyebrow">PERFIL DO JOGADOR · UID '+p.uid+'</div><h2>'+escape(p.name)+'</h2><div class="profile-meta">'+escape(p.club??'Sem clube')+' · '+escape(d.effective.nation??'—')+'</div></div><div class="profile-badges"><div class="ability-badge"><small>CA</small><strong>'+d.effective.currentAbility+'</strong></div><div class="ability-badge"><small>PA</small><strong>'+d.effective.potentialAbility+'</strong></div></div></div><form id="attributes-form" novalidate><div class="attribute-grid">'+groups.map(([title,keys])=>group(title,keys)).join('')+'</div>'+section('Habilidade, potencial e reputação',group('Habilidade',['currentAbility','potentialAbility'])+group('Reputação',['reputation.home','reputation.current','reputation.world'])+'<p class="profile-meta">PA: 1–200 para potencial fixo; -1/-2 preservam os valores especiais do formato. Zero não é aceito como novo alvo.</p>')+section('Posições, lados e pés',group('Posições',['goalkeeper','sweeper','defender','defensiveMidfielder','midfielder','attackingMidfielder','attacker','wingBack','freeRole'].map(k=>'position.'+k))+group('Lados',['rightSide','leftSide','central'].map(k=>'position.'+k))+group('Pés',['leftFoot','rightFoot']))+section('Dados pessoais',personal)+section('Personalidade',group('Personalidade',['adaptability','ambition','loyalty','pressure','professionalism','sportsmanship','temperament','controversy'].map(k=>'personality.'+k)))+section('Atributos ocultos e complementares',group('Complementares',['dirtiness','consistency','importantMatches','injuryProneness','versatility','throwIns']))+section('Atributos de goleiro',group('Goleiro',keeper))+'<div class="policy-box"><label for="ability-policy">Ao alterar CA ou a classificação de goleiro</label><select id="ability-policy"><option value="">Escolha uma política se alterar esses campos</option><option value="preserveValues">Preservar valores convertidos dos atributos</option><option value="preserveRelative">Manter bytes relativos e recalcular os atributos</option></select><small>A revisão mostra também os efeitos indiretos. Valores impossíveis serão recusados.</small></div><div class="profile-actions"><small>Revise todas as alterações antes de gravar.</small><button type="submit" class="primary">Adicionar à revisão</button></div></form><details><summary>Dados brutos de diagnóstico</summary><pre>'+escape(JSON.stringify(p.rawAttributes,null,2))+'</pre></details>';
  $<HTMLSelectElement>('ability-policy').value=policy;
  $<HTMLFormElement>('attributes-form').onsubmit=event=>{
    event.preventDefault();void task(async()=>{
      const edits=drafts();if(!edits.length){status('Nenhum campo novo foi alterado.');return;}
      for(const el of document.querySelectorAll<HTMLInputElement|HTMLSelectElement>('#profile [data-field]'))if(Number(el.value)!==Number(el.dataset.initial)&&(!el.checkValidity()||!Number.isInteger(Number(el.value))))throw Error('Verifique os limites e os valores inteiros dos campos alterados.');
      if(edits.some(e=>!Number.isFinite(e.value)))throw Error('Data de nascimento inválida.');
      status('Validando campos e efeitos sobre os atributos…');
      plan=await invoke<Pending>('stage_edits',{edits});detail=await invoke<Detail>('player_detail',{uid:p.uid});renderProfile();status('Alterações adicionadas à revisão. Nada foi gravado no save.','success');
    });
  };
  for(const el of document.querySelectorAll<HTMLInputElement|HTMLSelectElement>('#profile [data-field]'))el.oninput=()=>el.classList.toggle('changed',Number(el.value)!==p.fields[el.dataset.field!].value);
}
async function showPlayer(uid:number){if(!canLeave())return;detail=await invoke<Detail>('player_detail',{uid});renderProfile();for(const row of document.querySelectorAll<HTMLElement>('.player-row'))row.classList.toggle('selected',Number(row.dataset.uid)===uid);}
async function open(){if(!canLeave())return;if(plan.edits.length&&!window.confirm('Abrir outro save e descartar as alterações pendentes?'))return;await task(async()=>{status('Selecione o save. A leitura pode levar alguns segundos…');const data=await invoke<Summary|null>('open_save');if(!data){status('Abertura cancelada.');return;}summary=data;detail=null;plan={edits:[],undo:false,redo:false};page=0;$<HTMLInputElement>('search').value='';$('file-name').textContent=data.name;$('file-info').textContent=`${data.players.toLocaleString('pt-BR')} jogadores · banco 147`;$('file-name').title=data.path;$('player-count').textContent=data.players.toLocaleString('pt-BR');$('diagnostics').textContent=JSON.stringify(data,null,2);$('empty').hidden=true;$('workspace').hidden=false;$('profile').innerHTML='<div class="profile-placeholder"><h2>Escolha um jogador</h2><p>Use a busca para encontrar um nome ou elenco.</p></div>';view('players');await search();status('Save carregado. Selecione um jogador para começar.','success');});}
async function review(){if(!summary||!canLeave())return;view('changes');await task(async()=>{plan=await invoke<Pending>('pending_edits');if(!plan.edits.length){$('changes-list').textContent='Nenhuma alteração pendente.';return;}status('Validando o plano de edição…');const report=await invoke<Report>('preview_edits');$('changes-list').innerHTML=report.changes.map(c=>`<div class="change-item"><strong>${escape(c.name)}</strong><span>${escape(fieldLabel(c.attribute)+(c.derived?" (efeito indireto)":""))}</span><b>${c.before} → ${c.after}</b></div>`).join('');status(`Plano válido: ${report.changes.length} campos e ${report.changedBytes} bytes alterados.`,'success');});}
$('open-save').onclick=()=>void open();$('empty-open').onclick=()=>void open();$('nav-players').onclick=()=>{if(summary&&canLeave())view('players');};$('nav-changes').onclick=()=>void review();$('review').onclick=()=>void review();$('nav-diagnostics').onclick=()=>{if(summary&&canLeave())view('diagnostics');};
$('results').onclick=e=>{const row=(e.target as HTMLElement).closest<HTMLElement>('[data-uid]');if(row)void task(()=>showPlayer(Number(row.dataset.uid)));};
$<HTMLInputElement>('search').oninput=()=>{clearTimeout(timer);timer=setTimeout(()=>{page=0;void search().catch(e=>status(String(e),'error'));},180);};
$('previous').onclick=()=>{page--;void search().catch(e=>status(String(e),'error'));};$('next').onclick=()=>{page++;void search().catch(e=>status(String(e),'error'));};
for(const action of ['undo','redo','clear'])$(action).onclick=()=>void task(async()=>{plan=await invoke<Pending>('history_action',{action});if(detail){detail=await invoke<Detail>('player_detail',{uid:detail.player.uid});renderProfile();}if(plan.edits.length){const report=await invoke<Report>('preview_edits');$('changes-list').innerHTML=report.changes.map(c=>`<div class="change-item"><strong>${escape(c.name)}</strong><span>${escape(fieldLabel(c.attribute)+(c.derived?" (efeito indireto)":""))}</span><b>${c.before} → ${c.after}</b></div>`).join('');}else $('changes-list').textContent='Nenhuma alteração pendente.';status('Plano de alterações atualizado.');});
$('save-copy').onclick=()=>void task(async()=>{status('Escolha um nome novo para a cópia. O original não será sobrescrito.');const result=await invoke<{path:string;report:Report}|null>('save_copy');if(!result){status('Gravação cancelada.');return;}$('saved-path').textContent=`Cópia validada e salva: ${result.path}`;status(`Cópia salva e verificada: ${result.report.changedBytes} bytes alterados.`,'success');});
window.onbeforeunload=e=>{if(plan.edits.length||drafts().length){e.preventDefault();}};
syncPlan();
export {};
