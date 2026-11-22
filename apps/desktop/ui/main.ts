import { locale, localizeStaticUi, saveLocale, t, localizeIpcError } from './i18n.js';

localizeStaticUi();

interface StaffHeader {uid:number;currentAbility?:number;potentialAbility?:number;reputation?:number[];policy?:'preserveValues'|'preserveRelative'}
interface ReviewOperation {index:number;domains:string[];playerCount:number;playerFields:string[];staffFields?:string[]}
interface ReviewClub {uid:number;name:string}
interface ReviewClubs {players:Record<number,ReviewClub>;staff:Record<number,ReviewClub>}
interface Edit {uid:number;attribute:string;value:number;policy?:'preserveValues'|'preserveRelative'}
interface TransferBudgets {season:number;remaining:number}
interface StadiumCapacities {capacity:number;seatedCapacity:number}
interface Pending {clubStadiumFieldCount?:number;clubStadiumCapacities?:Record<number,StadiumCapacities>;operations?:ReviewOperation[];reviewClubs?:ReviewClubs;clubReputation?:Record<number,number>;clubTransferBudgetFieldCount?:number;clubYouthSetup?:Record<number,number>;clubTraining?:Record<number,number>;clubTransferBudgets?:Record<number,TransferBudgets>;clubBalances?:Record<number,number>;clubWageBudgets?:Record<number,number>;edits:Edit[];staffEdits?:Edit[];staffHeaderEdits?:StaffHeader[];healInjuries:number[];clearSuspensions:number[];undo:boolean;redo:boolean;undoOperation?:ReviewOperation|null;redoOperation?:ReviewOperation|null}
interface Field {value:number;min:number;max:number;width:number;offset:number}
interface Player {type?:number;uid:number;name:string;firstName:string;surname:string;club:string|null;nation:string|null;currentAbility:number;potentialAbility:number;attributes:Record<string,number>;rawAttributes:Record<string,number>;fields:Record<string,Field>}
interface ContractView {index:number;kind:number;kindLabel:string;entityType:number;entity:string|null;started:{year:number;day:number};monthly:boolean;transferStatus:number}
interface Detail {player:Player;effective:Player;age:number|null;contracts:ContractView[];marketTargets?:number[];marketJournalSupported?:boolean;editable:string[];pending:Edit[];nations:string[];injuries:{name:string;recordedOn:{year:number;day:number}|null;estimatedDaysRemaining?:number|null}[];healPending:boolean;suspensionCount:number;bookingRecordCount?:number;suspensionPending:boolean}
interface Change {uid:number;name:string;attribute:string;before:number;after:number;rawBefore:number;rawAfter:number;derived:boolean;context:string|null;displayBefore:string|null;displayAfter:string|null}
interface Report {staffChanges?:{uid:number;name?:string;attribute:string;before:number;after:number}[];changes:Change[];fieldChangedBytes:number;removedBytes:number;marketJournalEntriesAdded?:number;marketJournalBytesAdded?:number;removals:{uid:number;name:string;kind:'injuries'|'suspensions';count:number;removedBytes:number}[];sourceSha256:string;copySha256:string}
interface Page {total:number;rows:{uid:number;name:string;club:string|null;age:number|null;ca:number;pa:number}[]}
interface ClubRow {uid:number;name:string;shortName:string;players:number;staff:number}
interface ExpensePeriods {currentMonth:number;previousMonth:number;season:number}
interface FinancePeriod {income:number;expenses:number}
interface ClubInfo {transferInstallments?:{counterparty:string;playerName:string|null;personId:number;amountRaw:number;remaining:number;outgoing:boolean}[]|null;sellOnClauses?:{playerUid:number;playerName:string|null;beneficiary:string|null;percent:number}[];currentWagesWeekly?:number|null;wageBudgetWeekly?:number|null;payrollExpenses?:{wages:ExpensePeriods;bonuses:ExpensePeriods}|null;financeSummary?:{currentMonth:FinancePeriod;previousMonth:FinancePeriod;season:FinancePeriod}|null;reputation?:number|null;loanWageOffsetWeekly?:number;playerContractWagesWeekly?:number;transferBudgetSeason?:number|null;transferBudgetRemaining?:number|null;balanceEditable?:boolean;balance:number|null;trainingRaw:number;youthSetupRaw:number;averageAttendance:number}
interface ClubExpiry {uid:number;index:number;name:string;expires:{year:number;day:number};days:number|null;loan:boolean}
interface ClubSquadPlayer extends BatchPlayer {ca:number;pa:number;age:number|null;positions:string[];familiarity:Partial<Record<string,number>>;condition:number|null}
interface ClubAnalysisLoan {uid:number;index:number;expires:{year:number;day:number}|null}
interface ClubTactic {name:string;positions:{row:number;column:number}[];starters:(number|null)[]|null}
interface ClubWageContract {uid:number;index:number;name:string;loan:boolean;weeklyWage:number;squadStatus?:number;squadNumber?:number}
const squadStatusLabels=[t("Não especificado"),t("Não especificado"),t("Indispensável"),t("Importante"),t("Rodízio"),t("Apoio à equipe principal"),t("Grande promessa"),t("Jovem de qualidade"),t("Dispensável")];
interface ClubStaff {uid:number;name:string;role:string;isPlayer:boolean;attributes?:Record<string,number>}
interface ClubStadium {capacity:number;seatedCapacity:number;firstTeamClubUids:number[]}
interface ClubDetail {stadium?:ClubStadium|null;projectedPlayerPayrollWeekly?:number|null;expiringContracts?:ClubExpiry[];analysisLoans:ClubAnalysisLoan[];analysisTactic:ClubTactic|null;analysisTacticUnsupported:boolean;wageContracts?:ClubWageContract[];attributeAverages?:Record<string,number>;info?:ClubInfo|null;profile:{uid:number;name:string;shortName:string;playerUids:number[];staffUids:number[]};players:ClubSquadPlayer[];staff:ClubStaff[]}
interface ManagedClub {uid:number;name:string}
type MyClubCommand = 'cash'|'transfers'|'wages'|'injuries'|'suspensions'|'condition'|'morale';
interface MyClubShortcut {command:MyClubCommand;value:number|null;target:number|null;affected:number;inReview:number;unavailable:number;reason:string|null}
interface MyClubDetail {shortcuts:MyClubShortcut[]}
interface Summary {clubs?:number;managedClubs?:ManagedClub[];name:string;path:string;players:number;staff:number;sha256:string}
interface StaffProfile {uid:number;name:string;isPlayer:boolean;isHuman:boolean;team:string|null;clubJob:string;nationalTeam:string|null;nationalJob:string;nation:string|null;dateOfBirth:{year:number;day:number}|null;currentAbility:number;playerCurrentAbility?:number|null;potentialAbility:number;reputation:number[];rawAttributes:Record<string,number>|null;attributes:Record<string,number>|null;personality:Record<string,number>|null}
interface StaffDetail {profile:StaffProfile;recorded?:StaffProfile|null;age:number|null;pendingPlayerFields:number;editable?:string[];pendingHeader?:StaffHeader|null}
interface StaffPage {total:number;rows:{uid:number;name:string;team:string|null;role:string;nationalTeam:string|null;age:number|null;ca:number;pa:number;isPlayer:boolean}[]}
declare global {interface Window {__TAURI__:{core:{invoke:<T>(command:string,args?:Record<string,unknown>)=>Promise<T>}}}}
const $=<T extends HTMLElement=HTMLElement>(id:string)=>document.getElementById(id) as T;

const themeSelect = $<HTMLSelectElement>('theme-select');
const themeKey = 'cm0304.theme.v1';
try {
  themeSelect.value = localStorage.getItem(themeKey) ?? themeSelect.value;
} catch {
  // O seletor continua disponível quando o armazenamento local está indisponível.
}
if (themeSelect.selectedIndex === -1) themeSelect.selectedIndex = 0;
document.documentElement.dataset.theme = themeSelect.value;
themeSelect.addEventListener('change', () => {
  document.documentElement.dataset.theme = themeSelect.value;
  try {
    localStorage.setItem(themeKey, themeSelect.value);
  } catch {
    // A escolha já está aplicada nesta janela.
  }
});

const invoke=async <T>(command:string,args?:Record<string,unknown>)=>{
  try {
    return await window.__TAURI__.core.invoke<T>(command,args);
  } catch (error) {
    throw localizeIpcError(error);
  }
};
void invoke<string>('plugin:app|version').then(version=>{
  if(!version)return;
  const label=$('editor-version');
  label.textContent=' · Editor '+version;
  label.hidden=false;
}).catch(()=>{});
const escape=(text:unknown)=>String(text??'').replace(/[&<>"']/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]!));
const labels:Record<string,string>={goalkeeper:t("Goleiro"),sweeper:t("Líbero"),defender:t("Defensor"),defensiveMidfielder:t("Volante"),midfielder:t("Meio-campista"),attackingMidfielder:t("Meia ofensivo"),attacker:t("Atacante"),wingBack:t("Ala"),freeRole:t("Função livre"),rightSide:t("Direita"),leftSide:t("Esquerda"),central:t("Centro"),crossing:t("Cruzamentos"),dribbling:t("Drible"),finishing:t("Finalização"),heading:t("Cabeceio"),longShots:t("Chutes de longe"),marking:t("Marcação"),offTheBall:t("Sem bola"),passing:t("Passe"),penalties:t("Pênaltis"),tackling:t("Desarme"),creativity:t("Criatividade"),handling:t("Jogo de mãos"),aerialAbility:t("Jogo aéreo"),commandOfArea:t("Comando de área"),communication:t("Comunicação"),kicking:t("Pontapé"),throwing:t("Lançamento"),anticipation:t("Antecipação"),decisions:t("Decisões"),oneOnOnes:t("Um contra um"),positioning:t("Posicionamento"),reflexes:t("Reflexos"),throwIns:t("Laterais"),firstTouch:t("Primeiro toque"),technique:t("Técnica"),leftFoot:t("Pé esquerdo"),rightFoot:t("Pé direito"),flair:t("Instinto"),corners:t("Escanteios"),teamwork:t("Trabalho em equipe"),workRate:t("Índice de trabalho"),longThrows:t("Lançamentos longos"),eccentricity:t("Excentricidade"),rushingOut:t("Saídas"),tendencyToPunch:t("Sair de soco"),acceleration:t("Aceleração"),setPieces:t("Bolas paradas"),strength:t("Força"),stamina:t("Resistência"),pace:t("Velocidade"),jumping:t("Impulsão"),influence:t("Influência"),dirtiness:t("Jogo sujo"),balance:t("Equilíbrio"),bravery:t("Coragem"),consistency:t("Consistência"),aggression:t("Agressividade"),agility:t("Agilidade"),importantMatches:t("Jogos importantes"),injuryProneness:t("Propensão a lesões"),versatility:t("Versatilidade"),naturalFitness:t("Aptidão natural"),determination:t("Determinação")};
const groups:[string,string[]][]=[[t("Técnicos"),['crossing','dribbling','finishing','heading','longShots','marking','passing','penalties','tackling','firstTouch','technique','setPieces','corners','longThrows']],[t("Mentais"),['aggression','anticipation','bravery','creativity','decisions','determination','influence','flair','positioning','offTheBall','teamwork','workRate']],[t("Físicos"),['acceleration','agility','balance','strength','jumping','stamina','pace','naturalFitness']]];
const keeper=['handling','aerialAbility','commandOfArea','communication','kicking','throwing','reflexes','rushingOut','tendencyToPunch','oneOnOnes','eccentricity'];
let summary:Summary|null=null,detail:Detail|null=null,plan:Pending={edits:[],healInjuries:[],clearSuspensions:[],undo:false,redo:false},page=0,total=0,busy=false,searchGeneration=Date.now()*1024;
let validatedPlan:Pending|null=null;
let myClubUid:number|null=null;
let myClub:MyClubDetail|null=null;
let needsInitialPlayerSearch=false;
let timer:ReturnType<typeof setTimeout>;
let staffTimer:ReturnType<typeof setTimeout>|undefined,staffPage=0,staffTotal=0,staffGeneration=0,staffDetail:StaffDetail|null=null;
let profileTab='attributes';
type NoticeKind = 'info' | 'warning' | 'error' | 'success';
function notice(kind: NoticeKind, text: string) {
  return '<p class="notice" data-notice="'+kind+'">'+escape(text)+'</p>';
}
const toastLabels: Record<NoticeKind, string> = {
  info: t("Informação"),
  warning: t("Atenção"),
  error: t("Erro"),
  success: t("Concluído"),
};

function createToastRegion(container: HTMLElement) {
  const region = document.createElement('div');
  region.className = 'toast-region';
  region.innerHTML = '<div class="toast-announcement" role="status"></div>'
    + '<div class="toast-announcement" role="alert"></div>'
    + '<div class="toast notice" hidden><div class="toast-content"><strong></strong><p></p></div>'
    + `<button type="button" aria-label="${t("Dispensar notificação")}">×</button></div>`;
  container.append(region);
  const toast = region.querySelector<HTMLElement>('.toast')!;
  const title = toast.querySelector('strong')!;
  const message = toast.querySelector('p')!;
  const close = toast.querySelector('button')!;
  const polite = region.querySelector('[role="status"]')!;
  const alert = region.querySelector('[role="alert"]')!;
  let timeout: ReturnType<typeof setTimeout>;
  let previousFocus: HTMLElement | null = null;
  let temporary = false;

  const clear = () => {
    clearTimeout(timeout);
    if (document.activeElement === close) previousFocus?.focus({preventScroll: true});
    toast.hidden = true;
    message.textContent = '';
    polite.textContent = '';
    alert.textContent = '';
  };
  const schedule = () => {
    clearTimeout(timeout);
    if (!toast.hidden && temporary && !toast.matches(':hover, :focus-within')) timeout = setTimeout(clear, 6000);
  };
  close.onclick = clear;
  close.onfocus = event => {
    previousFocus = event.relatedTarget instanceof HTMLElement ? event.relatedTarget : null;
    schedule();
  };
  close.onblur = () => queueMicrotask(schedule);
  toast.onmouseenter = () => clearTimeout(timeout);
  toast.onmouseleave = schedule;
  container.addEventListener('close', clear);

  return {
    clear,
    show(text: string, kind: NoticeKind) {
      const unchanged = message.textContent === text && toast.dataset.notice === kind;
      toast.dataset.notice = kind;
      title.textContent = toastLabels[kind];
      message.textContent = text;
      toast.hidden = false;
      temporary = kind === 'info' || kind === 'success';
      if (!unchanged) {
        polite.textContent = temporary ? toastLabels[kind] + '. ' + text : '';
        alert.textContent = temporary ? '' : toastLabels[kind] + '. ' + text;
      }
      schedule();
    },
  };
}

const appToast = createToastRegion(document.body);
const dialogToasts = new Map(
  Array.from(document.querySelectorAll('dialog'), dialog => [dialog, createToastRegion(dialog)] as const),
);
const appHeader = document.querySelector<HTMLElement>('main > header')!;
const navigation = document.querySelector<HTMLElement>('.sidebar')!;
const positionToasts = () => {
  const bottom = Math.ceil(appHeader.getBoundingClientRect().bottom);
  document.documentElement.style.setProperty('--toast-header-bottom', bottom + 'px');
};
const toastLayoutObserver = new ResizeObserver(positionToasts);
toastLayoutObserver.observe(appHeader);
toastLayoutObserver.observe(navigation);
positionToasts();

function activeToast() {
  const dialog = document.querySelector<HTMLDialogElement>('#confirm-action[open]') ?? document.querySelector<HTMLDialogElement>('dialog[open]');
  return (dialog && dialogToasts.get(dialog)) || appToast;
}
function notify(text: string, kind: NoticeKind = 'info') {
  activeToast().show(text, kind);
}
function clearToast() {
  activeToast().clear();
}
async function task(action:()=>Promise<void>){
  if(busy)return;
  const focused=document.activeElement;
  busy=true;
  $('workspace').setAttribute('aria-busy','true');
  for(const button of document.querySelectorAll<HTMLButtonElement>('button:not(#confirm-action button, .toast button, .window-controls button)'))button.disabled=true;
  try {
    await action();
  } catch(error) {
    notify(String(error),'error');
  } finally {
    busy=false;
    $('workspace').removeAttribute('aria-busy');
    for(const button of document.querySelectorAll<HTMLButtonElement>('button'))button.disabled=false;
    syncPlan();
    syncPages();
    syncStaffPages();
    syncClubPages();
    syncPlayerDraft();
    syncStaffValidation();
    syncClubValidation();
    syncAuxiliaryValidation();
    $<HTMLButtonElement>('watchlist-delete').disabled=activeList==='Observação';
    if(document.activeElement===document.body&&focused instanceof HTMLElement)focused.focus({preventScroll:true});
  }
}
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
function staffDrafts():Edit[]{return staffDetail?Array.from(document.querySelectorAll<HTMLInputElement>('#staff-profile [data-staff-field]')).filter(el=>Number(el.value)!==Number(el.dataset.initial)).map(el=>({uid:staffDetail!.profile.uid,attribute:el.dataset.staffField!,value:Number(el.value)})):[];}
function staffHeaderDraft():StaffHeader|null {
  if(!staffDetail)return null;
  const header:StaffHeader={uid:staffDetail.profile.uid};let changed=false,rep=false;
  for(const el of document.querySelectorAll<HTMLInputElement>('#staff-profile [data-staff-header]'))if(Number(el.value)!==Number(el.dataset.initial)){
    changed=true;const key=el.dataset.staffHeader!;if(key==='currentAbility')header.currentAbility=Number(el.value);else if(key==='potentialAbility')header.potentialAbility=Number(el.value);else rep=true;
  }
  const policy=$<HTMLSelectElement>('staff-ca-policy');
  if(policy&&policy.value!==policy.dataset.initial&&staffDetail.pendingHeader?.currentAbility!=null){changed=true;header.currentAbility=staffDetail.profile.currentAbility;}
  if(header.currentAbility!==undefined&&policy?.value)header.policy=policy.value as StaffHeader['policy'];
  if(rep)header.reputation=[0,1,2].map(i=>Number($<HTMLInputElement>('staff-reputation-'+i).value));
  return changed?header:null;
}
function planCount(){return (plan.clubStadiumFieldCount??0)+Object.keys(plan.clubReputation??{}).length+Object.keys(plan.clubYouthSetup??{}).length+Object.keys(plan.clubTraining??{}).length+(plan.clubTransferBudgetFieldCount??0)+Object.keys(plan.clubBalances??{}).length+Object.keys(plan.clubWageBudgets??{}).length+plan.edits.length+(plan.staffEdits?.length??0)+(plan.staffHeaderEdits??[]).reduce((n,h)=>n+Number(h.currentAbility!=null)+Number(h.potentialAbility!=null)+(h.reputation?3:0),0)+plan.healInjuries.length+plan.clearSuspensions.length;}
function isClubDraft(field:HTMLInputElement){
  return field.value!==field.dataset.initial||field.validity.badInput;
}
function clubDrafts(){return Array.from(document.querySelectorAll<HTMLInputElement>('.club-fields-form input[data-initial]')).filter(isClubDraft);}
function hasFormDrafts(){
  return !!(clubDrafts().length||staffHeaderDraft()||staffDrafts().length||drafts().length||healingDraft()||suspensionDraft());
}
async function canLeave(){
  const club=clubDrafts();
  const staffClean=staffDrafts().length===0&&!staffHeaderDraft();
  const playerDirty=!!(drafts().length||healingDraft()||suspensionDraft());
  if(staffClean&&!club.length&&!playerDirty)return true;
  if(!await confirmAction(t("Descartar rascunhos?"), t("Os campos ainda não adicionados à revisão serão descartados."), t("Descartar")))return false;
  for(const field of club)field.value=field.dataset.initial!;
  if(club.length)syncClubValidation();
  if(!staffClean){
    for(const field of document.querySelectorAll<HTMLInputElement>('#staff-profile [data-staff-field],#staff-profile [data-staff-header]'))field.value=field.dataset.initial!;
    const policy=$<HTMLSelectElement>('staff-ca-policy');
    if(policy)policy.value=policy.dataset.initial??'';
  }
  if(playerDirty&&detail)renderProfile();
  return true;
}
function view(name:string){
  if(name==='diagnostics')renderPlayerDiagnostics();
  for(const section of ['my-club','players','staff','clubs','changes','diagnostics']){
    const active=section===name&&(!$('workspace').hidden);
    $(section+'-view').hidden=section!==name;
    const nav=$('nav-'+section);
    nav.classList.toggle('active',active);
    if(active)nav.setAttribute('aria-current','page');
    else nav.removeAttribute('aria-current');
  }
  const openSlot=$('empty').hidden?$('open-in-header'):$('open-in-start');
  if($('open-save').parentElement!==openSlot)openSlot.append($('open-save'));
}

const compactWorkspace=matchMedia('(max-width: 959px)');
const resultPositions=new WeakMap<HTMLElement,number>();

function prepareProfile(profile:HTMLElement){
  const grid=profile.parentElement!;
  const header=profile.querySelector<HTMLElement>('.profile-header')!;
  const title=header.querySelector('h2')!;
  title.tabIndex=-1;
  const back=document.createElement('button');
  back.type='button';
  back.className='back-to-results';
  back.textContent=t("Voltar à lista");
  back.onclick=()=>{
    delete grid.dataset.pane;
    const results=grid.querySelector<HTMLElement>('.results')!;
    results.scrollTop=resultPositions.get(results)??0;
    const selected=results.querySelector<HTMLElement>('.player-row.selected');
    if(selected){
      selected.focus({preventScroll:true});
      selected.scrollIntoView({block:'nearest',inline:'nearest'});
    }else grid.querySelector<HTMLElement>('input[type=search]')?.focus();
  };
  header.append(back);
}

function enterProfile(id:string, scrollTop=0){
  const profile=$(id);
  const results=profile.parentElement!.querySelector<HTMLElement>('.results')!;
  if(results.clientHeight)resultPositions.set(results,results.scrollTop);
  profile.parentElement!.dataset.pane='profile';
  profile.scrollTop=scrollTop;
  profile.querySelector<HTMLElement>('.profile-header h2')?.focus({preventScroll:true});
}

function clearPlayerSearchError(){
  $('player-search-error').textContent='';
  $('search').removeAttribute('aria-invalid');
  $('search').removeAttribute('aria-describedby');
}

function resultCount(count:number) {
  return count===1?t("1 resultado"):count.toLocaleString(locale)+t(" resultados");
}
function historyLabel(operation:ReviewOperation|null|undefined){
  if(!operation)return '';
  const [domain]=operation.domains;
  if(operation.domains.length===1&&domain==='Comissão'&&operation.staffFields?.length===1){
    return t(" edição de ")+staffFieldLabel(operation.staffFields[0]);
  }
  if(operation.domains.length===1&&domain==='Jogadores e contratos'&&operation.playerFields.length===1){
    const field=operation.playerFields[0];
    const label=fieldLabel(field);
    if(label!==field)return t(" edição de ")+label;
  }
  return ' · '+operation.domains.map(t).join(', ');
}

function syncPlan(){
  const count=planCount();
  const countLabel=count===1?t("1 alteração"):count.toLocaleString(locale)+t(" alterações");
  $('review-summary').textContent=countLabel+(count===1?t(" pendente"):t(" pendentes"));
  $('pending-count').textContent=String(count);
  $('review-count').textContent=String(count);
  $<HTMLButtonElement>('review').disabled=busy||!count;
  for(const id of ['save-copy','save-variant'])$<HTMLButtonElement>(id).disabled=busy||!count||validatedPlan!==plan;
  for(const id of ['export-plan-report','export-session-plan','clear'])$<HTMLButtonElement>(id).disabled=busy||!count;
  const historyActions=[
    {id:'undo',label:t("Desfazer"),shortcut:'Ctrl+Z',operation:plan.undoOperation},
    {id:'redo',label:t("Refazer"),shortcut:'Ctrl+Y',operation:plan.redoOperation},
  ] as const;
  for(const {id,label,shortcut,operation} of historyActions){
    const button=$<HTMLButtonElement>(id);
    button.disabled=busy||!plan[id];
    button.textContent=label+historyLabel(operation);
    button.title=button.textContent+' ('+shortcut+')';
  }
  $<HTMLButtonElement>('retry-review').disabled=busy;
  for(const section of ['players','staff','clubs','diagnostics'])$<HTMLButtonElement>('nav-'+section).disabled=busy||!summary;
  const myClubNav=$<HTMLButtonElement>('nav-my-club');
  myClubNav.hidden=!summary?.managedClubs?.length;
  myClubNav.disabled=busy||myClubNav.hidden;
  $<HTMLSelectElement>('my-club-select').disabled=busy;
  $<HTMLButtonElement>('my-club-undo').disabled=busy||!plan.undo;
  const myClubReview=$<HTMLButtonElement>('my-club-review');
  myClubReview.disabled=busy||!count;
  myClubReview.textContent=count?t("Ver revisão (")+count.toLocaleString(locale)+')':t("Ver revisão");
  syncMyClubShortcuts();
  $<HTMLButtonElement>('nav-changes').disabled=busy||(!summary&&$('saved-copy-history').hidden===true);
  $<HTMLButtonElement>('open-save').classList.toggle('primary',!summary);
  $('footer-state').textContent=count?countLabel+(count===1?t(" preparada."):t(" preparadas.")):t("Seu save original permanece intacto.");
}
function syncPages(){
  const unavailable=busy||$('results').hasAttribute('aria-busy');
  $<HTMLButtonElement>('previous').disabled=unavailable||page===0;
  $<HTMLButtonElement>('next').disabled=unavailable||(page+1)*50>=total;
  $('page-label').textContent=`${page+1} / ${Math.max(1,Math.ceil(total/50))}`;
  for(const id of ['batch-filtered','export-filtered-players','export-filtered-players-json']){
    $<HTMLButtonElement>(id).disabled=unavailable||total===0;
  }
}
const watchlistKey='cm0304.watchlists.v2';
const lists=new Map<string,number[]>([['Observação',[]]]);
let activeList='Observação';
const watchlist=new Set<number>();
try{
  const saved:unknown=JSON.parse(localStorage.getItem('cm0304.watchlist.v1')??'[]');
  if(Array.isArray(saved))lists.set(activeList,saved.filter(Number.isInteger));
  const stored=JSON.parse(localStorage.getItem(watchlistKey)??'null');
  if(Array.isArray(stored?.lists))for(const entry of stored.lists){
    if(Array.isArray(entry)&&typeof entry[0]==='string'&&Array.isArray(entry[1])){
      lists.set(entry[0],entry[1].filter(Number.isInteger));
    }
  }
  if(lists.has(stored?.active))activeList=stored.active;
  for(const uid of lists.get(activeList)??[])watchlist.add(uid);
}catch{/* Keep the editor usable if local storage is unavailable. */}
function saveWatchlists(next:Map<string,number[]>,active:string){
  try{localStorage.setItem(watchlistKey,JSON.stringify({lists:[...next],active}));}
  catch{notify(t("Não foi possível gravar as listas de observação."),'error');return false;}
  lists.clear();
  for(const [name,uids] of next)lists.set(name,uids);
  activeList=active;
  watchlist.clear();
  for(const uid of lists.get(active)??[])watchlist.add(uid);
  syncWatchlist();
  if(summary&&$<HTMLInputElement>('only-watchlist').checked){page=0;void task(search);}
  return true;
}
function currentSearchFilters(){
  return $<HTMLInputElement>('only-watchlist').checked?{...playerFilters,watchUids:Array.from(watchlist)}:playerFilters;
}
function syncWatchlist(){
  const select=$<HTMLSelectElement>('watchlist-select');
  select.replaceChildren(...[...lists.keys()].map(name=>new Option(name==='Observação'?t(name):name,name)));
  select.value=activeList;
  $<HTMLButtonElement>('watchlist-delete').disabled=activeList==='Observação';
  $('watch-count').textContent='('+watchlist.size+')';
  const button=document.getElementById('watch-player');
  if(button&&detail)button.textContent=watchlist.has(detail.player.uid)?t("Remover da observação"):t("Adicionar à observação");
}
let playerFilters:Record<string,number|string>={};
const resultColumns = [
  {key:'club',label:t("Clube"),width:'95px'},
  {key:'age',label:t("Idade"),width:'36px'},
  {key:'ca',label:'CA',width:'33px'},
  {key:'pa',label:'PA',width:'33px'},
  {key:'uid',label:'UID',width:'70px'},
] as const;
const resultColumnsKey = 'cm0304.player-columns.v1';
const defaultColumns = ['club','ca','pa'];
let visibleColumns = defaultColumns;
let resultRows: Page['rows'] = [];
try {
  const saved:unknown = JSON.parse(localStorage.getItem(resultColumnsKey) ?? 'null');
  if (Array.isArray(saved)) visibleColumns = resultColumns.filter(column => saved.includes(column.key)).map(column => column.key);
} catch { /* Use defaults when local preferences are unavailable. */ }

function renderPlayerResults() {
  if($('player-search-error').textContent)return;
  const columns = resultColumns.filter(column => visibleColumns.includes(column.key) && column.key !== 'club');
  const showClub = visibleColumns.includes('club');
  const results = $('results');
  results.style.setProperty('--result-columns', ['minmax(0,1fr)', ...columns.map(column=>column.width)].join(' '));
  results.style.setProperty('--result-min-width', 'calc('+['106px', ...columns.map(column=>column.width)].join(' + ')+')');
  const header = `<div class="table-label player-result-header"><span>${t("Jogador")}</span>`
    + columns.map(column => '<span>'+column.label+'</span>').join('') + '</div>';
  results.innerHTML = header + (resultRows.map(player => {
    const cells = columns.map(column => '<span>'+escape(String(player[column.key] ?? '—'))+'</span>').join('');
    return '<button class="player-row player-result-row '+(detail?.player.uid===player.uid?'selected':'')
      +'" data-uid="'+player.uid+'"><span class="result-identity"><strong>'+escape(player.name)+'</strong>'
      +(showClub?'<small>'+escape(player.club??t("Sem clube"))+'</small>':'')+'</span>'+cells+'</button>';
  }).join('') || `<p class="notice" data-notice="info">${t("Nenhum jogador encontrado.")}</p>`);
}

const resultColumnOptions = $('player-column-options');
resultColumnOptions.innerHTML = resultColumns.map(column => '<label><input type="checkbox" value="'+column.key+'"'
  +(visibleColumns.includes(column.key)?' checked':'')+'> '+column.label+'</label>').join('');
resultColumnOptions.onchange = () => {
  visibleColumns = Array.from(resultColumnOptions.querySelectorAll<HTMLInputElement>('input:checked'), input => input.value);
  try { localStorage.setItem(resultColumnsKey, JSON.stringify(visibleColumns)); }
  catch { notify(t("Não foi possível guardar as colunas para a próxima sessão."),'error'); }
  renderPlayerResults();
};

async function search() {
  renderActiveFilters();
  if(!summary)return;
  needsInitialPlayerSearch=false;
  clearPlayerSearchError();
  const generation=++searchGeneration;
  const query=$<HTMLInputElement>('search').value;
  const filters=currentSearchFilters();
  const preferences={query,filters:{...playerFilters},watchOnly:$<HTMLInputElement>('only-watchlist').checked};
  const cancel=$<HTMLButtonElement>('cancel-player-search');
  cancel.hidden=false;
  cancel.disabled=false;
  $('results').setAttribute('aria-busy','true');
  $('result-count').textContent=t("Buscando…");
  syncPages();
  try {
    const data=await invoke<Page|null>('search_players',{query,page,filters,generation});
    if(generation!==searchGeneration||!data)return;
    total=data.total;
    resultRows=data.rows;
    $('result-count').textContent=resultCount(total);
    renderPlayerResults();
    syncPages();
    try{localStorage.setItem(lastPlayerSearchKey,JSON.stringify(preferences));}
    catch{/* A storage failure does not affect results. */}
  } catch(error) {
    if(generation!==searchGeneration)return;
    $('result-count').textContent=t("Falha na busca");
    total=0;
    page=0;
    resultRows=[];
    $('results').replaceChildren();
    const message=String(error);
    const invalidUid=message.includes(t("UID de jogador inválido"));
    const input=$('search');
    if(invalidUid)input.setAttribute('aria-invalid','true');
    input.setAttribute('aria-describedby','player-search-error');
    const feedback=invalidUid
      ?t("UID de jogador inválido. Use uid: seguido do número, como uid:600.")
      :message;
    $('player-search-error').textContent=feedback;
    notify(feedback,'error');
    syncPages();
  } finally {
    if(generation===searchGeneration){
      cancel.hidden=true;
      $('results').removeAttribute('aria-busy');
      syncPages();
    }
  }
}
$('cancel-player-search').onclick=()=>{
  clearTimeout(timer);
  const generation=++searchGeneration;
  $('cancel-player-search').hidden=true;
  $('results').removeAttribute('aria-busy');
  $('result-count').textContent=t("Busca cancelada");
  total=0;
  page=0;
  resultRows=[];
  $('results').innerHTML=notice('info',t("Altere a pesquisa ou os filtros para buscar novamente."));
  notify(t("Busca cancelada."));
  syncPages();
  void invoke('cancel_player_search',{generation}).catch(error=>notify(String(error),'error'));
};
function fieldLabel(key:string):string {
  if(key===marketField)return t("Contrato 1 · Situação de mercado");
  const extra:Record<string,string>={currentAbility:t("Habilidade atual (CA)"),potentialAbility:t("Potencial (PA)"),height:t("Altura (cm)"),weight:t("Peso (kg)"),nationality:t("Nacionalidade principal"),'birth.year':t("Ano de nascimento"),'birth.day':t("Dia ordinal de nascimento"),'reputation.home':t("Reputação nacional"),'reputation.current':t("Reputação atual"),'reputation.world':t("Reputação mundial"),adaptability:t("Adaptação"),ambition:t("Ambição"),loyalty:t("Lealdade"),pressure:t("Pressão"),professionalism:t("Profissionalismo"),sportsmanship:t("Espírito esportivo"),temperament:t("Temperamento"),controversy:t("Controvérsia")};
  const career:Record<string,string>={'state.condition':t("Condição física (%)"),'state.fitness':t("Ritmo de jogo (%)"),'state.jadedness':t("Cansaço acumulado"),'state.morale':t("Moral (0–20)"),'state.estimatedValue':t("Valor estimado (£)"),'state.saleValue':t("Preço pedido (£)"),weeklyWage:t("Salário semanal (£)"),appearanceBonus:t("Bônus por presença (£)"),goalBonus:t("Bônus por gol (£)"),assistBonus:t("Bônus por assistência (£)"),releaseClause:t("Cláusula de rescisão (£)"),squadStatus:t("Situação no elenco"),squadNumber:t("Camisa (-1 sem número)"),'expires.year':t("Término"),'expires.day':t("Término")};
  if(key.startsWith('contract.')){const [,i,...parts]=key.split('.');return t("Contrato ")+(Number(i)+1)+' · '+(career[parts.join('.')]??parts.join('.'));}
  const simple=key.replace(/^position\.|^personality\./,'');return career[key]??extra[key]??extra[simple]??labels[simple]??key;
}
function syncStaffPages(){
  $<HTMLButtonElement>('staff-previous').disabled=busy||staffPage===0;
  $<HTMLButtonElement>('staff-next').disabled=busy||(staffPage+1)*50>=staffTotal;
  $('staff-page-label').textContent=`${staffPage+1} / ${Math.max(1,Math.ceil(staffTotal/50))}`;
}
function resetStaff(){
  staffDetail=null;staffPage=0;staffTotal=0;staffGeneration++;clearTimeout(staffTimer);staffTimer=undefined;
  $('staff-results').removeAttribute('aria-busy');
  $<HTMLInputElement>('staff-search').value='';$('staff-results').innerHTML='';$('staff-result-count').textContent=t("0 resultados");
  $('staff-count').textContent=(summary?.staff??0).toLocaleString(locale);
  $('staff-profile').innerHTML=`<div class="profile-placeholder"><h2>${t("Escolha um membro do staff")}</h2><p>${t("Consulte funções, vínculos e capacidade.")}</p></div>`;
  syncStaffPages();
}
async function searchStaff(){
  if(!summary)return;
  const generation=++staffGeneration;
  const result=await invoke<StaffPage>('search_staff',{query:$<HTMLInputElement>('staff-search').value,page:staffPage});
  if(generation!==staffGeneration)return;
  staffTotal=result.total;$('staff-result-count').textContent=resultCount(staffTotal);
  $('staff-results').innerHTML=result.rows.map(p=>'<button class="player-row '+(staffDetail?.profile.uid===p.uid?'selected':'')+'" data-staff-uid="'+p.uid+'"><span><strong>'+escape(p.name)+'</strong><small>'+escape(p.team??p.nationalTeam??t("Sem equipe"))+' · '+escape(t(p.role))+(p.isPlayer?t(" · Jogador/staff"):'')+'</small></span><span class="abilities">'+p.ca+' <span>·</span> '+p.pa+'</span></button>').join('')||`<p class="notice" data-notice="info">${t("Nenhum membro do staff encontrado.")}</p>`;
  syncStaffPages();
}
let staffTab='ratings';
async function showStaff(uid:number,checkDraft=true){
  if(checkDraft&&!await canLeave())return false;
  notify(t("Carregando perfil de staff…"));
  staffDetail=await invoke<StaffDetail>('staff_detail',{uid});
  const d=staffDetail,p=d.profile;
  const fact=(label:string,value:unknown)=>'<div><dt>'+escape(label)+'</dt><dd>'+escape(value??'—')+'</dd></div>';
  const birth=p.dateOfBirth&&d.age!==null?new Date(Date.UTC(p.dateOfBirth.year,0,1)+p.dateOfBirth.day*86400000).toLocaleDateString(locale,{timeZone:'UTC'}):'—';
  const professional:[string,string][]=[[t("Treino de jogadores de linha"),'coachingOutfieldPlayers'],[t("Treino de goleiros"),'coachingGoalkeepers'],[t("Fisioterapia"),'physiotherapy'],[t("Conhecimento tático"),'tacticalKnowledge'],[t("Trabalho com jovens"),'workingWithYoungsters'],[t("Gestão de pessoas"),'manManagement'],[t("Avaliação de capacidade"),'judgingPlayerAbility'],[t("Avaliação de potencial"),'judgingPlayerPotential'],[t("Motivação"),'motivating'],[t("Disciplina"),'discipline'],[t("Determinação"),'determination']];
  const headerField=(label:string,key:string,value:number,max:number)=>'<div><dt><label for="staff-'+key+'">'+escape(label)+'</label></dt><dd><input id="staff-'+key+'" type="number" required min="0" max="'+max+'" step="1" data-staff-header="'+key+'" data-initial="'+value+'" value="'+value+'"></dd></div>';
  const policy=d.pendingHeader?.policy??'';
  const headerForm=(p.playerCurrentAbility!=null?`<p class="notice" data-notice="info">${t("Dupla função: os atributos profissionais usam a CA de jogador")} `+p.playerCurrentAbility+`${t(". Alterar a CA de staff preserva essa base.")}</p>`:'')+`<h3 class="tab-heading">${t("Capacidade e reputação de staff")}</h3><dl class="staff-facts">`+headerField('CA','currentAbility',p.currentAbility,200)+headerField(t("PA fixa"),'potentialAbility',p.potentialAbility,200)+p.reputation.map((v,i)=>headerField([t("Reputação nacional"),t("Reputação atual"),t("Reputação mundial")][i],'reputation-'+i,v,10000)).join('')+`</dl><div id="staff-policy-box" class="notice policy-box" data-notice="warning" hidden><label for="staff-ca-policy">${t("Quer manter os atributos atuais ou recalculá-los?")}</label><select id="staff-ca-policy" data-initial="`+policy+`"><option value="">${t("Selecione uma opção")}</option><option value="preserveValues" `+(policy==='preserveValues'?'selected':'')+`>${t("Manter os atributos atuais")}</option><option value="preserveRelative" `+(policy==='preserveRelative'?'selected':'')+`>${t("Recalcular os atributos")}</option></select><p class="notice" data-notice="info">${t("A revisão mostra os efeitos da mudança de CA.")}</p></div>`;
  const personalityRatings=`<h3 class="tab-heading">${t("Personalidade")}</h3>`+(p.isPlayer?`<p class="notice" data-notice="info">${t("Esses dados são compartilhados com o perfil de jogador.")}</p>`:'')+'<dl class="staff-facts staff-personality">'+Object.entries(p.personality??{}).map(([key,value])=>{const field='personality.'+key;return d.editable?.includes(field)?'<div><dt><label for="staff-'+field+'">'+escape(fieldLabel(field))+'</label></dt><dd><input id="staff-'+field+'" type="number" required min="1" max="20" step="1" data-staff-field="'+field+'" data-initial="'+value+'" value="'+value+'"></dd></div>':fact(fieldLabel(field),value);}).join('')+'</dl>';
  const ratings=`<h3 class="tab-heading">${t("Atributos profissionais")}</h3><dl class="staff-facts staff-ratings">`+professional.map(([label,key])=>d.editable?.includes(key)?'<div><dt><label for="staff-'+key+'">'+escape(label)+'</label></dt><dd><input id="staff-'+key+'" type="number" required min="1" max="20" step="1" data-staff-field="'+key+'" data-initial="'+p.attributes?.[key]+'" value="'+p.attributes?.[key]+'"></dd></div>':fact(label,p.attributes?.[key])).join('')+'</dl>';
  const tabs=[['ratings',t("Atributos"),ratings],['ability',t("CA / PA e reputação"),headerForm],
    ['personality',t("Personalidade"),personalityRatings],['details',t("Dados e vínculos"),
      '<dl class="staff-facts">'+fact(t("Função no clube"),t(p.clubJob))+fact(t("Clube"),p.team)+fact(t("Seleção"),p.nationalTeam)+fact(t("Função na seleção"),t(p.nationalJob))+fact(t("Nacionalidade"),p.nation)+fact(t("Nascimento"),birth)+fact(t("Perfil"),p.isHuman?t("Treinador humano"):p.isPlayer?t("Jogador/staff"):'Staff')+`</dl><details><summary>${t("Atributos brutos de diagnóstico")}</summary><pre>`+escape(JSON.stringify(p.rawAttributes,null,2))+'</pre></details>']];
  const tabbar=`<div class="profile-tabs" role="tablist" aria-label="${t("Perfil de Staff")}">`+tabs.map(([id,label])=>
    '<button type="button" role="tab" id="staff-tab-'+id+'" aria-controls="staff-panel-'+id+'">'+label+'</button>').join('')+'</div>';
  const panels=tabs.map(([id,,content])=>'<section role="tabpanel" id="staff-panel-'+id+'" aria-labelledby="staff-tab-'+id+'">'+content+'</section>').join('');
  const footer=`<div class="profile-actions"><small id="staff-draft-state" role="status"></small><button id="go-to-staff-error" type="button" hidden>${t("Ir ao campo")}</button><button id="staff-stage" type="button" class="primary">${t("Adicionar à revisão")}</button></div>`;
  $('staff-profile').innerHTML='<div class="profile-header"><div><div class="eyebrow">STAFF · UID '+p.uid+'</div><h2>'+escape(p.name)+'</h2><div class="profile-meta">'+escape(p.team??p.nationalTeam??t("Sem equipe"))+((p.clubJob||p.nationalJob)?' · '+escape(t(p.clubJob||p.nationalJob)):'')+(d.age===null?'':' · '+d.age+t(" anos"))+'</div></div><div class="profile-badges"><div class="ability-badge"><small>CA</small><strong>'+p.currentAbility+'</strong></div><div class="ability-badge"><small>PA</small><strong>'+p.potentialAbility+'</strong></div></div></div>'
    +(p.isPlayer?`<div class="staff-profile-links"><button type="button" id="staff-open-player">${t("Abrir perfil de jogador →")}</button></div>`:'')
    +(d.pendingPlayerFields?notice('info',t("Este jogador tem ")+d.pendingPlayerFields+t(" campo(s) na revisão.")):'')
    +tabbar+panels+footer;
  const selectStaffTab=(id:string,focus=false)=>{
    staffTab=id;
    for(const [key] of tabs){
      const active=key===id;
      const tab=$('staff-tab-'+key);
      tab.setAttribute('aria-selected',String(active));
      tab.tabIndex=active?0:-1;
      $('staff-panel-'+key).hidden=!active;
      if(active&&focus)tab.focus({preventScroll:true});
    }
    $('staff-profile').scrollTop=0;
  };
  for(const [id] of tabs){
    const tab=$('staff-tab-'+id);
    tab.onclick=()=>selectStaffTab(id);
    tab.onkeydown=event=>{
      const index=tabs.findIndex(([key])=>key===id);
      const next=event.key==='Home'?0:event.key==='End'?tabs.length-1:
        event.key==='ArrowRight'?(index+1)%tabs.length:event.key==='ArrowLeft'?(index+tabs.length-1)%tabs.length:-1;
      if(next<0)return;
      event.preventDefault();
      selectStaffTab(tabs[next][0],true);
    };
  }
  selectStaffTab(staffTab);
  for(const input of $('staff-profile').querySelectorAll<HTMLInputElement>('input[type=number][data-initial]')){
    const original=d.recorded??p;
    const key=input.dataset.staffField;
    const header=input.dataset.staffHeader;
    const recorded=key?.startsWith('personality.')?original.personality?.[key.slice(12)]
      :key?original.attributes?.[key]
      :header==='currentAbility'?original.currentAbility
      :header==='potentialAbility'?original.potentialAbility
      :original.reputation[Number(header?.slice(11))];
    input.dataset.originalValue=String(recorded??input.dataset.initial);
    input.insertAdjacentHTML('afterend','<small class="attribute-difference" hidden></small>');
  }
  $('go-to-staff-error').onclick=()=>{
    const input=$('staff-profile').querySelector<HTMLInputElement>('input[aria-invalid=true]');
    if(!input)return;
    const panel=input.closest<HTMLElement>('[role=tabpanel]');
    if(panel)selectStaffTab(panel.id.replace('staff-panel-',''));
    input.focus();
  };
  $('staff-profile').scrollTop=0;
  for(const row of document.querySelectorAll<HTMLElement>('[data-staff-uid]'))row.classList.toggle('selected',Number(row.dataset.staffUid)===uid);
  prepareProfile($('staff-profile'));
  $('staff-stage').onclick=()=>void task(async()=>{
    const edits=staffDrafts(),header=staffHeaderDraft();
    if(syncStaffValidation()||(!edits.length&&!header))return;
    if(header?.currentAbility!==undefined&&!header.policy)throw Error(t("Escolha uma política para a CA de staff."));
    notify(t("Validando atributos e capacidade de staff…"));
    const scrollTop=$('staff-profile').scrollTop;
    plan=await invoke<Pending>('stage_staff_edits',{edits,header});
    await showStaff(uid,false);
    enterProfile('staff-profile',scrollTop);
    notify(t("Atributos de staff adicionados à revisão."),'success');
  });
  if(p.isPlayer)$('staff-open-player').onclick=()=>void task(async()=>{if(!await canLeave())return;view('players');page=0;$<HTMLInputElement>('search').value='uid:'+uid;await search();await showPlayer(uid,false);enterProfile('profile');});
  syncStaffValidation();
  notify(t("Perfil de staff carregado."),'success');
  return true;
}
let comparisonReference:{uid:number;name:string}|null=null;
function comparisonFields(a:Player,b:Player){
  const positions=[...playerPositionKeys,'rightSide','leftSide','central'];
  const keys=[...new Set([
    'currentAbility','potentialAbility',
    ...Object.keys(labels).map(key=>positions.includes(key)?'position.'+key:key),
    ...[...Object.keys(a.fields),...Object.keys(b.fields)].filter(key=>key.startsWith('personality.')||key.startsWith('reputation.')),
  ])];
  return keys.flatMap(key=>{
    const value=(p:Player)=>key==='currentAbility'?p.currentAbility:key==='potentialAbility'?p.potentialAbility:p.fields[key]?.value;
    const x=value(a),y=value(b);
    if(x==null&&y==null)return [];
    const difference=x==null||y==null||(key==='potentialAbility'&&(x<0||y<0))?null:y-x;
    return [{key,label:fieldLabel(key),left:x??null,right:y??null,difference}];
  });
}
type PlayerComparisonField=ReturnType<typeof comparisonFields>[number];
const comparisonDifference=new Intl.NumberFormat(locale,{signDisplay:'exceptZero'});
const playerComparisonTabs=[
  ['overview',t("Visão geral")],['positions',t("Posições")],['attributes',t("Atributos")],['contracts',t("Contratos")],
] as const;

function comparisonTable(grouped:[string,PlayerComparisonField[]][],columnLabels:[string,string],section:string,onlyDifferences:boolean){
  const sections=grouped.flatMap(([name,rows])=>{
    const visible=onlyDifferences?rows.filter(row=>row.left!==row.right):rows;
    if(!visible.length)return [];
    return '<tbody>'+(name?'<tr class="comparison-row-group"><th scope="rowgroup" colspan="4">'+escape(name)+'</th></tr>':'')
      +visible.map(({label,left,right,difference})=>
        '<tr><th scope="row">'+escape(label)+'</th><td>'+escape(left??'—')+'</td><td>'+escape(right??'—')
        +'</td><td class="comparison-difference" data-direction="'+(difference==null||difference===0?'neutral':difference>0?'increase':'decrease')+'">'
        +(difference==null?'—':comparisonDifference.format(difference))+'</td></tr>').join('')+'</tbody>';
  }).join('');
  if(!sections)return '<p class="comparison-empty">'+(onlyDifferences?t("Nenhuma diferença nesta seção."):t("Nenhum dado disponível nesta seção."))+'</p>';
  return '<table class="player-compare-table" aria-label="'+escape(section)+`"><thead><tr><th scope="col">${t("Campo")}</th>`
    +columnLabels.map(label=>'<th scope="col">'+escape(label)+'</th>').join('')
    +`<th scope="col">${t("Diferença")}</th></tr></thead>`+sections+'</table>';
}

interface SavePlayerComparison {
  current:Player;other:Player;currentName:string;otherName:string;
  currentDate:{year:number;day:number};otherDate:{year:number;day:number};
  currentContracts:ContractView[];otherContracts:ContractView[];
}
function comparisonDate(value:{year:number;day:number}):string {
  return new Date(Date.UTC(value.year,0,1)+value.day*86400000).toLocaleDateString(locale,{timeZone:'UTC'});
}
interface RosterMember {uid:number;name:string}
interface RosterContract {kind:number;entity:string|null;entityUid?:number|null;weeklyWageGbp:number;monthly:boolean;expires:[number,number]|null;appearanceBonusGbp?:number|null;goalBonusGbp?:number|null;assistBonusGbp?:number|null;releaseClauseGbp?:number|null;squadNumber?:number;squadStatus?:number;transferStatus?:number|null}
type SquadTerms={squadNumber?:number|null;squadStatus?:number|null};
function rosterSquadNumber(contract:SquadTerms):number|string|null {
  return contract.squadNumber===-1?t("Sem número"):contract.squadNumber??null;
}
function rosterSquadStatus(contract:SquadTerms):string|null {
  return contract.squadStatus==null?null:squadStatusLabels[contract.squadStatus+1]??String(contract.squadStatus);
}
function contractSquadSummary(contract:SquadTerms):string {
  const number=rosterSquadNumber(contract);
  return [number==null?null:t("Camisa: ")+number,rosterSquadStatus(contract)].filter(Boolean).join(' · ');
}
const contractIncentives=[['appearanceBonusGbp','appearanceBonus',t("Presença")],['goalBonusGbp','goalBonus',t("Gol")],['assistBonusGbp','assistBonus',t("Assistência")],['releaseClauseGbp','releaseClause',t("Rescisão")]] as const;
type ContractIncentives=Pick<RosterContract,typeof contractIncentives[number][0]>;
function contractIncentiveSummary(contract:ContractIncentives,money:Intl.NumberFormat):string {
  return contractIncentives.flatMap(([key,,label])=>{
    const amount=contract[key];
    return amount==null?[]:[label+' '+money.format(amount)];
  }).join(' · ');
}
interface RosterChange extends RosterMember {
  attributes:{field:string;before:number|null;after:number|null}[];
  contractsBefore:RosterContract[]|null;
  contractsAfter:RosterContract[]|null;
}
interface ClubSaveComparison {
  sourceName:string;otherName:string;
  report:{version:number;clubUid:number;beforeClubName:string;afterClubName:string;
    beforeDate:{year:number;day:number};afterDate:{year:number;day:number};
    joined:RosterMember[];left:RosterMember[];changed:RosterChange[];unchanged:number};
}
function rosterContractType(kind:number):string {
  return [t("Completo"),t("Empréstimo"),t("Teste")][kind]??(kind===5?t("Liga"):t("Tipo ")+kind);
}
function rosterContractExpiry(contract:RosterContract):string|null {
  if(contract.monthly)return t("Mensal");
  return contract.expires?comparisonDate({year:contract.expires[0],day:contract.expires[1]}):null;
}
function rosterComparisonCsv({report,sourceName,otherName}:ClubSaveComparison):string {
  const rows:(string|number|null)[][]=[[
    t("UID do clube"),t("Clube na origem"),t("Clube no segundo save"),t("Save de origem"),t("Data de origem"),t("Segundo save"),t("Data do segundo save"),
    t("Tipo"),t("UID do jogador"),t("Jogador"),t("Campo"),t("Antes"),t("Depois"),t("Vínculo"),t("Entidade"),t("Salário semanal (£)"),t("Vencimento"),t("UID da entidade"),
    t("Bônus por presença (£)"),t("Bônus por gol (£)"),t("Bônus por assistência (£)"),t("Cláusula de rescisão (£)"),t("Camisa"),t("Situação no elenco"),t("Situação de mercado"),
  ]];
  const metadata=[report.clubUid,report.beforeClubName,report.afterClubName,sourceName,comparisonDate(report.beforeDate),otherName,comparisonDate(report.afterDate)];
  for(const [label,players] of [[t("Entrada"),report.joined],[t("Saída"),report.left]] as const){
    for(const {uid,name} of players)rows.push([...metadata,label,uid,name]);
  }
  for(const player of report.changed){
    for(const {field,before,after} of player.attributes){
      rows.push([...metadata,t("Atributo"),player.uid,player.name,field,before,after]);
    }
    for(const [label,contracts] of [[t("Contrato na origem"),player.contractsBefore],[t("Contrato no segundo save"),player.contractsAfter]] as const){
      if(!contracts)continue;
      if(!contracts.length)rows.push([...metadata,label,player.uid,player.name,null,null,null,t("Sem vínculo")]);
      for(const contract of contracts){
        rows.push([...metadata,label,player.uid,player.name,null,null,null,
          rosterContractType(contract.kind),contract.entity,contract.weeklyWageGbp,rosterContractExpiry(contract),contract.entityUid??null,
          contract.appearanceBonusGbp??null,contract.goalBonusGbp??null,contract.assistBonusGbp??null,contract.releaseClauseGbp??null,
          rosterSquadNumber(contract),rosterSquadStatus(contract),rosterMarketStatus(contract)]);
      }
    }
  }
  rows.push([...metadata,t("Jogadores sem diferenças"),null,null,'Total',report.unchanged]);
  const quote=(value:string|number|null)=>{
    if(value==null)return '';
    if(typeof value==='number')return String(value);
    const safe=/^[\s\uFEFF]*[=+@-]/.test(value)||/^[\t\r\n]/.test(value)?"'"+value:value;
    return '"'+safe.replace(/"/g,'""')+'"';
  };
  return '\uFEFF'+rows.map(row=>rows[0].map((_,index)=>quote(row[index]??null)).join(';')).join('\r\n')+'\r\n';
}
async function compareClubSave(uid:number){
  if(!await canLeave())return;
  await task(async()=>{
    const comparison=await compareSaves<ClubSaveComparison>('compare_club_save',uid);
    if(!comparison)return;
    const {report,sourceName,otherName}=comparison;
    const members=(title:string,entries:RosterMember[])=>'<h3>'+title+' · '+entries.length+'</h3>'
      +(entries.length?'<ul>'+entries.map(entry=>'<li>'+escape(entry.name)+' · UID '+entry.uid+'</li>').join('')+'</ul>':'');
    const money=new Intl.NumberFormat(locale,{style:'currency',currency:'GBP',maximumFractionDigits:0});
    const contracts=(title:string,list:RosterContract[])=>'<section><h4>'+title+'</h4>'
      +(list.length?'<ul>'+list.map(contract=>{
        const terms=[contractSquadSummary(contract),contractIncentiveSummary(contract,money),rosterMarketStatus(contract)].filter(Boolean).join(' · ');
        return '<li><strong>'+escape(rosterContractType(contract.kind))+'</strong> · '
          +escape(contract.entity??t("Entidade desconhecida"))+' · '
          +escape(money.format(contract.weeklyWageGbp))+t("/semana · ")+escape(rosterContractExpiry(contract)??'—')
          +(terms?'<br>'+escape(terms):'')+'</li>';
      }).join('')+'</ul>':`<p class="notice" data-notice="info">${t("Nenhum vínculo registrado.")}</p>`)+'</section>';
    $('club-comparison-content').innerHTML='<h2>'+escape(report.beforeClubName)+'</h2><p>'
      +escape(sourceName)+' · '+comparisonDate(report.beforeDate)+' → '+escape(otherName)+' · '+comparisonDate(report.afterDate)
      +`</p><p class="notice" data-notice="info">${t("Comparação dos valores gravados nos dois saves.")}</p>`
      +members(t("Entradas"),report.joined)+members(t("Saídas"),report.left)
      +`<h3>${t("Jogadores com diferenças ·")} `+report.changed.length+'</h3>'
      +report.changed.map(player=>'<details><summary>'+escape(player.name)+' · UID '+player.uid+'</summary>'
        +player.attributes.map(change=>'<div class="change-item"><span>'+escape(fieldLabel(change.field))+'</span><b>'
          +escape(change.before??'—')+' → '+escape(change.after??'—')+'</b></div>').join('')
        +(player.contractsBefore&&player.contractsAfter?'<div class="compare-contracts">'
          +contracts(t("Contratos na origem"),player.contractsBefore)+contracts(t("Contratos no segundo save"),player.contractsAfter)+'</div>':'')
        +'</details>').join('')+`<p class="notice" data-notice="info">${t("Sem diferenças nos campos comparados ·")} `+report.unchanged+'</p>';
    $('export-club-comparison').onclick=()=>void task(async()=>{
      const json=JSON.stringify({...report,sourceName,otherName},null,2);
      const path=await invoke<string|null>('export_club_comparison',{json});
      if(path)notify(t("Comparação exportada: ")+path,'success');
    });
    $('export-club-comparison-csv').onclick=()=>void task(async()=>{
      const path=await invoke<string|null>('export_club_comparison_csv',{csv:rosterComparisonCsv(comparison)});
      if(path)notify(t("Comparação exportada: ")+path,'success');
    });
    $<HTMLDialogElement>('club-comparison').showModal();
    notify(t("Comparação de elenco concluída."),'success');
  });
}
const marketStatuses:Readonly<Partial<Record<number,string>>>={
  4:t("Transferência e empréstimo sem situação definida"),
  5:t("Listado para transferência · Empréstimo sem situação definida"),
  6:t("Transferência sem situação definida · Disponível para empréstimo"),
  7:t("Listado para transferência · Disponível para empréstimo"),
  68:t("Transferência sem situação definida · Clube não deseja emprestar"),
  69:t("Listado para transferência · Clube não deseja emprestar"),
};
const marketField='contract.0.transferStatus';
function rosterMarketStatus(contract:RosterContract):string|null {
  const code=contract.transferStatus;
  return code==null?null:marketStatuses[code]??t("Código de mercado ")+code;
}
function marketStatus(contract:ContractView):string|undefined {
  return contract.kind===0&&contract.entityType===0?marketStatuses[contract.transferStatus]:undefined;
}
function comparisonContractData(player:Player,contracts:ContractView[]){
  return contracts.map(contract=>{
    const prefix='contract.'+contract.index+'.';
    const year=player.fields[prefix+'expires.year']?.value;
    const day=player.fields[prefix+'expires.day']?.value;
    const incentives:ContractIncentives=Object.fromEntries(contractIncentives.map(([key,field])=>[key,player.fields[prefix+field]?.value??null]));
    return {...contract,marketStatus:marketStatus(contract)??null,weeklyWageGbp:player.fields[prefix+'weeklyWage']?.value??null,
      squadNumber:player.fields[prefix+'squadNumber']?.value??null,
      squadStatus:player.fields[prefix+'squadStatus']?.value??null,
      ...incentives,
      expires:!contract.monthly&&year!=null&&day!=null?{year,day}:null};
  });
}
function comparisonContracts(player:Player,contracts:ContractView[],heading:string):string {
  const format=new Intl.NumberFormat(locale);
  const money=new Intl.NumberFormat(locale,{style:'currency',currency:'GBP',maximumFractionDigits:0});
  const items=comparisonContractData(player,contracts).map(contract=>{
    const wage=contract.weeklyWageGbp;
    const duration=contract.monthly?t("Mensal"):contract.expires?comparisonDate(contract.expires):'—';
    const terms=[contractSquadSummary(contract),contractIncentiveSummary(contract,money)].filter(Boolean).join(' · ');
    return '<li><strong>'+escape(t(contract.kindLabel))+'</strong> · '+escape(contract.entity??t("Entidade desconhecida"))
      +`<dl><div><dt>${t("Salário")}</dt><dd>`+(wage==null?'—':'£'+format.format(wage)+t('/semana'))+'</dd></div>'
      +`<div><dt>${t("Vencimento")}</dt><dd>`+escape(duration)+'</dd></div></dl>'
      +(terms?'<small>'+escape(terms)+'</small>':'')
      +(contract.marketStatus?`<small>${t("Situação de mercado:")} `+escape(contract.marketStatus)+'</small>':'')+'</li>';
  }).join('');
  return '<section><h4>'+escape(heading)+'</h4>'+(items?'<ul>'+items+'</ul>':`<p class="notice" data-notice="info">${t("Nenhum vínculo registrado.")}</p>`)+'</section>';
}

function openPlayerComparison(left:Player,right:Player,leftContracts:ContractView[],rightContracts:ContractView[],saves?:SavePlayerComparison){
  const dialog=$<HTMLDialogElement>('player-comparison');
  const menu=document.querySelector<HTMLDetailsElement>('#profile .player-compare-menu');
  if(menu)menu.open=false;
  const columns:[string,string]=saves?[t("Save ativo"),t("Outro save")]:[t("Referência"),t("Comparado")];
  const headings:[string,string]=saves
    ?[saves.currentName+' · '+comparisonDate(saves.currentDate),saves.otherName+' · '+comparisonDate(saves.otherDate)]
    :[left.name,right.name];
  const card=(player:Player,index:0|1)=>'<div class="comparison-identity"><span>'+columns[index]+'</span>'
    +'<h3>'+escape(player.name)+'</h3>'
    +'<p>'+escape(saves?headings[index]:player.club??t("Sem clube"))+'</p>'
    +'<small>UID '+player.uid+'</small></div>';
  const fields=comparisonFields(left,right);
  const inOverview=(key:string)=>key==='currentAbility'||key==='potentialAbility'||key.startsWith('reputation.');
  const inPositions=(key:string)=>key.startsWith('position.')||key==='leftFoot'||key==='rightFoot';
  const attributes=fields.filter(({key})=>!inOverview(key)&&!inPositions(key));
  const attributeGroups:[string,string[]][]=[...groups,[t("Goleiro"),keeper],[t("Personalidade"),personalityFields]];
  const groupedAttributes:[string,PlayerComparisonField[]][]=attributeGroups.map(([label,keys])=>[label,attributes.filter(({key})=>keys.includes(key))]);
  const knownAttributes=new Set(attributeGroups.flatMap(([,keys])=>keys));
  groupedAttributes.push([t("Complementares"),attributes.filter(({key})=>!knownAttributes.has(key))]);
  const groupsByTab={
    overview:[['',fields.filter(({key})=>inOverview(key))]],
    positions:[['',fields.filter(({key})=>inPositions(key))]],
    attributes:groupedAttributes,
  } satisfies Record<string,[string,PlayerComparisonField[]][]>;
  $('player-comparison-content').innerHTML='<div class="comparison-identities">'+card(left,0)+card(right,1)+'</div>'
    +`<div class="profile-tabs comparison-tabs" role="tablist" aria-label="${t("Seções da comparação")}">`
    +playerComparisonTabs.map(([id,label])=>'<button type="button" role="tab" id="comparison-tab-'+id+'" aria-controls="comparison-panel-'+id+'">'+label+'</button>').join('')+'</div>'
    +`<div id="comparison-options"><span>${t("Diferença:")} `+columns[1]+' − '+columns[0]+'</span>'
    +`<label><input type="checkbox" id="comparison-only-differences"> ${t("Somente diferenças")}</label></div>`
    +`<div id="comparison-scroll" tabindex="0" role="region" aria-label="${t("Dados da comparação")}">`
    +playerComparisonTabs.map(([id])=>'<section role="tabpanel" id="comparison-panel-'+id+'" aria-labelledby="comparison-tab-'+id+'"></section>').join('')+'</div>';
  $('comparison-panel-contracts').innerHTML='<div class="compare-contracts">'
    +comparisonContracts(left,leftContracts,headings[0])+comparisonContracts(right,rightContracts,headings[1])+'</div>';
  let active:typeof playerComparisonTabs[number][0]='overview';
  const filter=$<HTMLInputElement>('comparison-only-differences');
  const renderTable=()=>{
    if(active==='contracts')return;
    const title=playerComparisonTabs.find(([id])=>id===active)![1];
    $('comparison-panel-'+active).innerHTML=comparisonTable(groupsByTab[active],columns,title,filter.checked);
  };
  const selectTab=(index:number,focus=false)=>{
    active=playerComparisonTabs[index][0];
    for(const [id] of playerComparisonTabs){
      const tab=$('comparison-tab-'+id);
      tab.setAttribute('aria-selected',String(id===active));
      tab.tabIndex=id===active?0:-1;
      $('comparison-panel-'+id).hidden=id!==active;
    }
    $('comparison-options').hidden=active==='contracts';
    renderTable();
    $('comparison-scroll').scrollTop=0;
    if(focus)$('comparison-tab-'+active).focus();
  };
  for(const [index,[id]] of playerComparisonTabs.entries()){
    const tab=$('comparison-tab-'+id);
    tab.onclick=()=>selectTab(index);
    tab.onkeydown=event=>{
      const count=playerComparisonTabs.length;
      const next=event.key==='Home'?0:event.key==='End'?count-1:
        event.key==='ArrowRight'?(index+1)%count:event.key==='ArrowLeft'?(index+count-1)%count:-1;
      if(next<0)return;
      event.preventDefault();
      selectTab(next,true);
    };
  }
  filter.onchange=renderTable;
  selectTab(0);
  prepareComparisonExports(left,right,leftContracts,rightContracts,fields,headings,saves);
  dialog.addEventListener('close',()=>menu?.querySelector('summary')?.focus({preventScroll:true}),{once:true});
  dialog.showModal();
}

function prepareComparisonExports(left:Player,right:Player,leftContracts:ContractView[],rightContracts:ContractView[],
  fields:PlayerComparisonField[],headings:[string,string],saves?:SavePlayerComparison){
  const side=(player:Player,contracts:ContractView[],save?:{name:string;date:{year:number;day:number}})=>
    ({uid:player.uid,name:player.name,club:player.club,save:save??null,contracts:comparisonContractData(player,contracts)});
  const first=side(left,leftContracts,saves?{name:saves.currentName,date:saves.currentDate}:undefined);
  const second=side(right,rightContracts,saves?{name:saves.otherName,date:saves.otherDate}:undefined);
  const json=JSON.stringify({version:1,left:first,right:second,fields},null,2);
  const columns=(contracts:ReturnType<typeof comparisonContractData>)=>new Map<string,string|number|null|undefined>(contracts.flatMap(contract=>{
    const prefix=t("Contrato ")+(contract.index+1)+' · ';
    const terms=contractIncentives.map(([key,,label])=>[prefix+label+' (£)',contract[key]] as const);
    return [[prefix+t("Vínculo"),t(contract.kindLabel)],[prefix+t("Entidade"),contract.entity],
      [prefix+t("Salário semanal (£)"),contract.weeklyWageGbp],
      [prefix+t("Vencimento"),contract.monthly?t("Mensal"):contract.expires?comparisonDate(contract.expires):null],
      [prefix+t("Camisa"),rosterSquadNumber(contract)],[prefix+t("Situação no elenco"),rosterSquadStatus(contract)],
      [prefix+t("Situação de mercado"),contract.marketStatus],...terms] as const;
  }));
  const leftColumns=columns(first.contracts),rightColumns=columns(second.contracts);
  const contractRows=Array.from(new Set([...leftColumns.keys(),...rightColumns.keys()]),label=>
    [label,leftColumns.get(label)??null,rightColumns.get(label)??null,null]);
  const exportMenu=document.querySelector<HTMLDetailsElement>('#player-comparison .comparison-export')!;
  exportMenu.open=false;
  const closeExportMenu=()=>{
    exportMenu.open=false;
    exportMenu.querySelector('summary')?.focus({preventScroll:true});
  };
  $('export-player-comparison').onclick=()=>{
    closeExportMenu();
    exportComparisonCsv([t("Campo"),headings[0]+' UID '+left.uid,headings[1]+' UID '+right.uid,t("Diferença")],fields,contractRows);
  };
  $('export-player-comparison-json').onclick=()=>void task(async()=>{
    closeExportMenu();
    const path=await invoke<string|null>('export_player_comparison_json',{json});
    if(path)notify(t("Comparação exportada: ")+path,'success');
  });
}
function exportComparisonCsv(headers:string[],fields:PlayerComparisonField[],contractRows:(string|number|null)[][]):void {
  const quote=(text:string)=>'"'+text.replace(/"/g,'""')+'"';
  const quoteLabel=(label:string)=>{
    const text=label.trim().replace(/\s+/g,' ');
    return quote(/^[=+@-]/.test(text)?"'"+text:text);
  };
  const rows=[headers.map(quoteLabel).join(';'),...fields.map(({label,left,right,difference})=>[
    quoteLabel(label),quote(String(left??'—')),quote(String(right??'—')),
    quote(difference==null?'—':comparisonDifference.format(difference)),
  ].join(';'))];
  for(const row of contractRows)rows.push(row.map(value=>{
    const cell=String(value??'');
    const unsafe=typeof value==='string'&&/^[\s\uFEFF]*[=+@-]/.test(cell);
    return quote(unsafe?"'"+cell:cell);
  }).join(';'));
  void task(async()=>{
    const path=await invoke<string|null>('export_player_comparison',{csv:'\uFEFF'+rows.join('\r\n')+'\r\n'});
    if(path)notify(t("Comparação exportada: ")+path,'success');
  });
}
async function compareSaves<T>(command:'compare_player_save'|'compare_club_save',uid:number):Promise<T|null>{
  notify(t("Selecione o segundo save. A comparação pode levar alguns segundos."));
  const cancel=$<HTMLButtonElement>('cancel-save-comparison');
  let cancelled=false;
  cancel.hidden=false;
  cancel.disabled=false;
  cancel.onclick=()=>{
    cancelled=true;
    cancel.disabled=true;
    notify(t("Cancelando a comparação…"));
    void invoke('cancel_save_comparison').catch(error=>notify(String(error),'error'));
  };
  try{
    const comparison=await invoke<T|null>(command,{uid});
    if(!cancelled&&comparison){
      notify(t("Comparação concluída."),'success');
      return comparison;
    }
    notify(t("Comparação cancelada."));
    return null;
  }finally{
    cancel.hidden=true;
    cancel.onclick=null;
  }
}

function bindPlayerComparison(player:Player){
  $('compare-save').onclick=async()=>{
    if(!await canLeave())return;
    void task(async()=>{
      const result=await compareSaves<SavePlayerComparison>('compare_player_save',player.uid);
      if(!result)return;
      openPlayerComparison(result.current,result.other,result.currentContracts,result.otherContracts,result);
    });
  };
  $('compare-reference').onclick=()=>{
    comparisonReference=comparisonReference?null:{uid:player.uid,name:player.name};
    const controls=$('compare-player');
    controls.hidden=!comparisonReference||comparisonReference.uid===player.uid;
    controls.textContent=comparisonReference?t("Comparar com ")+comparisonReference.name:t("Comparar");
    $('compare-reference').textContent=comparisonReference?t("Limpar referência"):t("Marcar para comparar");
  };
  $('compare-player').onclick=async()=>{
    const reference=comparisonReference;
    if(!reference||reference.uid===player.uid||!await canLeave())return;
    void task(async()=>{
      const left=await invoke<Detail>('player_detail',{uid:reference.uid});
      const right=await invoke<Detail>('player_detail',{uid:player.uid});
      openPlayerComparison(left.effective,right.effective,left.contracts,right.contracts);
    });
  };
}
function renderProfile(){
  if(!detail)return;
  const d=detail,p=d.player;
  const profile=$('profile');
  const expandedContract=profile.dataset.playerUid===String(p.uid)
    ? profile.querySelector<HTMLDetailsElement>('.player-contract[open]')?.id : undefined;
  profile.dataset.playerUid=String(p.uid);
  const value=(key:string)=>d.effective.fields[key]?.value;
  const playingPositions = ['goalkeeper','sweeper','defender','defensiveMidfielder','midfielder','attackingMidfielder','attacker','wingBack','freeRole'];
  const strongPositions = playingPositions.slice(0, 8)
    .filter(key => (value('position.'+key) ?? 0) >= 15)
    .map(key => fieldLabel('position.'+key));
  const fact=(kind:string,label:string,text:string)=>
    '<div class="player-fact player-fact-'+kind+'"><dt>'+escape(label)+'</dt><dd>'+escape(text)+'</dd></div>';
  const profileFacts='<dl class="player-profile-facts">'
    +fact('club',t("Clube"),p.club??t("Sem clube"))
    +(strongPositions.length?fact('position',t("Posições"),strongPositions.join(', ')):'')
    +fact('nation',t("Nacionalidade"),d.effective.nation??t("Não informada"))
    +(d.age===null?'':fact('age',t("Idade na data do save"),d.age+(d.age===1?t(" ano"):t(" anos"))))
    +'</dl>';
  const field=(key:string)=>{
    const meta=p.fields[key];if(!meta)return '';
    const v=value(key),scale=(key==='state.condition'||key==='state.fitness')?100:1;
    if(key.endsWith('.squadStatus')){
      const options=squadStatusLabels;
      return '<label class="attribute"><span>'+escape(fieldLabel(key))+'</span><select aria-label="'+escape(fieldLabel(key))+'" data-field="'+key+'" data-original="'+meta.value+'" data-initial="'+v+'">'+options.map((name,i)=>'<option value="'+(i-1)+'" '+(i-1===v?'selected':'')+'>'+name+'</option>').join('')+`</select><small class="original-value">${t("Original:")} `+escape(options[meta.value+1]??meta.value)+'</small></label>';
    }
    return '<label class="attribute '+(meta.max>200?'wide-field':'')+'"><span>'+escape(fieldLabel(key))+'</span><input aria-label="'+escape(fieldLabel(key))+'" title="'+t("Original: ")+meta.value/scale+'" data-field="'+key+'" data-original-value="'+meta.value/scale+'" data-scale="'+scale+'" data-initial="'+v+'" class="'+(v!==meta.value?'changed':'')+'" type="number" required min="'+meta.min/scale+'" max="'+meta.max/scale+'" step="'+(scale===100?'0.01':'1')+'" value="'+v/scale+'"><small class="attribute-difference" hidden></small></label>';
  };
  const dateControl=(prefix:string,label:string,allowMonthly=false)=>{
    const year=value(prefix+'.year'),day=value(prefix+'.day'),monthly=prefix!=='birth'&&year===1900&&day===1;
    const originalYear=p.fields[prefix+'.year']?.value,originalDay=p.fields[prefix+'.day']?.value;
    const originalDate=new Date(Date.UTC(originalYear,0,1)+originalDay*86400000);
    const originalLabel=prefix!=='birth'&&originalYear===1900&&originalDay===1?t("Mensal")
      :Number.isFinite(originalDate.getTime())?originalDate.toLocaleDateString(locale,{timeZone:'UTC'}):'';
    const date=new Date(Date.UTC(year,0,1)+day*86400000),iso=monthly?'':Number.isFinite(date.getTime())?date.toISOString().slice(0,10):'';
    const minYear=p.fields[prefix+'.year']?.min;
    const maxYear=p.fields[prefix+'.year']?.max;
    const limits=(minYear==null?'':' min="'+minYear+'-01-01"')+(maxYear==null?'':' max="'+maxYear+'-12-31"');
    return '<div class="date-control"><label class="attribute"><span>'+escape(label)+'</span><input aria-label="'+escape(label)+'" type="date"'+limits+' data-date="'+prefix+'" data-initial="'+iso+'" data-initial-monthly="'+monthly+'" data-staged="'+(year!==originalYear||day!==originalDay)+'" value="'+iso+'" '+(monthly?'disabled':'required')+'></label>'+(allowMonthly?'<label><input type="checkbox" data-monthly="'+prefix+'" '+(monthly?'checked':'')+`> ${t("Contrato mensal")}</label>`:'')+(originalLabel?`<small class="original-value">${t("Original:")} `+escape(originalLabel)+'</small>':'')+'</div>';
  };
  const group=(title:string,keys:string[])=>'<div class="attribute-group"><h3>'+title+'</h3>'+keys.map(field).join('')+'</div>';
  const section=(id:string,title:string,body:string)=>'<section role="tabpanel" id="panel-'+id+'" aria-labelledby="tab-'+id+'" '+(id===profileTab?'':'hidden')+'><h3 class="tab-heading">'+title+'</h3><div class="attribute-grid">'+body+'</div></section>';
  const tabs:[string,string][]=[['attributes',t("Atributos")],['ability','CA / PA'],['positions',t("Posições")],['personal',t("Dados pessoais")],['personality',t("Personalidade")],['hidden',t("Ocultos")],['keeper',t("Goleiro")],['state',t("Estado")],['market',t("Mercado")],['contracts',t("Contratos")]];
  const tabbar=`<div class="profile-tabs" role="tablist" aria-label="${t("Perfil do jogador")}">`+tabs.map(([id,title])=>'<button type="button" role="tab" id="tab-'+id+'" data-tab="'+id+'" aria-controls="panel-'+id+'" aria-selected="'+(id===profileTab)+'" tabindex="'+(id===profileTab?0:-1)+'">'+title+'</button>').join('')+'</div>';
  const policy=d.pending.find(e=>e.policy)?.policy??'';
  const selectedNation=value('nationality');
  const sortedNations = d.nations.map((name, index) => ({name, index}))
    .sort((a, b) => a.name.localeCompare(b.name, locale));
  const nationOptions=(selectedNation===-1?`<option value="-1" selected disabled>${t("Não informada")}</option>`:'')+sortedNations.map(({name,index})=>'<option value="'+index+'" '+(index===selectedNation?'selected':'')+'>'+escape(name)+'</option>').join('');
  const personal=`<div class="attribute-group"><h3>${t("Identificação")}</h3>`+dateControl('birth',t("Data de nascimento"))+`<div class="nation-picker"><div class="attribute"><label id="nation-label" for="nation-trigger">${t("Nacionalidade")}</label><button id="nation-trigger" type="button" aria-labelledby="nation-label nation-value" aria-expanded="false" aria-controls="nation-popup"><span id="nation-value"></span><span aria-hidden="true"> ▾</span></button></div><select id="player-nationality" data-field="nationality" data-initial="`+selectedNation+'" hidden>'+nationOptions+`</select><div id="nation-popup" hidden><input id="nation-search" aria-label="${t("Buscar país")}" type="search" placeholder="${t("Digite parte do nome")}" autocomplete="off" aria-controls="nation-results"><div id="nation-results" role="group" aria-label="${t("Países")}"></div><small id="nation-search-feedback" class="notice" data-notice="info" role="status"></small></div></div></div>`+`<div class="attribute-group personal-measures"><h3>${t("Medidas")}</h3>`+field('height')+field('weight')
    +notice('info',t("Use 0 se a medida for desconhecida."))
    +`<details><summary>${t("Como funciona a altura?")}</summary><p class="notice" data-notice="info">${t("O save guarda a altura em polegadas inteiras. O editor aceita as medidas em centímetros que podem ser representadas nesse formato.")}</p></details></div>`;
  const injuryBody=(d.injuries.length?'<details class="availability-detail" '+(d.healPending?'open':'')+`><summary>${t("Lesões ·")} `+d.injuries.length+'</summary>'+'<ul>'+d.injuries.map(injury=>'<li>'+escape(injury.name)+(injury.recordedOn?t(" · registrada em ")+escape(new Date(Date.UTC(injury.recordedOn.year,0,1)+injury.recordedOn.day*86400000).toLocaleDateString(locale,{timeZone:'UTC'})):'')+(injury.estimatedDaysRemaining?t(" · estimativa no save: ")+injury.estimatedDaysRemaining+t(" dia(s) restantes"):'')+'</li>').join('')+'</ul><label><input type="checkbox" id="remove-injuries" '+(d.healPending?'checked':'')+`> ${t("Remover as lesões deste jogador")}</label><p class="notice" data-notice="`+(d.healPending?'success':'info')+'">'+(d.healPending?t("Remoção preparada na revisão. Desmarque e adicione novamente para restaurar."):t("Adicione à revisão para preparar a remoção. Condição e ritmo podem ser ajustados separadamente."))+'</p></details>':`<span class="availability-clear">${t("Sem lesões")}</span>`);
  const bookingCount=d.bookingRecordCount??0;
  const bookingSummary=bookingCount>0?'<p>'+bookingCount+(bookingCount===1?t(" registro de cartões no save."):t(" registros de cartões no save."))+'</p>':'';
  const suspensionBody=d.suspensionCount||bookingCount
    ?'<details class="availability-detail" '+(d.suspensionPending?'open':'')+'><summary>'
      +(d.suspensionCount?t("Suspensões · ")+d.suspensionCount:t("Sem suspensões · Ver cartões"))+'</summary>'
      +(d.suspensionCount?'<label><input type="checkbox" id="remove-suspensions" '+(d.suspensionPending?'checked':'')+`> ${t("Remover as suspensões deste jogador")}</label><p class="notice" data-notice="`+(d.suspensionPending?'success':'warning')+'">'+(d.suspensionPending?t("Remoção preparada na revisão. Desmarque e adicione novamente para restaurar."):t("A contagem de cartões é preservada. Uma nova punição pode ocorrer em jogos futuros."))+'</p>':'')
      +bookingSummary+'</details>'
    :`<span class="availability-clear">${t("Sem suspensões")}</span>`;
  const stateBody='<div class="attribute-group player-preparation">'
    +['state.condition','state.fitness','state.jadedness','state.morale'].map(field).join('')
    +`<div class="preparation-actions"><button type="button" id="recover-player">${t("Recuperar condição e ritmo")}</button><button type="button" id="rest-player">${t("Descansar")}</button></div>`
    +`<p class="notice" data-notice="info">${t("Condição e ritmo são independentes. Cansaço menor indica mais descanso.")}</p></div>`
    +'<div class="player-availability">'+injuryBody+suspensionBody+'</div>';
  const saleValue=value('state.saleValue');
  const marketBody='<div class="attribute-group player-market-values">'+field('state.estimatedValue')+field('state.saleValue')
    +(saleValue==null?'':'<label class="no-asking-price"><input id="no-asking-price" type="checkbox" '+(saleValue===-1?'checked':'')+`> ${t("Preço pedido não especificado")}</label>`)
    +notice('info',t("O jogo pode recalcular estes valores."))+'</div>';
  const contractsBody=d.contracts.map(c=>{
    const prefix='contract.'+c.index,role=c.entityType===1?t("Seleção"):c.kind===0?t("Clube proprietário"):c.kind===1?t("Equipe de empréstimo"):t("Equipe vinculada");
    const start=new Date(Date.UTC(c.started.year,0,1)+c.started.day*86400000);
    const market=marketStatus(c);
    const targets=c.index===0?d.marketTargets??[]:[];
    const planned=plan.edits.find(edit=>edit.uid===p.uid&&edit.attribute===marketField)?.value??c.transferStatus;
    const marketLine=targets.length>1
      ?`<div class="market-status-field"><label for="player-market-status">${t("Situação de mercado")}</label><select id="player-market-status" data-field="`+marketField+'" data-original="'+c.transferStatus+'" data-initial="'+planned+'">'
        +targets.map(target=>'<option value="'+target+'" '+(target===planned?'selected':'')+'>'+escape(marketStatuses[target]??target)+'</option>').join('')
        +'</select><p id="selected-market-status" aria-hidden="true">'+escape(marketStatuses[planned]??planned)+'</p>'
        +`<small class="original-value">${t("Original:")} `+escape(market??c.transferStatus)+'</small></div>'
      :market?`<p class="profile-meta">${t("Situação de mercado:")} `+escape(market)+'</p>'
        +(c.index===0&&d.marketJournalSupported===false?notice('info',t("O diário deste save permite apenas consultar a situação de mercado.")):''):'';
    const cardId='contract-card-'+c.index;
    const wage=value(prefix+'.weeklyWage');
    const year=value(prefix+'.expires.year'),day=value(prefix+'.expires.day');
    const expiry=new Date(Date.UTC(year,0,1)+day*86400000);
    const expiryLabel=year===1900&&day===1?t("Mensal"):Number.isFinite(expiry.getTime())
      ?t("Até ")+expiry.toLocaleDateString(locale,{timeZone:'UTC'}):'';
    const summary=[c.entity??t("Equipe não informada"),
      wage==null?'':new Intl.NumberFormat(locale,{maximumFractionDigits:0}).format(wage)+t(" £/semana"),expiryLabel].filter(Boolean);
    return '<details class="contract-card player-contract" name="player-contracts" id="'+cardId+'" tabindex="-1" '+(expandedContract===cardId?'open':'')+'>'
      +`<summary><strong>${t("Contrato")} `+(c.index+1)+' · '+escape(t(c.kindLabel))+'</strong><span>'+escape(summary.join(' · '))+'</span></summary>'
      +'<div class="player-contract-fields"><p>'+escape(role)+': <strong>'+escape(c.entity??t("Não informada"))+`</strong></p><p>${t("Início:")} `+escape(Number.isFinite(start.getTime())?start.toLocaleDateString(locale,{timeZone:'UTC'}):t("Não informado"))+'</p>'
      +marketLine+field(prefix+'.weeklyWage')+dateControl(prefix+'.expires',t("Término do contrato ")+(c.index+1),c.kind===0)+field(prefix+'.squadStatus')+field(prefix+'.squadNumber')+field(prefix+'.appearanceBonus')+field(prefix+'.goalBonus')+field(prefix+'.assistBonus')+field(prefix+'.releaseClause')+'</div></details>';
  }).join('')||`<p class="notice" data-notice="info">${t("Sem contrato ativo para editar.")}</p>`;
  const loanHelp=d.contracts.some(c=>c.kind===1)?`<p class="notice" data-notice="warning">${t("No empréstimo, o salário do clube proprietário é o total do vínculo; o salário da equipe de empréstimo é a contribuição desse clube. O jogo pode mostrar separadamente a parcela que continua com o proprietário.")}</p>`:'';
  $('profile').innerHTML=`<div class="profile-header"><div><div class="eyebrow">${t("PERFIL DO JOGADOR · UID")} `+p.uid+'</div><h2>'+escape(p.name)+'</h2>'+profileFacts+'</div><div class="profile-badges"><div class="ability-badge"><small>CA</small><strong>'+d.effective.currentAbility+'</strong></div><div class="ability-badge"><small>PA</small><strong>'+d.effective.potentialAbility+ `</strong></div></div></div><div class="profile-compare-actions"><details class="player-compare-menu"><summary>${t("Comparar")}</summary><div><button id="compare-reference" type="button">`+(comparisonReference?t("Limpar referência"):t("Marcar para comparar"))+'</button> <button id="compare-player" type="button" '+(!comparisonReference||comparisonReference.uid===p.uid?'hidden':'')+'>'+escape(comparisonReference?t("Comparar com ")+comparisonReference.name:t("Comparar"))+`</button> <button id="compare-save" type="button">${t("Comparar com outro save")}</button></div></details> <button id="watch-player" type="button"></button></div><form id="attributes-form" novalidate>`+tabbar+section('attributes',t("Atributos"),groups.map(([title,keys])=>group(title,keys)).join(''))+section('ability',t("Habilidade, potencial e reputação"),`<div class="attribute-group"><h3>${t("Habilidade")}</h3>`+field('currentAbility')+field('potentialAbility')+notice('info',t("No banco inicial, -1 e -2 indicam potencial variável. O código -2 tende a gerar um potencial mais alto. Para definir um potencial fixo neste save, use um valor de 1 a 200."))+'</div>'+group(t("Reputação"),['reputation.home','reputation.current','reputation.world']))+section('positions',t("Posições, lados e pés"),group(t("Posições"),playingPositions.map(k=>'position.'+k))+group(t("Lados"),['rightSide','leftSide','central'].map(k=>'position.'+k))+group(t("Pés"),['leftFoot','rightFoot']))+section('personal',t("Dados pessoais"),personal)+section('personality',t("Personalidade"),group(t("Personalidade"),personalityFields))+section('hidden',t("Atributos ocultos e complementares"),group(t("Complementares"),['dirtiness','consistency','importantMatches','injuryProneness','versatility','throwIns']))+section('keeper',t("Atributos de goleiro"),group(t("Goleiro"),keeper))+section('state',t("Estado do jogador"),stateBody)+section('market',t("Valores de mercado"),marketBody)+section('contracts',t("Contratos existentes"),contractsBody+loanHelp+`<p class="notice" data-notice="info">${t("Salários e valores em libras (£); salário semanal. O jogo pode mostrar outra periodicidade e arredondar. Cada cartão edita apenas o contrato indicado.")}</p>`)+`<div id="player-ability-choice" class="policy-box notice" data-notice="warning" hidden><label for="ability-policy">${t("Quer manter os atributos atuais ou recalculá-los?")}</label><select id="ability-policy"><option value="">${t("Selecione uma opção")}</option><option value="preserveValues">${t("Manter os atributos atuais")}</option><option value="preserveRelative">${t("Recalcular os atributos")}</option></select><small>${t("A revisão mostra quais atributos mudam antes de gravar.")}</small>`+(p.type===3?`<small>${t("Este jogador também atua no staff. Confira na revisão os efeitos da CA sobre os atributos profissionais.")}</small>`:'')+`</div><div class="profile-actions"><small id="player-draft-count" role="status"></small><button id="go-to-player-error" type="button" hidden>${t("Ir ao campo")}</button><button id="stage-player" type="submit" class="primary">${t("Adicionar à revisão")}</button></div></form>`;
  bindNationalityPicker();
  const noPrice=document.querySelector<HTMLInputElement>('#no-asking-price');
  const price=document.querySelector<HTMLInputElement>('#panel-market [data-field="state.saleValue"]');
  if(noPrice&&price){
    let lastPrice=price.value==='-1'?'0':price.value;
    const syncPrice=()=>{
      price.hidden=noPrice.checked;
      price.min=noPrice.checked?'-1':'0';
    };
    syncPrice();
    noPrice.onchange=()=>{
      if(noPrice.checked)lastPrice=price.value;
      price.value=noPrice.checked?'-1':lastPrice;
      syncPrice();
      price.dispatchEvent(new Event('input',{bubbles:true}));
      decorateAttribute(price);
    };
  }
  const abilityChoice=$<HTMLSelectElement>('ability-policy');
  abilityChoice.value=policy;
  abilityChoice.dataset.initial=policy;
  for(const box of document.querySelectorAll<HTMLInputElement>('#profile [data-monthly]'))box.onchange=()=>{const date=document.querySelector<HTMLInputElement>('[data-date="'+box.dataset.monthly+'"]')!;date.disabled=box.checked;date.required=!box.checked;};
  const fill=(key:string,target:number)=>{const input=document.querySelector<HTMLInputElement>('[data-field="'+key+'"]')!;input.value=String(target/Number(input.dataset.scale??1));input.dispatchEvent(new Event('input',{bubbles:true}));};
  $('watch-player').onclick=()=>{
    const next=new Set(watchlist);
    if(next.has(p.uid))next.delete(p.uid);else next.add(p.uid);
    saveWatchlists(new Map(lists).set(activeList,[...next]),activeList);
  };
  syncWatchlist();
  bindPlayerComparison(p);
  $('recover-player').onclick=()=>{fill('state.condition',10000);fill('state.fitness',10000);notify(t("Condição e ritmo preparados; adicione à revisão para validar."));};
  $('rest-player').onclick=()=>{fill('state.jadedness',Math.min(value('state.jadedness'),-1000));notify(t("Descanso preparado; adicione à revisão para validar."));};

  function selectTab(id:string,focus=false){
    profileTab=id;
    for(const tab of document.querySelectorAll<HTMLButtonElement>('#profile [data-tab]')){
      const active=tab.dataset.tab===id;tab.setAttribute('aria-selected',String(active));tab.tabIndex=active?0:-1;if(active&&focus)tab.focus({preventScroll:true});
    }
    for(const panel of document.querySelectorAll<HTMLElement>('#profile [role=tabpanel]'))panel.hidden=panel.id!=='panel-'+id;
    $('profile').scrollTop=0;
    syncPlayerAbilityChoice();
  }
  for(const tab of document.querySelectorAll<HTMLButtonElement>('#profile [data-tab]')){
    tab.onclick=()=>selectTab(tab.dataset.tab!);
    tab.onkeydown=event=>{
      const i=tabs.findIndex(([id])=>id===profileTab);let next=i;
      if(event.key==='ArrowRight')next=(i+1)%tabs.length;else if(event.key==='ArrowLeft')next=(i+tabs.length-1)%tabs.length;else if(event.key==='Home')next=0;else if(event.key==='End')next=tabs.length-1;else return;
      event.preventDefault();selectTab(tabs[next][0],true);
    };
  }

  document.querySelectorAll<HTMLInputElement>('#profile [data-original-value]').forEach(decorateAttribute);
  syncPlayerDraft();
  $('go-to-player-error').onclick=()=>{
    const input=document.querySelector<HTMLInputElement>('#profile input[aria-invalid=true]');
    if(!input)return;
    const tab=input.closest('[role=tabpanel]')?.id.replace('panel-','');
    if(tab)selectTab(tab);
    const contract=input.closest<HTMLDetailsElement>('details');
    if(contract)contract.open=true;
    input.focus();
  };
  $<HTMLFormElement>('attributes-form').onsubmit=event=>{
    event.preventDefault();void task(async()=>{
      if (validatePlayerFields()) throw Error(t("Corrija os campos destacados antes de adicionar à revisão."));
      const choice=$<HTMLSelectElement>('ability-policy');
      if(!$('player-ability-choice').hidden&&!choice.value){
        const tab=choice.closest('[role=tabpanel]')?.id.replace('panel-','');
        if(tab) selectTab(tab);
        choice.focus();
        throw Error(t("Escolha se deseja manter ou recalcular os atributos."));
      }
      const edits=drafts(),healing=healingDraft(),suspension=suspensionDraft();if(!edits.length&&!healing&&!suspension){notify(t("Nenhum campo novo foi alterado."));return;}
      if(edits.some(e=>!Number.isFinite(e.value)))throw Error(t("Verifique as datas alteradas."));
      notify(t("Validando campos e efeitos sobre os atributos…"));
      const scrollTop=$('profile').scrollTop;
      plan=await invoke<Pending>('stage_edits',{edits,healing,suspension});
      await showPlayer(p.uid,false);
      enterProfile('profile',scrollTop);
      notify(t("Alterações adicionadas à revisão. Nada foi gravado no save."),'success');
    });
  };
  for(const el of document.querySelectorAll<HTMLInputElement|HTMLSelectElement>('#profile [data-field]'))el.oninput=()=>el.classList.toggle('changed',inputValue(el)!==(p.fields[el.dataset.field!]?.value??Number(el.dataset.original)));
  const marketSelect=document.querySelector<HTMLSelectElement>('#player-market-status');
  if(marketSelect)marketSelect.onchange=()=>{
    $('selected-market-status').textContent=marketSelect.selectedOptions[0]?.textContent??'';
  };
  const actions=$('profile').querySelector<HTMLElement>('.profile-compare-actions')!;
  const presetButton=document.createElement('button');
  presetButton.type='button';
  presetButton.textContent=t("Presets de edição");
  presetButton.onclick=async()=>{
    if(busy||!await canLeave())return;
    openPlayerBatch([{uid:p.uid,name:p.name,injuryCount:d.injuries.length,suspensionCount:d.suspensionCount}],presetButton,p.name);
  };
  actions.append(presetButton);
  prepareProfile($('profile'));
}
async function showPlayer(uid:number,checkDraft=true){
  if(checkDraft&&!await canLeave())return false;
  detail=await invoke<Detail>('player_detail',{uid});
  renderProfile();
  for(const row of $('results').querySelectorAll<HTMLElement>('[data-uid]')){
    row.classList.toggle('selected',Number(row.dataset.uid)===uid);
  }
  return true;
}
interface RecentSave {path:string;name:string}
const recentSavesKey='cm0304.recent-saves.v1';
function recentSaves():RecentSave[]{
  try{
    const items:unknown=JSON.parse(localStorage.getItem(recentSavesKey)??'[]');
    return Array.isArray(items)?items.filter(item=>item&&typeof item.path==='string'&&typeof item.name==='string').slice(0,6):[];
  }catch{return [];}
}
function renderRecentSaves(){
  const rows=recentSaves();
  $('recent-saves').hidden=!rows.length;
  $('recent-save-list').innerHTML=rows.map((entry,index)=>'<button type="button" data-recent-index="'+index+'" title="'+escape(entry.path)+'">'+escape(entry.name)+'</button>').join('');
  $('recent-save-list').onclick=e=>{
    const button=(e.target as HTMLElement).closest<HTMLElement>('[data-recent-index]');
    const selected=button?rows[Number(button.dataset.recentIndex)]:null;
    if(selected)void open(selected.path);
  };
}
function rememberSave(data:Summary){
  const next=[{name:data.name,path:data.path},...recentSaves().filter(item=>item.path.toLowerCase()!==data.path.toLowerCase())].slice(0,6);
  try{localStorage.setItem(recentSavesKey,JSON.stringify(next));}catch{return;}
  renderRecentSaves();
}
async function withSaveProgress<T>(operation:'open-save'|'save-copy',message:string,cancelCommand:string,run:()=>Promise<T>):Promise<T> {
  const progress=$(operation+'-progress');
  const cancel=$<HTMLButtonElement>('cancel-'+operation);
  progress.hidden=false;
  cancel.disabled=false;
  cancel.onclick=()=>{
    cancel.disabled=true;
    if(operation==='open-save') $('opening-message').textContent=message;
    notify(message);
    void invoke(cancelCommand).catch(error=>notify(String(error),'error'));
  };
  try {
    return await run();
  } finally {
    progress.hidden=true;
    cancel.onclick=null;
  }
}
function confirmAction(title:string,message:string,accept=t("Continuar")){
  const dialog=$<HTMLDialogElement>('confirm-action');
  if(dialog.open)return Promise.resolve(false);
  $('confirm-action-title').textContent=title;
  $('confirm-action-message').textContent=message;
  $('confirm-action-accept').textContent=accept;
  dialog.returnValue='';
  return new Promise<boolean>(resolve=>{
    dialog.addEventListener('close',()=>resolve(dialog.returnValue==='continue'),{once:true});
    dialog.showModal();
  });
}
async function open(recentPath?:string){
  if(busy)return;
  if((planCount()||hasFormDrafts())&&!await confirmAction(t("Abrir outro save?"),
    t("As alterações na revisão e os campos ainda em edição serão descartados se outro save for carregado.")))return;
  await task(async()=>{
  $('open-save-error').hidden=true;
  const path=recentPath??await invoke<string|null>('pick_save_file');
  if(!path){notify(t("Abertura cancelada."));return;}
  clearToast();
  $('opening-file').textContent=path.split(/[\\/]/).pop()??path;
  const loadingMessage=t("Lendo o save e preparando a busca.");
  notify(loadingMessage);
  $('opening-message').textContent=loadingMessage;
  let data:Summary|null;
  try {
    data=await withSaveProgress('open-save',t("Cancelando abertura…"),'cancel_open_save',
      ()=>invoke<Summary|null>('open_save',{recentPath:path}));
  } catch(error) {
    const active=summary?t(" O save atual continua aberto")+(planCount()?t(", com as alterações na revisão."):'.'):'';
    const message=String(error)+active;
    $('retry-open-save').onclick=()=>void open(path);
    $('open-save-error').hidden=false;
    notify(message,'error');
    return;
  }
  if(!data){notify(t("Abertura cancelada."));return;}
  rememberSave(data);
  await loadWorkspace(data,{edits:[],healInjuries:[],clearSuspensions:[],undo:false,redo:false});
  });
}

async function loadWorkspace(data:Summary, pending:Pending) {
  summary=data;
  resetMyClub();
  document.querySelector<HTMLElement>('.filebar')!.hidden=false;
  renderSavedCopies();
  detail=null;
  delete $('profile').dataset.playerUid;
  comparisonReference=null;
  clearTimeout(timer);
  searchGeneration++;
  needsInitialPlayerSearch=true;
  total=0;
  resultRows=[];
  $('results').replaceChildren();
  $('results').removeAttribute('aria-busy');
  $('cancel-player-search').hidden=true;
  restoreLastPlayerSearch();
  resetStaff();
  resetClubs();
  plan=pending;
  reviewQuery='';
  reviewPage=0;
  resetReview();
  $('saved-path').textContent='';
  for(const grid of document.querySelectorAll<HTMLElement>('.workspace-grid'))delete grid.dataset.pane;
  for(const results of document.querySelectorAll<HTMLElement>('.results')){
    resultPositions.delete(results);
    results.scrollTop=0;
  }
  page=0;
  $('file-name').textContent=data.name;
  const playerNoun=data.players===1?t("jogador"):t("jogadores");
  $('file-info').textContent=`${data.players.toLocaleString(locale)} ${playerNoun} ${t("· banco 147")}`;
  $('file-name').title=data.path;
  $('player-count').textContent=data.players.toLocaleString(locale);
  $('diagnostics').textContent=JSON.stringify(data,null,2);
  $('empty').hidden=true;
  $('workspace').hidden=false;
  $('profile').innerHTML=`<div class="profile-placeholder"><h2>${t("Escolha um jogador")}</h2><p>${t("Use a busca para encontrar um nome ou elenco.")}</p></div>`;
  if(data.managedClubs?.length){
    view('my-club');
    await loadMyClub();
    $('my-club-title').focus();
    notify(t("Save carregado."),'success');
  }else{
    view('players');
    await search();
    $('search').focus();
    notify(t("Save carregado. Selecione um jogador para começar."),'success');
  }
}
function staffFieldLabel(key:string):string {if(key.startsWith('personality.'))return fieldLabel(key);const labels:Record<string,string>={coachingOutfieldPlayers:t("Treino de jogadores de linha"),coachingGoalkeepers:t("Treino de goleiros"),physiotherapy:t("Fisioterapia"),tacticalKnowledge:t("Conhecimento tático"),workingWithYoungsters:t("Trabalho com jovens"),manManagement:t("Gestão de pessoas"),judgingPlayerAbility:t("Avaliação de capacidade"),judgingPlayerPotential:t("Avaliação de potencial"),motivating:t("Motivação"),discipline:t("Disciplina"),determination:t("Determinação")};return labels[key]??key;}
function renderPlayerExclusions(report:Report,visiblePlayers:Set<number>):string {
  const uids=new Set([...plan.edits.map(edit=>edit.uid),...plan.healInjuries,...plan.clearSuspensions]);
  for(const uid of uids)if(!visiblePlayers.has(uid))uids.delete(uid);
  if(!uids.size)return '';
  const names=new Map<number,string>();
  for(const change of report.changes)if(!change.attribute.startsWith('club.')&&!change.context?.startsWith('Staff'))names.set(change.uid,change.name);
  for(const removal of report.removals)names.set(removal.uid,removal.name);
  return `<details><summary>${t("Retirar jogadores do plano")}</summary><p class="notice" data-notice="warning">${t("Remove os campos e registros preparados na função de jogador.")}</p>`
    +Array.from(uids,uid=>'<button type="button" data-remove-plan-player="'+uid+`">${t("Retirar")} `+escape(names.get(uid)??('UID '+uid))+'</button>').join(' ')+'</details>';
}
const clubReviewLabels:Record<string,string>={
  'club.reputation':t("Reputação da equipe principal"),
  'club.youthSetup':t("Formação de jovens"),
  'club.training':t("Instalações de treino"),
  'club.balance':t("Balanço (£)"),
  'club.wageBudgetWeekly':t("Orçamento salarial (£/semana)"),
  'club.stadiumCapacity':t("Lotação total do estádio"),
  'club.stadiumSeatedCapacity':t("Lugares sentados do estádio"),
  'club.transferBudgetSeason':t("Orçamento da temporada (£)"),
  'club.transferBudgetRemaining':t("Orçamento restante (£)"),
};
const clubReviewFields:Record<string,string>={
  'club.reputation':'club-reputation','club.youthSetup':'club-youth','club.training':'club-training',
  'club.balance':'club-balance','club.wageBudgetWeekly':'club-wage-budget',
  'club.stadiumCapacity':'stadium-capacity','club.stadiumSeatedCapacity':'stadium-seated-capacity',
  'club.transferBudgetSeason':'club-budget-season','club.transferBudgetRemaining':'club-budget-remaining',
};
let reviewQuery='';
let reviewPage=0;
const normalizeReview=(text:string)=>text.normalize('NFD').replace(/\p{M}/gu,'').toLocaleLowerCase(locale);

function renderChanges(container:HTMLElement,report:Report,showExclusions=true) {
  type ReviewRow={html:string;search:string};
  const entities=new Map<string,{uid:number;name:string;type:string;rows:ReviewRow[]}>();
  const editable=new Set(showExclusions?plan.edits.map(edit=>edit.uid+':'+reviewField(edit.attribute)):[]);
  const add=(type:string,uid:number,name:string,label:string,before:string|number,after:string|number,context?:string|null,field?:string,editField=field)=>{
    const key=type+':'+uid;
    const entity=entities.get(key)??{uid,name,type,rows:[]};
    const html='<div class="change-item"><span>'+escape(label)
      +(context?'<small class="change-context">'+escape(context)+'</small>':'')
      +(showExclusions&&editField?'<button type="button" class="review-remove-field" data-edit-review-uid="'+uid
        +'" data-edit-review-type="'+type+'" data-edit-review-field="'+escape(editField)
        +'" aria-label="'+t("Ajustar ")+escape(label)+t(" de ")+escape(name)+`">${t("Ajustar")}</button>`:'')
      +(showExclusions&&field?'<button type="button" class="review-remove-field" data-remove-plan-player="'+uid
        +'" data-remove-plan-field="'+escape(field)+'" aria-label="'+t("Retirar ")+escape(label)+t(" de ")+escape(name)+`">${t("Retirar")}</button>`:'')
      +'</span><b class="review-values"><span>'+escape(before)+'</span><i aria-hidden="true"> → </i><strong>'+escape(after)+'</strong></b></div>';
    entity.rows.push({html,search:showExclusions?normalizeReview(name+' '+uid+' '+label+' '+(context??'')):''});
    entities.set(key,entity);
  };
  const seenDates=new Set<string>();
  for(const change of report.changes){
    const type=change.attribute.startsWith('club.')?t("Clube"):change.context?.startsWith('Staff')?t("Comissão"):t("Jogador");
    const field=reviewField(change.attribute);
    if(field!==change.attribute){
      const key=type+':'+change.uid+':'+field;
      if(seenDates.has(key))continue;
      seenDates.add(key);
    }
    const label=clubReviewLabels[change.attribute]??(change.attribute.startsWith('birth.')?t("Nascimento")
      :type===t("Comissão")&&staffFieldLabel(change.attribute)!==change.attribute
        ?staffFieldLabel(change.attribute):fieldLabel(change.attribute));
    const market=change.attribute===marketField;
    const removable=type===t("Jogador")&&editable.has(change.uid+':'+field)?field:undefined;
    const target=type===t("Clube")?clubReviewFields[field]:type===t("Comissão")&&!change.derived?field:removable;
    const display = (formatted:string|null,value:number) => formatted == null
      ? (market?marketStatuses[value]:undefined)??value
      : t(formatted).replace(/ \/ semana$/,t(" / semana"));
    let context=change.context;
    if(context&&change.attribute.startsWith('contract.')){
      const separator=context.indexOf(' · ');
      if(separator!==-1){
        const entity=context.slice(separator+3);
        context=t(context.slice(0,separator))+' · '+(entity==='Sem entidade'?t(entity):entity);
      }
    }else if(context&&(change.attribute.startsWith('club.')||context.startsWith('Staff · '))){
      context=t(context);
    }
    add(type,change.uid,change.name,label+(change.derived?t(" (efeito indireto)"):''),
      display(change.displayBefore,change.before),display(change.displayAfter,change.after),context,
      removable,target);
  }
  for(const change of report.staffChanges??[]){
    add(t("Comissão"),change.uid,change.name??('UID '+change.uid),staffFieldLabel(change.attribute),change.before,change.after,null,undefined,change.attribute);
  }
  for(const removal of report.removals){
    add(t("Jogador"),removal.uid,removal.name,removal.kind==='suspensions'
      ?t("Remover suspensões registradas"):t("Remover lesões registradas"),removal.count,0,null,
      removal.kind==='suspensions'?'clearSuspensions':'healInjuries');
  }
  const operations=showExclusions&&plan.operations?.length
    ?`<details><summary>${t("Operações desta sessão")}</summary><ol>`+plan.operations.map(operation=>{
      const fields=[...new Set(operation.playerFields.map(field=>field==='birth.day'||field==='birth.year'
        ?t("Nascimento"):fieldLabel(field)))];
      const count=operation.playerCount;
      const summary=count?' · '+count+' '+(count===1?t("jogador"):t("jogadores"))+' · '+fields.join(', '):'';
      return '<li value="'+(operation.index+1)+'">'+escape(operation.domains.map(t).join(' · ')+summary)
        +' <button type="button" data-remove-review-operation="'+operation.index+`">${t("Retirar operação")} `
        +(operation.index+1)+'</button></li>';
    }).join('')+'</ol></details>':'';
  const renderEntity=(entity:{name:string;type:string;rows:ReviewRow[]})=>
    '<details class="review-entity" open><summary><strong>'+escape(entity.name)+'</strong> · '
    +entity.type+' · '+entity.rows.length+' '+(entity.rows.length===1?t("alteração"):t("alterações"))
    +'</summary>'+`<div class="review-columns"><span>${t("Campo")}</span><span class="review-values"><span>${t("Original")}</span><span></span><span>${t("Novo")}</span></span></div>`+entity.rows.map(row=>row.html).join('')+'</details>';
  const clubs=showExclusions?plan.reviewClubs:undefined;
  const allRows=Array.from(entities.values());
  const pageSize=50;
  let rows=allRows;
  let pageCount=0;
  let page=showExclusions?reviewPage:0;
  container.innerHTML=operations+(showExclusions
    ?`<div class="review-filter"><label for="review-query">${t("Buscar alterações")}</label> `
      +`<input type="search" id="review-query" placeholder="${t("Nome, campo ou vínculo")}" value="`+escape(reviewQuery)+'"> '
      +'<small id="review-filter-count" role="status"></small></div>':'')
    +`<div class="pagination" data-review-pagination hidden><button type="button" data-review-prev>${t("Página anterior")}</button> `
      +'<span data-review-page aria-live="polite"></span> '
      +`<button type="button" data-review-next>${t("Próxima página")}</button></div>`
    +'<div data-review-entities></div>';
  const contents=container.querySelector<HTMLElement>('[data-review-entities]')!;
  const previous=container.querySelector<HTMLButtonElement>('[data-review-prev]');
  const next=container.querySelector<HTMLButtonElement>('[data-review-next]');
  const pageLabel=container.querySelector<HTMLElement>('[data-review-page]');
  const renderPage=()=>{
    if(!rows.length){contents.innerHTML=notice('info',t("Nenhuma alteração encontrada."));return;}
    const visible=rows.slice(page*pageSize,(page+1)*pageSize);
    const exclusions=showExclusions?renderPlayerExclusions(report,
      new Set(visible.filter(entity=>entity.type===t("Jogador")).map(entity=>entity.uid))):'';
    if(!clubs){
      contents.innerHTML=exclusions+visible.map(renderEntity).join('');
      return;
    }
    const groups=new Map<number|null,{name:string;entities:string[]}>();
    for(const entity of visible){
      const club=entity.type===t("Clube")?{uid:entity.uid,name:entity.name}
        :entity.type===t("Comissão")?clubs.staff[entity.uid]:clubs.players[entity.uid];
      const key=club?.uid??null;
      const group=groups.get(key)??{name:club?.name??t("Sem clube vinculado"),entities:[]};
      group.entities.push(renderEntity(entity));
      groups.set(key,group);
    }
    contents.innerHTML=exclusions+Array.from(groups.values(),group=>'<details class="review-club" open><summary>'
      +escape(group.name)+'</summary>'+group.entities.join('')+'</details>').join('');
  };
  const updatePage=()=>{
    pageCount=Math.ceil(rows.length/pageSize);
    page=Math.min(page,Math.max(0,pageCount-1));
    if(showExclusions)reviewPage=page;
    renderPage();
    container.querySelector<HTMLElement>('[data-review-pagination]')!.hidden=pageCount<=1;
    if(pageLabel&&previous&&next){
      pageLabel.textContent=(page+1)+' / '+pageCount+' · '+rows.length+t(" entidades");
      previous.hidden=page===0;
      next.hidden=page+1===pageCount;
    }
  };
  const filterRows=()=>{
    const words=normalizeReview(reviewQuery.trim()).split(/\s+/).filter(Boolean);
    rows=words.length?allRows.map(entity=>({...entity,rows:entity.rows.filter(row=>words.every(word=>row.search.includes(word)))}))
      .filter(entity=>entity.rows.length):allRows;
    const totalRows=allRows.reduce((count,entity)=>count+entity.rows.length,0);
    $('review-filter-count').textContent=rows.reduce((count,entity)=>count+entity.rows.length,0).toLocaleString(locale)+t(" de ")
      +totalRows.toLocaleString(locale)+(totalRows===1?t(" alteração"):t(" alterações"));
    updatePage();
  };
  const filter=container.querySelector<HTMLInputElement>('#review-query');
  if(filter){
    filter.oninput=()=>{reviewQuery=filter.value;page=0;filterRows();};
    filterRows();
  }else updatePage();
  if(previous&&next){
    previous.onclick=()=>{if(page>0){page--;updatePage();}};
    next.onclick=()=>{if(page+1<pageCount){page++;updatePage();}};
  }
}
function reviewField(attribute:string):string {
  return /^birth\.(day|year)$|^contract\.\d+\.expires\.(day|year)$/.test(attribute)
    ?attribute.replace(/\.(day|year)$/,''):attribute;
}

function reportSummary(report:Report){
  const fields=report.changes.length+(report.staffChanges?.length??0);
  const removed=report.removals.reduce((sum,r)=>sum+r.count,0);
  const market=report.marketJournalEntriesAdded??report.changes.filter(change=>change.attribute===marketField).length;
  const journalSize=report.marketJournalBytesAdded? ' ('+report.marketJournalBytesAdded+t(" bytes acrescentados)"):'';
  const fieldCount=fields+(fields===1?t(" campo"):t(" campos"));
  const byteCount=report.fieldChangedBytes+(report.fieldChangedBytes===1?' byte':' bytes');
  const summary=removed
    ? fieldCount+(fields===1?t(" editado"):t(" editados"))+t(" e ")+removed+(removed===1?t(" registro removido."):t(" registros removidos."))
    : fieldCount+t(" e ")+byteCount+t(" alterados.");
  return market?summary+` ${t("Diário de mercado com")} ${market} ${t("nova(s) entrada(s)")}${journalSize}.`:summary;
}
function resetReview(){
  validatedPlan=null;
  $('review-error').hidden=true;
  $('changes-list').replaceChildren();
}

async function validateReview(reloadPlan=false):Promise<boolean>{
  resetReview();
  try {
    if(reloadPlan)plan=await invoke<Pending>('pending_edits');
    if(!planCount()){
      $('changes-list').innerHTML=notice('info',t("Nenhuma alteração pendente."));
      clearToast();
      return true;
    }
    notify(t("Validando o plano de edição…"));
    const report=await invoke<Report>('preview_edits');
    renderChanges($('changes-list'),report);
    validatedPlan=plan;
    notify(t("Plano válido: ")+reportSummary(report),'success');
    return true;
  } catch(error) {
    $('review-error').hidden=false;
    notify(t("Não foi possível validar o plano. ")+String(error),'error');
    return false;
  }
}

async function review(){
  if(busy||!await canLeave()||busy)return;
  if(!summary){
    if($('saved-copy-history').hidden)return;
    $('empty').hidden=true;
    $('workspace').hidden=false;
    $('active-save-review').hidden=true;
    $('history-start').hidden=false;
    $<HTMLDetailsElement>('saved-copy-history').open=true;
    view('changes');
    $('history-heading').focus();
    return;
  }
  $('active-save-review').hidden=false;
  $('history-start').hidden=true;
  view('changes');
  $('review-title').focus();
  await task(async()=>{await validateReview(true);});
}

$('retry-review').onclick=()=>void review();
$('open-save').onclick=()=>void open();$('choose-other-save').onclick=()=>void open();
$('nav-players').onclick=async()=>{
  if(busy||!summary||!await canLeave()||busy)return;
  delete $('players-view').dataset.pane;
  view('players');
  if(needsInitialPlayerSearch)void search();
};
$('back-to-start').onclick=()=>{
  $('workspace').hidden=true;
  $('empty').hidden=false;
  view('players');
  $('open-save').focus();
};
$('nav-changes').onclick=()=>void review();$('review').onclick=()=>void review();$('nav-diagnostics').onclick=async()=>{if(summary&&await canLeave())view('diagnostics');};
$('results').onclick=e=>{
  const row=(e.target as HTMLElement).closest<HTMLElement>('[data-uid]');
  if(row&&!$('results').hasAttribute('aria-busy'))void task(async()=>{
    if(await showPlayer(Number(row.dataset.uid)))enterProfile('profile');
  });
};
const playerSearch = $<HTMLInputElement>('search');
playerSearch.oninput=()=>{
  if(!summary)return;
  clearPlayerSearchError();
  clearTimeout(timer);
  searchGeneration++;
  $('results').setAttribute('aria-busy','true');
  $('result-count').textContent=t("Buscando…");
  syncPages();
  timer=setTimeout(()=>{page=0;void search().catch(e=>notify(String(e),'error'));},180);
};
playerSearch.onkeydown = async event => {
  if (event.key !== 'Enter' && event.key !== 'ArrowDown') return;
  event.preventDefault();
  if ($('results').hasAttribute('aria-busy')) {
    clearTimeout(timer);
    page = 0;
    try { await search(); }
    catch (error) { notify(String(error), 'error'); return; }
  }
  if (document.activeElement !== playerSearch || $('results').hasAttribute('aria-busy')) return;
  const first = $('results').querySelector<HTMLButtonElement>('[data-uid]');
  if (event.key === 'Enter') first?.click();
  else first?.focus();
};
$('results').onkeydown = event => {
  if (!['ArrowDown', 'ArrowUp'].includes(event.key) || !(event.target instanceof HTMLButtonElement)) return;
  event.preventDefault();
  const next = event.key === 'ArrowDown' ? event.target.nextElementSibling : event.target.previousElementSibling;
  if (next instanceof HTMLButtonElement) next.focus();
  else if (event.key === 'ArrowUp') playerSearch.focus();
};
$('previous').onclick=()=>{page--;void search().catch(e=>notify(String(e),'error'));};$('next').onclick=()=>{page++;void search().catch(e=>notify(String(e),'error'));};
$('changes-list').onclick=async e=>{
  const target=(e.target as HTMLElement).closest<HTMLButtonElement>('[data-edit-review-field]');
  if(target){
    if(!await canLeave())return;
    void task(async()=>{
      const {editReviewUid,editReviewType,editReviewField}=target.dataset;
      const uid=Number(editReviewUid),field=editReviewField!;
      const domain=editReviewType===t("Clube")?'clubs':editReviewType===t("Comissão")?'staff':'players';
      const profileId=domain==='clubs'?'club-profile':domain==='staff'?'staff-profile':'profile';
      const profile=$(profileId);
      let control:HTMLElement|null;
      if(domain==='clubs'){
        await showClub(uid,false);
        control=profile.querySelector('#'+CSS.escape(field));
      }else if(domain==='staff'){
        await showStaff(uid,false);
        const key=field.replace(/^reputation\.(home|current|world)$/,(_,name)=>'reputation-'+['home','current','world'].indexOf(name));
        control=profile.querySelector('#'+CSS.escape('staff-'+key));
      }else{
        await showPlayer(uid,false);
        const key=CSS.escape(field);
        control=field==='healInjuries'?$('remove-injuries'):field==='clearSuspensions'?$('remove-suspensions')
          :field==='nationality'?$('nation-trigger'):profile.querySelector('[data-field='+key+'], [data-date='+key+']');
        if(control instanceof HTMLInputElement&&control.disabled&&control.dataset.date){
          control=profile.querySelector('[data-monthly='+key+']');
        }
      }
      if(!control)throw Error(t("Este campo não está disponível na ficha."));
      view(domain);
      enterProfile(profileId);
      const panel=control.closest('[role=tabpanel]');
      const tab=panel?.getAttribute('aria-labelledby');
      if(tab)$(tab).click();
      if(control.closest('#club-finance-editor')?.hasAttribute('hidden'))$('edit-club-finances').click();
      for(let ancestor=control.parentElement;ancestor&&ancestor!==profile;ancestor=ancestor.parentElement){
        if(ancestor instanceof HTMLDetailsElement)ancestor.open=true;
      }
      control.focus({preventScroll:true});
      control.scrollIntoView({block:'center',inline:'nearest'});
      notify(t("Ajuste o campo e adicione a mudança à revisão."));
    });
    return;
  }
  const button=(e.target as HTMLElement).closest<HTMLButtonElement>('[data-remove-plan-player], [data-remove-review-operation]');
  if(!button||!await canLeave())return;
  const uid=Number(button.dataset.removePlanPlayer);
  const field=button.dataset.removePlanField??null;
  const operation=button.dataset.removeReviewOperation;
  void task(async()=>{
    plan=operation===undefined?await invoke<Pending>('remove_player_from_plan',{uid,field})
      :await invoke<Pending>('remove_review_operation',{index:Number(operation)});
    if(detail){detail=await invoke<Detail>('player_detail',{uid:detail.player.uid});renderProfile();}
    if(staffDetail)await showStaff(staffDetail.profile.uid,false);
    if(clubSelected!==null)await showClub(clubSelected,false);
    if(await validateReview()){
      notify((operation!==undefined?t("Operação retirada do plano."):field?t("Alteração retirada do plano."):t("Jogador retirado do plano."))+t(" Use Desfazer para restaurar."),'success');
    }
  });
};
for(const action of ['undo','redo','clear'])$(action).onclick=async()=>{
  if(!await canLeave())return;
  void task(async()=>{
    plan=await invoke<Pending>('history_action',{action});
    resetReview();
    if(detail){detail=await invoke<Detail>('player_detail',{uid:detail.player.uid});renderProfile();}
    if(staffDetail)await showStaff(staffDetail.profile.uid,false);
    if(clubSelected!==null)await showClub(clubSelected,false);
    if(!$('my-club-view').hidden)await loadMyClub();
    if(await validateReview())notify(t("Plano de alterações atualizado."));
  });
};
interface SavedCopyEntry {
  path:string;
  source:string;
  date:string;
  result:string;
  sourceSha256:string;
  copySha256:string;
  reportId?:string;
}
const savedCopiesKey='cm0304.saved-copies.v1';
function accessSavedReport(id:string,report?:Report):Promise<Report|undefined> {
  return new Promise((resolve,reject)=>{
    const request=indexedDB.open('cm0304.saved-reports',1);
    request.onupgradeneeded=()=>request.result.createObjectStore('reports');
    request.onerror=()=>reject(request.error);
    request.onsuccess=()=>{
      const db=request.result;
      const transaction=db.transaction('reports',report?'readwrite':'readonly');
      const store=transaction.objectStore('reports');
      const operation=report?store.put(report,id):store.get(id);
      transaction.onabort=()=>{db.close();reject(transaction.error);};
      transaction.oncomplete=()=>{db.close();resolve(report??operation.result);};
    };
  });
}
function savedCopies():SavedCopyEntry[] {
  try {
    const saved:unknown=JSON.parse(localStorage.getItem(savedCopiesKey)??'[]');
    if(Array.isArray(saved))return saved.filter(entry=>entry&&typeof entry.path==='string'
      &&typeof entry.source==='string'&&typeof entry.date==='string'&&typeof entry.result==='string'
      &&typeof entry.sourceSha256==='string'&&typeof entry.copySha256==='string').slice(0,30);
  } catch { /* Use an empty history if the preference cannot be read. */ }
  return [];
}
function renderSavedCopies() {
  const copies=savedCopies();
  const filter=$<HTMLInputElement>('saved-copy-current-origin');
  filter.disabled=!summary;
  const origin=filter.checked?summary?.sha256.toLowerCase():undefined;
  const entries=origin?copies.filter(entry=>entry.sourceSha256.toLowerCase()===origin):copies;
  const history=$('saved-copy-history');
  history.hidden=copies.length===0;
  $('saved-copy-entries').innerHTML=entries.map((entry,index)=>
    '<article class="saved-copy-entry"><strong>'+escape(entry.path.split(/[\\/]/).pop()??entry.path)
    +'</strong> · '+escape(new Date(entry.date).toLocaleString(locale))
    +`<dl><dt>${t("Cópia")}</dt><dd>`+escape(entry.path)+`</dd><dt>${t("Origem")}</dt><dd>`+escape(entry.source)
    +`</dd><dt>${t("Resultado")}</dt><dd>`+escape(entry.result)+`</dd><dt>${t("SHA-256 da origem")}</dt><dd>`
    +escape(entry.sourceSha256)+`</dd><dt>${t("SHA-256 da cópia")}</dt><dd>`+escape(entry.copySha256)
    +'</dd></dl><button type="button" data-copy-index="'+index+`">${t("Abrir cópia")}</button>`
    +(entry.source?' <button type="button" data-origin-index="'+index+`">${t("Abrir origem")}</button>`:'')
    +(typeof entry.reportId==='string'?' <button type="button" data-report-index="'+index+'" aria-expanded="false" aria-controls="archived-report-'+index+`">${t("Ver relatório")}</button>`
      +' <button type="button" data-export-report-index="'+index+`">${t("Exportar relatório JSON")}</button>`
      +'<div id="archived-report-'+index+'" hidden></div>':'')+'</article>').join('')
    ||`<p class="notice" data-notice="info">${t("Nenhuma cópia gravada a partir do save ativo.")}</p>`;
  $('saved-copy-entries').onclick=event=>{
    const button=(event.target as HTMLElement).closest<HTMLButtonElement>('[data-copy-index], [data-origin-index], [data-report-index], [data-export-report-index]');
    if(!button)return;
    const index=Number(button.dataset.copyIndex??button.dataset.originIndex??button.dataset.reportIndex??button.dataset.exportReportIndex);
    const entry=entries[index];
    if(!entry)return;
    if(button.dataset.copyIndex!==undefined){void open(entry.path);return;}
    if(button.dataset.originIndex!==undefined){void open(entry.source);return;}
    const reportId=entry.reportId;
    if(!reportId)return;
    if(button.dataset.exportReportIndex!==undefined){
      void task(async()=>{
        const report=await accessSavedReport(reportId);
        if(!report)throw new Error(t("O relatório detalhado está indisponível."));
        const path=await invoke<string|null>('export_saved_report',{json:JSON.stringify(report,null,2)});
        notify(path?t("Relatório de gravação exportado: ")+path:t("Exportação cancelada."),path?'success':undefined);
      });
      return;
    }
    const content=$('archived-report-'+index);
    content.hidden=!content.hidden;
    button.setAttribute('aria-expanded',String(!content.hidden));
    if(content.hidden||content.dataset.loaded)return;
    void accessSavedReport(reportId).then(report=>{
      if(!report)throw new Error(t("Relatório ausente"));
      renderChanges(content,report,false);
      content.dataset.loaded='true';
    }).catch(()=>{
      const message=t("O relatório detalhado está indisponível.");
      content.textContent=message;
      if(content.isConnected && content.closest('details')?.open)notify(message,'warning');
    });
  };
}

$('saved-copy-current-origin').onchange=renderSavedCopies;

async function saveCopy(variantName:string|null=null){
  if(busy||!summary||!planCount()||validatedPlan!==plan||!await canLeave())return;
  if(busy||validatedPlan!==plan)return;
  const source=summary.path;
  void task(async()=>{
    notify(variantName?t("Gravando a variante na pasta do save…"):t("Escolha um nome novo para a cópia. O original não será sobrescrito."));
    const saved=await withSaveProgress('save-copy',t("Cancelando gravação…"),'cancel_save_copy',
      ()=>invoke<{path:string;report:Report}|null>('save_copy',{variantName}));
    if(!saved){notify(t("Gravação cancelada."));return;}
    const {path,report}=saved;
    const result=reportSummary(report);
    const confirmation=$('saved-path');
    const name=path.split(/[\\/]/).pop()??path;
    confirmation.innerHTML='<strong>'+escape(name)+'</strong><span>'+escape(path)+'</span>'
      +`<div class="actions"><button type="button" id="open-copy-folder">${t("Abrir pasta de destino")}</button>`
      +`<button type="button" id="open-copy-history">${t("Ver histórico de cópias")}</button></div>`;
    $('open-copy-folder').onclick=()=>void task(async()=>{await invoke('open_copy_folder');});
    $('open-copy-history').onclick=()=>{
      const history=$<HTMLDetailsElement>('saved-copy-history');
      history.open=true;
      history.querySelector('summary')?.focus();
    };
    const entry:SavedCopyEntry={path,source,date:new Date().toISOString(),result,sourceSha256:report.sourceSha256,copySha256:report.copySha256};
    const reportId=JSON.stringify([path,report.sourceSha256,report.copySha256]);
    try{
      await accessSavedReport(reportId,report);
      entry.reportId=reportId;
    }catch{ /* The summary can still be recorded. */ }
    try {
      localStorage.setItem(savedCopiesKey,JSON.stringify([entry,...savedCopies().filter(item=>item.path!==path)].slice(0,30)));
    } catch {
      notify(t("Cópia salva e verificada. O histórico local não pôde ser atualizado."),'error');return;
    }
    renderSavedCopies();
    notify(t("Cópia validada salva: ")+result+(entry.reportId?'':t(" O relatório detalhado não pôde ser arquivado.")),entry.reportId?'success':'warning');
  });
}
window.addEventListener('save-close-blocked',()=>{
  if (!$('save-copy-progress').hidden) {
    notify(t("Aguarde a gravação da cópia ou cancele a gravação antes de fechar o editor."),'warning');
  }
});
let closingEditor=false;
const maximizeWindow = $<HTMLButtonElement>('window-maximize');
let windowStateRequest = 0;
async function syncWindowMaximized() {
  const request = ++windowStateRequest;
  const maximized = await invoke<boolean>('plugin:window|is_maximized');
  if (request !== windowStateRequest) return;
  const label = maximized ? t("Restaurar janela") : t("Maximizar janela");
  maximizeWindow.toggleAttribute('data-maximized', maximized);
  maximizeWindow.setAttribute('aria-label', label);
  maximizeWindow.title = label;
}

function reportWindowError(error: unknown) {
  notify(t("Não foi possível controlar a janela. ") + String(error), 'error');
}

$('window-minimize').onclick = () => void invoke('plugin:window|minimize').catch(reportWindowError);
maximizeWindow.onclick = () => void invoke('plugin:window|toggle_maximize')
  .then(syncWindowMaximized).catch(reportWindowError);
$('window-close').onclick = () => void invoke('plugin:window|close').catch(reportWindowError);
window.addEventListener('resize', () => void syncWindowMaximized().catch(reportWindowError));
void syncWindowMaximized().catch(reportWindowError);

window.addEventListener('editor-close-requested',async()=>{
  if(busy||closingEditor||document.querySelector('dialog[open]'))return;
  closingEditor=true;
  try {
    if((planCount()||hasFormDrafts())&&!await confirmAction(t("Fechar o editor?"),
      t("As alterações da sessão, incluindo campos ainda em edição, serão descartadas."),t("Fechar editor")))return;
    await invoke('close_editor');
  } catch(error) {
    notify(String(error),'error');
  } finally {
    closingEditor=false;
  }
});
$('save-copy').onclick=()=>saveCopy();
$<HTMLFormElement>('save-variant-form').onsubmit=event=>{
  event.preventDefault();
  const name=$<HTMLInputElement>('save-variant-name').value.trim();
  if(!name){notify(t("Informe um nome para a variante."),'error');return;}
  saveCopy(name);
};
renderSavedCopies();

const shortcutButtons=[
  {id:'open-save',key:'o',mod:'Control',label:'Ctrl+O'},
  {id:'nav-changes',key:'r',mod:'Alt',label:'Alt+R'},
  {id:'save-copy',key:'s',mod:'Control+Shift',label:'Ctrl+Shift+S'},
  {id:'undo',key:'z',mod:'Control',label:'Ctrl+Z'},
  {id:'redo',key:'y',mod:'Control',label:'Ctrl+Y'},
];
for(const shortcut of shortcutButtons){
  const button=$<HTMLButtonElement>(shortcut.id);
  button.setAttribute('aria-keyshortcuts',shortcut.mod+'+'+shortcut.key.toUpperCase());
  button.title=button.textContent?.trim()+' ('+shortcut.label+')';
}
document.addEventListener('keydown',event=>{
  if(event.defaultPrevented||event.repeat||event.isComposing||event.metaKey)return;
  if(document.querySelector('dialog[open]'))return;
  const focused=document.activeElement;
  const editing=focused instanceof HTMLElement
    && (focused.isContentEditable||Boolean(focused.closest('input, textarea, select')));
  const modifiers=[event.ctrlKey?'Control':'',event.altKey?'Alt':'',event.shiftKey?'Shift':''].filter(Boolean).join('+');
  const shortcut=shortcutButtons.find(item=>item.key===event.key.toLowerCase()&&item.mod===modifiers);
  if(!shortcut)return;
  if(editing&&(shortcut.id==='undo'||shortcut.id==='redo'))return;
  if(shortcut.id==='nav-changes'&&!summary&&$('saved-copy-history').hidden)return;
  const button=$<HTMLButtonElement>(shortcut.id);
  if(button.closest('[hidden]'))return;
  event.preventDefault();
  if(!busy&&!button.disabled)button.click();
});
$('nav-staff').onclick=async()=>{
  if(!summary||!await canLeave())return;
  delete $('staff-view').dataset.pane;
  renderProfile();view('staff');
  void task(async()=>{await searchStaff();if(staffDetail)await showStaff(staffDetail.profile.uid);else notify(t("Selecione um membro do staff para consultar."));});
};
$('staff-results').onclick=e=>{
  const row=(e.target as HTMLElement).closest<HTMLElement>('[data-staff-uid]');
  if(row&&!$('staff-results').hasAttribute('aria-busy'))void task(async()=>{
    if(await showStaff(Number(row.dataset.staffUid)))enterProfile('staff-profile');
  });
};
const staffSearch = $<HTMLInputElement>('staff-search');
async function refreshStaffFilter() {
  clearTimeout(staffTimer);
  staffTimer = undefined;
  staffPage = 0;
  const generation = staffGeneration + 1;
  try {
    await searchStaff();
  } catch (error) {
    if (generation === staffGeneration) {
      $('staff-results').innerHTML = notice('error',t("Não foi possível buscar membros do staff."));
      notify(String(error), 'error');
    }
  } finally {
    if (generation === staffGeneration) $('staff-results').removeAttribute('aria-busy');
  }
}
staffSearch.oninput = () => {
  clearTimeout(staffTimer);
  staffGeneration++;
  $('staff-results').setAttribute('aria-busy', 'true');
  staffTimer = setTimeout(() => void refreshStaffFilter(), 180);
};
staffSearch.onkeydown = async event => {
  if (event.key !== 'Enter' && event.key !== 'ArrowDown') return;
  event.preventDefault();
  if ($('staff-results').hasAttribute('aria-busy')) await refreshStaffFilter();
  if (document.activeElement !== staffSearch || $('staff-results').hasAttribute('aria-busy')) return;
  const first = $('staff-results').querySelector<HTMLButtonElement>('[data-staff-uid]');
  if (event.key === 'Enter') first?.click();
  else first?.focus();
};
$('staff-results').onkeydown = event => {
  if (!['ArrowDown', 'ArrowUp'].includes(event.key) || !(event.target instanceof HTMLButtonElement)) return;
  event.preventDefault();
  const next = event.key === 'ArrowDown' ? event.target.nextElementSibling : event.target.previousElementSibling;
  if (next instanceof HTMLButtonElement) next.focus();
  else if (event.key === 'ArrowUp') staffSearch.focus();
};
$('staff-previous').onclick=()=>{if($('staff-results').hasAttribute('aria-busy'))return;staffPage--;void searchStaff().catch(e=>notify(String(e),'error'));};
$('staff-next').onclick=()=>{if($('staff-results').hasAttribute('aria-busy'))return;staffPage++;void searchStaff().catch(e=>notify(String(e),'error'));};
let clubPage=0,clubTotal=0,clubGeneration=0,clubSelected:number|null=null,clubTimer:ReturnType<typeof setTimeout>|undefined;
const clubExpiryFilters=new Map<number,{values:string[];open:boolean}>();
function syncClubPages(){$<HTMLButtonElement>('club-previous').disabled=busy||clubPage===0;$<HTMLButtonElement>('club-next').disabled=busy||(clubPage+1)*50>=clubTotal;$('club-page-label').textContent=(clubPage+1)+' / '+Math.max(1,Math.ceil(clubTotal/50));}
function resetClubs(){clubTab='finance';resetClubAnalysis();clubExpiryFilters.clear();clubPage=0;clubTotal=0;clubSelected=null;clubGeneration++;clearTimeout(clubTimer);clubTimer=undefined;$('club-results').removeAttribute('aria-busy');$<HTMLInputElement>('club-search').value='';$('club-results').innerHTML='';$('club-result-count').textContent=t("0 resultados");$('club-count').textContent=(summary?.clubs??0).toLocaleString(locale);$('club-profile').innerHTML=`<div class="profile-placeholder"><h2>${t("Escolha um clube")}</h2><p>${t("Consulte os vínculos e abra os perfis do elenco e staff.")}</p></div>`;syncClubPages();}
async function searchClubs(){if(!summary)return;const generation=++clubGeneration;const result=await invoke<{total:number;rows:ClubRow[]}>('search_clubs',{query:$<HTMLInputElement>('club-search').value,page:clubPage});if(generation!==clubGeneration)return;clubTotal=result.total;$('club-result-count').textContent=resultCount(clubTotal);$('club-results').innerHTML=result.rows.map(c=>'<button class="player-row '+(clubSelected===c.uid?'selected':'')+'" data-club-uid="'+c.uid+'"><span><strong>'+escape(c.name)+'</strong><small>UID '+c.uid+' · '+c.staff+` ${t("perfis de staff")}</small></span><span class="abilities">`+c.players+'</span></button>').join('')||`<p class="notice" data-notice="info">${t("Nenhum clube encontrado.")}</p>`;syncClubPages();}
const clubNumber=new Intl.NumberFormat(locale);

function clubFinanceAmount(value:number|null|undefined, prefix='£ '){
  return value==null?t("Indisponível"):prefix+clubNumber.format(value);
}

const myClubPlayerDescriptions:Record<string,[string,string]> = {
  injuries:[t("jogador lesionado"),t("jogadores lesionados")],
  suspensions:[t("jogador suspenso"),t("jogadores suspensos")],
  condition:[t("jogador abaixo de 100%"),t("jogadores abaixo de 100%")],
  morale:[t("jogador com moral abaixo de Excelente"),t("jogadores com moral abaixo de Excelente")],
};
function myClubActionLabel(shortcut:MyClubShortcut):string {
  if(shortcut.value!==null){
    const label=shortcut.inReview?t("Na revisão"):shortcut.command==='cash'?t("Saldo"):
      shortcut.command==='transfers'?t("Verba restante"):t("Orçamento salarial");
    return [label+' · '+clubFinanceAmount(shortcut.value)+(shortcut.command==='wages'?t('/semana'):''),
      shortcut.reason ? t(shortcut.reason) : null].filter(Boolean).join(' · ');
  }
  if(shortcut.reason)return t(shortcut.reason);
  const description=myClubPlayerDescriptions[shortcut.command];
  if(!description)return t("Campo financeiro ausente");
  const affected=shortcut.affected;
  const parts=[affected+' '+description[affected===1?0:1]];
  if(shortcut.inReview)parts.push(shortcut.inReview+t(" na revisão"));
  if(shortcut.unavailable)parts.push(shortcut.unavailable+(shortcut.unavailable===1?t(" indisponível"):t(" indisponíveis")));
  return parts.join(' · ');
}
function syncMyClubShortcuts(){
  const shortcuts=myClub?.shortcuts??[];
  for(const button of $('my-club-shortcuts').querySelectorAll<HTMLButtonElement>('[data-my-club-action]')){
    const shortcut=shortcuts.find(item=>item.command===button.dataset.myClubAction);
    const enabled=shortcut&&!shortcut.reason&&(shortcut.target!==null||shortcut.affected>0);
    button.setAttribute('aria-disabled',String(busy||!enabled));
    button.classList.toggle('my-club-staged',!!shortcut?.inReview);
    button.querySelector('span')!.textContent=shortcut?myClubActionLabel(shortcut):'';
  }
}
function resetMyClub(){
  const clubs=summary?.managedClubs??[];
  myClubUid=clubs.length===1?clubs[0].uid:null;
  myClub=null;
  const picker=$<HTMLSelectElement>('my-club-select');
  picker.replaceChildren(new Option(t("Escolha um clube"),''),...clubs.map(club=>new Option(club.name,String(club.uid))));
  $('my-club-picker').hidden=clubs.length<2;
  $('my-club-view').scrollTop=0;
}
async function loadMyClub(){
  const club=summary?.managedClubs?.find(item=>item.uid===myClubUid);
  $('my-club-title').textContent=club?.name??t("Meu clube");
  $('my-club-scope').hidden=!club;
  $('my-club-shortcuts').hidden=true;
  $('my-club-actions').hidden=true;
  $('my-club-message').hidden=!!club;
  $('my-club-message').textContent=t("Escolha o clube do treinador para usar os atalhos.");
  myClub=null;
  if(club)myClub=await invoke<MyClubDetail>('my_club_detail',{uid:club.uid});
  $('my-club-shortcuts').hidden=!myClub;
  $('my-club-actions').hidden=!myClub;
  syncMyClubShortcuts();
}
$('nav-my-club').onclick=async()=>{
  if(busy||!summary?.managedClubs?.length||!await canLeave()||busy)return;
  view('my-club');
  clearToast();
  void task(async()=>{await loadMyClub();$('my-club-title').focus();});
};
$<HTMLSelectElement>('my-club-select').onchange=e=>{
  if(busy)return;
  const value=(e.currentTarget as HTMLSelectElement).value;
  myClubUid=value?Number(value):null;
  clearToast();
  void task(loadMyClub);
};
$('my-club-shortcuts').onclick=e=>{
  const button=(e.target as HTMLElement).closest<HTMLButtonElement>('[data-my-club-action]');
  if(busy||!button||button.getAttribute('aria-disabled')==='true'||myClubUid===null)return;
  const uid=myClubUid;
  void task(async()=>{
    notify(t("Validando o atalho…"));
    const result=await invoke<{plan:Pending;club:MyClubDetail;applied:MyClubShortcut}>('stage_my_club_action',
      {uid,command:button.dataset.myClubAction});
    plan=result.plan;
    myClub=result.club;
    resetReview();
    const {value,target,affected}=result.applied;
    const changes=value!==null&&target!==null
      ? clubFinanceAmount(value)+' → '+clubFinanceAmount(target)+(result.applied.command==='wages'?t('/semana'):'')
      : affected+(affected===1?t(" jogador"):t(" jogadores"));
    notify(button.querySelector('strong')!.textContent+' · '+changes+t(" · na revisão."),'success');
  });
};
$('my-club-undo').onclick=()=>$('undo').click();
$('my-club-review').onclick=()=>void review();

function renderClubFinanceIndicators(uid:number,info:ClubInfo){
  const metric=(label:string,value:string,context:string,pending:string[])=>
    '<div><dt>'+label+'</dt><dd>'+value+'</dd><dd class="finance-metric-context">'+context+'</dd>'
    +pending.map(text=>'<dd class="finance-pending">'+text+'</dd>').join('')+'</div>';
  const balance=plan.clubBalances?.[uid];
  const transfer=plan.clubTransferBudgets?.[uid];
  const wage=plan.clubWageBudgets?.[uid];
  const pending=(label:string,original:number|null|undefined,value:number|undefined,prefix?:string)=>
    value==null||value===original?[]:[label+' · '+clubFinanceAmount(value,prefix)];
  return metric(t("Saldo registrado"),clubFinanceAmount(info.balance,''),t("Valor bruto"),
    pending(t("Na revisão"),info.balance,balance,''))
    +metric(t("Verba de transferências restante"),clubFinanceAmount(info.transferBudgetRemaining),
      t("Temporada · ")+clubFinanceAmount(info.transferBudgetSeason),[
        ...pending(t("Na revisão"),info.transferBudgetRemaining,transfer?.remaining),
        ...pending(t("Temporada na revisão"),info.transferBudgetSeason,transfer?.season),
      ])
    +metric(t("Orçamento salarial"),clubFinanceAmount(info.wageBudgetWeekly),t("Por semana"),
      pending(t("Na revisão"),info.wageBudgetWeekly,wage));
}

function clubNumberField({id,label,hint,min=0,max=2147483647,value,recorded,original,help}:{
  id:string;label:string;hint?:string;min?:number;max?:number;value:number;
  recorded:string;original:number|null|undefined;help?:string;
}){
  const description=id+'-recorded'+(help?' '+id+'-help':'');
  return '<div class="club-number-field"><label for="'+id+'">'+escape(label)
    +(hint?' <span>('+escape(hint)+')</span>':'')+'</label>'
    +'<input id="'+id+'" type="number" required step="1" min="'+min+'" max="'+max+'" value="'+value
    +'" data-initial="'+value+'" data-recorded="'+(original??'')+'" aria-describedby="'+description+'">'
    +'<small id="'+id+`-recorded">${t("Registrado ·")} `+escape(recorded)+'</small>'
    +(help?'<small id="'+id+'-help">'+escape(help)+'</small>':'')+'</div>';
}

function renderClubInfo(uid:number,info:ClubInfo|null|undefined,projectedPayroll: number|null|undefined) {
  if(!info){const unavailable=notice('info',t("Dados do clube indisponíveis neste save."));return {finance:unavailable,infrastructure:unavailable};}
  const format=clubNumber;
  const financeAvailability = info.balanceEditable ? ''
    : notice('info',info.balance === null
      ? t("Este registro não tem dados de saldo ou orçamento.")
      : t("Registro financeiro básico. O saldo bruto está disponível para consulta."));
  const target=plan.clubBalances?.[uid]??info.balance;
  const budgets=plan.clubTransferBudgets?.[uid];
  const moneyInput=(id:string,label:string,unit:string,recorded:number|null|undefined,value:number|null|undefined,min=0,max=2147483647)=>value==null?''
    : clubNumberField({id,label,hint:unit,recorded:clubFinanceAmount(recorded),original:recorded,value,min,max});
  const form=info.balanceEditable&&target!==null
    ? '<section id="club-finance-editor" aria-labelledby="club-finance-editor-title" hidden>'
      +`<div class="panel-heading"><h3 id="club-finance-editor-title" tabindex="-1">${t("Editar finanças")}</h3>`
      +`<button id="close-club-finances" type="button">${t("Recolher")}</button></div>`
      +'<form id="club-balance-form" class="club-fields-form" novalidate>'
      +moneyInput('club-balance',t("Saldo"),'£',info.balance,target,-2147483648)
      +moneyInput('club-budget-season',t("Verba da temporada"),'£',info.transferBudgetSeason,budgets?.season??info.transferBudgetSeason)
      +moneyInput('club-budget-remaining',t("Verba restante"),'£',info.transferBudgetRemaining,budgets?.remaining??info.transferBudgetRemaining)
      +moneyInput('club-wage-budget',t("Orçamento salarial"),t('£/semana'),info.wageBudgetWeekly,plan.clubWageBudgets?.[uid]??info.wageBudgetWeekly,0,536870911)
      +`<button type="submit" form="club-balance-form" class="primary" disabled>${t("Adicionar à revisão")}</button></form></section>`
    : '';
  const levelInput=(id:string,label:string,original:number,value:number)=>
    clubNumberField({id,label,hint:'1–20',min:1,max:20,recorded:format.format(original),original,value});
  const reputation=plan.clubReputation?.[uid]??info.reputation;
  const reputationInput=reputation==null?''
    : clubNumberField({id:'club-reputation',label:t("Reputação da equipe principal"),hint:'0–10.000',max:10000,
      value:reputation,original:info.reputation,recorded:info.reputation==null?t("Indisponível"):format.format(info.reputation),
      help:t("A reputação pode mudar como o jogo descreve as condições de treino.")});
  const trainingForm='<form id="club-training-form" class="club-fields-form club-fields-card" novalidate>'
    +levelInput('club-training',t("Nível de treino"),info.trainingRaw,plan.clubTraining?.[uid]??info.trainingRaw)
    +levelInput('club-youth',t("Formação de jovens"),info.youthSetupRaw,plan.clubYouthSetup?.[uid]??info.youthSetupRaw)
    +reputationInput
    +`<button type="submit" form="club-training-form" class="primary" disabled>${t("Adicionar à revisão")}</button></form>`;
  const financeRow=(label:string,amounts:number[],className='')=>
    '<tr'+(className?' class="'+className+'"':'')+'><th scope="row">'+label+'</th>'
    +amounts.map(amount=>'<td>'+format.format(amount)+'</td>').join('')+'</tr>';
  const finance=info.financeSummary;
  const periods=finance?[finance.currentMonth,finance.previousMonth,finance.season]:[];
  const totals=finance?financeRow(t("Receitas"),periods.map(period=>period.income))
    +financeRow(t("Despesas"),periods.map(period=>period.expenses))
    +financeRow(t("Resultado"),periods.map(period=>period.income-period.expenses),'finance-result'):'';
  const payroll=info.payrollExpenses;
  const expenseRow=(label:string,amounts:ExpensePeriods)=>
    financeRow(label,[amounts.currentMonth,amounts.previousMonth,amounts.season]);
  const payrollRows=payroll?expenseRow(t("Despesas com remunerações"),payroll.wages)
    +expenseRow(t("Despesas com prêmios"),payroll.bonuses):'';
  const financialTable=totals||payrollRows
    ? `<div class="finance-table-wrap" role="region" aria-label="${t("Movimentação financeira")}" tabindex="0"><table class="club-expenses"><caption>${t("Movimentação financeira registrada (£)")}</caption><thead><tr><th scope="col">${t("Rubrica")}</th><th scope="col">${t("Este mês")}</th>`
      +`<th scope="col">${t("Mês anterior")}</th><th scope="col">${t("Temporada")}</th></tr></thead><tbody>`
      +totals+payrollRows+'</tbody></table></div>'
    : `<section class="finance-movement"><h3>${t("Movimentação financeira")}</h3><p class="profile-meta">${t("Dados indisponíveis neste save.")}</p></section>`;
  const clauses=info.sellOnClauses??[];
  const sellOn=clauses.length?`<details><summary>${t("Cláusulas de revenda")}</summary><ul>`
    +clauses.map(clause=>'<li>'+escape(clause.playerName??'UID '+clause.playerUid)
      +' · '+clause.percent+t("% para ")+escape(clause.beneficiary??t("clube não identificado"))+'</li>').join('')
    +'</ul></details>':'';
  const installments=info.transferInstallments??[];
  const installmentTable=installments.length?
    `<table><thead><tr><th scope="col">${t("Jogador")}</th><th scope="col">${t("Clube")}</th><th scope="col">${t("Movimento")}</th>`
    +`<th scope="col">${t("Valor bruto por prestação")}</th><th scope="col">${t("Restantes")}</th></tr></thead><tbody>`
    +installments.map(row=>'<tr><td>'+escape(row.playerName??'ID '+row.personId)+'</td><td>'+escape(row.counterparty)
      +'</td><td>'+(row.outgoing?t("A pagar"):t("A receber"))+'</td><td>'+format.format(row.amountRaw)+'</td><td>'+row.remaining+'</td></tr>').join('')
    +'</tbody></table>'
    : '<p class="notice" data-notice="'+(info.transferInstallments===null?'warning':'info')+'">'+(info.transferInstallments===null
      ? t("Formato de prestações indisponível neste save.") : t("Nenhuma prestação registrada para este clube."))+'</p>';
  const installmentSection=`<details><summary>${t("Prestações de transferências")}</summary>`+installmentTable+'</details>';
  const payrollFact=(label:string,weekly:number|null|undefined)=>weekly==null?''
    : '<dt>'+label+'</dt><dd>'+clubFinanceAmount(weekly*4)+'</dd>';
  const contracts=info.playerContractWagesWeekly;
  const loanOffset=info.loanWageOffsetWeekly;
  const payrollFacts=payrollFact(t("Total de salários registrado"),info.currentWagesWeekly)
    +payrollFact(t("Salários dos contratos de jogadores"),contracts)
    +payrollFact(t("Folha estimada após contribuições de empréstimos"),contracts==null||loanOffset==null?null:contracts-loanOffset)
    +payrollFact(t("Folha projetada de jogadores com o plano"),projectedPayroll);
  const payrollDetails=payrollFacts?`<details><summary>${t("Detalhes da folha")}</summary>`
    +`<p class="profile-meta">${t("Valores mensais em libras, calculados com quatro semanas.")}</p>`
    +'<dl class="club-finance-facts">'+payrollFacts+'</dl></details>':'';
  const editAction=form?'<div class="finance-edit-action"><span id="club-finance-draft" role="status"></span>'
    +`<button id="edit-club-finances" type="button" aria-expanded="false" aria-controls="club-finance-editor">${t("Editar finanças")}</button></div>`:'';
  const financePanel=`<div class="panel-heading finance-heading"><h3>${t("Visão geral")}</h3>`+editAction+'</div>'
    +'<div class="finance-layout"><div class="finance-overview"><dl id="club-finance-indicators" class="finance-metrics">'
    +renderClubFinanceIndicators(uid,info)+'</dl>'+financialTable+financeAvailability
    +'<div class="finance-details">'+payrollDetails+sellOn+installmentSection+'</div></div>'+form+'</div>';
  const infrastructure=`<h3 class="club-section-title">${t("Infraestrutura")}</h3>`+trainingForm;
  return {finance:financePanel,infrastructure};
}
function renderClubStadium(uid:number,stadium:ClubStadium|null|undefined):string {
  if(!stadium)return notice('info',t("Dados do estádio indisponíveis neste save."));
  const format=clubNumber;
  const clubs = stadium.firstTeamClubUids.length;
  const target = plan.clubStadiumCapacities?.[uid]??stadium;
  const title=`<h3 class="club-section-title">${t("Estádio")}</h3>`;
  if(clubs===1)return title+'<form id="club-stadium-form" class="club-fields-form club-fields-card">'
    +clubNumberField({id:'stadium-capacity',label:t("Lotação total"),min:1,value:target.capacity,original:stadium.capacity,recorded:format.format(stadium.capacity)})
    +clubNumberField({id:'stadium-seated-capacity',label:t("Lugares sentados"),value:target.seatedCapacity,original:stadium.seatedCapacity,recorded:format.format(stadium.seatedCapacity)})
    +`<button type="submit" form="club-stadium-form" class="primary" disabled>${t("Adicionar à revisão")}</button></form>`;
  return title+`<dl class="club-finance-facts"><dt>${t("Lotação total registrada")}</dt><dd>`+format.format(stadium.capacity)
    +`</dd><dt>${t("Lugares sentados registrados")}</dt><dd>`+format.format(stadium.seatedCapacity)+'</dd>'
    +(clubs>1?`<dt>${t("Clubes que usam o estádio com a equipe principal")}</dt><dd>`+format.format(clubs)+'</dd>':'')+'</dl>'
    +notice('info',t("Estádio compartilhado disponível para consulta."));
}
function renderClubExpiryRows(rows:ClubExpiry[],contracts:Map<string,ClubWageContract>):string {
  const format=new Intl.NumberFormat(locale);
  if(!rows.length)return `<p class="notice" data-notice="info">${t("Nenhum vínculo corresponde aos filtros.")}</p>`;
  const wages=rows.flatMap(row=>{
    const weekly=contracts.get(row.uid+':'+row.index)?.weeklyWage;
    return weekly===undefined?[]:[weekly];
  });
  const count=rows.length===1?t("1 vínculo"):format.format(rows.length)+t(" vínculos");
  const subtotal=wages.length===rows.length
    ?t(" · Soma dos salários exibidos: £")+format.format(wages.reduce((sum,wage)=>sum+wage,0))+t('/semana')
    :'';
  const entries=rows.map(row=>{
    const label=row.days===null?t("Mensal"):comparisonDate(row.expires);
    const remaining=row.days===null?t("Sem data fixa"):row.days<0?t("Vencido"):row.days===0?t("Hoje"):row.days===1?t("1 dia"):row.days+t(" dias");
    const contract=contracts.get(row.uid+':'+row.index);
    const wage=contract?.weeklyWage;
    const squadStatus=contract?.squadStatus;
    return '<button class="player-row" data-club-player="'+row.uid+'" data-club-contract="'+row.index+'"><span><strong>'+escape(row.name)
      +'</strong><small>'+(row.loan?t("Empréstimo"):t("Contrato completo"))+' · '+escape(label)
      +(wage===undefined?'':' · £'+format.format(wage)+t('/semana'))
      +(squadStatus===undefined?'':' · '+escape(squadStatusLabels[squadStatus+1]??squadStatus))
      +'</small></span><span class="abilities">'+remaining+'</span></button>';
  }).join('');
  return '<p class="profile-meta">'+count+subtotal+'</p>'+entries;
}

function renderClubExpiries(rows:ClubExpiry[]):string {
  const upcoming=rows.filter(row=>row.days!==null&&row.days>=0&&row.days<=90).length;
  return `<details><summary>${t("Contratos do clube · próximos 90 dias:")} `+upcoming+'</summary>'
    +`<div id="club-contract-filters"><label>${t("Prazo")} <select id="club-contract-period">`
    +`<option value="30">${t("Próximos 30 dias")}</option><option value="60">${t("Próximos 60 dias")}</option>`
    +`<option value="upcoming" selected>${t("Próximos 90 dias")}</option><option value="180">${t("Próximos 180 dias")}</option>`
    +`<option value="expired">${t("Vencidos")}</option>`
    +`<option value="monthly">${t("Mensais")}</option><option value="all">${t("Todos")}</option></select></label> `
    +`<label>${t("Vínculo")} <select id="club-contract-kind"><option value="all">${t("Todos")}</option>`
    +`<option value="owned">${t("Contrato completo")}</option><option value="loan">${t("Empréstimo")}</option></select></label> `
    +`<label>${t("Situação")} <select id="club-contract-status"><option value="all">${t("Todas")}</option>`
    +squadStatusLabels.slice(1).map((label,index)=>'<option value="'+index+'">'+label+'</option>').join('')
    +'</select></label> '
    +`<label>${t("Ordenar")} <select id="club-contract-order"><option value="date">${t("Vencimento")}</option>`
    +`<option value="wage">${t("Maior salário semanal")}</option></select></label></div>`
    +'<div id="club-contract-results" aria-live="polite"></div></details>';
}

function bindClubExpiries(uid:number,rows:ClubExpiry[],contracts:ClubWageContract[]){
  const section=$('club-contract-filters').closest('details')!;
  const controls=['period','kind','status','order'].map(field=>$<HTMLSelectElement>('club-contract-'+field));
  const previous=clubExpiryFilters.get(uid);
  if(previous){
    controls.forEach((control,index)=>control.value=previous.values[index]);
    section.open=previous.open;
  }
  const remember=()=>{
    if(section.isConnected)clubExpiryFilters.set(uid,{values:controls.map(control=>control.value),open:section.open});
  };
  const byContract=new Map(contracts.map(contract=>[contract.uid+':'+contract.index,contract]));
  const wage=(row:ClubExpiry)=>byContract.get(row.uid+':'+row.index)?.weeklyWage??-1;
  const render=()=>{
    const [period,kind,squad,order]=controls.map(control=>control.value);
    const horizon=period==='upcoming'?90:Number(period);
    const matches=rows.filter(row=>{
      if(kind!=='all'&&row.loan!==(kind==='loan'))return false;
      const squadStatus=byContract.get(row.uid+':'+row.index)?.squadStatus;
      if(squad!=='all'&&(squadStatus===undefined||Math.max(squadStatus,0)!==Number(squad)))return false;
      if(period==='all')return true;
      if(period==='monthly')return row.days===null;
      if(row.days===null)return false;
      return period==='expired'?row.days<0:row.days>=0&&row.days<=horizon;
    });
    if(order==='wage')matches.sort((a,b)=>wage(b)-wage(a));
    $('club-contract-results').innerHTML=renderClubExpiryRows(matches,byContract);
  };
  section.ontoggle=remember;
  $('club-contract-filters').onchange=()=>{remember();render();};
  render();
}

let batchPreviewSequence=0;
interface BatchTarget {attribute?:string;uid:number;name:string;before:number;after:number;limited:boolean;unavailable?:boolean}
interface PresetAdjustment {attribute:string;kind:string;value:number}
interface BatchPreset {name:string;adjustments:PresetAdjustment[]}
const personalityFields=['adaptability','ambition','loyalty','pressure','professionalism','sportsmanship','temperament','controversy'].map(k=>'personality.'+k)
const batchPresetKey='cm0304.batch-presets.v2';
const initialBatchPresets:BatchPreset[]=[
  {name:'Meio-campo · Passe mínimo',adjustments:[{attribute:'passing',kind:'floor',value:12}]},
  {name:'Goleiros · Reflexos mínimos',adjustments:[{attribute:'reflexes',kind:'floor',value:12}]},
  {name:'Elenco · Profissionalismo mínimo',adjustments:[{attribute:'personality.professionalism',kind:'floor',value:12}]},
  {name:'Elenco · Recuperação',adjustments:[
    {attribute:'state.condition',kind:'set',value:100},
    {attribute:'state.fitness',kind:'set',value:100},
    {attribute:'state.jadedness',kind:'ceiling',value:0},
    {attribute:'state.morale',kind:'floor',value:18},
  ]},
  {name:'Moral',adjustments:[{attribute:'state.morale',kind:'floor',value:18}]},
  {name:'Consistência',adjustments:[{attribute:'consistency',kind:'floor',value:15}]},
  {name:'Força e velocidade',adjustments:[{attribute:'strength',kind:'floor',value:15},{attribute:'pace',kind:'floor',value:15}]},
  {name:'Propensão a lesões',adjustments:[{attribute:'injuryProneness',kind:'ceiling',value:5}]},
  {name:'Dois pés',adjustments:[{attribute:'leftFoot',kind:'set',value:20},{attribute:'rightFoot',kind:'set',value:20}]},
];
type BatchPlayer={uid:number;name:string;injuryCount?:number;suspensionCount?:number};
const playerBatchModes=[['adjust',t("Ajustar atributos")],['recovery',t("Recuperação e ocorrências")],['copy',t("Copiar atributos")]] as const;
function renderClubBatch(players:BatchPlayer[],filtered=false):string {
  if(!players.length)return '';
  const prefix=filtered?'dialog-batch':'batch';
  const keys=[...new Set([...groups.flatMap(([,fields])=>fields),...keeper,'leftFoot','rightFoot','throwIns','dirtiness','consistency','importantMatches','injuryProneness','versatility'])];
  const stateFields=['state.condition','state.fitness','state.jadedness','state.morale'];
  const sourceOptions=players.map(player=>'<option value="'+player.uid+'">'+escape(player.name)+' · '+player.uid+'</option>').join('');
  const grouped=[...groups,[t("Goleiro"),keeper] as [string,string[]]];
  const groupedKeys=new Set(grouped.flatMap(([,fields])=>fields));
  grouped.push([t("Complementares"),keys.filter(key=>!groupedKeys.has(key))]);
  const copyGroupButtons=grouped.map(([label,fields])=>'<button type="button" data-copy-group="'+fields.join(',')+'">'+escape(label)+'</button>').join(' ');
  const copyFields=keys.map(key=>'<label class="batch-player"><input type="checkbox" name="batch-copy-field" value="'+key+'"> '+escape(fieldLabel(key))+'</label>').join('');
  const attributes=[...keys,...personalityFields,...stateFields].map(key=>'<option value="'+key+'">'+escape(fieldLabel(key))+'</option>').join('');
  return (filtered?'':`<details class="club-player-batch"><summary>${t("Editar jogadores em lote")}</summary>`)
    +'<form id="club-batch-form" novalidate><div class="batch-body">'
    +'<div class="batch-selection-bar"><strong id="batch-selection" role="status" tabindex="-1"></strong>'
    +`<details id="batch-selection-details"><summary>${t("Alterar seleção")}</summary>`
    +'<label '+(players.length===1?'hidden':'')+`><input type="checkbox" id="batch-all"> ${t("Selecionar")} `+(filtered?t("todos os resultados"):t("todo o elenco"))+' ('+players.length+')</label>'
    +'<div class="batch-players" id="batch-player-list"></div><p '+(players.length<=50?'hidden':'')+`><button type="button" id="batch-prev-players">${t("Anterior")}</button> <span id="batch-player-page"></span> <button type="button" id="batch-next-players">${t("Próxima")}</button></p>`
    +`<details><summary>${t("Grupos salvos")}</summary><div class="batch-controls"><label>${t("Grupo")} <select id="batch-saved-group"></select></label> `
    +`<button type="button" id="batch-load-group">${t("Selecionar grupo")}</button> <button type="button" id="batch-delete-group">${t("Excluir grupo")}</button></div>`
    +`<div class="batch-controls"><label>${t("Nome do grupo")} <input id="batch-group-name" maxlength="60"></label> <button type="button" id="batch-save-group">${t("Salvar seleção")}</button></div></details></details></div>`
    +`<div class="profile-tabs batch-tabs" role="tablist" aria-label="${t("Operações do lote")}">`+playerBatchModes.map(([id,label],index)=>
      '<button type="button" role="tab" id="'+prefix+'-tab-'+id+'" aria-controls="'+prefix+'-panel-'+id+'" aria-selected="'+(index===0)+'" tabindex="'+(index===0?0:-1)+'">'+label+'</button>').join('')+'</div>'
    +'<section id="'+prefix+'-panel-adjust" role="tabpanel" aria-labelledby="'+prefix+'-tab-adjust">'
    +`<div class="batch-adjust-fields"><label>${t("Campo")} <select id="batch-attribute">`+attributes+'</select></label> '
    +`<label>${t("Operação")} <select id="batch-operation"><option value="set">${t("Definir")}</option><option value="add">${t("Somar pontos")}</option><option value="percent">${t("Ajustar em %")}</option><option value="floor">${t("Definir piso")}</option><option value="ceiling">${t("Definir teto")}</option></select></label> `
    +`<label>${t("Valor")} <input id="batch-value" type="number" min="-2147483648" max="2147483647" required step="1"></label></div>`
    +'<p id="'+prefix+`-percent-help" class="profile-meta" hidden>${t("Ajustes percentuais partem do valor atual e arredondam para a unidade mais próxima. Em caso de empate, o resultado se afasta de zero.")}</p>`
    +'<p id="'+prefix+`-scale-help" class="profile-meta" hidden>${t("Condição e ritmo usam passos de 0,01 ponto percentual.")}</p>`
    +`<div class="batch-controls"><button type="button" id="batch-queue-adjustment">${t("Adicionar ajuste à lista")}</button></div><ul id="batch-adjustment-list" hidden></ul>`
    +`<div class="batch-controls"><label>${t("Usar preset")} <select id="batch-preset"></select></label> `
    +`<button type="button" id="batch-preset-load">${t("Carregar")}</button></div>`
    +`<details id="batch-presets"><summary>${t("Gerenciar presets")}</summary>`
    +`<div class="batch-controls"><label>${t("Nome do preset")} <input id="batch-preset-name" maxlength="60"></label> <button type="button" id="batch-preset-save">${t("Salvar preset")}</button></div>`
    +`<div class="batch-controls"><button type="button" id="batch-preset-rename">${t("Renomear selecionado")}</button> <button type="button" id="batch-preset-duplicate">${t("Duplicar selecionado")}</button> `
    +`<button type="button" id="batch-preset-delete">${t("Excluir selecionado")}</button></div>`
    +`<div class="batch-controls"><button type="button" id="batch-preset-export">${t("Exportar presets")}</button> `
    +`<button type="button" id="batch-open-preset-file">${t("Importar JSON")}</button><input type="file" id="batch-preset-import" accept=".json,application/json" hidden></div></details></section>`
    +'<section id="'+prefix+'-panel-recovery" role="tabpanel" aria-labelledby="'+prefix+'-tab-recovery" hidden>'
    +`<p class="profile-meta">${t("A recuperação ajusta condição e ritmo a 100%, cansaço a no máximo 0 e moral a pelo menos 18. As remoções marcadas entram na mesma prévia.")}</p>`
    +`<fieldset class="batch-occurrences"><legend>${t("Lesões e suspensões")}</legend><div class="batch-controls">`
    +`<label><input type="checkbox" id="batch-heal-injuries"> ${t("Remover lesões")}</label> `
    +`<label><input type="checkbox" id="batch-clear-suspensions"> ${t("Remover suspensões")}</label> `
    +`<button type="button" id="batch-select-health">${t("Selecionar jogadores com ocorrências")}</button></div></fieldset></section>`
    +'<section id="'+prefix+'-panel-copy" role="tabpanel" aria-labelledby="'+prefix+`-tab-copy" hidden><div class="batch-controls"><label>${t("Jogador de origem")} <select id="batch-copy-source"><option value="">${t("Escolha o jogador")}</option>`+sourceOptions+'</select></label></div>'
    +'<div class="batch-controls">'+copyGroupButtons+` <button type="button" data-copy-group="">${t("Limpar atributos")}</button></div>`
    +'<div class="batch-players">'+copyFields+'</div></section>'
    +'<div id="batch-preview" aria-live="polite"></div></div>'
    +'<div class="batch-footer"><span id="batch-preview-summary" role="status"></span><div class="batch-footer-actions">'
    +`<button type="button" id="batch-preset-preview" hidden>${t("Prévia do preset")}</button>`
    +`<button type="submit" id="preview-player-adjustment" class="primary">${t("Gerar prévia")}</button>`
    +`<button type="submit" id="batch-adjustment-preview" hidden>${t("Prévia dos ajustes da lista")}</button>`
    +`<button type="submit" id="batch-recover" class="primary" hidden>${t("Prévia de recuperação")}</button>`
    +`<button type="submit" id="batch-health" hidden>${t("Prévia das remoções")}</button>`
    +`<button type="submit" id="batch-copy" class="primary" hidden>${t("Gerar prévia da cópia")}</button>`
    +`<button id="batch-stage" type="button" class="primary" hidden>${t("Adicionar lote à revisão")}</button></div></div></form>`+(filtered?'':'</details>');
}
function bindClubBatch(root:HTMLElement,players:BatchPlayer[],initialUid?:number,onStage?:()=>Promise<void>){
  const $=<T extends HTMLElement=HTMLElement>(id:string)=>root.querySelector<T>('#'+id)!;
  const form=$<HTMLFormElement>('club-batch-form');
  if(!form)return;
  const inDialog=root.id==='filtered-batch-content';
  const prefix=inDialog?'dialog-batch':'batch';
  const selected=new Set<number>(initialUid===undefined?[]:[initialUid]);
  const occurrences=(player:BatchPlayer)=>({
    injuries:plan.healInjuries.includes(player.uid)?0:player.injuryCount??0,
    suspensions:plan.clearSuspensions.includes(player.uid)?0:player.suspensionCount??0,
  });
  const pageSize=50;
  let playerPage=0;
  const renderPlayers=()=>{
    const rows=players.slice(playerPage*pageSize,(playerPage+1)*pageSize);
    $('batch-player-list').innerHTML=rows.map(player=>{
      const {injuries,suspensions}=occurrences(player);
      return '<label class="batch-player"><input type="checkbox" name="batch-player" value="'+player.uid+'" '+(selected.has(player.uid)?'checked':'')+'> '+escape(player.name)
        +(injuries?t(" · Lesões: ")+injuries:'')+(suspensions?t(" · Suspensões: ")+suspensions:'')+'</label>';
    }).join('');
    $('batch-player-page').textContent=(playerPage+1)+' / '+Math.ceil(players.length/pageSize);
    $('batch-prev-players').hidden=playerPage===0;
    $('batch-next-players').hidden=(playerPage+1)*pageSize>=players.length;
  };
  $('batch-prev-players').onclick=()=>{if(playerPage>0){playerPage--;renderPlayers();}};
  $('batch-next-players').onclick=()=>{if((playerPage+1)*pageSize<players.length){playerPage++;renderPlayers();}};
  renderPlayers();
  const all=$<HTMLInputElement>('batch-all');
  const stage=$<HTMLButtonElement>('batch-stage');
  let revision=0;
  let activePreviewId=0;
  const invalidate=()=>{
    revision++;
    activePreviewId=0;
    stage.hidden=true;
    $('batch-preview-summary').textContent='';
    $('batch-preview').textContent='';
    if(inDialog)clearToast();
    const count=selected.size;
    all.checked=count===players.length;
    all.indeterminate=count>0&&count<players.length;
    const player=count===1?players.find(player=>selected.has(player.uid)):undefined;
    const label=player?player.name+t(" · 1 jogador selecionado"):count.toLocaleString(locale)+t(" jogadores selecionados");
    if($('batch-selection').textContent!==label)$('batch-selection').textContent=label;
    syncFooter();
  };
  const groupKey='cm0304.player-groups.v1';
  const groupSelect=$<HTMLSelectElement>('batch-saved-group');
  const groupName=$<HTMLInputElement>('batch-group-name');
  let savedGroups:{name:string;uids:number[]}[]=[];
  try {
    const stored:unknown=JSON.parse(localStorage.getItem(groupKey)??'[]');
    if(Array.isArray(stored))savedGroups=stored.filter(group=>group&&typeof group.name==='string'
      &&Array.isArray(group.uids)&&group.uids.every(Number.isInteger));
  } catch { /* An unreadable preference leaves the library empty. */ }
  const renderGroups=(name='')=>{
    groupSelect.replaceChildren(new Option(t("Selecione um grupo"),''),
      ...savedGroups.map(group=>new Option(group.name,group.name)));
    groupSelect.value=name;
  };
  const storeGroups=(groups:typeof savedGroups,name='')=>{
    try {localStorage.setItem(groupKey,JSON.stringify(groups));}
    catch {notify(t("Não foi possível gravar os grupos locais."),'error');return;}
    savedGroups=groups;
    renderGroups(name);
  };
  renderGroups();
  $('batch-save-group').onclick=async()=>{
    const name=groupName.value.trim();
    if(!name||!selected.size){notify(t("Informe um nome e selecione pelo menos um jogador."),'error');return;}
    if(savedGroups.some(group=>group.name===name)&&!await confirmAction(t("Substituir grupo?"),
      t("A seleção atual substituirá os jogadores do grupo “")+name+'”.',t("Substituir")))return;
    storeGroups([...savedGroups.filter(group=>group.name!==name),{name,uids:Array.from(selected)}],name);
  };
  $('batch-delete-group').onclick=async()=>{
    const name=groupSelect.value;
    if(name&&await confirmAction(t("Excluir grupo?"),t("O grupo “")+name+t("” será removido deste editor."),t("Excluir"))){
      storeGroups(savedGroups.filter(group=>group.name!==name));
    }
  };
  $('batch-load-group').onclick=()=>{
    const group=savedGroups.find(group=>group.name===groupSelect.value);
    if(!group)return;
    const available=new Set(players.map(player=>player.uid));
    const matches=group.uids.filter(uid=>available.has(uid));
    selected.clear();
    for(const uid of matches)selected.add(uid);
    playerPage=0;
    renderPlayers();
    invalidate();
    groupName.value=group.name;
    notify(group.name+' · '+selected.size+t(" de ")+group.uids.length+t(" jogadores disponíveis nesta lista."),'success');
  };
  for(const button of form.querySelectorAll<HTMLButtonElement>('[data-copy-group]')){
    button.onclick=()=>{
      const keys=button.dataset.copyGroup?.split(',').filter(Boolean)??[];
      const fields=Array.from(form.querySelectorAll<HTMLInputElement>('[name="batch-copy-field"]'));
      const group=fields.filter(field=>keys.includes(field.value));
      const select=group.some(field=>!field.checked);
      if(group.length)for(const field of group)field.checked=select;
      else for(const field of fields)field.checked=false;
      invalidate();
    };
  }
  const presetSelect=$<HTMLSelectElement>('batch-preset');
  const presetName=$<HTMLInputElement>('batch-preset-name');
  const attributeField=$<HTMLSelectElement>('batch-attribute');
  const operationField=$<HTMLSelectElement>('batch-operation');
  const valueField=$<HTMLInputElement>('batch-value');
  const queued=new Map<string,{kind:string;value:number;input:number}>();
  const queuedList=$('batch-adjustment-list');
  let mode:typeof playerBatchModes[number][0]='adjust';
  function syncFooter(){
    const choices=[
      ['preview-player-adjustment',mode==='adjust',!queued.size],
      ['batch-adjustment-preview',mode==='adjust'&&queued.size>0,true],
      ['batch-recover',mode==='recovery',true],
      ['batch-health',mode==='recovery',false],
      ['batch-copy',mode==='copy',true],
    ] as const;
    for(const [id,visible,primary] of choices){
      const button=$(id);
      button.hidden=!visible;
      button.classList.toggle('primary',primary&&!!stage.hidden);
    }
    $('batch-preset-preview').hidden=mode!=='adjust'||!presetSelect.value||!stage.hidden;
    $('preview-player-adjustment').textContent=queued.size?t("Prévia deste ajuste"):t("Gerar prévia");
  }
  const selectMode=(index:number,focus=false)=>{
    const next=playerBatchModes[index][0];
    if(mode!==next){
      mode=next;
      for(const [id] of playerBatchModes){
        const active=id===mode;
        const tab=$(prefix+'-tab-'+id);
        tab.setAttribute('aria-selected',String(active));
        tab.tabIndex=active?0:-1;
        $(prefix+'-panel-'+id).hidden=!active;
      }
      invalidate();
    }
    if(focus)$(prefix+'-tab-'+mode).focus();
  };
  for(const [index,[id]] of playerBatchModes.entries()){
    const tab=$(prefix+'-tab-'+id);
    tab.onclick=()=>selectMode(index);
    tab.onkeydown=event=>{
      const last=playerBatchModes.length-1;
      const next=event.key==='Home'?0:event.key==='End'?last:
        event.key==='ArrowRight'?(index+1)%(last+1):event.key==='ArrowLeft'?(index+last)%(last+1):-1;
      if(next<0)return;
      event.preventDefault();
      selectMode(next,true);
    };
  }
  const percentField=(field:string)=>field==='state.condition'||field==='state.fitness';
  const adjustmentScale=(field:string,kind:string)=>percentField(field)&&kind!=='percent'?100:1;
  const scaledPresetValue=({attribute,kind,value}:PresetAdjustment)=>{
    const scale=adjustmentScale(attribute,kind);
    const scaled=Math.round(value*scale);
    return scaled/scale===value?scaled:NaN;
  };
  const updateValueStep=()=>{
    const attribute=attributeField.value;
    const kind=operationField.value;
    const scale=adjustmentScale(attribute,kind);
    valueField.step=String(1/scale);
    valueField.dataset.scale=String(scale);
    const targetLimits: Record<string,[number,number]> = {
      'state.condition':[0,100], 'state.fitness':[0,100],
      'state.jadedness':[-1500,1000], 'state.morale':[0,20],
    };
    const [min,max] = kind==='set' ? targetLimits[attribute]??[1,20]
      : [-2147483648/scale,2147483647/scale];
    valueField.min=String(min);
    valueField.max=String(max);
    $(prefix+'-percent-help').hidden=kind!=='percent';
    $(prefix+'-scale-help').hidden=scale!==100;
    const help=kind==='percent'?'-percent-help':scale===100?'-scale-help':null;
    if(help)valueField.setAttribute('aria-describedby',prefix+help);
    else valueField.removeAttribute('aria-describedby');
    syncAuxiliaryValidation(form);
  };
  updateValueStep();
  const readOperation=()=>{
    const kind=operationField.value;
    const input=valueField.valueAsNumber;
    const value=scaledPresetValue({attribute:attributeField.value,kind,value:input});
    return {kind,value,input};
  };
  const validValue=(value:number)=>Number.isInteger(value)&&value>=-2147483648&&value<=2147483647;
  const renderQueue=()=>{
    queuedList.hidden=queued.size===0;
    syncFooter();
    queuedList.replaceChildren(...Array.from(queued,([attribute,adjustment])=>{
      const item=document.createElement('li');
      const operation=Array.from(operationField.options).find(option=>option.value===adjustment.kind)?.textContent;
      item.textContent=fieldLabel(attribute)+' · '+operation+' '+adjustment.input+' ';
      const remove=document.createElement('button');
      remove.type='button';
      remove.textContent=t("Remover");
      remove.setAttribute('aria-label',t("Remover ajuste de ")+fieldLabel(attribute));
      remove.onclick=()=>{queued.delete(attribute);renderQueue();invalidate();};
      item.append(remove);
      return item;
    }));
  };
  $('batch-queue-adjustment').onclick=()=>{
    const adjustment=readOperation();
    if(!valueField.validity.valid||!validValue(adjustment.value)){notify(t("Verifique o intervalo e a precisão do ajuste."),'error');return;}
    queued.set(attributeField.value,adjustment);
    renderQueue();
    invalidate();
  };
  const parsePreset=(raw:unknown):BatchPreset|null=>{
    if(!raw||typeof raw!=='object')return null;
    const input=raw as Partial<BatchPreset & PresetAdjustment>;
    const name=typeof input.name==='string'?input.name.trim():'';
    const adjustments=Array.isArray(input.adjustments)?input.adjustments:[input];
    if(!name||name.length>60||!adjustments.length)return null;
    const fields=new Set<string>();
    const parsed:PresetAdjustment[]=[];
    for(const adjustment of adjustments){
      if(!adjustment||typeof adjustment.attribute!=='string'||typeof adjustment.kind!=='string'
        ||typeof adjustment.value!=='number'
        ||!Array.from(attributeField.options).some(option=>option.value===adjustment.attribute)
        ||!Array.from(operationField.options).some(option=>option.value===adjustment.kind)
        ||fields.has(adjustment.attribute))return null;
      const {attribute,kind,value}=adjustment;
      if(!validValue(scaledPresetValue({attribute,kind,value})))return null;
      fields.add(attribute);
      parsed.push({attribute,kind,value});
    }
    return {name,adjustments:parsed};
  };
  let presets:BatchPreset[]=initialBatchPresets;
  try{
    const raw=localStorage.getItem(batchPresetKey);
    const stored:unknown=JSON.parse(raw??localStorage.getItem('cm0304.batch-presets.v1')??'null');
    if(Array.isArray(stored)){
      presets=stored.map(parsePreset).filter(preset=>preset!==null);
      if(raw===null)presets.push(...initialBatchPresets.slice(4).filter(preset=>!presets.some(item=>item.name===preset.name)));
    }
  }catch{ /* Keep starter presets when browser storage is unavailable. */ }
  const renderPresets=(selected='')=>{
    presetSelect.replaceChildren(new Option(t("Selecione um preset"),''),
      ...presets.map(preset=>new Option(initialBatchPresets.some(builtin=>builtin.name===preset.name)?t(preset.name):preset.name,preset.name)));
    presetSelect.value=selected;
    syncFooter();
  };
  const storePresets=(next:BatchPreset[],selected='')=>{
    try{localStorage.setItem(batchPresetKey,JSON.stringify(next));}
    catch{notify(t("Não foi possível gravar os presets locais."),'error');return;}
    presets=next;
    renderPresets(selected);
  };
  renderPresets();
  presetSelect.onchange=syncFooter;
  $('batch-preset-load').onclick=()=>{
    const preset=presets.find(item=>item.name===presetSelect.value);
    if(!preset)return;
    queued.clear();
    const [first]=preset.adjustments;
    attributeField.value=first.attribute;
    operationField.value=first.kind;
    valueField.value=String(first.value);
    updateValueStep();
    if(preset.adjustments.length>1){
      for(const adjustment of preset.adjustments){
        queued.set(adjustment.attribute,{kind:adjustment.kind,input:adjustment.value,value:scaledPresetValue(adjustment)});
      }
    }
    renderQueue();
    presetName.value=preset.name;
    invalidate();
    notify(t("Preset carregado. Gere uma prévia para a seleção atual."),'success');
  };
  $('batch-preset-preview').onclick=()=>{
    const preset=presets.find(item=>item.name===presetSelect.value);
    if(!preset){notify(t("Selecione um preset."),'error');return;}
    $('batch-preset-load').click();
    form.requestSubmit($<HTMLButtonElement>(preset.adjustments.length>1?'batch-adjustment-preview':'preview-player-adjustment'));
  };
  $('batch-preset-save').onclick=async()=>{
    const name=presetName.value.trim();
    const adjustments=queued.size
      ?Array.from(queued,([attribute,{kind,input}])=>({attribute,kind,value:input}))
      :[{attribute:attributeField.value,kind:operationField.value,value:valueField.valueAsNumber}];
    const preset=parsePreset({name,adjustments});
    if(!preset){
      notify(t("Informe um nome e um valor válido para o preset."),'error');return;
    }
    if(presets.some(item=>item.name===name)&&!await confirmAction(t("Substituir preset?"),
      t("Os ajustes atuais substituirão o preset “")+name+'”.',t("Substituir")))return;
    storePresets([...presets.filter(item=>item.name!==name),preset],name);
  };
  const copyPreset=(rename:boolean)=>{
    const selected=presets.find(preset=>preset.name===presetSelect.value);
    const name=presetName.value.trim();
    if(!selected||!name||name.length>60){
      notify(t("Selecione um preset e informe um novo nome de até 60 caracteres."),'error');return;
    }
    if(presets.some(preset=>preset.name===name)){
      notify(t("Já existe um preset com esse nome."),'error');return;
    }
    const copy={...selected,name};
    const next=rename ? presets.map(preset=>preset===selected?copy:preset) : [...presets,copy];
    storePresets(next,name);
  };
  $('batch-preset-rename').onclick=()=>copyPreset(true);
  $('batch-preset-duplicate').onclick=()=>copyPreset(false);
  $('batch-preset-delete').onclick=async()=>{
    const name=presetSelect.value;
    if(name&&await confirmAction(t("Excluir preset?"),t("O preset “")+name+t("” será removido deste editor."),t("Excluir"))){
      storePresets(presets.filter(item=>item.name!==name));
    }
  };
  $('batch-preset-export').onclick=()=>void task(async()=>{
    if(!presets.length){notify(t("Salve um preset antes de exportar."),'error');return;}
    const path=await invoke<string|null>('export_batch_presets',{json:JSON.stringify({version:2,presets},null,2)});
    if(path)notify(t("Presets exportados: ")+path,'success');
  });
  const importField=$<HTMLInputElement>('batch-preset-import');
  $('batch-open-preset-file').onclick=()=>importField.click();
  importField.onchange=()=>{
    const file=importField.files?.[0];
    importField.value='';
    if(!file)return;
    void task(async()=>{
      if(file.size>256*1024)throw new Error(t("O arquivo de presets excede 256 KB."));
      const data=JSON.parse(await file.text());
      if(![1,2].includes(data?.version)||!Array.isArray(data.presets))throw new Error(t("Formato de presets incompatível."));
      const imported=new Map<string,BatchPreset>();
      for(const raw of data.presets){
        const preset=parsePreset(raw);
        if(!preset||imported.has(preset.name)){
          throw new Error(t("O arquivo contém um preset inválido ou repetido."));
        }
        imported.set(preset.name,preset);
      }
      if(presets.some(preset=>imported.has(preset.name))&&!await confirmAction(t("Substituir presets?"),
        t("Os presets importados substituirão os presets locais com o mesmo nome."),t("Substituir")))return;
      storePresets([...presets.filter(preset=>!imported.has(preset.name)),...imported.values()]);
    });
  };
  all.onchange=()=>{
    selected.clear();
    if(all.checked)for(const player of players)selected.add(player.uid);
    renderPlayers();
    invalidate();
  };
  $('batch-select-health').onclick=()=>{
    const heal=$<HTMLInputElement>('batch-heal-injuries').checked;
    const clear=$<HTMLInputElement>('batch-clear-suspensions').checked;
    if(!heal&&!clear){notify(t("Selecione pelo menos uma remoção."),'error');return;}
    selected.clear();
    for(const player of players){
      const {injuries,suspensions}=occurrences(player);
      if((heal&&injuries)||(clear&&suspensions))selected.add(player.uid);
    }
    renderPlayers();
    invalidate();
  };
  form.oninput=e=>{
    const field=e.target;
    if(!(field instanceof HTMLInputElement||field instanceof HTMLSelectElement))return;
    if(field===all||field===$('batch-only-unavailable'))return;
    if(field===attributeField||field===operationField)updateValueStep();
    if(field.name==='batch-player'){
      if((field as HTMLInputElement).checked)selected.add(Number(field.value));
      else selected.delete(Number(field.value));
    }else if(!['batch-attribute','batch-operation','batch-value','batch-heal-injuries','batch-clear-suspensions','batch-copy-source'].includes(field.id)
      &&field.name!=='batch-copy-field')return;
    invalidate();
  };
  form.onsubmit=e=>{
    e.preventDefault();
    if(busy||!e.submitter||e.submitter.hidden)return;
    const uids=Array.from(selected);
    if(!uids.length){
      $('batch-selection').focus();
      notify(t("Selecione pelo menos um jogador."),'error');
      return;
    }
    invalidate();
    const version=revision;
    const previewId=++batchPreviewSequence;
    const attribute=$<HTMLSelectElement>('batch-attribute').value;
    const recover=e.submitter?.id==='batch-recover';
    const health=e.submitter?.id==='batch-health';
    const healInjuries=$<HTMLInputElement>('batch-heal-injuries').checked;
    const clearSuspensions=$<HTMLInputElement>('batch-clear-suspensions').checked;
    if(health&&!healInjuries&&!clearSuspensions){notify(t("Selecione pelo menos uma remoção."),'error');return;}
    const copy=e.submitter?.id==='batch-copy';
    const multiple=e.submitter?.id==='batch-adjustment-preview';
    const format=new Intl.NumberFormat(locale,{maximumFractionDigits:2});
    const renderValue=(value:number,field:string)=>percentField(field)?format.format(value/100)+'%':format.format(value);
    const {kind,value}=readOperation();
    if(!recover&&!health&&!copy&&!multiple&&(!valueField.validity.valid||!validValue(value))){
      notify(t("Verifique o intervalo e a precisão do ajuste."),'error');
      return;
    }
    const source=$<HTMLSelectElement>('batch-copy-source').value;
    const attributes=Array.from(form.querySelectorAll<HTMLInputElement>('[name="batch-copy-field"]:checked')).map(field=>field.value);
    if(copy&&(!source||!attributes.length)){
      notify(t("Escolha a origem e pelo menos um atributo para copiar."),'error');
      return;
    }
    const operation=multiple?{kind:'adjust',value:Array.from(queued,([attribute,{kind,value}])=>({attribute,operation:{kind,value}}))}
      :recover?{kind:'recover'}:copy?{kind:'copy',value:{sourceUid:Number(source),attributes}}:{kind,value};
    void task(async()=>{
      const targets=health||(recover&&(healInjuries||clearSuspensions))
        ?await invoke<BatchTarget[]>('preview_health_batch',{previewId,uids,healInjuries,clearSuspensions,recover})
        :await invoke<BatchTarget[]>('preview_player_batch',{previewId,request:{uids,attribute,operation}});
      if(version!==revision)return;
      activePreviewId=previewId;
      const changed=new Set(targets.filter(row=>row.before!==row.after).map(row=>row.uid)).size;
      const unavailable=new Set(targets.filter(row=>row.unavailable).map(row=>row.uid));
      const allTargets=targets.filter(row=>row.unavailable||!unavailable.has(row.uid));
      let visibleTargets=allTargets;
      let previewPageCount=Math.ceil(visibleTargets.length/pageSize);
      $('batch-preview').innerHTML=`<h3 id="batch-preview-title" tabindex="-1">${t("Prévia do lote")}</h3>`+(unavailable.size
        ?`<button id="batch-exclude-unavailable" type="button">${t("Excluir jogadores com campos indisponíveis da seleção")}</button>`
          +`<label class="preview-attention"><input id="batch-only-unavailable" type="checkbox"> ${t("Mostrar apenas campos indisponíveis")}</label>`
        :'')
        +'<ul id="batch-preview-rows"></ul>'+(previewPageCount>1
          ?`<p><button id="batch-prev-preview" type="button">${t("Página anterior da prévia")}</button> `
            +'<span id="batch-preview-page" aria-live="polite"></span> '
            +`<button id="batch-next-preview" type="button">${t("Próxima página da prévia")}</button></p>`:'');
      let previewPage=0;
      const renderPreviewPage=()=>{
        const start=previewPage*pageSize;
        $('batch-preview-rows').innerHTML=visibleTargets.slice(start,start+pageSize).map(row=>{
          const field=row.attribute??attribute;
          const remove=' <button type="button" data-batch-exclude="'+row.uid+'" aria-label="'+t("Excluir ")+escape(row.name)+`${t(" da seleção")}">${t("Excluir")}</button>`;
          if(row.unavailable)return '<li>'+escape(row.name)+' · '+escape(fieldLabel(field))+t(": indisponível")+remove+'</li>';
          const label=field==='healInjuries'?t("Lesões"):field==='clearSuspensions'?t("Suspensões"):fieldLabel(field);
          return '<li>'+escape(row.name)+(recover||health||copy||multiple?' · '+escape(label):'')+': '+renderValue(row.before,field)+' → '+renderValue(row.after,field)+(row.limited?t(" (limitado)"):'')+remove+'</li>';
        }).join('');
        if($('batch-preview-page')){
          $('batch-preview-page').textContent=(previewPage+1)+' / '+previewPageCount;
          $('batch-preview-page').hidden=previewPageCount<=1;
          $('batch-prev-preview').hidden=previewPage===0;
          $('batch-next-preview').hidden=previewPage+1>=previewPageCount;
        }
      };
      renderPreviewPage();
      if($('batch-prev-preview')){
        $('batch-prev-preview').onclick=()=>{previewPage--;renderPreviewPage();};
        $('batch-next-preview').onclick=()=>{previewPage++;renderPreviewPage();};
      }
      const exclude=(uids:Iterable<number>)=>{
        for(const uid of uids)selected.delete(uid);
        renderPlayers();
        invalidate();
        notify(t("Seleção atualizada. Gere uma nova prévia para continuar."));
        (e.submitter as HTMLElement|null)?.focus();
      };
      $('batch-preview-rows').onclick=event=>{
        const control=(event.target as HTMLElement).closest<HTMLButtonElement>('[data-batch-exclude]');
        if(control)exclude([Number(control.dataset.batchExclude)]);
      };
      if(unavailable.size){
        $('batch-exclude-unavailable').onclick=()=>exclude(unavailable);
        const onlyUnavailable=$<HTMLInputElement>('batch-only-unavailable');
        onlyUnavailable.onchange=()=>{
          visibleTargets=onlyUnavailable.checked?allTargets.filter(row=>row.unavailable):allTargets;
          previewPageCount=Math.ceil(visibleTargets.length/pageSize);
          previewPage=0;
          renderPreviewPage();
        };
      }
      stage.hidden=changed===0||unavailable.size>0;
      const count=new Set(targets.map(row=>row.uid)).size;
      $('batch-preview-summary').textContent=unavailable.size?t("Há campos indisponíveis"):
        changed+t(" de ")+count+(count===1?t(" jogador com alterações"):t(" jogadores com alterações"));
      syncFooter();
      $('batch-preview-title').focus();
      if(unavailable.size)notify(unavailable.size+t(" jogador(es) com campos indisponíveis. Revise a seleção para continuar."),'warning');
    });
  };
  stage.onclick=()=>{
    if(stage.hidden||!activePreviewId)return;
    void task(async()=>{
      const updated=await invoke<Pending>('stage_player_batch',{previewId:activePreviewId});
      plan=updated;
      renderPlayers();
      invalidate();
      if(onStage)await onStage();
      notify(t("Lote adicionado à revisão."),'success');
    });
  };
  invalidate();
}
function renderWageBatch(contracts:ClubWageContract[]):string {
  if(!contracts.length)return '';
  const group=(title:string,loan:boolean)=>'<fieldset><legend>'+title+'</legend>'
    +contracts.map((contract,index)=>contract.loan===loan
      ?'<label class="batch-player"><input type="checkbox" name="contract" value="'+index+'"> '
        +escape(contract.name)+' · £'+contract.weeklyWage+`${t("/semana")}</label>`:'').join('')+'</fieldset>';
  return `<details><summary>${t("Ajustar contratos em lote")}</summary><form id="wage-batch-form">`
    +'<div class="batch-players">'+group(t("Contratos do clube"),false)+group(t("Empréstimos"),true)+'</div>'
    +`<p><label>${t("Ajuste (%)")} <input name="percent" type="number" required step="1" min="-100" max="2147483647"></label> `
    +`<label>${t("Teto semanal (£)")} <input name="ceiling" type="number" step="1" min="0" max="1000000"></label></p>`
    +`<button type="submit" id="preview-wage-adjustment">${t("Prévia salarial")}</button>`
    +`<p><label>${t("Novo vencimento")} <input name="expiry" type="date" min="1900-01-01" max="2200-12-31"></label> `
    +`<button type="submit" id="preview-expiry-batch" formnovalidate>${t("Prévia de vencimentos")}</button></p>`
    +`<p><label>${t("Nova situação no elenco")} <select name="squadStatus"><option value="">${t("Selecione")}</option>`
    +squadStatusLabels.map((label,index)=>index===0?'':'<option value="'+(index-1)+'">'+escape(label)+'</option>').join('')
    +`</select></label> <button type="submit" id="preview-squad-status-batch" formnovalidate>${t("Prévia da situação no elenco")}</button></p>`
    +'<div id="wage-batch-preview" aria-live="polite"></div>'
    +`<button type="button" id="stage-wage-batch" hidden>${t("Adicionar lote à revisão")}</button></form></details>`;
}

function bindShirtSwap(clubUid:number,contracts:ClubWageContract[]){
  const numbered=contracts.flatMap(contract=>contract.squadNumber!==undefined&&contract.squadNumber>0
    ? [{...contract,squadNumber:contract.squadNumber}]
    : []);
  if(new Set(numbered.map(contract=>contract.uid)).size<2)return;

  const form=document.createElement('form');
  form.id='shirt-swap-form';
  form.className='contract-card';
  const options=`<option value="">${t("Selecione um jogador")}</option>`+numbered.map((contract,index)=>
    '<option value="'+index+'">'+contract.squadNumber+' · '+escape(contract.name)
    +(contract.loan?t(" (empréstimo)"):'')+'</option>').join('');
  form.innerHTML=`<h3>${t("Trocar números de camisa")}</h3>`
    +`<label class="attribute"><span>${t("Primeiro jogador")}</span><select required>`+options+'</select></label>'
    +`<label class="attribute"><span>${t("Segundo jogador")}</span><select required>`+options+'</select></label>'
    +`<button type="submit">${t("Adicionar troca à revisão")}</button>`;
  $('club-tool-shirts').append(form);
  $('club-open-shirts').hidden=false;
  const [first,second]=form.querySelectorAll('select');
  form.onsubmit=async event=>{
    event.preventDefault();
    const a=first.value===''?undefined:numbered[Number(first.value)];
    const b=second.value===''?undefined:numbered[Number(second.value)];
    if(!a||!b||a.uid===b.uid||a.squadNumber===b.squadNumber){
      notify(t("Selecione dois jogadores com números diferentes."),'error');
      return;
    }
    const edits=[
      {uid:a.uid,attribute:'contract.'+a.index+'.squadNumber',value:b.squadNumber},
      {uid:b.uid,attribute:'contract.'+b.index+'.squadNumber',value:a.squadNumber},
    ];
    if(!await canLeave())return;
    void task(async()=>{
      plan=await invoke<Pending>('stage_edits',{edits});
      if(detail){
        detail=await invoke<Detail>('player_detail',{uid:detail.player.uid});
        renderProfile();
      }
      await showClub(clubUid,false);
      notify(t("Troca de camisas adicionada à revisão."),'success');
    });
  };
}

function bindWageBatch(clubUid:number,contracts:ClubWageContract[]){
  const form=document.querySelector<HTMLFormElement>('#wage-batch-form');
  if(!form)return;
  const preview=$('wage-batch-preview');
  const stage=$('stage-wage-batch');
  let revision=0;
  let activePreviewId=0;
  let stagedMessage='';
  function invalidate(){revision++;preview.replaceChildren();stage.hidden=true;}
  preview.onclick=event=>{
    const button=(event.target as HTMLElement).closest<HTMLButtonElement>('[data-exclude-contract]');
    if(!button||busy)return;
    const checkbox=form.querySelector<HTMLInputElement>('[name="contract"][value="'+Number(button.dataset.excludeContract)+'"]');
    if(!checkbox?.checked)return;
    checkbox.checked=false;
    invalidate();
    checkbox.focus();
    notify(t("Vínculo retirado da seleção. Gere uma nova prévia."));
  };
  form.oninput=invalidate;
  form.onsubmit=event=>{
    event.preventDefault();
    invalidate();
    const version=revision;
    const previewId=++batchPreviewSequence;
    const fields=new FormData(form);
    const selectedIndices=fields.getAll('contract').map(Number);
    const selected=selectedIndices.map(index=>contracts[index]);
    if(!selected.length){notify(t("Selecione pelo menos um vínculo."),'error');return;}
    const ceiling=fields.get('ceiling');
    const expiry=event.submitter?.id==='preview-expiry-batch';
    const squadStatus=event.submitter?.id==='preview-squad-status-batch';
    const statusValue=fields.get('squadStatus');
    if(squadStatus&&(!statusValue||typeof statusValue!=='string')){
      notify(t("Selecione a nova situação no elenco."),'error');return;
    }
    const selections=selected.map(({uid,index})=>({uid,index}));
    const exclude=(position:number,contract:ClubWageContract)=>{
      const role=contract.loan?t("empréstimo"):t("contrato do clube");
      return ' <button type="button" data-exclude-contract="'+selectedIndices[position]+'" aria-label="'+t("Retirar ")
        +escape(contract.name+' · '+role)+`${t(" da prévia")}">${t("Retirar")}</button>`;
    };
    const dateField=form.elements.namedItem('expiry') as HTMLInputElement;
    if(expiry&&(!dateField.value||!dateField.validity.valid)){
      dateField.focus();
      notify(t("Informe uma data de vencimento válida."),'error');return;
    }
    void task(async()=>{
      if(squadStatus){
        const targets=await invoke<BatchTarget[]>('preview_squad_status_batch',{previewId,clubUid,request:{contracts:selections,status:Number(statusValue)}});
        if(version!==revision)return;
        activePreviewId=previewId;
        const byContract=new Map(targets.map(target=>[target.uid+':'+target.attribute,target]));
        preview.innerHTML='<ul>'+selected.map((contract,position)=>{
          const target=byContract.get(contract.uid+':contract.'+contract.index+'.squadStatus');
          if(!target)return '';
          const before=rosterSquadStatus({squadStatus:target.before});
          const after=rosterSquadStatus({squadStatus:target.after});
          return '<li>'+escape(target.name)+' · '+(contract.loan?t("Empréstimo"):t("Contrato do clube"))
            +' · '+escape(before??'—')+' → '+escape(after??'—')+exclude(position,contract)+'</li>';
        }).join('')+'</ul>';
        stagedMessage=t("Lote de situação no elenco adicionado à revisão.");
        stage.hidden=!targets.some(target=>target.before!==target.after);
        return;
      }
      if(expiry){
        const year=Number(dateField.value.slice(0,4));
        const day=(dateField.valueAsNumber-Date.UTC(year,0,1))/86400000;
        const targets=await invoke<BatchTarget[]>('preview_expiry_batch',{previewId,clubUid,request:{contracts:selections,year,day}});
        if(version!==revision)return;
        activePreviewId=previewId;
        const byField=new Map(targets.map(target=>[target.uid+':'+target.attribute,target]));
        const dateLabel=(year:number,day:number)=>year===1900&&day===1?t("Mensal"):comparisonDate({year,day});
        preview.innerHTML='<ul>'+selected.map((contract,position)=>{
          const key=contract.uid+':contract.'+contract.index+'.expires.';
          const oldYear=byField.get(key+'year'),oldDay=byField.get(key+'day');
          return oldYear&&oldDay?'<li>'+escape(contract.name)+' · '+(contract.loan?t("Empréstimo"):t("Contrato do clube"))
            +' · '+dateLabel(oldYear.before,oldDay.before)+' → '+dateLabel(year,day)+exclude(position,contract)+'</li>':'';
        }).join('')+'</ul>';
        stagedMessage=t("Lote de vencimentos adicionado à revisão.");
        stage.hidden=!targets.some(target=>target.before!==target.after);
        return;
      }
      const targets=await invoke<BatchTarget[]>('preview_wage_batch',{previewId,clubUid,request:{
        contracts:selections,
        operation:{kind:'percent',value:Number(fields.get('percent'))},
        ceiling:ceiling===''?null:Number(ceiling),
      }});
      if(version!==revision)return;
      activePreviewId=previewId;
      stagedMessage=t("Lote salarial adicionado à revisão.");
      const key=(uid:number,index:number)=>uid+':contract.'+index+'.weeklyWage';
      const byContract=new Map(targets.map(target=>[target.uid+':'+target.attribute,target]));
      const format=new Intl.NumberFormat(locale);
      const groupTotal=(loan:boolean)=>{
        const group=contracts.filter(contract=>contract.loan===loan);
        const before=group.reduce((sum,contract)=>sum+contract.weeklyWage,0);
        const after=group.reduce((sum,contract)=>sum+(byContract.get(key(contract.uid,contract.index))?.after??contract.weeklyWage),0);
        return '<li>'+(loan?t("Empréstimos"):t("Contratos do clube"))+' · £'+format.format(before)
          +' → £'+format.format(after)+t("/semana (diferença £")+format.format(after-before)+')</li>';
      };
      preview.innerHTML=notice('info',t("Salários registrados por vínculo"))+'<ul>'+groupTotal(false)+groupTotal(true)+'</ul><ul>'
        +selected.map((contract,position)=>{
          const target=byContract.get(key(contract.uid,contract.index));
          return target?'<li>'+escape(target.name)+' · '+(contract.loan?t("Empréstimo"):t("Contrato do clube"))
            +' · £'+format.format(target.before)+' → £'+format.format(target.after)+(target.limited?t(" (limite aplicado)"):'')
            +exclude(position,contract)+'</li>':'';
        }).join('')+'</ul>';
      stage.hidden=!targets.some(target=>target.before!==target.after);
    });
  };
  stage.onclick=()=>void task(async()=>{
    try{
      plan=await invoke<Pending>('stage_wage_batch',{previewId:activePreviewId});
      await showClub(clubUid,false);
      notify(stagedMessage,'success');
    }finally{invalidate();}
  });
}

function renderClubStaffComparison(staff:ClubStaff[]):string {
  if(!staff.some(person=>Object.keys(person.attributes??{}).length))return '';
  return `<details id="club-staff-comparison"><summary>${t("Comparar comissão técnica")}</summary>`
    +`<p class="notice" data-notice="info">${t("Atributos profissionais do save aberto.")}</p>`
    +`<label>${t("Função")} <select id="staff-comparison-role"></select></label> `
    +`<label>${t("Atributo")} <select id="staff-comparison-attribute"></select></label>`
    +'<div id="staff-comparison-results" aria-live="polite"></div>'
    +`<details><summary>${t("Editar comissão em lote")}</summary><form id="staff-batch-form">`
    +`<fieldset><legend>${t("Membros da função selecionada")}</legend><div id="staff-batch-members"></div></fieldset>`
    +`<fieldset><legend>${t("Valores a aplicar")}</legend><div id="staff-batch-fields"></div></fieldset>`
    +`<fieldset><legend>${t("Presets da comissão")}</legend><label>Preset <select id="staff-batch-preset"></select></label> `
    +`<button type="button" id="staff-batch-load-preset">${t("Carregar preset")}</button> `
    +`<label>${t("Nome")} <input id="staff-batch-preset-name" maxlength="80"></label> `
    +`<button type="button" id="staff-batch-save-preset">${t("Salvar valores como preset")}</button></fieldset>`
    +`<button type="submit">${t("Prévia da comissão")}</button><div id="staff-batch-preview" aria-live="polite"></div>`
    +`<button type="button" id="staff-batch-stage" hidden>${t("Adicionar lote da comissão à revisão")}</button></form></details></details>`;
}

function bindClubStaffComparison(staff:ClubStaff[],clubUid:number){
  const role=document.querySelector<HTMLSelectElement>('#staff-comparison-role');
  if(!role)return;
  const attribute=$<HTMLSelectElement>('staff-comparison-attribute');
  const people=staff.filter(person=>Object.keys(person.attributes??{}).length>0);
  const roles=[...new Set(people.map(person=>person.role))].sort((a,b)=>t(a).localeCompare(t(b),locale));
  role.replaceChildren(new Option(t("Todas"),''),...roles.map(name=>new Option(t(name),name)));
  const fields=[...new Set(people.flatMap(person=>Object.keys(person.attributes??{})))];
  attribute.replaceChildren(...fields.sort((a,b)=>staffFieldLabel(a).localeCompare(staffFieldLabel(b),locale))
    .map(key=>new Option(staffFieldLabel(key),key)));
  const render=()=>{
    const rows=people.flatMap(person=>{
      const value=person.attributes?.[attribute.value];
      return value==null||(role.value&&person.role!==role.value)?[]:[{person,value}];
    }).sort((a,b)=>b.value-a.value||a.person.name.localeCompare(b.person.name,locale)||a.person.uid-b.person.uid);
    $('staff-comparison-results').innerHTML=rows.map(({person,value})=>
      '<button type="button" class="player-row" data-club-ranked-staff="'+person.uid+'"><span><strong>'
      +escape(person.name)+'</strong><small>'+escape(t(person.role))+'</small></span><span class="abilities">'
      +value+'</span></button>').join('')||`<p class="notice" data-notice="info">${t("Nenhum membro com esse atributo na função selecionada.")}</p>`;
  };
  const form=$<HTMLFormElement>('staff-batch-form');
  const stage=$('staff-batch-stage');
  const preview=$('staff-batch-preview');
  let revision=0;
  let activePreviewId=0;
  const invalidate=()=>{revision++;preview.replaceChildren();stage.hidden=true;};
  const renderMembers=()=>{
    $('staff-batch-members').innerHTML=people.filter(person=>!role.value||person.role===role.value)
      .map(person=>'<label class="batch-player"><input name="staffUid" type="checkbox" value="'+person.uid+'"> '
        +escape(person.name)+' · '+escape(t(person.role))+(person.isPlayer?t(" · Jogador/staff"):'')+'</label>').join('');
  };
  $('staff-batch-fields').innerHTML=fields.map(key=>'<label class="batch-player">'+escape(staffFieldLabel(key))
    +' <input name="'+escape(key)+'" type="number" min="1" max="20" step="1" aria-label="'+escape(staffFieldLabel(key))+'"></label>').join('');
  const readValues=()=>Object.fromEntries(fields.flatMap(key=>{
    const input=form.elements.namedItem(key) as HTMLInputElement;
    return input.value===''?[]:[[key,input.valueAsNumber]];
  }));
  bindStaffBatchPresets(form,fields,readValues,invalidate);
  role.onchange=()=>{render();renderMembers();invalidate();};
  attribute.onchange=render;
  form.oninput=invalidate;
  form.onsubmit=event=>{
    event.preventDefault();
    invalidate();
    const version=revision;
    const previewId=++batchPreviewSequence;
    const data=new FormData(form);
    const uids=data.getAll('staffUid').map(Number);
    const values=readValues();
    if(!uids.length||!Object.keys(values).length){notify(t("Selecione membros e informe pelo menos um atributo."),'error');return;}
    void task(async()=>{
      const targets=await invoke<BatchTarget[]>('preview_staff_batch',{previewId,clubUid,uids,values});
      if(version!==revision)return;
      activePreviewId=previewId;
      const blocked=targets.some(target=>target.unavailable);
      if(blocked)notify(t("Há atributos indisponíveis. Ajuste a seleção e gere outra prévia."),'warning');
      preview.innerHTML='<ul>'+targets.map(target=>'<li>'+escape(target.name)+' · '+escape(staffFieldLabel(target.attribute??''))
        +': '+(target.unavailable?t("indisponível"):target.before+' → '+target.after)+'</li>').join('')+'</ul>';
      stage.hidden=blocked||!targets.some(target=>target.before!==target.after);
    });
  };
  stage.onclick=()=>void task(async()=>{
    plan=await invoke<Pending>('stage_staff_batch',{previewId:activePreviewId});
    invalidate();
    notify(t("Lote da comissão adicionado à revisão."),'success');
  });
  renderMembers();
  render();
}

function bindStaffBatchPresets(form:HTMLFormElement,fields:string[],readValues:()=>Record<string,number>,invalidate:()=>void){
  const storageKey='cm0304.staff-presets.v1';
  const select=$<HTMLSelectElement>('staff-batch-preset');
  const nameField=$<HTMLInputElement>('staff-batch-preset-name');
  const validValue=(value:unknown)=>typeof value==='number'&&Number.isInteger(value)&&value>=1&&value<=20;
  const presets=new Map<string,Record<string,number>>();
  try {
    const stored:unknown=JSON.parse(localStorage.getItem(storageKey)??'[]');
    if(Array.isArray(stored))for(const item of stored){
      if(!item||typeof item.name!=='string'||!item.name.trim()||!item.values||typeof item.values!=='object'||Array.isArray(item.values))continue;
      const values=Object.entries(item.values);
      if(values.length&&values.every(([,value])=>validValue(value)))presets.set(item.name,item.values);
    }
  } catch { /* Invalid local data leaves the preset list empty. */ }
  const render=(selected='')=>{
    select.replaceChildren(new Option(t("Selecione um preset"),''),...Array.from(presets.keys(),name=>new Option(name,name)));
    select.value=selected;
  };
  $('staff-batch-save-preset').onclick=async()=>{
    const name=nameField.value.trim();
    const values=readValues();
    if(!name||!Object.keys(values).length||!Object.values(values).every(validValue)){
      notify(t("Informe um nome e pelo menos um atributo válido de 1 a 20."),'error');return;
    }
    if(presets.has(name)&&!await confirmAction(t("Substituir preset da comissão?"),
      t("Os atributos atuais substituirão o preset “")+name+'”.',t("Substituir")))return;
    const updated=new Map(presets).set(name,values);
    try {localStorage.setItem(storageKey,JSON.stringify(Array.from(updated,([name,values])=>({name,values}))));}
    catch {notify(t("Não foi possível salvar o preset da comissão."),'error');return;}
    presets.set(name,values);
    render(name);
    notify(t("Preset da comissão salvo."),'success');
  };
  $('staff-batch-load-preset').onclick=()=>{
    const values=presets.get(select.value);
    if(!values){notify(t("Selecione um preset da comissão."),'error');return;}
    if(Object.keys(values).some(key=>!fields.includes(key))){notify(t("O preset contém um atributo indisponível neste clube."),'error');return;}
    for(const key of fields)(form.elements.namedItem(key) as HTMLInputElement).value=String(values[key]??'');
    nameField.value=select.value;
    invalidate();
    notify(t("Valores do preset carregados. Gere uma prévia."),'success');
  };
  render();
}

function renderSquadSummary(detail:ClubDetail):string {
  if(!detail.players.length)return '';
  const format=new Intl.NumberFormat(locale,{maximumFractionDigits:1,minimumFractionDigits:1});
  const players=(matches:ClubSquadPlayer[])=>matches.map(player=>'<button type="button" class="player-row" data-squad-player="'
    +player.uid+'"><strong>'+escape(player.name)+'</strong></button>').join('');
  const section=(label:string,matches:ClubSquadPlayer[])=>matches.length?
    '<details><summary>'+escape(label)+' · '+matches.length+'</summary>'+players(matches)+'</details>':'';
  const ages=[[t("Até 20"),0,20],[t("21 a 25"),21,25],[t("26 a 30"),26,30],[t("31 ou mais"),31,150]] as const;
  return `<details id="club-squad-summary"><summary>${t("Perfil do elenco")}</summary><p class="notice" data-notice="info">${t("Dados do save aberto. Um jogador pode aparecer em várias posições (familiaridade de 15 ou mais).")}</p>`
    +`<div class="squad-summary-grid"><section><h4>${t("Faixas etárias")}</h4>`+ages.map(([label,min,max])=>
      section(label,detail.players.filter(p=>p.age!=null&&p.age>=min&&p.age<=max))).join('')
    +section(t("Idade indisponível"),detail.players.filter(p=>p.age==null))+'</section>'
    +`<section><h4>${t("Posições")}</h4>`+playerPositionKeys.map(key=>section(fieldLabel(key),detail.players.filter(p=>p.positions?.includes(key)))).join('')
    +section(t("Sem posição com familiaridade 15+"),detail.players.filter(p=>!p.positions?.length))+'</section>'
    +`<section><h4>${t("Médias de atributos")}</h4><p class="notice" data-notice="info">${t("Reflexos dos goleiros e demais atributos dos jogadores de linha.")}</p><dl>`
    +Object.entries(detail.attributeAverages??{}).map(([key,value])=>'<dt>'+escape(fieldLabel(key))+'</dt><dd>'+format.format(value)+'</dd>').join('')
    +'</dl></section></div></details>';
}

interface AnalysisSlot {id:string;label?:string;name:string;position:string;side?:string;column?:number;starter?:number|null}
const analysisSlots={
  GOL:{id:'GOL',name:t("Goleiro"),position:'goalkeeper'},
  LE:{id:'LE',name:t("Lateral esquerdo"),position:'defender',side:'leftSide'},
  LD:{id:'LD',name:t("Lateral direito"),position:'defender',side:'rightSide'},
  ZE:{id:'ZE',name:t("Zagueiro pela esquerda"),position:'defender',side:'central'},
  ZC:{id:'ZC',name:t("Zagueiro central"),position:'defender',side:'central'},
  ZD:{id:'ZD',name:t("Zagueiro pela direita"),position:'defender',side:'central'},
  AE:{id:'AE',name:t("Ala esquerdo"),position:'wingBack',side:'leftSide'},
  AD:{id:'AD',name:t("Ala direito"),position:'wingBack',side:'rightSide'},
  VE:{id:'VE',name:t("Volante pela esquerda"),position:'defensiveMidfielder',side:'central'},
  VOL:{id:'VOL',name:t("Volante central"),position:'defensiveMidfielder',side:'central'},
  VD:{id:'VD',name:t("Volante pela direita"),position:'defensiveMidfielder',side:'central'},
  ME:{id:'ME',name:t("Meio-campista esquerdo"),position:'midfielder',side:'leftSide'},
  MCE:{id:'MCE',name:t("Meio-campista central pela esquerda"),position:'midfielder',side:'central'},
  MCD:{id:'MCD',name:t("Meio-campista central pela direita"),position:'midfielder',side:'central'},
  MD:{id:'MD',name:t("Meio-campista direito"),position:'midfielder',side:'rightSide'},
  MOE:{id:'MOE',name:t("Meia ofensivo esquerdo"),position:'attackingMidfielder',side:'leftSide'},
  MOC:{id:'MOC',name:t("Meia ofensivo central"),position:'attackingMidfielder',side:'central'},
  MOD:{id:'MOD',name:t("Meia ofensivo direito"),position:'attackingMidfielder',side:'rightSide'},
  ATE:{id:'ATE',name:t("Atacante pela esquerda"),position:'attacker',side:'central'},
  ATA:{id:'ATA',name:t("Atacante central"),position:'attacker',side:'central'},
  ATD:{id:'ATD',name:t("Atacante pela direita"),position:'attacker',side:'central'},
} satisfies Record<string,AnalysisSlot>;
type AnalysisSlotId=keyof typeof analysisSlots;
const analysisFormations:Record<string,AnalysisSlotId[][]>={
  '4-4-2':[['ATE','ATD'],['ME','MCE','MCD','MD'],['LE','ZE','ZD','LD'],['GOL']],
  '4-2-3-1':[['ATA'],['MOE','MOC','MOD'],['VE','VD'],['LE','ZE','ZD','LD'],['GOL']],
  '4-3-3':[['MOE','ATA','MOD'],['MCE','VOL','MCD'],['LE','ZE','ZD','LD'],['GOL']],
  '3-5-2':[['ATE','ATD'],['MOC'],['AE','MCE','MCD','AD'],['ZE','ZC','ZD'],['GOL']],
};
let clubAnalysis={formation:'',scenario:'',slot:''};
function resetClubAnalysis(){clubAnalysis={formation:'',scenario:'',slot:''};}

function savedAnalysisRows(tactic:ClubTactic):AnalysisSlot[][] {
  const positions=[
    {labels:['','','GOL','',''],name:t("Goleiro"),position:'goalkeeper'},
    {labels:['LIB E','LIB E','LIB','LIB D','LIB D'],name:t("Líbero"),position:'sweeper'},
    {labels:['LE','ZE','ZC','ZD','LD'],name:t("Zagueiro"),position:'defender'},
    {labels:['AE','VE','VOL','VD','AD'],name:t("Volante"),position:'defensiveMidfielder'},
    {labels:['ME','MCE','MC','MCD','MD'],name:t("Meio-campista"),position:'midfielder'},
    {labels:['MOE','MOCE','MOC','MOCD','MOD'],name:t("Meia ofensivo"),position:'attackingMidfielder'},
    {labels:['PE','ATE','ATA','ATD','PD'],name:t("Atacante"),position:'attacker'},
  ];
  const sides=[t(" esquerdo"),t(" pela esquerda"),t(" central"),t(" pela direita"),t(" direito")];
  const slots=tactic.positions.map((slot,index)=>({...slot,starter:tactic.starters?.[index]}));
  return positions.map(({labels,name,position},row)=>slots.filter(slot=>slot.row===row).sort((a,b)=>a.column-b.column).map(({column,starter})=>{
    const id='save-'+row+'-'+column;
    if(row===2&&column===0)return {...analysisSlots.LE,id,label:'LE',column,starter};
    if(row===2&&column===4)return {...analysisSlots.LD,id,label:'LD',column,starter};
    if(row===3&&column===0)return {...analysisSlots.AE,id,label:'AE',column,starter};
    if(row===3&&column===4)return {...analysisSlots.AD,id,label:'AD',column,starter};
    return {id,label:labels[column],position,name:row===0?name:name+sides[column],starter,
      side:row===0?undefined:column===0?'leftSide':column===4?'rightSide':'central',column};
  })).reverse().filter(row=>row.length);
}

interface AnalysisCandidate {player:ClubSquadPlayer;position:number;side:number|null;score:number}
function analysisCandidates(slot:AnalysisSlot,players:ClubSquadPlayer[]):AnalysisCandidate[]{
  return players.flatMap(player=>{
    const position=player.familiarity[slot.position];
    const side=slot.side?player.familiarity[slot.side]:null;
    if(position==null||position<15||position>20||(slot.side&&(side==null||side<15||side>20)))return [];
    return [{player,position,side:side??null,score:Math.min(position,side??position)}];
  }).sort((a,b)=>b.score-a.score||a.player.name.localeCompare(b.player.name,locale)||a.player.uid-b.player.uid);
}

function assignAnalysisSlots(slots:string[],candidates:Map<string,AnalysisCandidate[]>){
  const byPlayer=new Map<number,string>();
  // Try free players first, then move a player if another candidate can cover their current slot.
  const assign=(slot:string,visited:Set<string>):boolean=>{
    if(visited.has(slot))return false;
    visited.add(slot);
    const options=candidates.get(slot)!;
    const candidate=options.find(({player})=>!byPlayer.has(player.uid))
      ??options.find(({player})=>assign(byPlayer.get(player.uid)!,visited));
    if(!candidate)return false;
    byPlayer.set(candidate.player.uid,slot);
    return true;
  };
  const byScarcity=[...slots].sort((a,b)=>candidates.get(a)!.length-candidates.get(b)!.length);
  for(const slot of byScarcity)assign(slot,new Set());
  const bySlot=new Map(Array.from(byPlayer,([uid,slot])=>[slot,uid]));
  return {byPlayer,bySlot};
}

function renderClubAnalysis(d:ClubDetail):string {
  if(!d.players.length)return notice('info',t("Este clube não tem jogadores no elenco registrado."));
  return `<h3 class="tab-heading">${t("Mapa do elenco")}</h3><div class="analysis-controls">`
    +`<div><label for="club-analysis-formation">${t("Formação")}</label><select id="club-analysis-formation">`
    +(d.analysisTactic?'<option value="save">'+escape(d.analysisTactic.name?t("Tática do save · ")+d.analysisTactic.name:t("Tática do save"))+'</option>':'')
    +`<optgroup label="${t("Formações manuais")}">`+Object.keys(analysisFormations).map(name=>'<option>'+name+'</option>').join('')
    +'</optgroup></select></div>'
    +`<div><label for="club-analysis-scenario">${t("Simulação")}</label><select id="club-analysis-scenario"></select></div></div>`
    +(d.analysisTactic?'':notice('info',d.analysisTacticUnsupported
      ?t("Formato da tática do save não reconhecido. Escolha uma formação manual.")
      :t("Tática do save indisponível para este clube. Escolha uma formação manual.")))
    +'<p id="club-analysis-summary" role="status"></p>'
    +`<div class="analysis-layout"><div id="club-analysis-pitch" role="group" aria-label="${t("Vagas na formação")}"></div>`
    +'<section id="club-analysis-slot" aria-labelledby="club-analysis-slot-title"></section></div>';
}

function bindClubAnalysis(d:ClubDetail){
  const formation=$<HTMLSelectElement>('club-analysis-formation');
  if(!formation)return;
  const scenario=$<HTMLSelectElement>('club-analysis-scenario');
  const pitch=$('club-analysis-pitch');
  const slotPanel=$('club-analysis-slot');
  const players=[...d.players].sort((a,b)=>a.name.localeCompare(b.name,locale)||a.uid-b.uid);
  const playerByUid=new Map(players.map(player=>[player.uid,player]));
  const scenarios=new Map<string,{uid:number;label:string}>();
  const options=(title:string,entries:[string,{uid:number;label:string}][])=>{
    if(!entries.length)return '';
    for(const [key,value] of entries)scenarios.set(key,value);
    return '<optgroup label="'+title+'">'+entries.map(([key,value])=>
      '<option value="'+key+'">'+escape(value.label)+'</option>').join('')+'</optgroup>';
  };
  scenario.innerHTML=`<option value="">${t("Sem simulação")}</option>`
    +options(t("Ausência"),players.map(player=>['absence:'+player.uid,{uid:player.uid,label:player.name}]))
    +options(t("Fim de empréstimo"),d.analysisLoans.flatMap(loan=>{
      const player=playerByUid.get(loan.uid);
      if(!player)return [];
      const date=loan.expires?comparisonDate(loan.expires):t("data indisponível");
      return [['loan:'+loan.uid+':'+loan.index,{uid:loan.uid,label:player.name+' · '+date}]];
    }));
  const savedRows=d.analysisTactic?savedAnalysisRows(d.analysisTactic):null;
  if(Array.from(formation.options).some(option=>option.value===clubAnalysis.formation))formation.value=clubAnalysis.formation;
  scenario.value=scenarios.has(clubAnalysis.scenario)?clubAnalysis.scenario:'';

  const refresh=()=>{
    const useSaved=formation.value==='save'&&savedRows!==null;
    const rows:AnalysisSlot[][]=useSaved?savedRows:analysisFormations[formation.value].map(row=>row.map(id=>analysisSlots[id]));
    const slots=rows.flat().map(slot=>slot.id);
    const slotById=new Map(rows.flat().map(slot=>[slot.id,slot]));
    const simulated=scenarios.get(scenario.value);
    const baseline=new Map(rows.flat().map(slot=>[slot.id,analysisCandidates(slot,players)]));
    const candidates=simulated
      ?new Map(slots.map(id=>[id,baseline.get(id)!.filter(({player})=>player.uid!==simulated.uid)]))
      :baseline;
    const savedStarters=new Map(rows.flat().flatMap(({id,starter})=>starter==null?[]:[[id,starter] as const]));
    const lineupKnown=d.analysisTactic?.starters!=null;
    const affected=useSaved&&simulated?Array.from(savedStarters).find(([,uid])=>uid===simulated.uid)?.[0]:undefined;
    const {byPlayer,bySlot}=useSaved
      ?{byPlayer:new Map(Array.from(savedStarters,([id,uid])=>[uid,id])),bySlot:savedStarters}
      :assignAnalysisSlots(slots,candidates);
    const previous=!useSaved&&simulated?assignAnalysisSlots(slots,baseline).bySlot.size:null;
    const freeSlots=new Map<number,number>();
    for(const choices of candidates.values()){
      for(const {player} of choices){
        if(!byPlayer.has(player.uid))freeSlots.set(player.uid,(freeSlots.get(player.uid)??0)+1);
      }
    }
    const count=bySlot.size+t(" de ")+slots.length+t(" vagas preenchidas");
    const summary=$('club-analysis-summary');
    if(useSaved){
      const simulatedPlayer=simulated?playerByUid.get(simulated.uid)?.name:undefined;
      summary.textContent=!lineupKnown?t("Escalação indisponível para esta tática.")
        :affected?simulatedPlayer+t(" fora na simulação. Veja as opções para a vaga.")
        :simulated?simulatedPlayer+t(" não está entre os titulares do save.")
        :savedStarters.size+t(" de ")+slots.length+t(" titulares escalados no jogo.");
    }else summary.textContent=previous===null
        ?count+t(" na distribuição do elenco.")
        :count+t(" com a simulação · ")+previous+t(" antes.");
    const selected=affected??slots.find(id=>id===clubAnalysis.slot)??slots[0];
    pitch.classList.toggle('analysis-saved-grid',useSaved);
    pitch.innerHTML=rows.map(row=>'<div class="analysis-pitch-row">'+row.map(({id,label,name,column})=>{
      const choices=candidates.get(id)!;
      const uid=bySlot.get(id);
      const assigned=uid===undefined?undefined:playerByUid.get(uid);
      const reserves=choices.filter(({player})=>!byPlayer.has(player.uid)).length;
      const absent=id===affected;
      const coverage=useSaved?(absent?t("Fora na simulação"):assigned?t("Escalado no jogo"):'')
        :assigned?(reserves===0?t("Sem opção livre"):reserves===1?t("1 opção livre"):reserves+t(" opções livres"))
          :choices.length?t("Opções em outras vagas"):t("Sem opção");
      const displayName=assigned?.name??(useSaved?(lineupKnown?t("Sem titular"):t("Indisponível")):t("Vaga aberta"));
      const description=name+' · '+displayName+(coverage?' · '+coverage:'');
      return '<button type="button" data-analysis-slot="'+id+'" data-coverage="'+(absent||!assigned?'empty':reserves?'covered':'thin')+'"'
        +(column===undefined?'':' data-analysis-column="'+column+'"')
        +' aria-label="'+escape(description)+'" title="'+escape(description)+'" aria-controls="club-analysis-slot">'
        +'<strong>'+escape(t(label??id))+'</strong><span>'+escape(displayName)+'</span>'+(coverage?'<small>'+coverage+'</small>':'')+'</button>';
    }).join('')+'</div>').join('');

    const showSlot=(id:string)=>{
      clubAnalysis.slot=id;
      for(const button of pitch.querySelectorAll<HTMLButtonElement>('[data-analysis-slot]')){
        button.setAttribute('aria-pressed',String(button.dataset.analysisSlot===id));
      }
      const slot=slotById.get(id)!;
      const choices=candidates.get(id)!;
      const currentUid=bySlot.get(id);
      const current=currentUid===undefined?undefined:playerByUid.get(currentUid);
      const currentFit=baseline.get(id)!.find(({player})=>player.uid===currentUid)?.score;
      const free=choices.filter(({player})=>!byPlayer.has(player.uid));
      const elsewhere=choices.filter(({player})=>byPlayer.has(player.uid)&&byPlayer.get(player.uid)!==id);
      const playerRow=(player:ClubSquadPlayer,score?:number,context?:string)=>{
        const condition=player.condition==null?t("Condição indisponível"):(player.condition/100).toLocaleString(locale)+t("% de condição");
        const status=[player.injuryCount?t("Lesão registrada"):null,player.suspensionCount?t("Suspensão registrada"):null].filter(Boolean);
        return '<button type="button" class="player-row analysis-player" data-club-player="'+player.uid+'"><span><strong>'
          +escape(player.name)+'</strong><small>'+escape([score===undefined?null:score===20?t("Posição natural"):t("Adaptado"),condition,context].filter(Boolean).join(' · '))+'</small>'
          +(status.length?'<small class="analysis-player-alert">'+escape(status.join(' · '))+'</small>':'')+'</span></button>';
      };
      const optionRow=({player,score}:AnalysisCandidate)=>playerRow(player,score);
      const replacing=id===affected;
      const withStatus=replacing?free.filter(({player})=>player.injuryCount||player.suspensionCount):[];
      const options=replacing?free.filter(({player})=>!player.injuryCount&&!player.suspensionCount):free;
      const starterPanel='<h4 id="club-analysis-slot-title">'+escape(slot.name)+'</h4>'
        +'<div class="analysis-primary"><h5>'+(replacing?t("Titular fora na simulação"):useSaved?t("Escalado no jogo"):t("Sugestão do mapa"))+'</h5>'
        +(current?playerRow(current,currentFit):'<p class="profile-meta">'+(useSaved?(lineupKnown?t("Sem titular nesta posição"):t("Escalação indisponível")):t("Vaga aberta"))+'</p>')+'</div>';
      if(useSaved&&!lineupKnown){
        slotPanel.innerHTML=starterPanel;
        return;
      }
      slotPanel.innerHTML=starterPanel+'<h5>'+(replacing?t("Quem pode entrar"):t("Opções livres"))+(options.length?' · '+options.length:'')+'</h5>'
        +(options.length?options.slice(0,3).map(optionRow).join(''):`<p class="profile-meta">${t("Nenhuma opção livre para esta vaga.")}</p>`)
        +(options.length>3?`<details><summary>${t("Mais opções livres (")}`+(options.length-3)+')</summary>'
          +options.slice(3).map(optionRow).join('')+'</details>':'')
        +(withStatus.length?`<details><summary>${t("Com lesão ou suspensão registrada (")}`+withStatus.length+')</summary>'
          +withStatus.map(optionRow).join('')+'</details>':'')
        +(elsewhere.length?`<details><summary>${t("Jogadores em outras vagas (")}`+elsewhere.length+')</summary>'
          +elsewhere.map(({player,score})=>playerRow(player,score,slotById.get(byPlayer.get(player.uid)!)?.name)).join('')+'</details>':'')
        +`<details id="club-analysis-details"><summary>${t("Detalhes da posição")}</summary>`
        +`<p>${t("O mapa considera familiaridade de 15 a 20 na posição e no lado, quando aplicável. Lesões e suspensões são as registradas no save. A simulação só muda este mapa.")}</p>`
        +(simulated?`<p>${t("Opções após simular:")} `+choices.length+t(" de ")+baseline.get(id)!.length+'.</p>':'')
        +(choices.length?'<ul>'+choices.map(({player,position,side})=>{
          const shared=freeSlots.get(player.uid)??0;
          return '<li><strong>'+escape(player.name)+`</strong> ${t("· Posição")} `+position+'/20'
            +(side===null?'':t(" · Lado ")+side+'/20')+(shared>1?t(" · Também cobre ")+(shared-1)+t(" outra(s) vaga(s)"):'')+'</li>';
        }).join('')+'</ul>':'')+'</details>';
    };
    for(const button of pitch.querySelectorAll<HTMLButtonElement>('[data-analysis-slot]')){
      const id=button.dataset.analysisSlot!;
      button.onclick=()=>showSlot(id);
    }
    showSlot(selected);
  };
  formation.onchange=()=>{clubAnalysis.formation=formation.value;refresh();};
  scenario.onchange=()=>{clubAnalysis.scenario=scenario.value;refresh();};
  refresh();
}

let clubTab='finance';
async function showClub(uid:number,checkDraft=true){
  if(checkDraft&&!await canLeave())return false;
  const d=await invoke<ClubDetail>('club_detail',{uid});
  if(clubSelected!==uid)resetClubAnalysis();
  clubSelected=uid;
  const profile=$('club-profile');
  const info=renderClubInfo(uid,d.info,d.projectedPlayerPayrollWeekly);
  const squad=renderClubRoster(d);
  const tabs=[['finance',t("Finanças"),info.finance],['infrastructure',t("Infraestrutura"),info.infrastructure],
    ['stadium',t("Estádio"),renderClubStadium(uid,d.stadium)+(d.info?`<p class="profile-meta">${t("Público médio registrado:")} `+d.info.averageAttendance.toLocaleString(locale)+'</p>':'')],['squad',t("Elenco"),squad],
    ['analysis',t("Análise"),renderClubAnalysis(d)]];
  const shortName=d.profile.shortName!==d.profile.name?'<p class="profile-meta">'+escape(d.profile.shortName)+'</p>':'';
  profile.innerHTML=`<div class="profile-header"><div><div class="eyebrow">${t("CLUBE · UID")} `+uid+'</div><h2>'+escape(d.profile.name)+'</h2>'+shortName+'</div></div>'
    +`<div class="profile-tabs" role="tablist" aria-label="${t("Ficha do clube")}">`+tabs.map(([id,label])=>
      '<button type="button" role="tab" id="club-tab-'+id+'" aria-controls="club-panel-'+id+'">'+label+'</button>').join('')+'</div>'
    +tabs.map(([id,,content])=>'<section role="tabpanel" id="club-panel-'+id+'" aria-labelledby="club-tab-'+id+'">'+content+'</section>').join('');
  const selectTab=(id:string,focus=false)=>{
    clubTab=id;
    for(const [key] of tabs){
      const active=key===id;
      const tab=$('club-tab-'+key);
      tab.setAttribute('aria-selected',String(active));
      tab.tabIndex=active?0:-1;
      $('club-panel-'+key).hidden=!active;
      if(active&&focus)tab.focus({preventScroll:true});
    }
    profile.scrollTop=0;
  };
  for(const [id] of tabs){
    const tab=$('club-tab-'+id);
    tab.onclick=()=>selectTab(id);
    tab.onkeydown=event=>{
      const index=tabs.findIndex(([key])=>key===id);
      const next=event.key==='Home'?0:event.key==='End'?tabs.length-1:
        event.key==='ArrowRight'?(index+1)%tabs.length:event.key==='ArrowLeft'?(index+tabs.length-1)%tabs.length:-1;
      if(next<0)return;
      event.preventDefault();
      selectTab(tabs[next][0],true);
    };
  }
  selectTab(clubTab);
  prepareProfile(profile);
  syncClubValidation();
  for(const row of document.querySelectorAll<HTMLElement>('[data-club-uid]'))row.classList.toggle('selected',Number(row.dataset.clubUid)===uid);
  bindClubRoster();
  bindClubAnalysis(d);
  $('compare-club').onclick=()=>{$<HTMLDetailsElement>('club-tools').open=false;void compareClubSave(uid);};
  bindClubExpiries(uid,d.expiringContracts??[],d.wageContracts??[]);
  const expiryPanel=$('club-roster-expiries');
  const expiryOpen=expiryPanel.querySelector('details')!.open;
  expiryPanel.hidden=!expiryOpen;
  $('club-open-expiries').setAttribute('aria-expanded',String(expiryOpen));
  bindClubStaffComparison(d.staff,uid);
  bindWageBatch(uid,d.wageContracts??[]);
  bindShirtSwap(uid,d.wageContracts??[]);
  bindClubBatch($('club-profile'),d.players);
  const stadiumForm=$<HTMLFormElement>('club-stadium-form');
  if(stadiumForm)stadiumForm.onsubmit=e=>{
    e.preventDefault();
    if(validateClubForm(stadiumForm))return;
    const capacity=$<HTMLInputElement>('stadium-capacity');
    const seated=$<HTMLInputElement>('stadium-seated-capacity');
    const target={capacity:capacity.valueAsNumber,seatedCapacity:seated.valueAsNumber};
    if(target.seatedCapacity>target.capacity){
      notify(t("Os lugares sentados devem caber na lotação total."),'error');
      return;
    }
    void task(async()=>{
      plan=await invoke<Pending>('stage_club_stadium',{uid,target});
      capacity.dataset.initial=String(target.capacity);
      seated.dataset.initial=String(target.seatedCapacity);
      if(document.activeElement===document.body)capacity.focus({preventScroll:true});
      notify(t("Lotação adicionada à revisão."),'success');
    });
  };
  const trainingForm=$<HTMLFormElement>('club-training-form');
  if(trainingForm)trainingForm.onsubmit=e=>{
    e.preventDefault();
    if(validateClubForm(trainingForm))return;
    const training=$<HTMLInputElement>('club-training');
    const youth=$<HTMLInputElement>('club-youth');
    const reputation=document.querySelector<HTMLInputElement>('#club-reputation');
    const fields=reputation?[training,youth,reputation]:[training,youth];
    const values=fields.map(field=>field.value===field.dataset.initial?null:field.valueAsNumber);
    if(values.slice(0,2).some(value=>value!==null&&(!Number.isInteger(value)||value<1||value>20))){
      notify(t("Informe um nível de infraestrutura entre 1 e 20."),'error');
      return;
    }
    const rep=values[2]??null;
    if(rep!==null&&(!Number.isInteger(rep)||rep<0||rep>10000)){
      notify(t("Informe uma reputação entre 0 e 10.000."),'error');
      return;
    }
    void task(async()=>{
      const submitted=fields.map(field=>field.value);
      plan=await invoke<Pending>('stage_club_infrastructure',{uid,training:values[0],youth:values[1],reputation:rep});
      fields.forEach((field,i)=>field.dataset.initial=submitted[i]);
      if(document.activeElement===document.body)training.focus({preventScroll:true});
      notify(t("Plano de infraestrutura atualizado."),'success');
    });
  };
  if(d.info&&$('club-balance-form'))bindClubFinanceEditor(uid,d.info);
  return true;
}

function bindClubFinanceEditor(uid:number,info:ClubInfo){
  const editor=$('club-finance-editor');
  const toggle=$<HTMLButtonElement>('edit-club-finances');
  const balance=$<HTMLInputElement>('club-balance');
  const form=$<HTMLFormElement>('club-balance-form');
  const setOpen=(open:boolean)=>{
    editor.hidden=!open;
    toggle.setAttribute('aria-expanded',String(open));
    if(open)balance.focus();
    else {
      $('club-profile').scrollTop=0;
      toggle.focus();
    }
  };
  toggle.onclick=()=>setOpen(editor.hidden===true);
  $('close-club-finances').onclick=()=>setOpen(false);
  form.onsubmit=e=>{
    e.preventDefault();
    if(editor.hidden||validateClubForm(form))return;
    const value=balance.valueAsNumber;
    void task(async()=>{
      const season=$<HTMLInputElement>('club-budget-season');
      const remaining=$<HTMLInputElement>('club-budget-remaining');
      const budgets=season&&remaining&&(season.value!==season.dataset.initial||remaining.value!==remaining.dataset.initial)
        ? {season:season.valueAsNumber,remaining:remaining.valueAsNumber} : null;
      const wage=$<HTMLInputElement>('club-wage-budget');
      const wageBudget=wage&&wage.value!==wage.dataset.initial?wage.valueAsNumber:null;
      plan=await invoke<Pending>('stage_club_balance',{uid,value,budgets,wageBudget});
      if(wage&&wageBudget!==null)wage.dataset.initial=String(wageBudget);
      balance.dataset.initial=String(value);
      if(budgets){season.dataset.initial=String(budgets.season);remaining.dataset.initial=String(budgets.remaining);}
      $('club-finance-indicators').innerHTML=renderClubFinanceIndicators(uid,info);
      if(document.activeElement===document.body)$('club-finance-editor-title').focus({preventScroll:true});
      notify(t("Plano de balanço atualizado."),'success');
    });
  };
}
$('nav-clubs').onclick=async()=>{if(!summary||!await canLeave())return;delete $('clubs-view').dataset.pane;renderProfile();view('clubs');void task(async()=>{await searchClubs();if(clubSelected!==null)await showClub(clubSelected,false);});};
$('club-results').onclick=e=>{
  const row=(e.target as HTMLElement).closest<HTMLElement>('[data-club-uid]');
  if(row&&!$('club-results').hasAttribute('aria-busy'))void task(async()=>{
    if(await showClub(Number(row.dataset.clubUid))){clearToast();enterProfile('club-profile');}
  });
};
$('club-profile').onclick=e=>{
  const target=e.target as HTMLElement;
  const player=target.closest<HTMLElement>('[data-club-player], [data-squad-player]');
  const staff=target.closest<HTMLElement>('[data-club-staff], [data-club-ranked-staff]');
  if(!player&&!staff)return;
  void task(async()=>{
    if(!await canLeave())return;
    if(player){
      view('players');
      const contract=player.dataset.clubContract;
      if(contract!==undefined)profileTab='contracts';
      await showPlayer(Number(player.dataset.clubPlayer??player.dataset.squadPlayer),false);
      enterProfile('profile');
      if(contract!==undefined){
        const card=document.querySelector<HTMLDetailsElement>('#contract-card-'+contract);
        if(card)card.open=true;
        card?.focus({preventScroll:true});
        card?.scrollIntoView({block:'nearest'});
      }
    }else if(staff){
      renderProfile();
      view('staff');
      if(await showStaff(Number(staff.dataset.clubStaff??staff.dataset.clubRankedStaff),false))enterProfile('staff-profile');
    }
  });
};
const clubSearch = $<HTMLInputElement>('club-search');
async function refreshClubFilter() {
  clearTimeout(clubTimer);
  clubTimer = undefined;
  clubPage = 0;
  const generation = clubGeneration + 1;
  try {
    await searchClubs();
  } catch (error) {
    if (generation === clubGeneration) {
      $('club-results').innerHTML = notice('error',t("Não foi possível buscar clubes."));
      notify(String(error), 'error');
    }
  } finally {
    if (generation === clubGeneration) $('club-results').removeAttribute('aria-busy');
  }
}
clubSearch.oninput = () => {
  clearTimeout(clubTimer);
  clubGeneration++;
  $('club-results').setAttribute('aria-busy', 'true');
  clubTimer = setTimeout(() => void refreshClubFilter(), 180);
};
clubSearch.onkeydown = async event => {
  if (event.key !== 'Enter' && event.key !== 'ArrowDown') return;
  event.preventDefault();
  if ($('club-results').hasAttribute('aria-busy')) await refreshClubFilter();
  if (document.activeElement !== clubSearch || $('club-results').hasAttribute('aria-busy')) return;
  const first = $('club-results').querySelector<HTMLButtonElement>('[data-club-uid]');
  if (event.key === 'Enter') first?.click();
  else first?.focus();
};
$('club-results').onkeydown = event => {
  if (!['ArrowDown', 'ArrowUp'].includes(event.key) || !(event.target instanceof HTMLButtonElement)) return;
  event.preventDefault();
  const next = event.key === 'ArrowDown' ? event.target.nextElementSibling : event.target.previousElementSibling;
  if (next instanceof HTMLButtonElement) next.focus();
  else if (event.key === 'ArrowUp') clubSearch.focus();
};
$('club-previous').onclick=()=>{if($('club-results').hasAttribute('aria-busy'))return;clubPage--;void searchClubs().catch(e=>notify(String(e),'error'));};$('club-next').onclick=()=>{if($('club-results').hasAttribute('aria-busy'))return;clubPage++;void searchClubs().catch(e=>notify(String(e),'error'));};
const playerFilterForm=$<HTMLFormElement>('player-filter-form');
const playerAttributeFilterError=$('player-attribute-filter-error');
const lastPlayerSearchKey='cm0304.last-player-search.v1';
function setPlayerFilterFields(filters:Record<string,number|string>){
  playerAttributeFilterError.textContent='';
  for(const field of playerFilterForm.querySelectorAll<HTMLInputElement|HTMLSelectElement>('[name]')){
    field.value=String(filters[field.name]??'');
  }
}
function restoreLastPlayerSearch(){
  let query='';
  let watchOnly=false;
  playerFilters={};
  try{
    const stored=JSON.parse(localStorage.getItem(lastPlayerSearchKey)??'null');
    if(stored&&typeof stored.query==='string'&&stored.filters&&typeof stored.filters==='object'&&!Array.isArray(stored.filters)){
      query=stored.query;
      watchOnly=stored.watchOnly===true;
      playerFilters=Object.fromEntries(Array.from(playerFilterForm.querySelectorAll<HTMLInputElement|HTMLSelectElement>('[name]'))
        .map(field=>[field.name,stored.filters[field.name]])
        .filter(([,value])=>typeof value==='string'||typeof value==='number'));
    }
  }catch{/* Start with the default search when local preferences cannot be read. */}
  $<HTMLInputElement>('search').value=query;
  $<HTMLInputElement>('only-watchlist').checked=watchOnly;
  setPlayerFilterFields(playerFilters);
}
const filterChoices=(id:string,keys:string[])=>{
  $<HTMLSelectElement>(id).replaceChildren(new Option(t("Qualquer"),''),...keys.map(key=>new Option(fieldLabel(key),key)));
};
const playerPositionKeys=['goalkeeper','sweeper','defender','defensiveMidfielder','midfielder','attackingMidfielder','attacker','wingBack','freeRole'];
filterChoices('player-filter-position',playerPositionKeys);
filterChoices('player-filter-attribute',Object.keys(labels).filter(key=>!playerPositionKeys.includes(key)&&!['rightSide','leftSide','central'].includes(key)));
playerFilterForm.addEventListener('input',event=>{
  if(event.target instanceof HTMLElement&&event.target.matches('[name="attribute"], [name="minAttribute"]')){
    playerAttributeFilterError.textContent='';
  }
});
playerFilterForm.onsubmit=e=>{
  e.preventDefault();
  if(syncAuxiliaryValidation(playerFilterForm))return;
  const filters=Object.fromEntries(Array.from(new FormData(playerFilterForm)).filter(([,value])=>value!=='').map(([key,value])=>[key,['nation','order','position','attribute','foot','contract'].includes(key)?String(value):Number(value)]));
  if(['Age','Ca','Pa'].some(field=>Number(filters['min'+field])>Number(filters['max'+field]))){
    notify(t("O mínimo deve ser menor ou igual ao máximo."),'error');return;
  }
  if((filters.attribute===undefined)!==(filters.minAttribute===undefined)){
    $<HTMLDetailsElement>('player-filter-attributes').open=true;
    playerAttributeFilterError.textContent=t("Escolha um atributo e informe seu valor mínimo.");
    const missing=filters.attribute===undefined?'attribute':'minAttribute';
    (playerFilterForm.elements.namedItem(missing) as HTMLElement).focus();
    notify(playerAttributeFilterError.textContent,'error');
    return;
  }
  void task(async()=>{playerFilters=filters;page=0;await search();});
};
playerFilterForm.onreset=()=>{
  playerAttributeFilterError.textContent='';
  queueMicrotask(()=>syncAuxiliaryValidation(playerFilterForm));
  playerFilters={};page=0;
  if(summary&&!busy)void task(search);
};
interface SavedPlayerSearch {name:string;query:string;filters:Record<string,number|string>}
const playerSearchKey='cm0304.player-searches.v1';
const savedPlayerSearch=$<HTMLSelectElement>('saved-player-search');
let savedPlayerSearches:SavedPlayerSearch[]=[];
try{
  const stored:unknown=JSON.parse(localStorage.getItem(playerSearchKey)??'[]');
  if(Array.isArray(stored))savedPlayerSearches=stored.filter(item=>item&&typeof item.name==='string'&&typeof item.query==='string'&&item.filters&&typeof item.filters==='object');
}catch{/* A malformed local entry cannot prevent opening a save. */}
function renderSavedPlayerSearches(selected=''){
  savedPlayerSearch.replaceChildren(new Option(t("Selecione uma busca"),''),...savedPlayerSearches.map(item=>new Option(item.name,item.name)));
  savedPlayerSearch.value=selected;
}
function storePlayerSearches(next:SavedPlayerSearch[],selected=''){
  try{localStorage.setItem(playerSearchKey,JSON.stringify(next));}
  catch{notify(t("Não foi possível gravar as buscas locais."),'error');return;}
  savedPlayerSearches=next;
  renderSavedPlayerSearches(selected);
}
renderSavedPlayerSearches();
$('save-player-search').onclick=async()=>{
  if(!summary)return;
  const name=$<HTMLInputElement>('player-search-name').value.trim();
  if(!name){notify(t("Informe um nome para a busca."),'error');return;}
  if(savedPlayerSearches.some(item=>item.name===name)&&!await confirmAction(t("Substituir busca?"),
    t("A consulta e os filtros atuais substituirão a busca “")+name+'”.',t("Substituir")))return;
  storePlayerSearches([...savedPlayerSearches.filter(item=>item.name!==name),{name,query:$<HTMLInputElement>('search').value,filters:{...playerFilters}}],name);
};
$('load-player-search').onclick=()=>{
  const saved=savedPlayerSearches.find(item=>item.name===savedPlayerSearch.value);
  if(!summary||!saved)return;
  void task(async()=>{
    $<HTMLInputElement>('search').value=saved.query;
    $<HTMLInputElement>('player-search-name').value=saved.name;
    setPlayerFilterFields(saved.filters);
    playerFilters={...saved.filters};
    page=0;
    await search();
  });
};
$('delete-player-search').onclick=async()=>{
  const name=savedPlayerSearch.value;
  if(name&&await confirmAction(t("Excluir busca?"),t("A busca “")+name+t("” será removida deste editor."),t("Excluir"))){
    storePlayerSearches(savedPlayerSearches.filter(item=>item.name!==name));
  }
};
$('batch-filtered').onclick=async()=>{
  if(!summary||!await canLeave())return;
  void task(async()=>{
    const players=await invoke<BatchPlayer[]>('select_filtered_players',{query:$<HTMLInputElement>('search').value,filters:currentSearchFilters()});
    if(!players.length){notify(t("Nenhum jogador corresponde à busca."),'error');return;}
    openPlayerBatch(players,$('batch-filtered'));
  });
};
let batchReturnFocus:HTMLElement|null=null;
function openPlayerBatch(players:BatchPlayer[],trigger:HTMLElement,playerName?:string){
  const dialog=$<HTMLDialogElement>('filtered-batch-dialog');
  const content=$('filtered-batch-content');
  const player=playerName===undefined?undefined:players[0];
  const profileScroll=$('profile').scrollTop;
  batchReturnFocus=trigger;
  content.innerHTML=renderClubBatch(players,true);
  bindClubBatch(content,players,player?.uid,player?async()=>{
    await showPlayer(player.uid,false);
    $('profile').scrollTop=profileScroll;
    batchReturnFocus=$('profile').querySelector('.profile-compare-actions button:last-child');
  }:undefined);
  dialog.showModal();
}
$<HTMLDialogElement>('filtered-batch-dialog').oncancel=event=>{if(busy)event.preventDefault();};
$<HTMLDialogElement>('filtered-batch-dialog').onclose=()=>{
  $('filtered-batch-content').replaceChildren();
  batchReturnFocus?.focus({preventScroll:true});
  batchReturnFocus=null;
};
$('filtered-batch-close').onclick=()=>$<HTMLDialogElement>('filtered-batch-dialog').close();
$<HTMLSelectElement>('watchlist-select').onchange=()=>{
  const name=$<HTMLSelectElement>('watchlist-select').value;
  if(!saveWatchlists(new Map(lists),name))syncWatchlist();
};
$<HTMLFormElement>('watchlist-form').onsubmit=(event)=>{
  event.preventDefault();
  const input=$<HTMLInputElement>('watchlist-name');
  const name=input.value.trim();
  if(!name||[...lists.keys()].some(existing=>existing.toLocaleLowerCase()===name.toLocaleLowerCase())){
    notify(t("Digite um nome diferente para a nova lista."),'error');
    return;
  }
  if(saveWatchlists(new Map(lists).set(name,[]),name))input.value='';
};
$('watchlist-delete').onclick=async()=>{
  const name=activeList;
  if(name==='Observação'||!await confirmAction(t("Excluir lista de observação?"),
    t("A lista “")+name+t("” será removida deste editor."),t("Excluir")))return;
  const next=new Map(lists);
  next.delete(name);
  saveWatchlists(next,'Observação');
};
$<HTMLInputElement>('only-watchlist').onchange=()=>{if(summary){page=0;void task(search);}};
syncWatchlist();
$('export-filtered-players').onclick=()=>exportFilteredPlayers('csv');
$('export-filtered-players-json').onclick=()=>exportFilteredPlayers('json');
function exportFilteredPlayers(format:'csv'|'json'){
  if(!summary)return;
  void task(async()=>{
    const path=await invoke<string|null>('export_filtered_players',{format,
      query:$<HTMLInputElement>('search').value,filters:currentSearchFilters()
    });
    if(path)notify(t("Resultados exportados: ")+path,'success');
  });
}
$('export-plan-report').onclick=async()=>{
  if(!await canLeave()||!planCount())return;
  void task(async()=>{
    const path=await invoke<string|null>('export_plan_report');
    if(path)notify(t("Relatório exportado: ")+path,'success');
  });
};
$('export-session-plan').onclick=async()=>{
  if(!planCount()||!await canLeave())return;
  void task(async()=>{
    const path=await invoke<string|null>('export_session_plan');
    if(path)notify(t("Plano guardado: ")+path,'success');
  });
};
$('import-session-plan').onclick=async()=>{
  if(!summary||!await canLeave())return;
  const replaceExisting=planCount()>0;
  if(replaceExisting&&!await confirmAction(t("Substituir alterações pendentes?"),
    t("O plano guardado substituirá a revisão atual se for carregado com sucesso."),t("Substituir")))return;
  void task(async()=>{
    const restored=await invoke<Pending|null>('import_session_plan',{replaceExisting});
    if(!restored)return;
    plan=restored;
    if(detail){detail=await invoke<Detail>('player_detail',{uid:detail.player.uid});renderProfile();}
    if(staffDetail)await showStaff(staffDetail.profile.uid,false);
    if(clubSelected!==null)await showClub(clubSelected,false);
    if(await validateReview())notify(t("Plano restaurado e validado."),'success');
  });
};
window.onbeforeunload=e=>{if(busy||planCount()||hasFormDrafts()){e.preventDefault();}};
renderRecentSaves();

const languageSelect = $<HTMLSelectElement>('language-select');
const resumeAfterLanguageKey = 'cm0304.resume-after-language';
languageSelect.value = locale;
languageSelect.addEventListener('change', () => {
  const next = languageSelect.value === 'en-GB' ? 'en-GB' : 'pt-BR';
  languageSelect.value = locale;
  if (next === locale) return;
  if (busy) {
    notify(t('Aguarde a operação atual para trocar o idioma.'), 'warning');
    return;
  }
  if (hasFormDrafts()) {
    notify(t('Adicione os rascunhos à revisão antes de trocar o idioma.'), 'warning');
    return;
  }
  try {
    if (summary) sessionStorage.setItem(resumeAfterLanguageKey, '1');
    saveLocale(next);
  } catch {
    notify(t('Não foi possível guardar a preferência de idioma.'), 'error');
    return;
  }
  location.reload();
});

function shouldResumeAfterLanguageChange() {
  try {
    if (!sessionStorage.getItem(resumeAfterLanguageKey)) return false;
    sessionStorage.removeItem(resumeAfterLanguageKey);
    return true;
  } catch {
    return false;
  }
}

if (shouldResumeAfterLanguageChange()) {
  queueMicrotask(() => void task(async () => {
    const session = await invoke<[Summary, Pending] | null>('resume_editor');
    if (session) await loadWorkspace(...session);
  }));
} else {
  void invoke<boolean>('download_update').then(async ready => {
    if (!ready || summary || busy || closingEditor || document.querySelector('dialog[open]')) return;
    notify(t('Instalando a atualização. O editor vai reabrir em seguida.'), 'info');
    document.body.inert = true;
    try {
      await invoke('install_update');
    } catch {
      notify(t('Não foi possível instalar a atualização. O editor tentará novamente na próxima abertura.'), 'warning');
    } finally {
      document.body.inert = false;
    }
  }).catch(error => console.warn('Atualização do editor:', error));
}
syncPlan();
export {};

function renderActiveFilters() {
  const chips = $('active-filters');
  const form = $<HTMLFormElement>('player-filter-form');
  const labels = Object.entries(playerFilters).flatMap(([key, value]) => {
    const control = form.elements.namedItem(key);
    if (!(control instanceof HTMLInputElement || control instanceof HTMLSelectElement)) return [];
    const label = control.getAttribute('aria-label') ?? control.labels?.[0]?.firstChild?.textContent?.trim() ?? key;
    const display = control instanceof HTMLSelectElement
      ? Array.from(control.options).find(option => option.value === String(value))?.text ?? value
      : value;
    return [label + ' · ' + display];
  });
  if ($<HTMLInputElement>('only-watchlist').checked) labels.push(activeList==='Observação'?t(activeList):activeList);
  chips.replaceChildren(...labels.map(label => {
    const chip = document.createElement('span');
    chip.textContent = label;
    return chip;
  }));
  chips.hidden = !labels.length;
}

for (const divider of document.querySelectorAll<HTMLElement>('.pane-divider')) {
  const grid = divider.parentElement!;
  const roster = grid.querySelector<HTMLElement>('.roster')!;
  const setWidth = (width: number) => {
    const max = Math.round(grid.clientWidth * 0.52);
    const size = Math.round(Math.max(230, Math.min(max, width)));
    grid.style.setProperty('--roster-width', size + 'px');
    divider.setAttribute('aria-valuemax', String(max));
    divider.setAttribute('aria-valuenow', String(size));
  };
  new ResizeObserver(() => {
    if (!compactWorkspace.matches&&grid.clientWidth) setWidth(roster.clientWidth + 2);
  }).observe(grid);
  divider.addEventListener('pointerdown', event => {
    if (event.button !== 0) return;
    divider.setPointerCapture(event.pointerId);
  });
  divider.addEventListener('pointermove', event => {
    if (divider.hasPointerCapture(event.pointerId)) {
      setWidth(event.clientX - grid.getBoundingClientRect().left);
    }
  });
  divider.addEventListener('keydown', event => {
    if (!['ArrowLeft', 'ArrowRight', 'Home', 'End'].includes(event.key)) return;
    event.preventDefault();
    const width = roster.getBoundingClientRect().width;
    setWidth(event.key === 'Home' ? 230 : event.key === 'End' ? grid.clientWidth
      : width + (event.key === 'ArrowLeft' ? -16 : 16));
  });
}

$('profile').addEventListener('input', event => {
  const input = event.target;
  if (input instanceof HTMLInputElement && input.dataset.originalValue !== undefined) {
    decorateAttribute(input);
  }
});

function decorateAttribute(input: HTMLInputElement) {
  const original = input.dataset.originalValue!;
  const current = input.value;
  const changed = current !== '' && Number(current) !== Number(original);
  input.classList.toggle('changed', changed);
  input.dataset.band = input.min === '1' && input.max === '20' && current !== ''
    ? Number(current) >= 15 ? 'high' : Number(current) >= 8 ? 'mid' : 'low'
    : '';
  const difference = input.parentElement?.querySelector<HTMLElement>('.attribute-difference');
  if (difference) {
    difference.hidden = !changed || input.getAttribute('aria-invalid') === 'true';
    difference.textContent = changed ? original + ' → ' + current : '';
  }
  const scale=Number(input.dataset.scale??1);
  const staged=Number(input.dataset.initial)/scale!==Number(original);
  const draft=current===''||inputValue(input)!==Number(input.dataset.initial);
  showFieldStatus(input,draft,staged,difference??input);
}

function showFieldStatus(input:HTMLElement,draft:boolean,staged:boolean,after=input) {
  let marker=after.nextElementSibling;
  if(!marker?.classList.contains('field-edit-status')){
    if(!draft&&!staged)return;
    marker=document.createElement('small');
    marker.className='field-edit-status';
    after.after(marker);
  }
  const label=draft?t("Rascunho"):staged?t("Na revisão"):'';
  if(marker.textContent!==label)marker.textContent=label;
  if(marker instanceof HTMLElement)marker.hidden=!label;
  input.classList.toggle('changed',draft||staged);
}

function syncPlayerDraft() {
  const label = document.getElementById('player-draft-count');
  if (!label) return;
  syncPlayerAbilityChoice();
  const invalid = validatePlayerFields();
  const fields = new Set(drafts().map(edit =>
    edit.attribute.replace(/(\.expires|^birth)\.(year|day)$/, '$1')));
  for(const input of $('profile').querySelectorAll<HTMLInputElement|HTMLSelectElement>('select[data-original], input[data-date]')){
    const key=input.dataset.date??input.dataset.field!;
    showFieldStatus(input,fields.has(key),input.dataset.date
      ?input.dataset.staged==='true':input.dataset.initial!==input.dataset.original);
  }
  const nationality=detail?.player.fields.nationality;
  if(nationality){
    const select=$<HTMLSelectElement>('player-nationality');
    const trigger=$('nation-trigger');
    showFieldStatus(trigger,fields.has('nationality'),Number(select.dataset.initial)!==nationality.value);
    trigger.title=t("Original: ")+(detail?.nations[nationality.value]??t("Não informada"));
  }
  for(const [id,change,staged] of [
    ['remove-injuries',healingDraft(),detail?.healPending],
    ['remove-suspensions',suspensionDraft(),detail?.suspensionPending],
  ] as const){
    const input=$(id);
    if(input)showFieldStatus(input,!!change,!!staged,input.parentElement!);
  }
  const count = fields.size + Number(!!healingDraft()) + Number(!!suspensionDraft());
  label.classList.toggle('notice',invalid>0);
  if(invalid)label.dataset.notice='error';else delete label.dataset.notice;
  const invalidInput=document.querySelector<HTMLInputElement>('#profile input[aria-invalid=true]');
  const invalidTab=invalidInput?.closest('[role=tabpanel]')?.getAttribute('aria-labelledby');
  const tabLabel=invalidTab ? $(invalidTab).textContent : '';
  $('go-to-player-error').hidden=!invalid;
  label.textContent = invalid ? (invalid===1 && tabLabel ? t("Corrija 1 campo em ")+tabLabel+'.' : t("Corrija os campos destacados para continuar.")) : count === 0 ? t("Nenhum campo editado neste jogador")
    : count === 1 ? t("1 campo editado neste jogador") : count + t(" campos editados neste jogador");
  $<HTMLButtonElement>('stage-player').disabled = busy || count === 0 || invalid > 0;
}

$('profile').addEventListener('input', syncPlayerDraft);
$('profile').addEventListener('change', syncPlayerDraft);

function syncPlayerAbilityChoice() {
  const box = $('player-ability-choice');
  if (!box) return;
  const inputs = Array.from(document.querySelectorAll<HTMLInputElement>(
    '#profile [data-field="currentAbility"], #profile [data-field="position.goalkeeper"]'));
  const changed = inputs.filter(input => input.value !== '' &&
    inputValue(input) !== Number(input.dataset.initial));
  const anchor = (changed.find(input => !input.closest('[role=tabpanel]')?.hasAttribute('hidden'))
    ?? changed[0])?.closest('label');
  box.hidden = !anchor;
  if (!anchor) {
    const choice = $<HTMLSelectElement>('ability-policy');
    choice.value = choice.dataset.initial ?? '';
  }
  if (anchor && anchor.nextElementSibling !== box) anchor.after(box);
}

function renderPlayerDiagnostics() {
  const player = detail?.player;
  const section = $<HTMLDetailsElement>('player-diagnostics');
  const raw = $('player-diagnostics-raw');
  section.open = false;
  $('player-diagnostics-context').textContent = player
    ? player.name + ' · UID ' + player.uid
    : t("Selecione um jogador na aba Jogadores para consultar os dados técnicos.");
  $('player-diagnostics-context').classList.toggle('notice', !player);
  if (!player) $('player-diagnostics-context').dataset.notice = 'info';
  else delete $('player-diagnostics-context').dataset.notice;
  raw.hidden = !player;
  raw.textContent = player ? JSON.stringify(player.rawAttributes, null, 2) : '';
}

function bindNationalityPicker() {
  const picker = $('nation-trigger');
  const popup = $('nation-popup');
  const results = $('nation-results');
  const search = $<HTMLInputElement>('nation-search');
  const nationality = $<HTMLSelectElement>('player-nationality');
  const key = (value: string) => value.normalize('NFD').replace(/\p{M}/gu, '').toLocaleLowerCase(locale);
  const countries = Array.from(nationality.options).filter(option => !option.disabled);
  const refreshLabel = () => $('nation-value').textContent = nationality.selectedOptions[0]?.text ?? t("Não informada");
  const setOpen = (open: boolean, restoreFocus = false) => {
    picker.setAttribute('aria-expanded', String(open));
    popup.hidden = !open;
    if (open) {
      search.value = '';
      renderResults();
      search.focus();
    } else if (restoreFocus) picker.focus();
  };
  const renderResults = () => {
    const query = key(search.value.trim());
    const matches = countries.filter(country => key(country.text).includes(query));
    results.replaceChildren(...matches.map(country => {
      const button = document.createElement('button');
      button.type = 'button';
      button.textContent = country.text;
      if (country.selected) button.setAttribute('aria-current', 'true');
      button.onclick = () => {
        nationality.value = country.value;
        nationality.dispatchEvent(new Event('input', {bubbles:true}));
        refreshLabel();
        setOpen(false, true);
      };
      return button;
    }));
    $('nation-search-feedback').textContent = matches.length ? '' : t("Nenhum país encontrado.");
  };
  refreshLabel();
  picker.onclick = () => setOpen(popup.hasAttribute('hidden'), true);
  const container = picker.closest<HTMLElement>('.nation-picker')!;
  container.addEventListener('focusout', event => {
    if (!(event.relatedTarget instanceof Node) || !container.contains(event.relatedTarget)) setOpen(false);
  });
  search.oninput = renderResults;
  container.onkeydown = event => {
    if (popup.hidden) return;
    const buttons = Array.from(results.querySelectorAll('button'));
    const index = buttons.indexOf(event.target as HTMLButtonElement);
    if (event.key === 'Escape') {
      event.preventDefault();
      setOpen(false, true);
    } else if (event.key === 'Enter' && event.target === search) {
      event.preventDefault();
      buttons[0]?.click();
    } else if (event.key === 'ArrowDown' || event.key === 'ArrowUp') {
      event.preventDefault();
      if (event.key === 'ArrowUp' && index < 1) search.focus();
      else buttons[index + (event.key === 'ArrowDown' ? 1 : -1)]?.focus();
    }
  };
}

for (const [searchId, resultsId] of [['search', 'results'], ['staff-search', 'staff-results'], ['club-search', 'club-results']]) {
  const input = $<HTMLInputElement>(searchId);
  const clearSearch = (event: KeyboardEvent) => {
    if (event.key !== 'Escape' || event.isComposing || event.defaultPrevented) return;
    event.preventDefault();
    input.focus();
    if (!input.value) return;
    input.value = '';
    input.dispatchEvent(new Event('input', {bubbles:true}));
  };
  input.addEventListener('keydown', clearSearch);
  $(resultsId).addEventListener('keydown', clearSearch);
}

function validateNumbers(container: HTMLElement, extraError?: (input: HTMLInputElement) => string | undefined,
  selector = 'input[type=number][data-initial]') {
  let invalid = 0;
  for (const input of container.querySelectorAll<HTMLInputElement>(selector)) {
    const {validity} = input;
    const initial = Number(input.dataset.initial) / Number(input.dataset.scale ?? 1);
    const edited = !input.disabled && (validity.badInput || (input.dataset.initial !== undefined
      ? input.type === 'date' ? input.value !== input.dataset.initial : input.value === '' || Number(input.value) !== initial
      : input.value !== ''));
    const extra = extraError?.(input);
    const error = extra || edited && (!validity.valid || (input.type === 'number' && !Number.isInteger(inputValue(input))));
    const messageId = 'numeric-error-' + (input.id || input.dataset.field || input.dataset.date || container.id + '-' + input.name);
    let message = document.getElementById(messageId);
    if (error) {
      invalid++;
      if (!message) {
        message = document.createElement('small');
        message.id = messageId;
        message.className = 'notice numeric-error';
        message.dataset.notice = 'error';
        input.after(message);
      }
      const name = input.dataset.field ? fieldLabel(input.dataset.field) : (input.getAttribute('aria-label') ?? Array.from(input.labels?.[0]?.childNodes ?? []).filter(node=>node.nodeType===Node.TEXT_NODE).map(node=>node.textContent).join('').trim()) || t("Este campo");
      const format = new Intl.NumberFormat(locale, {maximumFractionDigits: 2});
      const limit = (bound: string) => input.type === 'date' ? bound.split('-').reverse().join('/') : format.format(Number(bound));
      const text = extra ?? (validity.rangeUnderflow || validity.rangeOverflow
        ? name + t(" deve estar entre ") + limit(input.min) + t(" e ") + limit(input.max) + '.'
        : validity.valueMissing || validity.badInput
          ? (input.type === 'date' ? t("Informe uma data válida.") : t("Informe um número para ") + name + '.')
          : name + t(" não aceita essa precisão."));
      if (message.textContent !== text) message.textContent = text;
      input.setAttribute('aria-invalid', 'true');
      const description = new Set(input.getAttribute('aria-describedby')?.split(/\s+/).filter(Boolean));
      description.add(messageId);
      input.setAttribute('aria-describedby', [...description].join(' '));
    } else if (input.hasAttribute('aria-invalid')) {
      message?.remove();
      input.removeAttribute('aria-invalid');
      const description=input.getAttribute('aria-describedby')?.split(/\s+/).filter(id=>id!==messageId).join(' ');
      if(description)input.setAttribute('aria-describedby',description);else input.removeAttribute('aria-describedby');
    }
    if (input.dataset.originalValue !== undefined) decorateAttribute(input);
  }
  return invalid;
}

function syncStaffValidation() {
  const invalid = validateNumbers($('staff-profile'));
  const button = $<HTMLButtonElement>('staff-stage');
  if (button) {
    const header=staffHeaderDraft();
    const hasChanges=!!header||staffDrafts().length>0;
    $('staff-policy-box').hidden=header?.currentAbility===undefined && staffDetail?.pendingHeader?.currentAbility==null;
    $('go-to-staff-error').hidden=!invalid;
    const state=$('staff-draft-state');
    state.classList.toggle('notice',invalid>0);
    if(invalid)state.dataset.notice='error';else delete state.dataset.notice;
    state.textContent=invalid?t("Corrija os campos destacados para continuar."):
      hasChanges?t("Há campos editados neste perfil."):t("Nenhum campo editado neste perfil.");
    button.disabled = busy || invalid > 0 || !hasChanges;
  }
  return invalid;
}

function validateClubForm(form: HTMLElement) {
  const capacity = form.querySelector<HTMLInputElement>('#stadium-capacity');
  const invalid = validateNumbers(form, input => input.id === 'stadium-seated-capacity'
    && capacity && input.valueAsNumber > capacity.valueAsNumber
    ? t("Os lugares sentados devem caber na lotação total.") : undefined);
  const button=form.querySelector<HTMLButtonElement>('button[type=submit]');
  const fields=Array.from(form.querySelectorAll<HTMLInputElement>('input[data-initial]'));
  const dirty=fields.some(isClubDraft);
  for(const field of fields)showFieldStatus(field,isClubDraft(field),field.dataset.initial!==field.dataset.recorded);
  if(form.id==='club-balance-form'){
    const draftStatus=$('club-finance-draft');
    const label=dirty?t("Rascunho de finanças"):'';
    if(draftStatus.textContent!==label)draftStatus.textContent=label;
  }
  const blocked=busy||invalid>0||!dirty;
  if(button)button.disabled=blocked;
  return blocked;
}

function syncClubValidation() {
  document.querySelectorAll<HTMLElement>('.club-fields-form').forEach(validateClubForm);
}

$('staff-profile').addEventListener('input', syncStaffValidation);
$('staff-profile').addEventListener('change', syncStaffValidation);
$('club-profile').addEventListener('input', event => {
  if (!(event.target instanceof HTMLInputElement)) return;
  const form = event.target.closest<HTMLElement>('.club-fields-form');
  if (form) validateClubForm(form);
});

function syncAuxiliaryValidation(form?: HTMLFormElement): number {
  const selectors = '#player-filter-form, #club-batch-form, #wage-batch-form, #staff-batch-form';
  if (!form) {
    for (const item of document.querySelectorAll<HTMLFormElement>(selectors)) syncAuxiliaryValidation(item);
    return 0;
  }
  let invalid = 0;
  const update = (fields: string, buttons: string, extra?: (input: HTMLInputElement) => string | undefined) => {
    const errors = validateNumbers(form, extra, fields);
    invalid += errors;
    for (const button of form.querySelectorAll<HTMLButtonElement>(buttons)) button.disabled = busy || errors > 0;
  };
  switch (form.id) {
    case 'player-filter-form':
      update('input[type=number]', 'button[type=submit]', input => {
        if (!input.name.startsWith('max') || !input.validity.valid || !input.value) return;
        const min = form.elements.namedItem('min' + input.name.slice(3)) as HTMLInputElement | null;
        if (min?.validity.valid && min.valueAsNumber > input.valueAsNumber) return t("O máximo deve ser maior ou igual ao mínimo.");
      });
      break;
    case 'club-batch-form':
      update('#batch-value', '#batch-queue-adjustment, #preview-player-adjustment');
      break;
    case 'wage-batch-form':
      update('input[type=number]', '#preview-wage-adjustment');
      update('input[type=date]', '#preview-expiry-batch');
      break;
    case 'staff-batch-form':
      update('input[type=number]', 'button[type=submit]');
  }
  return invalid;
}

document.addEventListener('input', event => {
  if (!(event.target instanceof HTMLInputElement || event.target instanceof HTMLSelectElement)) return;
  const form = event.target.form;
  if (form) syncAuxiliaryValidation(form);
});

document.addEventListener('focusout', event => {
  const input = event.target;
  if (!(input instanceof HTMLInputElement) || input.getAttribute('aria-invalid') !== 'true') return;
  const description = input.nextElementSibling;
  if (description?.matches('.numeric-error') && description.textContent) {
    notify(description.textContent, 'error');
  }
});

let validationFeedbackPending = false;
document.addEventListener('invalid', event => {
  const input = event.target;
  if (!(input instanceof HTMLInputElement || input instanceof HTMLSelectElement || input instanceof HTMLTextAreaElement)) return;
  event.preventDefault();
  if (validationFeedbackPending) return;
  validationFeedbackPending = true;
  queueMicrotask(() => {
    validationFeedbackPending = false;
    input.focus();
    const label = input.labels?.[0]?.textContent?.trim() || input.getAttribute('aria-label');
    const field = label ? '“' + label + '”' : t("o campo destacado");
    notify(input.validity.valueMissing ? t("Preencha ") + field + '.' : t("Verifique o valor em ") + field + '.', 'error');
  });
}, true);

function validatePlayerFields() {
  return validateNumbers($('profile'), input => {
    if (input.dataset.field !== 'height' || !input.validity.valid || input.value === input.dataset.initial) return;
    const height=input.valueAsNumber;
    const inches=Math.floor(height/2.54);
    const nearest=[inches,inches+1].map(value=>Math.round(value*2.54));
    if (nearest.includes(height)) return;
    const allowed=nearest.filter(value=>value>=Number(input.min)&&value<=Number(input.max));
    return t("Essa altura não pode ser salva. Use ")+allowed.join(t(" ou "))+' cm.';
  }, 'input[type=number][data-initial], input[data-date]');
}

function renderClubRoster(detail: ClubDetail) {
  const players=detail.players.map(player=>{
    const meta=[...(player.positions??[]).map(fieldLabel),player.age==null?'':player.age+t(" anos")].filter(Boolean).join(' · ');
    return '<button type="button" class="player-row" data-club-player="'+player.uid+'"><span><strong>'+escape(player.name)+'</strong>'
      +(meta?'<small>'+escape(meta)+'</small>':'')+'</span><span class="abilities" title="CA / PA">'+player.ca+' · '+player.pa+'</span></button>';
  }).join('');
  const staff=detail.staff.map(person=>'<button type="button" class="player-row" data-club-staff="'+person.uid+'"><span><strong>'
    +escape(person.name)+'</strong><small>'+escape(t(person.role))+(person.isPlayer?t(" · Jogador/staff"):'')+'</small></span></button>').join('');
  const expiries=detail.expiringContracts??[];
  const upcoming=expiries.filter(row=>row.days!==null&&row.days>=0&&row.days<=90).length;
  const contracts=renderWageBatch(detail.wageContracts??[]);
  const playerBatch=renderClubBatch(detail.players);
  return `<div class="club-roster-toolbar"><div class="club-roster-modes" role="group" aria-label="${t("Grupo do elenco")}">`
    +`<button id="club-roster-players" type="button" aria-pressed="true">${t("Jogadores ·")} `+detail.players.length+'</button>'
    +`<button id="club-roster-staff" type="button" aria-pressed="false">${t("Staff e direção ·")} `+detail.staff.length+'</button></div>'
    +`<button id="club-open-expiries" type="button" aria-expanded="false" aria-controls="club-roster-expiries">${t("Vencimentos ·")} `+upcoming+` ${t("em 90 dias")}</button>`
    +`<details id="club-tools"><summary>${t("Ferramentas")}</summary><div><button id="compare-club" type="button">${t("Comparar elenco com outro save")}</button>`
    +(playerBatch?`<button id="club-open-player-batch" type="button">${t("Editar jogadores em lote")}</button>`:'')
    +(playerBatch?`<button id="club-open-presets" type="button">${t("Presets de edição do elenco")}</button>`:'')
    +(contracts?`<button id="club-open-contract-batch" type="button">${t("Ajustar contratos em lote")}</button>`:'')
    +`<button id="club-open-shirts" type="button" hidden>${t("Trocar números de camisa")}</button></div></details></div>`
    +`<label class="search-box"><span>${t("Buscar")}</span><input id="club-roster-search" type="search" placeholder="${t("Nome do jogador")}" aria-label="${t("Buscar no elenco")}" aria-controls="club-roster-results"></label>`
    +'<div id="club-roster-results"><div id="club-roster-player-list">'+players+'</div><div id="club-roster-staff-list" hidden>'+staff+'</div></div>'
    +`<p id="club-roster-empty" class="notice" data-notice="info" role="status" hidden>${t("Nenhum resultado.")}</p>`
    +'<div id="club-roster-expiries" hidden>'+renderClubExpiries(expiries)+'</div>'
    +'<div id="club-tool-player-batch" hidden>'+playerBatch+'</div><div id="club-tool-contract-batch" hidden>'+contracts+'</div>'
    +'<div id="club-tool-shirts" hidden></div>'+renderSquadSummary(detail)
    +'<div id="club-roster-staff-tools" hidden>'+renderClubStaffComparison(detail.staff)+'</div>';
}

function bindClubRoster() {
  const input=$<HTMLInputElement>('club-roster-search');
  const results=$('club-roster-results');
  const playerList=$('club-roster-player-list'),staffList=$('club-roster-staff-list');
  const normalize=(value:string)=>value.normalize('NFD').replace(/\p{M}/gu,'').toLocaleLowerCase(locale);
  let staff=false;
  const visible=()=>Array.from((staff?staffList:playerList).querySelectorAll<HTMLButtonElement>('button:not([hidden])'));
  const filter=()=>{
    const query=normalize(input.value.trim());
    for(const row of (staff?staffList:playerList).querySelectorAll<HTMLButtonElement>('button')){
      row.hidden=!normalize(row.querySelector('strong')?.textContent??'').includes(query);
    }
    $('club-roster-empty').hidden=visible().length>0;
  };
  const select=(showStaff:boolean)=>{
    staff=showStaff;
    playerList.hidden=staff;
    staffList.hidden=!staff;
    $('club-roster-staff-tools').hidden=!staff;
    $('club-squad-summary')?.toggleAttribute('hidden',staff);
    $('club-roster-players').setAttribute('aria-pressed',String(!staff));
    $('club-roster-staff').setAttribute('aria-pressed',String(staff));
    input.placeholder=staff?t("Nome do membro do staff"):t("Nome do jogador");
    input.value='';
    filter();
  };
  $('club-roster-players').onclick=()=>select(false);
  $('club-roster-staff').onclick=()=>select(true);
  input.oninput=filter;
  const navigate=(event:KeyboardEvent)=>{
    const rows=visible();
    const index=rows.indexOf(event.target as HTMLButtonElement);
    if(event.key==='Escape'){event.preventDefault();input.value='';filter();input.focus();}
    else if(event.key==='ArrowDown'){event.preventDefault();rows[index+1]?.focus();}
    else if(event.key==='ArrowUp'&&index>=0){event.preventDefault();(rows[index-1]??input).focus();}
    else if(event.key==='Enter'&&event.target===input){event.preventDefault();rows[0]?.click();}
  };
  input.onkeydown=navigate;
  results.onkeydown=navigate;
  const expiry=$('club-roster-expiries');
  $('club-open-expiries').onclick=()=>{
    expiry.hidden=!expiry.hidden;
    $('club-open-expiries').setAttribute('aria-expanded',String(!expiry.hidden));
    expiry.querySelector('details')!.open=!expiry.hidden;
    if(!expiry.hidden)expiry.querySelector('select')?.focus();
  };
  const tools=$<HTMLDetailsElement>('club-tools');
  const toolNames=['player-batch','contract-batch','shirts'];
  for(const name of toolNames){
    const button=$('club-open-'+name);
    if(!button)continue;
    button.onclick=()=>{
      const panel=$('club-tool-'+name);
      const open=panel.hasAttribute('hidden');
      for(const key of toolNames)$('club-tool-'+key).hidden=key!==name||!open;
      tools.open=false;
      if(open){
        const section=panel.querySelector('details');
        if(section)section.open=true;
        panel.scrollIntoView({block:'nearest'});
      }
    };
  }
  const presets=$<HTMLDetailsElement>('club-tool-player-batch').querySelector<HTMLDetailsElement>('#batch-presets');
  if(presets)$('club-open-presets').onclick=()=>{
    if($('club-tool-player-batch').hidden)$('club-open-player-batch').click();
    tools.open=false;
    $('batch-tab-adjust').click();
    $('batch-preset').focus();
  };
  select(false);
}
