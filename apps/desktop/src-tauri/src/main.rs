#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cm0304_core::{
    batch,
    career::{self, ContractView},
    container::hash,
    database::{self, Date, Player},
    editor::Edit,
    health::{self, InjuryView},
    search,
    staff::{self, Staff},
    tactics::{self, ClubTactic},
    transaction::{self, StadiumCapacities, Transaction},
};
use serde::Serialize;
mod my_club;
mod updates;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
};
use tauri::{Manager, State};
use tauri_plugin_dialog::DialogExt;
type Result<T> = std::result::Result<T, String>;

struct Loaded {
    source: PathBuf,
    bytes: Arc<Vec<u8>>,
    players: Vec<Player>,
    clubs: Vec<cm0304_core::clubs::ClubProfile>,
    managed_squads: BTreeMap<i32, Vec<i32>>,
    club_info: BTreeMap<i32, ClubInfo>,
    club_stadiums: BTreeMap<i32, cm0304_core::clubs::ClubStadium>,
    club_contract_dates: BTreeMap<i32, Vec<ClubContractDate>>,
    club_loan_wages: BTreeMap<i32, Vec<ClubLoanWages>>,
    club_tactics: Result<BTreeMap<i32, Result<ClubTactic>>>,
    nations: Vec<String>,
    game_date: Date,
    search_keys: Vec<String>,
    staff: Vec<Staff>,
    staff_search_keys: Vec<String>,
    contracts: BTreeMap<i32, Vec<ContractView>>,
    market_sources: BTreeMap<i32, u8>,
    market_journal_supported: bool,
    injuries: BTreeMap<i32, Vec<InjuryView>>,
    discipline: BTreeMap<i32, Discipline>,
}
#[derive(Default)]
struct Session {
    loaded: Option<Arc<Loaded>>,
    summary: Option<Summary>,
    plan: Transaction,
    undo: Vec<Transaction>,
    redo: Vec<Transaction>,
    batch_preview: Option<PreparedBatch>,
    latest_batch_id: Option<u64>,
    last_saved_copy: Option<PathBuf>,
}
type Shared = Mutex<Session>;
#[derive(Default)]
struct SaveComparisonTask(Mutex<Option<Arc<AtomicBool>>>);
#[derive(Default)]
struct OpenSaveTask(Mutex<Option<Arc<AtomicBool>>>);
#[derive(Default)]
struct SaveCopyTask(Mutex<Option<Arc<AtomicBool>>>);

impl SaveCopyTask {
    fn is_active(&self) -> bool {
        self.0.lock().map_or(true, |active| active.is_some())
    }

    fn cancel(&self) -> Result<()> {
        if let Some(cancelled) = self.0.lock().map_err(|e| e.to_string())?.as_ref() {
            cancelled.store(true, Ordering::Relaxed);
        }
        Ok(())
    }
}

struct ClubLoanWages {
    uid: i32,
    owner_index: usize,
    loan_index: usize,
    owner_wage: i32,
    loan_wage: i32,
}

impl ClubLoanWages {
    fn contribution_delta(&self, contract_edits: &BTreeMap<(i32, usize, &str), i32>) -> i64 {
        let wage = |index, original| {
            contract_edits
                .get(&(self.uid, index, "weeklyWage"))
                .copied()
                .unwrap_or(original)
        };
        career::loan_wage_contribution(
            wage(self.owner_index, self.owner_wage),
            wage(self.loan_index, self.loan_wage),
        ) - career::loan_wage_contribution(self.owner_wage, self.loan_wage)
    }
}

#[test]
fn planned_loan_wages_change_the_owning_clubs_offset_up_to_the_owners_wage() {
    let pair = ClubLoanWages {
        uid: 100,
        owner_index: 0,
        loan_index: 2,
        owner_wage: 1_000,
        loan_wage: 400,
    };
    let mut edits = BTreeMap::from([((100, 2, "weeklyWage"), 1_200)]);
    assert_eq!(pair.contribution_delta(&edits), 600);
    edits.insert((100, 0, "weeklyWage"), 800);
    assert_eq!(pair.contribution_delta(&edits), 400);
    edits.clear();
    edits.insert((101, 2, "weeklyWage"), 500);
    assert_eq!(pair.contribution_delta(&edits), 0);
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct Summary {
    name: String,
    path: String,
    players: usize,
    clubs: usize,
    staff: usize,
    sha256: String,
    managed_clubs: Vec<my_club::ManagedClub>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Row {
    uid: i32,
    name: String,
    club: Option<String>,
    age: Option<u16>,
    ca: i16,
    pa: i16,
}
#[derive(Serialize)]
struct Page {
    total: usize,
    rows: Vec<Row>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct StaffRow {
    uid: i32,
    name: String,
    team: Option<String>,
    role: String,
    national_team: Option<String>,
    age: Option<u16>,
    ca: i16,
    pa: i16,
    is_player: bool,
}
#[derive(Serialize)]
struct StaffPage {
    total: usize,
    rows: Vec<StaffRow>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct StaffDetail {
    profile: Staff,
    recorded: Option<Staff>,
    age: Option<u16>,
    pending_player_fields: usize,
    editable: Vec<String>,
    pending_header: Option<cm0304_core::staff_header::Edit>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Detail {
    player: Player,
    effective: Player,
    age: Option<u16>,
    contracts: Vec<ContractView>,
    market_targets: Vec<u8>,
    market_journal_supported: bool,
    editable: Vec<String>,
    pending: Vec<Edit>,
    nations: Vec<String>,
    injuries: Vec<InjuryView>,
    heal_pending: bool,
    suspension_count: usize,
    booking_record_count: usize,
    suspension_pending: bool,
}

#[derive(Clone, Copy, Default)]
struct Discipline {
    suspensions: usize,
    booking_records: usize,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Pending {
    club_transfer_budget_field_count: usize,
    edits: Vec<Edit>,
    staff_edits: Vec<cm0304_core::staff_editor::Edit>,
    club_balances: BTreeMap<i32, i32>,
    club_wage_budgets: BTreeMap<i32, i32>,
    club_stadium_capacities: BTreeMap<i32, StadiumCapacities>,
    club_stadium_field_count: usize,
    club_transfer_budgets: BTreeMap<i32, transaction::TransferBudgets>,
    club_training: BTreeMap<i32, u8>,
    club_reputation: BTreeMap<i32, i16>,
    club_youth_setup: BTreeMap<i32, u8>,
    staff_header_edits: Vec<cm0304_core::staff_header::Edit>,
    heal_injuries: Vec<i32>,
    clear_suspensions: Vec<i32>,
    undo: bool,
    redo: bool,
    undo_operation: Option<ReviewOperation>,
    redo_operation: Option<ReviewOperation>,
    operations: Vec<ReviewOperation>,
    review_clubs: ReviewClubs,
}

#[derive(Default, Serialize)]
struct ReviewClubs {
    players: BTreeMap<i32, ReviewClub>,
    staff: BTreeMap<i32, ReviewClub>,
}

#[derive(Clone, Serialize)]
struct ReviewClub {
    uid: i32,
    name: String,
}

fn review_clubs(session: &Session) -> ReviewClubs {
    let Some(data) = &session.loaded else {
        return ReviewClubs::default();
    };
    let player_uids = session
        .plan
        .edits
        .iter()
        .map(|edit| edit.uid)
        .chain(session.plan.market_statuses.keys().copied())
        .chain(session.plan.heal_injuries.iter().copied())
        .chain(session.plan.clear_suspensions.iter().copied())
        .collect::<std::collections::BTreeSet<_>>();
    let staff_uids = session
        .plan
        .staff_edits
        .iter()
        .map(|edit| edit.uid)
        .chain(session.plan.staff_header_edits.iter().map(|edit| edit.uid))
        .chain(player_uids.iter().copied())
        .collect::<std::collections::BTreeSet<_>>();
    let mut clubs = ReviewClubs::default();
    for club in &data.clubs {
        let entry = || ReviewClub {
            uid: club.uid,
            name: club.name.clone(),
        };
        for uid in club
            .player_uids
            .iter()
            .filter(|uid| player_uids.contains(uid))
        {
            clubs.players.insert(*uid, entry());
        }
        for uid in club
            .staff_uids
            .iter()
            .filter(|uid| staff_uids.contains(uid))
        {
            clubs.staff.insert(*uid, entry());
        }
    }
    clubs
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ReviewOperation {
    index: usize,
    domains: Vec<&'static str>,
    player_count: usize,
    player_fields: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    staff_fields: Vec<String>,
}

const MARKET_FIELD: &str = "contract.0.transferStatus";

fn market_edit(uid: i32, value: u8) -> Edit {
    Edit {
        uid,
        attribute: MARKET_FIELD.into(),
        value: value.into(),
        policy: None,
    }
}

fn review_operations(session: &Session) -> Vec<ReviewOperation> {
    session
        .undo
        .iter()
        .zip(
            session
                .undo
                .iter()
                .skip(1)
                .chain(std::iter::once(&session.plan)),
        )
        .enumerate()
        .filter_map(|(index, (before, after))| describe_operation(index, before, after))
        .collect()
}

fn describe_operation(
    index: usize,
    before: &Transaction,
    after: &Transaction,
) -> Option<ReviewOperation> {
    let domains = [
        (
            before.edits != after.edits || before.market_statuses != after.market_statuses,
            "Jogadores e contratos",
        ),
        (
            before.staff_edits != after.staff_edits
                || before.staff_header_edits != after.staff_header_edits,
            "Comissão",
        ),
        (
            before.heal_injuries != after.heal_injuries
                || before.clear_suspensions != after.clear_suspensions,
            "Lesões e suspensões",
        ),
        (
            before.club_balances != after.club_balances
                || before.club_wage_budgets != after.club_wage_budgets
                || before.club_stadium_capacities != after.club_stadium_capacities
                || before.club_transfer_budgets != after.club_transfer_budgets
                || before.club_training != after.club_training
                || before.club_reputation != after.club_reputation
                || before.club_youth_setup != after.club_youth_setup,
            "Clubes",
        ),
    ]
    .into_iter()
    .filter_map(|(changed, label)| changed.then_some(label))
    .collect::<Vec<_>>();
    if domains.is_empty() {
        return None;
    }
    let mut player_uids = std::collections::BTreeSet::new();
    let mut player_fields = std::collections::BTreeSet::new();
    for &uid in before
        .market_statuses
        .keys()
        .chain(after.market_statuses.keys())
    {
        if before.market_statuses.get(&uid) != after.market_statuses.get(&uid) {
            player_uids.insert(uid);
            player_fields.insert(MARKET_FIELD.to_owned());
        }
    }
    if before.edits != after.edits {
        let old = before
            .edits
            .iter()
            .map(|edit| ((edit.uid, edit.attribute.as_str()), edit))
            .collect::<BTreeMap<_, _>>();
        let new = after
            .edits
            .iter()
            .map(|edit| ((edit.uid, edit.attribute.as_str()), edit))
            .collect::<BTreeMap<_, _>>();
        for (uid, field) in old.keys().chain(new.keys()).copied() {
            if old.get(&(uid, field)) != new.get(&(uid, field)) {
                player_uids.insert(uid);
                player_fields.insert(field.to_owned());
            }
        }
    }
    Some(ReviewOperation {
        index,
        domains,
        player_count: player_uids.len(),
        player_fields: player_fields.into_iter().collect(),
        staff_fields: Vec::new(),
    })
}

fn describe_history_operation(
    index: usize,
    before: &Transaction,
    after: &Transaction,
) -> Option<ReviewOperation> {
    let mut operation = describe_operation(index, before, after)?;
    if operation.domains == ["Comissão"] && before.staff_header_edits == after.staff_header_edits {
        let old = before
            .staff_edits
            .iter()
            .map(|edit| ((edit.uid, edit.attribute.as_str()), edit))
            .collect::<BTreeMap<_, _>>();
        let new = after
            .staff_edits
            .iter()
            .map(|edit| ((edit.uid, edit.attribute.as_str()), edit))
            .collect::<BTreeMap<_, _>>();
        operation.staff_fields = old
            .keys()
            .chain(new.keys())
            .filter(|key| old.get(key) != new.get(key))
            .map(|(_, field)| (*field).to_owned())
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect();
    }
    Some(operation)
}

#[cfg(test)]
#[test]
fn review_describes_only_the_player_fields_changed_by_each_operation() {
    let edit = |uid, attribute: &str, value| Edit {
        uid,
        attribute: attribute.into(),
        value,
        policy: None,
    };
    let initial = Transaction::default();
    let first = Transaction {
        edits: vec![edit(7, "passing", 16), edit(8, "passing", 16)],
        ..Default::default()
    };
    let mut second = first.clone();
    second.edits.push(edit(7, "state.morale", 20));
    let mut third = second.clone();
    third.market_statuses.insert(8, 6);
    let session = Session {
        undo: vec![initial, first, second],
        plan: third,
        ..Default::default()
    };
    let operations = review_operations(&session);
    assert_eq!(operations.len(), 3);
    assert_eq!(operations[0].player_count, 2);
    assert_eq!(operations[0].player_fields, ["passing"]);
    assert_eq!(operations[1].player_count, 1);
    assert_eq!(operations[1].player_fields, ["state.morale"]);
    assert_eq!(operations[2].domains, ["Jogadores e contratos"]);
    assert_eq!(operations[2].player_count, 1);
    assert_eq!(operations[2].player_fields, [MARKET_FIELD]);
}

#[cfg(test)]
#[test]
fn pending_describes_top_undo_and_redo_operations() {
    let edit = |attribute: &str| cm0304_core::staff_editor::Edit {
        uid: 7,
        attribute: attribute.into(),
        value: 16,
    };
    let initial = Transaction::default();
    let first = Transaction {
        staff_edits: vec![edit("physiotherapy")],
        ..Default::default()
    };
    let second = Transaction {
        staff_edits: vec![edit("physiotherapy"), edit("discipline")],
        ..Default::default()
    };
    let state = pending(&Session {
        undo: vec![initial],
        plan: first,
        redo: vec![second],
        ..Default::default()
    });
    let undo = state.undo_operation.unwrap();
    let redo = state.redo_operation.unwrap();
    assert_eq!(undo.domains, ["Comissão"]);
    assert_eq!(undo.staff_fields, ["physiotherapy"]);
    assert_eq!(redo.staff_fields, ["discipline"]);
}
fn pending(s: &Session) -> Pending {
    let club_transfer_budget_field_count = s
        .plan
        .club_transfer_budgets
        .iter()
        .filter_map(|(uid, targets)| {
            let original = s.loaded.as_ref()?.club_info.get(uid)?;
            Some(
                usize::from(original.transfer_budget_season != Some(targets.season))
                    + usize::from(original.transfer_budget_remaining != Some(targets.remaining)),
            )
        })
        .sum();
    Pending {
        club_transfer_budget_field_count,
        club_stadium_field_count: s
            .plan
            .club_stadium_capacities
            .iter()
            .filter_map(|(uid, target)| {
                let source = &s.loaded.as_ref()?.club_stadiums.get(uid)?.stadium;
                Some(
                    usize::from(source.capacity != target.capacity)
                        + usize::from(source.seated_capacity != target.seated_capacity),
                )
            })
            .sum(),
        edits: s
            .plan
            .edits
            .iter()
            .cloned()
            .chain(
                s.plan
                    .market_statuses
                    .iter()
                    .map(|(&uid, &value)| market_edit(uid, value)),
            )
            .collect(),
        staff_edits: s.plan.staff_edits.clone(),
        club_balances: s.plan.club_balances.clone(),
        club_wage_budgets: s.plan.club_wage_budgets.clone(),
        club_stadium_capacities: s.plan.club_stadium_capacities.clone(),
        club_transfer_budgets: s.plan.club_transfer_budgets.clone(),
        club_training: s.plan.club_training.clone(),
        club_reputation: s.plan.club_reputation.clone(),
        club_youth_setup: s.plan.club_youth_setup.clone(),
        staff_header_edits: s.plan.staff_header_edits.clone(),
        heal_injuries: s.plan.heal_injuries.iter().copied().collect(),
        clear_suspensions: s.plan.clear_suspensions.iter().copied().collect(),
        undo: !s.undo.is_empty(),
        redo: !s.redo.is_empty(),
        undo_operation: s
            .undo
            .last()
            .and_then(|before| describe_history_operation(s.undo.len() - 1, before, &s.plan)),
        redo_operation: s
            .redo
            .last()
            .and_then(|after| describe_history_operation(s.undo.len(), &s.plan, after)),
        operations: review_operations(s),
        review_clubs: review_clubs(s),
    }
}
fn record(s: &mut Session) {
    s.batch_preview = None;
    s.latest_batch_id = None;
    s.undo.push(s.plan.clone());
    s.redo.clear();
}
fn loaded(state: &State<Shared>) -> Result<Arc<Loaded>> {
    state
        .lock()
        .map_err(|e| e.to_string())?
        .loaded
        .clone()
        .ok_or("Abra um save primeiro".into())
}

fn save_name(path: &Path) -> std::borrow::Cow<'_, str> {
    path.file_name()
        .unwrap_or(path.as_os_str())
        .to_string_lossy()
}

fn save_io_error(path: &Path, error: std::io::Error) -> String {
    format!(
        "Não foi possível ler o arquivo {}. Detalhe: {error}",
        save_name(path)
    )
}

fn parse_save(
    path: &Path,
    bytes: &[u8],
    cancelled: impl Fn() -> bool,
) -> Result<(database::Database, usize)> {
    let name = save_name(path);
    database::load_with_cancel(bytes, &cancelled).map_err(|error| {
        format!(
            "Não foi possível abrir {name} como um save compatível. Selecione um save .cm4 do CM0304 4.1.5 no formato sem compactação. Detalhe: {error}"
        )
    })
}

#[tauri::command]
async fn pick_save_file(app: tauri::AppHandle) -> Result<Option<PathBuf>> {
    tauri::async_runtime::spawn_blocking(move || {
        app.dialog()
            .file()
            .add_filter("CM0304", &["cm4"])
            .blocking_pick_file()
            .map(|file| file.into_path().map_err(|error| error.to_string()))
            .transpose()
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
async fn open_save(
    state: State<'_, Shared>,
    task: State<'_, OpenSaveTask>,
    recent_path: PathBuf,
) -> Result<Option<Summary>> {
    let cancelled = Arc::new(AtomicBool::new(false));
    {
        let mut active = task.0.lock().map_err(|e| e.to_string())?;
        if active.is_some() {
            return Err("Já existe uma abertura de save em andamento".into());
        }
        *active = Some(Arc::clone(&cancelled));
    }
    let worker_cancelled = Arc::clone(&cancelled);
    let result =
        tauri::async_runtime::spawn_blocking(move || -> Result<Option<(Arc<Loaded>, Summary)>> {
            let source = {
                let path = recent_path;
                if !path.is_absolute()
                    || path
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .is_none_or(|ext| !ext.eq_ignore_ascii_case("cm4"))
                {
                    return Err("Caminho do save inválido".into());
                }
                path
            };
            let is_cancelled = || worker_cancelled.load(Ordering::Relaxed);
            if is_cancelled() {
                return Ok(None);
            }
            let file =
                std::fs::File::open(&source).map_err(|error| save_io_error(&source, error))?;
            let Some(bytes) = read_save_bytes(file, &worker_cancelled)
                .map_err(|error| save_io_error(&source, error))?
            else {
                return Ok(None);
            };
            let (db, base) = parse_save(&source, &bytes, is_cancelled)?;
            if is_cancelled() {
                return Ok(None);
            }
            let players = db.players(base)?;
            if is_cancelled() {
                return Ok(None);
            }
            let clubs = cm0304_core::clubs::profiles(&db)?;
            let club_tactics = tactics::clubs_with_current_tactics(&bytes, &db);
            let club_stadiums = cm0304_core::clubs::stadiums_by_club(&db);
            let player_wages = career::player_wages_by_club(&db);
            let mut club_loan_wages = BTreeMap::<i32, Vec<ClubLoanWages>>::new();
            for (club, uid, (owner_index, owner), (loan_index, loan)) in
                career::player_loan_wage_pairs(&db)
            {
                club_loan_wages
                    .entry(db.clubs[club].uid)
                    .or_default()
                    .push(ClubLoanWages {
                        uid,
                        owner_index,
                        loan_index,
                        owner_wage: owner.weekly_wage,
                        loan_wage: loan.weekly_wage,
                    });
            }
            let mut reputations = BTreeMap::new();
            for team in db
                .teams
                .iter()
                .filter(|t| t.container_type == 0 && t.squad == 0)
            {
                reputations
                    .entry(team.container_index)
                    .and_modify(|value| *value = None)
                    .or_insert(Some(team.reputation));
            }
            let player_names = players
                .iter()
                .map(|p| (p.uid, p.name.as_str()))
                .collect::<BTreeMap<_, _>>();
            let mut transfer_installments = BTreeMap::<usize, Vec<ClubTransferInstallment>>::new();
            let transfer_references = cm0304_core::transfer_lab::inspect_references(&bytes);
            let installments_readable = transfer_references.is_ok();
            if let Ok(references) = transfer_references {
                for row in references {
                    let source = row.source_club_index as usize;
                    let destination = row.destination_club_index as usize;
                    let (Some(seller), Some(buyer)) =
                        (db.clubs.get(source), db.clubs.get(destination))
                    else {
                        continue;
                    };
                    let player_name = db
                        .people
                        .get(row.person_id as usize)
                        .filter(|person| person.id as u32 == row.person_id)
                        .and_then(|person| player_names.get(&person.uid))
                        .map(|name| (*name).to_owned());
                    for (club, counterparty, outgoing) in
                        [(source, buyer, false), (destination, seller, true)]
                    {
                        transfer_installments.entry(club).or_default().push(
                            ClubTransferInstallment {
                                counterparty: counterparty.name.clone(),
                                player_name: player_name.clone(),
                                person_id: row.person_id,
                                amount_raw: row.installment_amount_raw,
                                remaining: row.installments_remaining,
                                outgoing,
                            },
                        );
                    }
                }
            }
            let club_info = db
                .clubs
                .iter()
                .enumerate()
                .filter_map(|(index, club)| {
                    let d = club.details.as_ref()?;
                    Some((
                        club.uid,
                        ClubInfo {
                            balance: d.balance,
                            reputation: reputations.get(&(index as i32)).copied().flatten(),
                            loan_wage_offset_weekly: club_loan_wages
                                .get(&club.uid)
                                .into_iter()
                                .flatten()
                                .map(|pair| {
                                    career::loan_wage_contribution(pair.owner_wage, pair.loan_wage)
                                })
                                .sum(),
                            player_contract_wages_weekly: player_wages
                                .get(&index)
                                .copied()
                                .unwrap_or_default(),
                            transfer_budget_season: d.transfer_budget_season,
                            transfer_budget_remaining: d.transfer_budget_remaining,
                            balance_editable: d.finance_kind == 1,
                            payroll_expenses: d.payroll_expenses,
                            finance_summary: d.finance_summary,
                            transfer_installments: installments_readable
                                .then(|| transfer_installments.remove(&index).unwrap_or_default()),
                            sell_on_clauses: d
                                .sell_on_clauses
                                .iter()
                                .map(|clause| ClubSellOnClause {
                                    player_uid: clause.player_uid,
                                    player_name: player_names
                                        .get(&clause.player_uid)
                                        .map(|name| (*name).to_owned()),
                                    beneficiary: db
                                        .clubs
                                        .get(clause.beneficiary_club_index as usize)
                                        .map(|club| club.name.clone()),
                                    percent: clause.percent,
                                })
                                .collect(),
                            current_wages_weekly: d.current_wages_weekly,
                            wage_budget_weekly: d.wage_budget_weekly,
                            training_raw: d.training_raw,
                            youth_setup_raw: d.youth_setup_raw,
                            average_attendance: d.average_attendance,
                        },
                    ))
                })
                .collect();
            drop(reputations);
            drop(player_wages);
            let mut club_contract_dates = BTreeMap::<i32, Vec<ClubContractDate>>::new();

            for person in &db.people {
                let Some(name) = player_names.get(&person.uid) else {
                    continue;
                };
                for (index, contract) in person.contracts.iter().enumerate() {
                    if contract.entity_type != 0 || !matches!(contract.kind, 0 | 1) {
                        continue;
                    }
                    let Some(club) = usize::try_from(contract.entity_index)
                        .ok()
                        .and_then(|index| db.clubs.get(index))
                    else {
                        continue;
                    };
                    club_contract_dates
                        .entry(club.uid)
                        .or_default()
                        .push(ClubContractDate {
                            uid: person.uid,
                            name: (*name).to_owned(),
                            started: contract.started.clone(),
                            expires: contract.expires.clone(),
                            index,
                            loan: contract.kind == 1,
                            weekly_wage: contract.weekly_wage,
                            squad_status: contract.squad_status,
                            squad_number: contract.squad_number,
                        });
                }
            }
            drop(player_names);

            if is_cancelled() {
                return Ok(None);
            }
            let staff = staff::profiles(&db, &bytes, base)
                .map_err(|error| format!("Falha ao ler staff: {error}"))?;
            let staff_search_keys = staff
                .iter()
                .map(|p| {
                    search::key(&format!(
                        "{} {} {} {} {} {}",
                        p.uid,
                        p.name,
                        p.team.as_deref().unwrap_or(""),
                        p.club_job,
                        p.national_team.as_deref().unwrap_or(""),
                        p.national_job
                    ))
                })
                .collect();
            let mut contracts: BTreeMap<_, _> = db
                .people
                .iter()
                .filter(|p| p.playing.is_some())
                .map(|p| (p.uid, career::contracts(&db, p)))
                .collect();
            contracts.retain(|_, contracts| !contracts.is_empty());
            let market_sources = cm0304_core::market_lab::editable_statuses(&db, &bytes);
            let market_journal_supported = cm0304_core::market_lab::supports_journal(&bytes);
            let nations = db.nations.iter().map(|n| n.name.clone()).collect();
            let mut injuries: BTreeMap<_, _> = db
                .people
                .iter()
                .filter(|p| p.playing.is_some())
                .map(|p| (p.uid, health::injuries(&db, p)))
                .collect();
            injuries.retain(|_, injuries| !injuries.is_empty());
            let discipline = db
                .people
                .iter()
                .filter_map(|p| {
                    p.playing.as_ref().map(|v| {
                        (
                            p.uid,
                            Discipline {
                                suspensions: v.health_records.bans.len(),
                                booking_records: v.health_records.bookings.len(),
                            },
                        )
                    })
                })
                .collect();
            let game_date = db.game_date.clone();
            let managed_squads = my_club::managed_squads(&db);
            let managed_clubs = clubs
                .iter()
                .filter(|club| managed_squads.contains_key(&club.uid))
                .map(|club| my_club::ManagedClub {
                    uid: club.uid,
                    name: club.name.clone(),
                })
                .collect();
            drop(db);
            let search_keys = players
                .iter()
                .map(|p| {
                    search::key(&format!(
                        "{} {} {} {} {}",
                        p.uid,
                        p.name,
                        p.first_name,
                        p.surname,
                        p.club.as_deref().unwrap_or("")
                    ))
                })
                .collect();
            let summary = Summary {
                name: source
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into(),
                path: source.display().to_string(),
                players: players.len(),
                clubs: clubs.len(),
                staff: staff.len(),
                sha256: hash(&bytes),
                managed_clubs,
            };
            Ok(Some((
                Arc::new(Loaded {
                    source,
                    bytes: Arc::new(bytes),
                    players,
                    clubs,
                    managed_squads,
                    club_info,
                    club_stadiums,
                    club_contract_dates,
                    club_loan_wages,
                    club_tactics,
                    nations,
                    game_date,
                    search_keys,
                    staff,
                    staff_search_keys,
                    contracts,
                    market_sources,
                    market_journal_supported,
                    injuries,
                    discipline,
                }),
                summary,
            )))
        })
        .await;
    let mut active = task.0.lock().map_err(|e| e.to_string())?;
    *active = None;
    if cancelled.load(Ordering::Relaxed) {
        return Ok(None);
    }
    let result = result.map_err(|e| e.to_string())??;
    if let Some((data, summary)) = result {
        let mut s = state.lock().map_err(|e| e.to_string())?;
        *s = Session {
            loaded: Some(data),
            summary: Some(summary.clone()),
            ..Default::default()
        };
        Ok(Some(summary))
    } else {
        Ok(None)
    }
}

#[tauri::command]
fn resume_editor(state: State<'_, Shared>) -> Result<Option<(Summary, Pending)>> {
    let session = state.lock().map_err(|error| error.to_string())?;
    Ok(session
        .summary
        .clone()
        .map(|summary| (summary, pending(&session))))
}

#[tauri::command]
fn cancel_open_save(task: State<OpenSaveTask>) -> Result<()> {
    if let Some(cancelled) = task.0.lock().map_err(|e| e.to_string())?.as_ref() {
        cancelled.store(true, Ordering::Relaxed);
    }
    Ok(())
}
#[tauri::command]
fn search_staff(query: String, page: usize, state: State<Shared>) -> Result<StaffPage> {
    let data = loaded(&state)?;
    let q = search::key(query.trim());
    let matches = data
        .staff
        .iter()
        .zip(&data.staff_search_keys)
        .filter(|(_, key)| key.contains(&q))
        .map(|(p, _)| p)
        .collect::<Vec<_>>();
    Ok(StaffPage {
        total: matches.len(),
        rows: matches
            .into_iter()
            .skip(page.saturating_mul(50))
            .take(50)
            .map(|p| StaffRow {
                uid: p.uid,
                name: p.name.clone(),
                team: p.team.clone(),
                role: if p.team.is_none() && p.national_team.is_some() {
                    p.national_job
                } else {
                    p.club_job
                }
                .to_owned(),
                national_team: p.national_team.clone(),
                age: p
                    .date_of_birth
                    .as_ref()
                    .and_then(|d| d.age_on(&data.game_date)),
                ca: p.current_ability,
                pa: p.potential_ability,
                is_player: p.is_player,
            })
            .collect(),
    })
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ClubRow {
    uid: i32,
    name: String,
    short_name: String,
    players: usize,
    staff: usize,
}
#[derive(Serialize)]
struct ClubPage {
    total: usize,
    rows: Vec<ClubRow>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ClubStaff {
    uid: i32,
    name: String,
    role: String,
    is_player: bool,
    attributes: BTreeMap<&'static str, i8>,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClubInfo {
    payroll_expenses: Option<cm0304_core::database::PayrollExpenses>,
    finance_summary: Option<cm0304_core::database::FinanceSummary>,
    sell_on_clauses: Vec<ClubSellOnClause>,
    transfer_installments: Option<Vec<ClubTransferInstallment>>,
    current_wages_weekly: Option<i32>,
    wage_budget_weekly: Option<i32>,
    reputation: Option<i16>,
    balance: Option<i32>,
    balance_editable: bool,
    player_contract_wages_weekly: i64,
    loan_wage_offset_weekly: i64,
    transfer_budget_season: Option<i32>,
    transfer_budget_remaining: Option<i32>,
    training_raw: u8,
    youth_setup_raw: u8,
    average_attendance: i32,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClubSellOnClause {
    player_uid: i32,
    player_name: Option<String>,
    beneficiary: Option<String>,
    percent: u8,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClubTransferInstallment {
    counterparty: String,
    player_name: Option<String>,
    person_id: u32,
    amount_raw: u32,
    remaining: u8,
    outgoing: bool,
}
struct ClubContractDate {
    uid: i32,
    name: String,
    started: Date,
    expires: Date,
    index: usize,
    loan: bool,
    weekly_wage: i32,
    squad_status: i8,
    squad_number: i8,
}

#[derive(Serialize)]
struct ClubExpiry {
    uid: i32,
    index: usize,
    name: String,
    expires: Date,
    days: Option<i32>,
    loan: bool,
}

fn date_ordinal(date: &Date) -> Option<i32> {
    date.month_day()?;
    let y = i32::from(date.year).checked_sub(1)?;
    (y >= 0).then(|| y * 365 + y / 4 - y / 100 + y / 400 + i32::from(date.day))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ClubDetail {
    stadium: Option<cm0304_core::clubs::ClubStadium>,
    projected_player_payroll_weekly: Option<i64>,
    info: Option<ClubInfo>,
    expiring_contracts: Vec<ClubExpiry>,
    analysis_loans: Vec<ClubAnalysisLoan>,
    analysis_tactic: Option<ClubTactic>,
    analysis_tactic_unsupported: bool,
    wage_contracts: Vec<ClubWageContract>,
    profile: cm0304_core::clubs::ClubProfile,
    players: Vec<ClubSquadPlayer>,
    attribute_averages: BTreeMap<&'static str, f64>,
    staff: Vec<ClubStaff>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ClubAnalysisLoan {
    uid: i32,
    index: usize,
    expires: Option<Date>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ClubWageContract {
    uid: i32,
    index: usize,
    name: String,
    loan: bool,
    weekly_wage: i32,
    squad_status: i32,
    squad_number: i32,
}

#[derive(Serialize)]
struct ClubSquadPlayer {
    #[serde(flatten)]
    row: Row,
    positions: Vec<&'static str>,
    familiarity: BTreeMap<&'static str, i8>,
    condition: Option<i32>,
    #[serde(flatten)]
    health: BatchHealth,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct BatchHealth {
    injury_count: usize,
    suspension_count: usize,
}

impl BatchHealth {
    fn from_save(data: &Loaded, uid: i32) -> Self {
        Self {
            injury_count: data.injuries.get(&uid).map_or(0, Vec::len),
            suspension_count: data
                .discipline
                .get(&uid)
                .map_or(0, |record| record.suspensions),
        }
    }
}

#[tauri::command]
fn search_clubs(query: String, page: usize, state: State<Shared>) -> Result<ClubPage> {
    let data = loaded(&state)?;
    let q = search::key(query.trim());
    let uid = q
        .strip_prefix("uid:")
        .map(|n| n.parse::<i32>().map_err(|_| "UID de clube inválido"))
        .transpose()?;
    let found = data
        .clubs
        .iter()
        .filter(|c| {
            uid.map_or_else(
                || search::key(&format!("{} {} {}", c.uid, c.name, c.short_name)).contains(&q),
                |u| c.uid == u,
            )
        })
        .collect::<Vec<_>>();
    Ok(ClubPage {
        total: found.len(),
        rows: found
            .into_iter()
            .skip(page.saturating_mul(50))
            .take(50)
            .map(|c| ClubRow {
                uid: c.uid,
                name: c.name.clone(),
                short_name: c.short_name.clone(),
                players: c.player_uids.len(),
                staff: c.staff_uids.len(),
            })
            .collect(),
    })
}
fn project_club_expiries(
    records: &[ClubContractDate],
    game_date: &Date,
    edits: &[Edit],
) -> Vec<ClubExpiry> {
    let dates = edits
        .iter()
        .filter_map(|edit| {
            let (index, field) = edit
                .attribute
                .strip_prefix("contract.")?
                .split_once(".expires.")?;
            let index = index.parse::<usize>().ok()?;
            let value = u16::try_from(edit.value).ok()?;
            Some(((edit.uid, index, field), value))
        })
        .collect::<BTreeMap<_, _>>();
    let today = date_ordinal(game_date);
    let mut expiring_contracts = records
        .iter()
        .filter_map(|record| {
            let expires = Date {
                year: dates
                    .get(&(record.uid, record.index, "year"))
                    .copied()
                    .unwrap_or(record.expires.year),
                day: dates
                    .get(&(record.uid, record.index, "day"))
                    .copied()
                    .unwrap_or(record.expires.day),
            };
            let days = if career::is_monthly(&expires) {
                None
            } else {
                Some(date_ordinal(&expires)? - today?)
            };
            Some(ClubExpiry {
                uid: record.uid,
                index: record.index,
                name: record.name.clone(),
                expires,
                days,
                loan: record.loan,
            })
        })
        .collect::<Vec<_>>();
    expiring_contracts.sort_by_key(|row| (row.days.unwrap_or(i32::MAX), row.uid, row.index));
    expiring_contracts
}

fn club_analysis_loans(
    records: &[ClubContractDate],
    game_date: &Date,
    squad: &[i32],
) -> Vec<ClubAnalysisLoan> {
    let today = date_ordinal(game_date);
    records
        .iter()
        .filter(|record| record.loan && squad.binary_search(&record.uid).is_ok())
        .filter_map(|record| {
            let expires = (!career::is_monthly(&record.expires))
                .then(|| date_ordinal(&record.expires))
                .flatten();
            if let Some(today) = today
                && (date_ordinal(&record.started).is_some_and(|start| start > today)
                    || expires.is_some_and(|end| end < today))
            {
                return None;
            }
            Some(ClubAnalysisLoan {
                uid: record.uid,
                index: record.index,
                expires: expires.map(|_| record.expires.clone()),
            })
        })
        .collect()
}

#[tauri::command]
fn club_detail(uid: i32, state: State<Shared>) -> Result<ClubDetail> {
    let (data, edits) = {
        let session = state.lock().map_err(|e| e.to_string())?;
        (
            session.loaded.clone().ok_or("Abra um save primeiro")?,
            session.plan.edits.clone(),
        )
    };
    let records = data
        .club_contract_dates
        .get(&uid)
        .map(Vec::as_slice)
        .unwrap_or_default();
    let expiring_contracts = project_club_expiries(records, &data.game_date, &edits);
    let contract_edits = edits
        .iter()
        .filter_map(|edit| {
            let (index, field) = edit.attribute.strip_prefix("contract.")?.split_once('.')?;
            Some(((edit.uid, index.parse::<usize>().ok()?, field), edit.value))
        })
        .collect::<BTreeMap<_, _>>();
    let wage_contracts: Vec<_> = records
        .iter()
        .map(|record| ClubWageContract {
            uid: record.uid,
            index: record.index,
            name: record.name.clone(),
            loan: record.loan,
            weekly_wage: contract_edits
                .get(&(record.uid, record.index, "weeklyWage"))
                .copied()
                .unwrap_or(record.weekly_wage),
            squad_status: contract_edits
                .get(&(record.uid, record.index, "squadStatus"))
                .copied()
                .unwrap_or(i32::from(record.squad_status)),
            squad_number: contract_edits
                .get(&(record.uid, record.index, "squadNumber"))
                .copied()
                .unwrap_or(i32::from(record.squad_number)),
        })
        .collect();
    let projected_player_payroll_weekly = data.club_info.get(&uid).and_then(|info| {
        let gross_delta: i64 = records
            .iter()
            .zip(&wage_contracts)
            .map(|(original, effective)| {
                i64::from(effective.weekly_wage) - i64::from(original.weekly_wage)
            })
            .sum();
        let offset_delta: i64 = data
            .club_loan_wages
            .get(&uid)
            .into_iter()
            .flatten()
            .map(|pair| pair.contribution_delta(&contract_edits))
            .sum();
        let delta = gross_delta - offset_delta;
        (delta != 0)
            .then_some(info.player_contract_wages_weekly - info.loan_wage_offset_weekly + delta)
    });
    let c = data
        .clubs
        .iter()
        .find(|c| c.uid == uid)
        .ok_or("Clube não encontrado")?;
    let analysis_loans = club_analysis_loans(records, &data.game_date, &c.player_uids);
    let tactic = data.club_tactics.as_ref().map(|clubs| clubs.get(&uid));
    let analysis_tactic_unsupported = matches!(tactic, Err(_) | Ok(Some(Err(_))));
    let analysis_tactic = tactic
        .ok()
        .flatten()
        .and_then(|value| value.as_ref().ok())
        .cloned();
    let squad = data
        .players
        .iter()
        .filter(|p| c.player_uids.binary_search(&p.uid).is_ok())
        .collect::<Vec<_>>();
    let players = squad
        .iter()
        .map(|p| ClubSquadPlayer {
            health: BatchHealth::from_save(&data, p.uid),
            row: Row {
                uid: p.uid,
                name: p.name.clone(),
                club: p.club.clone(),
                age: p.date_of_birth.age_on(&data.game_date),
                ca: p.current_ability,
                pa: p.potential_ability,
            },
            positions: cm0304_core::attributes::FIELDS[..9]
                .iter()
                .copied()
                .filter(|key| p.raw_attributes.get(*key).is_some_and(|&value| value >= 15))
                .collect(),
            familiarity: cm0304_core::attributes::FIELDS[..12]
                .iter()
                .filter_map(|&key| p.raw_attributes.get(key).copied().map(|value| (key, value)))
                .collect(),
            condition: p
                .fields
                .get("state.condition")
                .map(|field| field.value)
                .filter(|value| (0..=10000).contains(value)),
        })
        .collect();
    let attribute_averages = ["passing", "finishing", "tackling", "pace", "reflexes"]
        .into_iter()
        .filter_map(|key| {
            let mut sum = 0u32;
            let mut count = 0u32;
            for player in &squad {
                let goalkeeper = player
                    .raw_attributes
                    .get("goalkeeper")
                    .is_some_and(|&value| value >= 15);
                if (key == "reflexes") != goalkeeper {
                    continue;
                }
                if let Some(&value) = player
                    .attributes
                    .get(key)
                    .filter(|&&value| (1..=20).contains(&value))
                {
                    sum += value as u32;
                    count += 1;
                }
            }
            (count > 0).then(|| (key, f64::from(sum) / f64::from(count)))
        })
        .collect();
    let staff = data
        .staff
        .iter()
        .filter(|p| c.staff_uids.binary_search(&p.uid).is_ok())
        .map(|p| ClubStaff {
            uid: p.uid,
            name: p.name.clone(),
            role: p.club_job.to_owned(),
            is_player: p.is_player,
            attributes: cm0304_core::staff_editor::EDITABLE
                .into_iter()
                .filter_map(|key| {
                    p.attributes
                        .as_ref()?
                        .get(key)
                        .copied()
                        .map(|value| (key, value))
                })
                .collect(),
        })
        .collect();
    Ok(ClubDetail {
        info: data.club_info.get(&uid).cloned(),
        stadium: data.club_stadiums.get(&uid).cloned(),
        projected_player_payroll_weekly,
        expiring_contracts,
        analysis_loans,
        analysis_tactic,
        analysis_tactic_unsupported,
        wage_contracts,
        profile: c.clone(),
        players,
        attribute_averages,
        staff,
    })
}
#[tauri::command]
async fn staff_detail(uid: i32, state: State<'_, Shared>) -> Result<StaffDetail> {
    let (data, plan) = session_snapshot(&state)?;
    let mut profile = data
        .staff
        .iter()
        .find(|p| p.uid == uid)
        .cloned()
        .ok_or("Staff não encontrado")?;
    let pending_player_fields = plan.edits.iter().filter(|e| e.uid == uid).count();
    let pending_header = plan
        .staff_header_edits
        .iter()
        .find(|h| h.uid == uid)
        .cloned();
    let mut recorded = None;
    if pending_player_fields > 0
        || plan.staff_edits.iter().any(|e| e.uid == uid)
        || pending_header.is_some()
    {
        let bytes = data.bytes.clone();
        recorded = Some(profile);
        profile = tauri::async_runtime::spawn_blocking(move || -> Result<Staff> {
            let (output, _) = transaction::apply(&bytes, &plan)?;
            let (db, base) = database::load(&output)?;
            staff::profiles(&db, &output, base)?
                .into_iter()
                .find(|p| p.uid == uid)
                .ok_or("Staff ausente após projeção".into())
        })
        .await
        .map_err(|e| e.to_string())??;
    }
    let age = profile
        .date_of_birth
        .as_ref()
        .and_then(|date| date.age_on(&data.game_date));
    let mut editable = if profile.attributes.is_some() {
        cm0304_core::staff_editor::EDITABLE
            .iter()
            .map(|k| k.to_string())
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    if profile.personality.is_some() {
        editable.extend(
            database::PERSONALITY_FIELDS
                .iter()
                .map(|k| format!("personality.{k}")),
        );
    }
    Ok(StaffDetail {
        editable,
        profile,
        recorded,
        age,
        pending_player_fields,
        pending_header,
    })
}
#[tauri::command]
async fn stage_staff_edits(
    edits: Vec<cm0304_core::staff_editor::Edit>,
    header: Option<cm0304_core::staff_header::Edit>,
    state: State<'_, Shared>,
) -> Result<Pending> {
    let (data, previous) = session_snapshot(&state)?;
    let mut next = previous.clone();
    next.merge_staff(&data.staff, edits, header)?;
    if !next.is_empty() {
        let bytes = data.bytes.clone();
        let plan = next.clone();
        tauri::async_runtime::spawn_blocking(move || transaction::apply(&bytes, &plan).map(|_| ()))
            .await
            .map_err(|e| e.to_string())??;
    }
    let mut s = state.lock().map_err(|e| e.to_string())?;
    if s.plan != previous || !s.loaded.as_ref().is_some_and(|p| Arc::ptr_eq(p, &data)) {
        return Err("A sessão mudou durante a validação; tente novamente".into());
    }
    if s.plan != next {
        record(&mut s);
        s.plan = next;
    }
    Ok(pending(&s))
}
#[tauri::command]
fn stage_club_balance(
    uid: i32,
    value: i32,
    budgets: Option<transaction::TransferBudgets>,
    wage_budget: Option<i32>,
    state: State<Shared>,
) -> Result<Pending> {
    let mut session = state.lock().map_err(|e| e.to_string())?;
    let info = session
        .loaded
        .as_ref()
        .and_then(|d| d.club_info.get(&uid))
        .filter(|info| info.balance_editable)
        .ok_or("Clube sem registro financeiro completo")?;
    let original = info.balance.ok_or("Balanço indisponível")?;
    let target = (value != original).then_some(value);
    let old_wage_budget = session.plan.club_wage_budgets.get(&uid).copied();
    let mut wage_target = old_wage_budget;
    if let Some(value) = wage_budget {
        if !(0..=i32::MAX / 4).contains(&value) {
            return Err("Orçamento salarial fora do intervalo".into());
        }
        let original = info
            .wage_budget_weekly
            .ok_or("Orçamento salarial indisponível")?;
        wage_target = (value != original).then_some(value);
    }
    let mut budget_target = session.plan.club_transfer_budgets.get(&uid).cloned();
    if let Some(budgets) = budgets {
        if budgets.season < 0 || budgets.remaining < 0 {
            return Err("Informe orçamentos não negativos".into());
        }
        let original = info
            .transfer_budget_season
            .zip(info.transfer_budget_remaining)
            .ok_or("Orçamentos indisponíveis")?;
        budget_target = ((budgets.season, budgets.remaining) != original).then_some(budgets);
    }
    if session.plan.club_balances.get(&uid).copied() != target
        || old_wage_budget != wage_target
        || session.plan.club_transfer_budgets.get(&uid) != budget_target.as_ref()
    {
        record(&mut session);
        if let Some(value) = target {
            session.plan.club_balances.insert(uid, value);
        } else {
            session.plan.club_balances.remove(&uid);
        }
        if let Some(budgets) = budget_target {
            session.plan.club_transfer_budgets.insert(uid, budgets);
        } else {
            session.plan.club_transfer_budgets.remove(&uid);
        }
        if let Some(value) = wage_target {
            session.plan.club_wage_budgets.insert(uid, value);
        } else {
            session.plan.club_wage_budgets.remove(&uid);
        }
    }
    Ok(pending(&session))
}

#[tauri::command]
fn stage_club_infrastructure(
    uid: i32,
    training: Option<u8>,
    youth: Option<u8>,
    reputation: Option<i16>,
    state: State<Shared>,
) -> Result<Pending> {
    let mut session = state.lock().map_err(|e| e.to_string())?;
    let info = session
        .loaded
        .as_ref()
        .and_then(|d| d.club_info.get(&uid))
        .ok_or("Infraestrutura indisponível")?;
    let previous_reputation = session.plan.club_reputation.get(&uid).copied();
    let next_reputation = match reputation {
        Some(value) => {
            let original = info.reputation.ok_or("Equipe principal indisponível")?;
            if value != original && !(0..=10_000).contains(&value) {
                return Err("Informe uma reputação entre 0 e 10000".into());
            }
            (value != original).then_some(value)
        }
        None => previous_reputation,
    };
    let targets = [
        (training, info.training_raw, &session.plan.club_training),
        (youth, info.youth_setup_raw, &session.plan.club_youth_setup),
    ];
    let mut next = [None; 2];
    for (i, (value, original, staged)) in targets.into_iter().enumerate() {
        next[i] = match value {
            Some(value) if value == original => None,
            Some(value) if (1..=20).contains(&value) => Some(value),
            Some(_) => return Err("Informe um nível de infraestrutura entre 1 e 20".into()),
            None => staged.get(&uid).copied(),
        };
    }
    if next_reputation != previous_reputation
        || next
            != [
                session.plan.club_training.get(&uid).copied(),
                session.plan.club_youth_setup.get(&uid).copied(),
            ]
    {
        record(&mut session);
        let plan = &mut session.plan;
        if let Some(value) = next_reputation {
            plan.club_reputation.insert(uid, value);
        } else {
            plan.club_reputation.remove(&uid);
        }
        for (target, staged) in next
            .into_iter()
            .zip([&mut plan.club_training, &mut plan.club_youth_setup])
        {
            if let Some(value) = target {
                staged.insert(uid, value);
            } else {
                staged.remove(&uid);
            }
        }
    }
    Ok(pending(&session))
}
#[tauri::command]
fn stage_club_stadium(
    uid: i32,
    target: StadiumCapacities,
    state: State<Shared>,
) -> Result<Pending> {
    target.validate()?;
    let mut session = state.lock().map_err(|e| e.to_string())?;
    let ground = session
        .loaded
        .as_ref()
        .and_then(|data| data.club_stadiums.get(&uid))
        .ok_or("Estádio indisponível")?;
    if ground.first_team_club_uids.as_slice() != [uid] {
        return Err("O estádio é compartilhado".into());
    }
    let original = StadiumCapacities {
        capacity: ground.stadium.capacity,
        seated_capacity: ground.stadium.seated_capacity,
    };
    let next = (target != original).then_some(target);
    if session.plan.club_stadium_capacities.get(&uid).copied() != next {
        record(&mut session);
        match next {
            Some(value) => {
                session.plan.club_stadium_capacities.insert(uid, value);
            }
            None => {
                session.plan.club_stadium_capacities.remove(&uid);
            }
        }
    }
    Ok(pending(&session))
}

#[derive(Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlayerFilters {
    watch_uids: Option<std::collections::BTreeSet<i32>>,
    min_age: Option<u16>,
    max_age: Option<u16>,
    min_ca: Option<i16>,
    max_ca: Option<i16>,
    min_pa: Option<i16>,
    max_pa: Option<i16>,
    nation: Option<String>,
    order: Option<PlayerOrder>,
    position: Option<String>,
    attribute: Option<String>,
    min_attribute: Option<i32>,
    foot: Option<PlayerFoot>,
    contract: Option<PlayerContractFilter>,
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
enum PlayerOrder {
    Name,
    Age,
    Ca,
    Pa,
}

#[derive(Clone, Copy, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
enum PlayerFoot {
    Left,
    Right,
    Both,
}

#[derive(Clone, Copy, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
enum PlayerContractFilter {
    Full,
    Loan,
    Monthly,
    NoClubContract,
}

#[derive(Default)]
struct PlayerSearch(Arc<AtomicU64>);

#[tauri::command]
fn cancel_player_search(generation: u64, search: State<PlayerSearch>) {
    search.0.fetch_max(generation, Ordering::Relaxed);
}

#[tauri::command]
async fn search_players(
    query: String,
    page: usize,
    filters: Option<PlayerFilters>,
    generation: u64,
    state: State<'_, Shared>,
    search: State<'_, PlayerSearch>,
) -> Result<Option<Page>> {
    let data = loaded(&state)?;
    let latest = search.0.clone();
    latest.fetch_max(generation, Ordering::Relaxed);
    tauri::async_runtime::spawn_blocking(move || {
        let active = || latest.load(Ordering::Relaxed) == generation;
        let matches = matching_players(&data, &query, filters.unwrap_or_default(), active)?;
        if !active() {
            return Ok(None);
        }
        Ok(Some(Page {
            total: matches.len(),
            rows: matches
                .into_iter()
                .skip(page.saturating_mul(50))
                .take(50)
                .map(|(p, age)| Row {
                    uid: p.uid,
                    name: p.name.clone(),
                    club: p.club.clone(),
                    age,
                    ca: p.current_ability,
                    pa: p.potential_ability,
                })
                .collect(),
        }))
    })
    .await
    .map_err(|error| error.to_string())?
}
#[derive(Serialize)]
struct BatchPlayerRow {
    uid: i32,
    name: String,
    #[serde(flatten)]
    health: BatchHealth,
}

#[tauri::command]
async fn select_filtered_players(
    query: String,
    filters: Option<PlayerFilters>,
    state: State<'_, Shared>,
) -> Result<Vec<BatchPlayerRow>> {
    let data = loaded(&state)?;
    tauri::async_runtime::spawn_blocking(move || {
        Ok(
            matching_players(&data, &query, filters.unwrap_or_default(), || true)?
                .into_iter()
                .map(|(player, _)| BatchPlayerRow {
                    uid: player.uid,
                    name: player.name.clone(),
                    health: BatchHealth::from_save(&data, player.uid),
                })
                .collect(),
        )
    })
    .await
    .map_err(|error| error.to_string())?
}

fn matching_players<'a>(
    data: &'a Loaded,
    query: &str,
    filters: PlayerFilters,
    active: impl Fn() -> bool,
) -> Result<Vec<(&'a Player, Option<u16>)>> {
    if !active() {
        return Ok(Vec::new());
    }
    let q = search::key(query.trim());
    let uid = q
        .strip_prefix("uid:")
        .map(|value| {
            value
                .trim()
                .parse::<i32>()
                .map_err(|_| "UID de jogador inválido")
        })
        .transpose()?;

    if filters
        .min_age
        .zip(filters.max_age)
        .is_some_and(|(min, max)| min > max)
        || filters
            .min_ca
            .zip(filters.max_ca)
            .is_some_and(|(min, max)| min > max)
        || filters
            .min_pa
            .zip(filters.max_pa)
            .is_some_and(|(min, max)| min > max)
    {
        return Err("O mínimo deve ser menor ou igual ao máximo".into());
    }
    let position = filters
        .position
        .as_deref()
        .map(|name| {
            if !cm0304_core::attributes::FIELDS[..9].contains(&name) {
                return Err("Posição desconhecida");
            }
            Ok(format!("position.{name}"))
        })
        .transpose()?;
    let attribute = match (filters.attribute.as_deref(), filters.min_attribute) {
        (None, None) => None,
        (Some(key), Some(min))
            if cm0304_core::attributes::FIELDS[12..].contains(&key) && (0..=20).contains(&min) =>
        {
            Some((key, min))
        }
        _ => return Err("Escolha um atributo e um mínimo entre 0 e 20".into()),
    };
    let nation = filters
        .nation
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| {
            let query = search::key(value);
            data.nations
                .iter()
                .map(String::as_str)
                .filter(|name| search::key(name).contains(&query))
                .collect::<BTreeSet<_>>()
        });
    let mut matches = data
        .players
        .iter()
        .zip(&data.search_keys)
        .take_while(|_| active())
        .filter(|(player, _)| {
            filters
                .watch_uids
                .as_ref()
                .is_none_or(|uids| uids.contains(&player.uid))
        })
        .filter(|(player, key)| uid.map_or_else(|| key.contains(&q), |uid| player.uid == uid))
        .filter(|(player, _)| {
            nation.as_ref().is_none_or(|names| {
                player
                    .nation
                    .as_deref()
                    .is_some_and(|name| names.contains(name))
            })
        })
        .filter_map(|(player, _)| {
            if position.as_ref().is_some_and(|key| {
                player
                    .fields
                    .get(key.as_str())
                    .is_none_or(|value| value.value < 15)
            }) || attribute.is_some_and(|(key, min)| {
                player.fields.get(key).is_none_or(|value| value.value < min)
            }) {
                return None;
            }
            if let Some(foot) = filters.foot {
                let left = player.fields.get("leftFoot")?.value;
                let right = player.fields.get("rightFoot")?.value;
                let matches = match foot {
                    PlayerFoot::Left => left > right,
                    PlayerFoot::Right => right > left,
                    PlayerFoot::Both => left >= 15 && right >= 15,
                };
                if !matches {
                    return None;
                }
            }
            if let Some(filter) = filters.contract {
                let contracts = data
                    .contracts
                    .get(&player.uid)
                    .map(Vec::as_slice)
                    .unwrap_or_default();
                let mut club = contracts.iter().filter(|c| c.entity_type == 0);
                let matches = match filter {
                    PlayerContractFilter::Full => club.any(|c| c.kind == 0 && !c.monthly),
                    PlayerContractFilter::Loan => club.any(|c| c.kind == 1),
                    PlayerContractFilter::Monthly => club.any(|c| c.kind == 0 && c.monthly),
                    PlayerContractFilter::NoClubContract => !club.any(|c| matches!(c.kind, 0 | 1)),
                };
                if !matches {
                    return None;
                }
            }
            let age = player.date_of_birth.age_on(&data.game_date);
            (filters
                .min_age
                .is_none_or(|min| age.is_some_and(|age| age >= min))
                && filters
                    .max_age
                    .is_none_or(|max| age.is_some_and(|age| age <= max))
                && filters
                    .min_ca
                    .is_none_or(|min| player.current_ability >= min)
                && filters
                    .max_ca
                    .is_none_or(|max| player.current_ability <= max)
                && filters
                    .min_pa
                    .is_none_or(|min| player.potential_ability >= min)
                && filters
                    .max_pa
                    .is_none_or(|max| player.potential_ability <= max))
            .then_some((player, age))
        })
        .collect::<Vec<_>>();
    if !active() {
        return Ok(matches);
    }
    match filters.order {
        Some(PlayerOrder::Name) => {
            matches.sort_by_cached_key(|(p, _)| (search::key(&p.name), p.uid))
        }
        Some(PlayerOrder::Age) => matches.sort_by_key(|(p, age)| (age.unwrap_or(u16::MAX), p.uid)),
        Some(PlayerOrder::Ca) => {
            matches.sort_by_key(|(p, _)| (std::cmp::Reverse(p.current_ability), p.uid))
        }
        Some(PlayerOrder::Pa) => {
            matches.sort_by_key(|(p, _)| (std::cmp::Reverse(p.potential_ability), p.uid))
        }
        None => {}
    }
    Ok(matches)
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SavePlayerComparison {
    current: Player,
    other: Player,
    current_contracts: Vec<ContractView>,
    other_contracts: Vec<ContractView>,
    current_name: String,
    other_name: String,
    current_date: Date,
    other_date: Date,
}

#[tauri::command]
async fn compare_player_save(
    uid: i32,
    app: tauri::AppHandle,
    state: State<'_, Shared>,
    task: State<'_, SaveComparisonTask>,
) -> Result<Option<SavePlayerComparison>> {
    let data = loaded(&state)?;
    run_save_comparison(&task, move |cancelled| {
        let Some(file) = app
            .dialog()
            .file()
            .add_filter("CM0304", &["cm4"])
            .blocking_pick_file()
        else {
            return Ok(None);
        };
        let path = file.into_path().map_err(|e| e.to_string())?;
        let Some(bytes) = read_comparison_save(&path, cancelled)? else {
            return Ok(None);
        };
        let (db, base) = parse_save(&path, &bytes, || cancelled.load(Ordering::Relaxed))?;
        let current = data
            .players
            .iter()
            .find(|p| p.uid == uid)
            .ok_or("Jogador não encontrado no save aberto")?
            .clone();
        if cancelled.load(Ordering::Relaxed) {
            return Ok(None);
        }
        let mut matches = db.players_by_uid(base, uid)?.into_iter();
        let other_name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        let other = matches.next().ok_or_else(|| {
            format!(
                "{} (UID {uid}) não foi encontrado em {other_name}.",
                current.name
            )
        })?;
        if matches.next().is_some() {
            return Err(format!("O UID {uid} está repetido em {other_name}."));
        }
        let current_contracts = data.contracts.get(&uid).cloned().unwrap_or_default();
        let other_person = db
            .people
            .iter()
            .find(|person| person.uid == uid && person.playing.is_some())
            .ok_or("Registro de contratos do jogador não encontrado")?;
        let other_contracts = career::contracts(&db, other_person);
        Ok(Some(SavePlayerComparison {
            current_contracts,
            other_contracts,
            current,
            other,
            current_name: data
                .source
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned(),
            other_name,
            current_date: data.game_date.clone(),
            other_date: db.game_date,
        }))
    })
    .await
}
#[tauri::command]
async fn player_detail(uid: i32, state: State<'_, Shared>) -> Result<Detail> {
    let (data, plan) = session_snapshot(&state)?;
    let p = data
        .players
        .iter()
        .find(|p| p.uid == uid)
        .ok_or("Jogador não encontrado")?;
    let editable = p.fields.keys().map(ToString::to_string).collect();
    let pending = plan
        .edits
        .iter()
        .filter(|e| e.uid == uid)
        .cloned()
        .collect::<Vec<_>>();
    let heal_pending = plan.heal_injuries.contains(&uid);
    let suspension_pending = plan.clear_suspensions.contains(&uid);
    let shared_personality = plan
        .staff_edits
        .iter()
        .any(|e| e.uid == uid && e.attribute.starts_with("personality."));
    let effective =
        if pending.is_empty() && !heal_pending && !suspension_pending && !shared_personality {
            p.clone()
        } else {
            let bytes = data.bytes.clone();
            // Related entities can be necessary for validity (e.g. swapping shirts).
            // Always project the complete transaction, then select this player.
            let edits = plan;
            tauri::async_runtime::spawn_blocking(move || -> Result<Player> {
                let (output, _) = transaction::apply(&bytes, &edits)?;
                let (db, base) = database::load(&output)?;
                db.players_by_uid(base, uid)?
                    .into_iter()
                    .next()
                    .ok_or("Jogador não encontrado após revisão".into())
            })
            .await
            .map_err(|e| e.to_string())??
        };
    let discipline = data.discipline.get(&uid).copied().unwrap_or_default();
    Ok(Detail {
        player: p.clone(),
        market_journal_supported: data.market_journal_supported,
        age: effective.date_of_birth.age_on(&data.game_date),
        effective,
        editable,
        contracts: data.contracts.get(&uid).cloned().unwrap_or_default(),
        market_targets: data
            .market_sources
            .get(&uid)
            .map_or_else(Vec::new, |&source| {
                std::iter::once(source)
                    .chain(
                        cm0304_core::market_lab::supported_targets(source)
                            .iter()
                            .copied(),
                    )
                    .collect()
            }),
        pending,
        nations: data.nations.clone(),
        injuries: data.injuries.get(&uid).cloned().unwrap_or_default(),
        heal_pending,
        suspension_count: discipline.suspensions,
        booking_record_count: discipline.booking_records,
        suspension_pending,
    })
}
#[tauri::command]
async fn stage_edits(
    edits: Vec<Edit>,
    healing: Option<HealingUpdate>,
    suspension: Option<HealingUpdate>,
    state: State<'_, Shared>,
) -> Result<Pending> {
    let (data, previous) = session_snapshot(&state)?;
    let mut next = previous.clone();
    let (market, edits): (Vec<_>, Vec<_>) =
        edits.into_iter().partition(|e| e.attribute == MARKET_FIELD);
    let mut seen_market = std::collections::BTreeSet::new();
    for edit in market {
        if !seen_market.insert(edit.uid) || edit.policy.is_some() {
            return Err("Alterações de mercado duplicadas ou incompatíveis".into());
        }
        let source = data
            .market_sources
            .get(&edit.uid)
            .ok_or("Vínculo indisponível para editar a situação de mercado")?;
        let target = u8::try_from(edit.value).map_err(|_| "Situação de mercado inválida")?;
        if target == *source {
            next.market_statuses.remove(&edit.uid);
        } else {
            if !cm0304_core::market_lab::supported_targets(*source).contains(&target) {
                return Err("Transição de mercado indisponível para o estado original".into());
            }
            next.market_statuses.insert(edit.uid, target);
        }
    }
    next.merge_fields(&data.players, edits)?;
    // Returning playing CA to its source removes redundant professional targets.
    next.merge_staff(&data.staff, Vec::new(), None)?;
    if let Some(update) = healing {
        if !data.players.iter().any(|p| p.uid == update.uid) {
            return Err("Jogador não encontrado".into());
        }
        if update.enabled {
            if data.injuries.get(&update.uid).is_none_or(|v| v.is_empty()) {
                return Err("Jogador sem registros de lesão".into());
            }
            next.heal_injuries.insert(update.uid);
        } else {
            next.heal_injuries.remove(&update.uid);
        }
    }
    if let Some(update) = suspension {
        let discipline = data
            .discipline
            .get(&update.uid)
            .ok_or("Jogador não encontrado")?;
        if update.enabled {
            if discipline.suspensions == 0 {
                return Err("Jogador sem registros de suspensão".into());
            }
            next.clear_suspensions.insert(update.uid);
        } else {
            next.clear_suspensions.remove(&update.uid);
        }
    }
    if !next.is_empty() {
        let bytes = data.bytes.clone();
        let plan = next.clone();
        tauri::async_runtime::spawn_blocking(move || transaction::apply(&bytes, &plan).map(|_| ()))
            .await
            .map_err(|e| e.to_string())??;
    }
    let mut s = state.lock().map_err(|e| e.to_string())?;
    if s.plan != previous || !s.loaded.as_ref().is_some_and(|p| Arc::ptr_eq(p, &data)) {
        return Err("A sessão mudou durante a validação; tente novamente".into());
    }
    if s.plan == next {
        return Ok(pending(&s));
    }
    record(&mut s);
    s.plan = next;
    Ok(pending(&s))
}
#[derive(serde::Deserialize)]
struct HealingUpdate {
    uid: i32,
    enabled: bool,
}
#[tauri::command]
fn pending_edits(state: State<Shared>) -> Result<Pending> {
    let session = state.lock().map_err(|e| e.to_string())?;
    Ok(pending(&session))
}
#[tauri::command]
fn history_action(action: String, state: State<Shared>) -> Result<Pending> {
    let mut s = state.lock().map_err(|e| e.to_string())?;
    let current = s.plan.clone();
    let previous = match action.as_str() {
        "undo" => {
            let p = s.undo.pop();
            if p.is_some() {
                s.redo.push(current);
            }
            p
        }
        "redo" => {
            let p = s.redo.pop();
            if p.is_some() {
                s.undo.push(current);
            }
            p
        }
        "clear" => {
            record(&mut s);
            Some(Transaction::default())
        }
        _ => return Err("Ação inválida".into()),
    };
    if let Some(plan) = previous {
        s.batch_preview = None;
        s.latest_batch_id = None;
        s.plan = plan;
    }
    Ok(pending(&s))
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum BatchKind {
    Players,
    Contracts,
    Staff,
}

struct PreparedBatch {
    id: u64,
    source: Arc<Loaded>,
    previous: Transaction,
    next: Transaction,
    kind: BatchKind,
}

#[tauri::command]
async fn preview_player_batch(
    request: batch::Request,
    state: State<'_, Shared>,
    preview_id: u64,
) -> Result<Vec<batch::Target>> {
    let mut uids: BTreeSet<_> = request.uids.iter().copied().collect();
    if let batch::Operation::Copy(copy) = &request.operation {
        uids.insert(copy.source_uid);
    }
    preview_batch(
        &state,
        BatchKind::Players,
        preview_id,
        uids,
        move |_, players| batch::project(players, &request),
    )
    .await
}

#[tauri::command]
async fn preview_health_batch(
    uids: Vec<i32>,
    heal_injuries: bool,
    clear_suspensions: bool,
    recover: bool,
    state: State<'_, Shared>,
    preview_id: u64,
) -> Result<Vec<batch::Target>> {
    if uids.is_empty() || (!heal_injuries && !clear_suspensions && !recover) {
        return Err("Selecione jogadores e pelo menos um ajuste de recuperação".into());
    }
    prepare_batch(
        &state,
        BatchKind::Players,
        preview_id,
        move |source, plan| {
            let mut targets = Vec::new();
            if recover {
                targets = project_batch_fields(
                    source,
                    plan,
                    &uids.iter().copied().collect(),
                    |_, players| {
                        batch::project(
                            players,
                            &batch::Request {
                                uids: uids.clone(),
                                attribute: String::new(),
                                operation: batch::Operation::Recover,
                            },
                        )
                    },
                )?;
                if targets.iter().any(|target| target.unavailable) {
                    return Ok(targets);
                }
            }
            let players = source
                .players
                .iter()
                .map(|p| (p.uid, &p.name))
                .collect::<BTreeMap<_, _>>();
            let counts = uids
                .into_iter()
                .collect::<std::collections::BTreeSet<_>>()
                .into_iter()
                .map(|uid| {
                    let name = players
                        .get(&uid)
                        .ok_or_else(|| format!("Jogador {uid} não encontrado"))?;
                    let health = BatchHealth::from_save(source, uid);
                    Ok(batch::HealthCounts {
                        uid,
                        name,
                        injuries: health.injury_count,
                        suspensions: health.suspension_count,
                    })
                })
                .collect::<Result<Vec<_>>>()?;
            targets.extend(batch::project_health_removals(
                plan,
                &counts,
                heal_injuries,
                clear_suspensions,
            ));
            Ok(targets)
        },
    )
    .await
}

#[tauri::command]
async fn preview_wage_batch(
    club_uid: i32,
    request: batch::WageRequest,
    state: State<'_, Shared>,
    preview_id: u64,
) -> Result<Vec<batch::Target>> {
    preview_batch(
        &state,
        BatchKind::Contracts,
        preview_id,
        request
            .contracts
            .iter()
            .map(|contract| contract.uid)
            .collect(),
        move |source, players| {
            validate_club_selection(source, club_uid, &request.contracts)?;
            batch::project_wages(players, &request)
        },
    )
    .await
}

#[tauri::command]
async fn preview_expiry_batch(
    club_uid: i32,
    request: batch::ExpiryRequest,
    state: State<'_, Shared>,
    preview_id: u64,
) -> Result<Vec<batch::Target>> {
    preview_batch(
        &state,
        BatchKind::Contracts,
        preview_id,
        request
            .contracts
            .iter()
            .map(|contract| contract.uid)
            .collect(),
        move |source, players| {
            validate_club_selection(source, club_uid, &request.contracts)?;
            batch::project_expiries(players, &request)
        },
    )
    .await
}

#[tauri::command]
async fn preview_squad_status_batch(
    club_uid: i32,
    request: batch::SquadStatusRequest,
    state: State<'_, Shared>,
    preview_id: u64,
) -> Result<Vec<batch::Target>> {
    preview_batch(
        &state,
        BatchKind::Contracts,
        preview_id,
        request
            .contracts
            .iter()
            .map(|contract| contract.uid)
            .collect(),
        move |source, players| {
            validate_club_selection(source, club_uid, &request.contracts)?;
            batch::project_squad_status(players, &request)
        },
    )
    .await
}

fn validate_club_selection(
    source: &Loaded,
    club_uid: i32,
    contracts: &[batch::ContractSelection],
) -> Result<()> {
    let records = source
        .club_contract_dates
        .get(&club_uid)
        .ok_or("Clube sem vínculos disponíveis")?;
    if contracts.iter().any(|selection| {
        !records
            .iter()
            .any(|row| row.uid == selection.uid && row.index == selection.index)
    }) {
        return Err("O lote contém um vínculo que não pertence ao clube selecionado".into());
    }
    Ok(())
}

#[tauri::command]
async fn preview_staff_batch(
    club_uid: i32,
    uids: Vec<i32>,
    values: BTreeMap<String, i8>,
    state: State<'_, Shared>,
    preview_id: u64,
) -> Result<Vec<batch::Target>> {
    use cm0304_core::staff_editor::{EDITABLE, Edit};
    if uids.is_empty()
        || values.is_empty()
        || values
            .iter()
            .any(|(key, value)| !EDITABLE.contains(&key.as_str()) || !(1..=20).contains(value))
    {
        return Err("Selecione membros da comissão e atributos profissionais de 1 a 20".into());
    }
    prepare_batch(&state, BatchKind::Staff, preview_id, move |source, plan| {
        let club = source
            .clubs
            .iter()
            .find(|club| club.uid == club_uid)
            .ok_or("Clube não encontrado")?;
        let members = uids.into_iter().collect::<std::collections::BTreeSet<_>>();
        if members
            .iter()
            .any(|uid| club.staff_uids.binary_search(uid).is_err())
        {
            return Err("O lote contém membros fora da comissão selecionada".into());
        }
        let projected;
        let profiles = if plan.edits.is_empty()
            && plan.staff_edits.is_empty()
            && plan.staff_header_edits.is_empty()
        {
            &source.staff
        } else {
            let (bytes, _) = transaction::apply(&source.bytes, plan)?;
            let (db, base) = database::load(&bytes)?;
            projected = staff::profiles(&db, &bytes, base)?;
            &projected
        };
        let mut targets = Vec::new();
        let mut edits = Vec::new();
        for uid in members {
            let profile = profiles
                .iter()
                .find(|p| p.uid == uid)
                .ok_or("Membro da comissão não encontrado")?;
            for (key, &after) in &values {
                let before = profile
                    .attributes
                    .as_ref()
                    .and_then(|fields| fields.get(key.as_str()))
                    .copied();
                targets.push(batch::Target {
                    uid,
                    name: profile.name.clone(),
                    attribute: key.clone(),
                    before: i32::from(before.unwrap_or(0)),
                    after: i32::from(after),
                    limited: false,
                    unavailable: before.is_none(),
                });
                if before.is_some_and(|before| before != after) {
                    edits.push(Edit {
                        uid,
                        attribute: key.clone(),
                        value: after,
                    });
                }
            }
        }
        if !targets.iter().any(|target| target.unavailable) {
            plan.merge_staff(&source.staff, edits, None)?;
        }
        Ok(targets)
    })
    .await
}

#[tauri::command]
fn stage_staff_batch(state: State<'_, Shared>, preview_id: u64) -> Result<Pending> {
    stage_batch(&state, BatchKind::Staff, preview_id)
}

async fn preview_batch(
    state: &State<'_, Shared>,
    kind: BatchKind,
    preview_id: u64,
    uids: BTreeSet<i32>,
    project: impl FnOnce(&Loaded, &[Player]) -> Result<(Vec<Edit>, Vec<batch::Target>)> + Send + 'static,
) -> Result<Vec<batch::Target>> {
    prepare_batch(state, kind, preview_id, move |data, next| {
        project_batch_fields(data, next, &uids, project)
    })
    .await
}

fn project_batch_fields(
    data: &Loaded,
    next: &mut Transaction,
    uids: &BTreeSet<i32>,
    project: impl FnOnce(&Loaded, &[Player]) -> Result<(Vec<Edit>, Vec<batch::Target>)>,
) -> Result<Vec<batch::Target>> {
    let projected;
    let players = if next.edits.is_empty() {
        &data.players
    } else {
        let (bytes, _) = cm0304_core::editor::apply(&data.bytes, &next.edits)?;
        let (db, base) = database::load(&bytes)?;
        projected = db.players_matching(base, |uid| uids.contains(&uid))?;
        &projected
    };
    let (edits, targets) = project(data, players)?;
    if !targets.iter().any(|target| target.unavailable) {
        next.merge_fields(&data.players, edits)?;
        next.merge_staff(&data.staff, Vec::new(), None)?;
    }
    Ok(targets)
}

async fn prepare_batch(
    state: &State<'_, Shared>,
    kind: BatchKind,
    preview_id: u64,
    project: impl FnOnce(&Loaded, &mut Transaction) -> Result<Vec<batch::Target>> + Send + 'static,
) -> Result<Vec<batch::Target>> {
    let (source, previous) = {
        let mut session = state.lock().map_err(|e| e.to_string())?;
        session.batch_preview = None;
        session.latest_batch_id = Some(preview_id);
        (
            session.loaded.clone().ok_or("Abra um save primeiro")?,
            session.plan.clone(),
        )
    };
    let data = Arc::clone(&source);
    let baseline = previous.clone();
    let (next, targets) = tauri::async_runtime::spawn_blocking(move || -> Result<_> {
        let mut next = baseline.clone();
        let targets = project(&data, &mut next)?;
        if next != baseline && !next.is_empty() {
            transaction::apply(&data.bytes, &next)?;
        }
        Ok((next, targets))
    })
    .await
    .map_err(|e| e.to_string())??;
    let mut session = state.lock().map_err(|e| e.to_string())?;
    if session.latest_batch_id != Some(preview_id)
        || session.plan != previous
        || !session
            .loaded
            .as_ref()
            .is_some_and(|data| Arc::ptr_eq(data, &source))
    {
        return Err("A sessão mudou durante a prévia. Gere uma nova prévia.".into());
    }
    if !targets.iter().any(|target| target.unavailable) {
        session.batch_preview = Some(PreparedBatch {
            id: preview_id,
            source,
            previous,
            next,
            kind,
        });
    }
    Ok(targets)
}

#[tauri::command]
fn stage_player_batch(state: State<Shared>, preview_id: u64) -> Result<Pending> {
    stage_batch(&state, BatchKind::Players, preview_id)
}

#[tauri::command]
fn stage_wage_batch(state: State<Shared>, preview_id: u64) -> Result<Pending> {
    stage_batch(&state, BatchKind::Contracts, preview_id)
}

fn stage_batch(state: &State<Shared>, kind: BatchKind, preview_id: u64) -> Result<Pending> {
    let mut session = state.lock().map_err(|e| e.to_string())?;
    if session
        .batch_preview
        .as_ref()
        .is_none_or(|preview| preview.kind != kind || preview.id != preview_id)
    {
        return Err("A prévia deste lote expirou. Gere uma nova prévia.".into());
    }
    let preview = session
        .batch_preview
        .take()
        .ok_or("Gere uma prévia do lote primeiro")?;
    if session.plan != preview.previous
        || !session
            .loaded
            .as_ref()
            .is_some_and(|data| Arc::ptr_eq(data, &preview.source))
    {
        return Err("A sessão mudou desde a prévia. Gere uma nova prévia.".into());
    }
    if preview.next != session.plan {
        record(&mut session);
        session.plan = preview.next;
    }
    Ok(pending(&session))
}

#[tauri::command]
async fn remove_player_from_plan(
    uid: i32,
    field: Option<String>,
    state: State<'_, Shared>,
) -> Result<Pending> {
    let (data, previous) = {
        let session = state.lock().map_err(|e| e.to_string())?;
        (
            session.loaded.clone().ok_or("Abra um save primeiro")?,
            session.plan.clone(),
        )
    };
    let mut next = previous.clone();
    match field.as_deref() {
        None => {
            next.edits.retain(|edit| edit.uid != uid);
            next.market_statuses.remove(&uid);
            next.heal_injuries.remove(&uid);
            next.clear_suspensions.remove(&uid);
        }
        Some("healInjuries") => {
            next.heal_injuries.remove(&uid);
        }
        Some("clearSuspensions") => {
            next.clear_suspensions.remove(&uid);
        }
        Some(MARKET_FIELD) => {
            next.market_statuses.remove(&uid);
        }
        Some(field) => {
            let year = format!("{field}.year");
            let day = format!("{field}.day");
            let date =
                field == "birth" || (field.starts_with("contract.") && field.ends_with(".expires"));
            next.edits.retain(|edit| {
                edit.uid != uid
                    || (edit.attribute != field
                        && !(date && (edit.attribute == year || edit.attribute == day)))
            });
        }
    }
    next.merge_fields(&data.players, Vec::new())?;
    next.merge_staff(&data.staff, Vec::new(), None)?;
    if next != previous && !next.is_empty() {
        let bytes = data.bytes.clone();
        let plan = next.clone();
        tauri::async_runtime::spawn_blocking(move || transaction::apply(&bytes, &plan).map(|_| ()))
            .await
            .map_err(|e| e.to_string())??;
    }
    let mut session = state.lock().map_err(|e| e.to_string())?;
    if session.plan != previous
        || !session
            .loaded
            .as_ref()
            .is_some_and(|loaded| Arc::ptr_eq(loaded, &data))
    {
        return Err("A sessão mudou durante a validação. Tente novamente.".into());
    }
    if next != previous {
        record(&mut session);
        session.plan = next;
    }
    Ok(pending(&session))
}
#[tauri::command]
async fn remove_review_operation(index: usize, state: State<'_, Shared>) -> Result<Pending> {
    let (data, previous, history) = {
        let session = state.lock().map_err(|e| e.to_string())?;
        (
            session.loaded.clone().ok_or("Abra um save primeiro")?,
            session.plan.clone(),
            session.undo.clone(),
        )
    };
    let mut next = history
        .get(index)
        .ok_or("Operação ausente no histórico atual")?
        .clone();
    for (before, after) in history.iter().skip(index + 1).zip(
        history
            .iter()
            .skip(index + 2)
            .chain(std::iter::once(&previous)),
    ) {
        next.replay_delta(
            before,
            after,
            |uid| {
                let info = data.club_info.get(&uid)?;
                Some(transaction::TransferBudgets {
                    season: info.transfer_budget_season?,
                    remaining: info.transfer_budget_remaining?,
                })
            },
            |uid| {
                data.staff
                    .iter()
                    .find(|profile| profile.uid == uid)
                    .map(|profile| profile.reputation)
            },
        );
    }
    next.merge_fields(&data.players, Vec::new())?;
    next.merge_staff(&data.staff, Vec::new(), None)?;
    if next != previous && !next.is_empty() {
        let bytes = Arc::clone(&data.bytes);
        let plan = next.clone();
        tauri::async_runtime::spawn_blocking(move || transaction::apply(&bytes, &plan).map(|_| ()))
            .await
            .map_err(|e| e.to_string())??;
    }
    let mut session = state.lock().map_err(|e| e.to_string())?;
    if session.plan != previous
        || session.undo != history
        || !session
            .loaded
            .as_ref()
            .is_some_and(|loaded| Arc::ptr_eq(loaded, &data))
    {
        return Err("A sessão mudou durante a validação. Tente novamente.".into());
    }
    if next != previous {
        record(&mut session);
        session.plan = next;
    }
    Ok(pending(&session))
}

#[tauri::command]
async fn preview_edits(state: State<'_, Shared>) -> Result<transaction::Report> {
    let (data, edits) = session_snapshot(&state)?;
    tauri::async_runtime::spawn_blocking(move || {
        transaction::apply(&data.bytes, &edits).map(|(_, r)| r)
    })
    .await
    .map_err(|e| e.to_string())?
}
#[derive(Serialize)]
struct Saved {
    path: String,
    report: transaction::Report,
}

fn sibling_save_path(source: &Path, name: &str) -> Result<PathBuf> {
    let name = name.trim();
    let stem = name
        .split('.')
        .next()
        .unwrap_or_default()
        .trim_end()
        .to_ascii_uppercase();
    let reserved = matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || stem
            .strip_prefix("COM")
            .or_else(|| stem.strip_prefix("LPT"))
            .is_some_and(|suffix| {
                matches!(
                    suffix,
                    "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "¹" | "²" | "³"
                )
            });
    if name.is_empty()
        || name.starts_with('.')
        || name.ends_with('.')
        || name.encode_utf16().count() > 26
        || reserved
        || name.to_ascii_lowercase().ends_with(".cm4")
        || name
            .chars()
            .any(|c| c.is_control() || "<>:\"/\\|?*".contains(c))
    {
        return Err("Informe um nome válido de até 26 caracteres, sem extensão ou caminho.".into());
    }
    Ok(source.with_file_name(format!("{name}.cm4")))
}

#[test]
fn variant_name_stays_in_the_source_directory() {
    let source = Path::new("games").join("original.cm4");
    assert_eq!(
        sibling_save_path(&source, "  Cenário B  ").unwrap(),
        Path::new("games").join("Cenário B.cm4")
    );
    for name in [
        "../outra",
        "C:\\outra",
        "a/b",
        "a:b",
        "NUL",
        "COM¹",
        "LPT3 .copia",
        "original.cm4",
        "a.",
        "",
    ] {
        assert!(sibling_save_path(&source, name).is_err(), "accepted {name}");
    }
    assert!(sibling_save_path(&source, &"x".repeat(27)).is_err());
}

#[tauri::command]
async fn save_copy(
    app: tauri::AppHandle,
    state: State<'_, Shared>,
    task: State<'_, SaveCopyTask>,
    variant_name: Option<String>,
) -> Result<Option<Saved>> {
    let (data, edits) = session_snapshot(&state)?;
    let data_for_history = Arc::clone(&data);
    let destination = variant_name
        .as_deref()
        .map(|name| sibling_save_path(&data.source, name))
        .transpose()?;
    let cancelled = Arc::new(AtomicBool::new(false));
    {
        let mut active = task.0.lock().map_err(|e| e.to_string())?;
        if active.is_some() {
            return Err("Já existe uma gravação em andamento".into());
        }
        *active = Some(Arc::clone(&cancelled));
    }
    let result = tauri::async_runtime::spawn_blocking(move || -> Result<Option<Saved>> {
        let path = if let Some(destination) = destination {
            destination
        } else {
            let Some(file) = app
                .dialog()
                .file()
                .add_filter("CM0304", &["cm4"])
                .set_file_name("save_editado.cm4")
                .blocking_save_file()
            else {
                return Ok(None);
            };
            file.into_path().map_err(|e| e.to_string())?
        };
        let report = match transaction::save_snapshot(
            &data.source,
            &path,
            &edits,
            &data.bytes,
            &cancelled,
        ) {
            Err(error) if error == cm0304_core::editor::CANCELLED => return Ok(None),
            result => result?,
        };
        Ok(Some(Saved {
            path: path.display().to_string(),
            report,
        }))
    })
    .await;
    let mut active = task.0.lock().map_err(|e| e.to_string())?;
    *active = None;
    let saved = result.map_err(|e| e.to_string())??;
    if let Some(copy) = &saved {
        let mut session = state.lock().map_err(|e| e.to_string())?;
        if session
            .loaded
            .as_ref()
            .is_some_and(|loaded| Arc::ptr_eq(loaded, &data_for_history))
        {
            session.last_saved_copy = Some(PathBuf::from(&copy.path));
        }
    }
    Ok(saved)
}

#[tauri::command]
fn cancel_save_copy(task: State<SaveCopyTask>) -> Result<()> {
    task.cancel()
}

#[tauri::command]
fn open_copy_folder(state: State<Shared>) -> Result<()> {
    let path = state
        .lock()
        .map_err(|e| e.to_string())?
        .last_saved_copy
        .clone()
        .ok_or("Nenhuma cópia foi gravada nesta sessão.")?;
    let folder = path
        .parent()
        .filter(|path| path.is_dir())
        .ok_or("A pasta de destino não está disponível.")?;
    std::process::Command::new("explorer.exe")
        .arg(folder)
        .spawn()
        .map_err(|e| format!("Não foi possível abrir a pasta de destino: {e}"))?;
    Ok(())
}

#[tauri::command]
fn close_editor(window: tauri::WebviewWindow, task: State<SaveCopyTask>) -> Result<()> {
    if window.label() != "main" {
        return Err("Janela do editor inválida.".into());
    }
    if task.is_active() {
        return Err("Aguarde a gravação da cópia antes de fechar o editor.".into());
    }
    window.destroy().map_err(|error| error.to_string())
}
#[tauri::command]
async fn export_batch_presets(app: tauri::AppHandle, json: String) -> Result<Option<String>> {
    export_document(app, json, "json", "cm0304-presets.json").await
}

const MAX_PLAN_BYTES: u64 = 16 * 1024 * 1024;

#[derive(serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct SessionPlan {
    version: u32,
    source_sha256: String,
    plan: Transaction,
}

fn session_snapshot(state: &State<Shared>) -> Result<(Arc<Loaded>, Transaction)> {
    let session = state.lock().map_err(|e| e.to_string())?;
    Ok((
        session.loaded.clone().ok_or("Abra um save primeiro")?,
        session.plan.clone(),
    ))
}

#[tauri::command]
async fn export_session_plan(
    app: tauri::AppHandle,
    state: State<'_, Shared>,
) -> Result<Option<String>> {
    let (data, plan) = session_snapshot(&state)?;
    if plan.is_empty() {
        return Err("Não há alterações para guardar".into());
    }
    let content = tauri::async_runtime::spawn_blocking(move || {
        encode_session_plan(&SessionPlan {
            version: 1,
            source_sha256: hash(&data.bytes),
            plan,
        })
    })
    .await
    .map_err(|e| e.to_string())??;
    export_document(app, content, "json", "cm0304-plano.json").await
}

fn validate_session_plan_size(size: usize) -> Result<()> {
    if size as u64 > MAX_PLAN_BYTES {
        return Err("O arquivo de plano excede 16 MB".into());
    }
    Ok(())
}

fn encode_session_plan(plan: &SessionPlan) -> Result<String> {
    let formatted = serde_json::to_string_pretty(plan).map_err(|error| error.to_string())?;
    if validate_session_plan_size(formatted.len()).is_ok() {
        return Ok(formatted);
    }
    drop(formatted);
    let compact = serde_json::to_string(plan).map_err(|error| error.to_string())?;
    validate_session_plan_size(compact.len())?;
    Ok(compact)
}

fn restore_session_plan(bytes: &[u8], source: &[u8]) -> Result<Transaction> {
    let stored: SessionPlan =
        serde_json::from_slice(bytes).map_err(|e| format!("Plano inválido: {e}"))?;
    if stored.version != 1 {
        return Err("Versão do plano não suportada".into());
    }
    if stored.source_sha256 != hash(source) {
        return Err("Este plano pertence a outro save. Abra a origem usada para criá-lo.".into());
    }
    transaction::apply(source, &stored.plan)?;
    Ok(stored.plan)
}

fn restore_session_plan_from_source(
    bytes: &[u8],
    source_path: &Path,
    source_snapshot: &[u8],
) -> Result<Transaction> {
    let plan = restore_session_plan(bytes, source_snapshot)?;
    let file =
        std::fs::File::open(source_path).map_err(|error| save_io_error(source_path, error))?;
    if !cm0304_core::editor::matches_snapshot(&file, source_snapshot, &AtomicBool::new(false))? {
        return Err(
            "O save de origem mudou no disco. Reabra o arquivo antes de retomar o plano.".into(),
        );
    }
    Ok(plan)
}

#[tauri::command]
async fn import_session_plan(
    app: tauri::AppHandle,
    state: State<'_, Shared>,
    replace_existing: bool,
) -> Result<Option<Pending>> {
    let (data, previous) = session_snapshot(&state)?;
    if !previous.is_empty() && !replace_existing {
        return Err("Confirme a substituição do plano atual".into());
    }
    let source = Arc::clone(&data);
    let next = tauri::async_runtime::spawn_blocking(move || -> Result<Option<Transaction>> {
        use std::io::Read as _;
        let Some(file) = app
            .dialog()
            .file()
            .add_filter("JSON", &["json"])
            .blocking_pick_file()
        else {
            return Ok(None);
        };
        let path = file.into_path().map_err(|e| e.to_string())?;
        let mut bytes = Vec::new();
        std::fs::File::open(path)
            .map_err(|e| e.to_string())?
            .take(MAX_PLAN_BYTES + 1)
            .read_to_end(&mut bytes)
            .map_err(|e| e.to_string())?;
        validate_session_plan_size(bytes.len())?;
        restore_session_plan_from_source(&bytes, &source.source, &source.bytes).map(Some)
    })
    .await
    .map_err(|e| e.to_string())??;
    let Some(next) = next else { return Ok(None) };
    let mut session = state.lock().map_err(|e| e.to_string())?;
    if session.plan != previous
        || !session
            .loaded
            .as_ref()
            .is_some_and(|loaded| Arc::ptr_eq(loaded, &data))
    {
        return Err("A sessão mudou enquanto o plano era validado. Tente novamente.".into());
    }
    if next != previous {
        record(&mut session);
        session.plan = next;
    }
    Ok(Some(pending(&session)))
}
#[tauri::command]
async fn export_plan_report(
    app: tauri::AppHandle,
    state: State<'_, Shared>,
) -> Result<Option<String>> {
    let report = preview_edits(state).await?;
    let content = serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?;
    export_document(app, content, "json", "cm0304-revisao.json").await
}
#[tauri::command]
async fn export_saved_report(app: tauri::AppHandle, json: String) -> Result<Option<String>> {
    export_document(app, json, "json", "cm0304-gravacao.json").await
}
#[tauri::command]
async fn export_player_comparison(app: tauri::AppHandle, csv: String) -> Result<Option<String>> {
    export_document(app, csv, "csv", "cm0304-comparacao.csv").await
}

#[tauri::command]
async fn export_player_comparison_json(
    app: tauri::AppHandle,
    json: String,
) -> Result<Option<String>> {
    export_document(app, json, "json", "cm0304-comparacao.json").await
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ClubSaveComparison {
    source_name: String,
    other_name: String,
    report: cm0304_core::roster_comparison::RosterComparison,
}

fn read_save_bytes(
    mut reader: impl std::io::Read,
    cancelled: &AtomicBool,
) -> std::io::Result<Option<Vec<u8>>> {
    let mut bytes = Vec::new();
    let mut buffer = [0; 64 * 1024];
    while !cancelled.load(Ordering::Relaxed) {
        let count = match reader.read(&mut buffer) {
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            result => result?,
        };
        if count == 0 {
            return Ok(Some(bytes));
        }
        bytes.extend_from_slice(&buffer[..count]);
    }
    Ok(None)
}

#[test]
fn save_file_read_discards_partial_bytes_when_cancelled() {
    struct CancelOnRead<'a>(&'a AtomicBool);
    impl std::io::Read for CancelOnRead<'_> {
        fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
            assert!(!self.0.swap(true, Ordering::Relaxed));
            buffer[0] = 42;
            Ok(1)
        }
    }
    let cancelled = AtomicBool::new(false);
    assert_eq!(
        read_save_bytes(&[1, 2, 3][..], &cancelled).unwrap(),
        Some(vec![1, 2, 3])
    );
    assert!(
        read_save_bytes(CancelOnRead(&cancelled), &cancelled)
            .unwrap()
            .is_none()
    );
}

#[tauri::command]
async fn compare_club_save(
    uid: i32,
    app: tauri::AppHandle,
    state: State<'_, Shared>,
    task: State<'_, SaveComparisonTask>,
) -> Result<Option<ClubSaveComparison>> {
    let data = loaded(&state)?;
    run_save_comparison(&task, move |cancelled| {
        let Some(file) = app
            .dialog()
            .file()
            .add_filter("CM0304", &["cm4"])
            .blocking_pick_file()
        else {
            return Ok(None);
        };
        let path = file.into_path().map_err(|error| error.to_string())?;
        let Some(bytes) = read_comparison_save(&path, cancelled)? else {
            return Ok(None);
        };
        let report =
            cm0304_core::roster_comparison::compare_with_cancel(&data.bytes, &bytes, uid, || {
                cancelled.load(Ordering::Relaxed)
            })
            .map_err(|error| {
                format!(
                    "Falha ao comparar {} e {}. {error}",
                    save_name(&data.source),
                    save_name(&path)
                )
            })?;
        Ok(Some(ClubSaveComparison {
            source_name: save_name(&data.source).into_owned(),
            other_name: save_name(&path).into_owned(),
            report,
        }))
    })
    .await
}

fn read_comparison_save(path: &Path, cancelled: &AtomicBool) -> Result<Option<Vec<u8>>> {
    if cancelled.load(Ordering::Relaxed) {
        return Ok(None);
    }
    let file = std::fs::File::open(path).map_err(|error| save_io_error(path, error))?;
    read_save_bytes(file, cancelled).map_err(|error| save_io_error(path, error))
}

async fn run_save_comparison<T: Send + 'static>(
    task: &SaveComparisonTask,
    compare: impl FnOnce(&AtomicBool) -> Result<Option<T>> + Send + 'static,
) -> Result<Option<T>> {
    let cancelled = Arc::new(AtomicBool::new(false));
    {
        let mut active = task.0.lock().map_err(|error| error.to_string())?;
        if active.is_some() {
            return Err("Já existe uma comparação de saves em andamento".into());
        }
        *active = Some(Arc::clone(&cancelled));
    }
    let worker_cancelled = Arc::clone(&cancelled);
    let result = tauri::async_runtime::spawn_blocking(move || compare(&worker_cancelled)).await;
    let mut active = task.0.lock().map_err(|error| error.to_string())?;
    *active = None;
    if cancelled.load(Ordering::Relaxed) {
        return Ok(None);
    }
    result.map_err(|error| error.to_string())?
}

#[test]
fn comparison_cancellation_discards_a_late_result_and_allows_retry() {
    use std::{sync::mpsc, time::Duration};

    let task = Arc::new(SaveComparisonTask::default());
    let (started_tx, started_rx) = mpsc::channel();
    let (resume_tx, resume_rx) = mpsc::channel();
    let worker_task = Arc::clone(&task);
    let worker = tauri::async_runtime::spawn(async move {
        run_save_comparison(&worker_task, move |cancelled| {
            started_tx.send(()).unwrap();
            resume_rx.recv_timeout(Duration::from_secs(5)).unwrap();
            assert!(cancelled.load(Ordering::Relaxed));
            Ok(Some(1))
        })
        .await
    });
    started_rx.recv_timeout(Duration::from_secs(5)).unwrap();

    let already_running =
        tauri::async_runtime::block_on(run_save_comparison(&task, |_| Ok(Some(2))));
    assert!(already_running.unwrap_err().contains("em andamento"));
    task.0
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .store(true, Ordering::Relaxed);
    resume_tx.send(()).unwrap();
    assert_eq!(
        tauri::async_runtime::block_on(worker).unwrap().unwrap(),
        None
    );
    assert_eq!(
        tauri::async_runtime::block_on(run_save_comparison(&task, |_| Ok(Some(3)))).unwrap(),
        Some(3)
    );
}

#[tauri::command]
fn cancel_save_comparison(task: State<SaveComparisonTask>) -> Result<()> {
    if let Some(cancelled) = task.0.lock().map_err(|e| e.to_string())?.as_ref() {
        cancelled.store(true, Ordering::Relaxed);
    }
    Ok(())
}

#[tauri::command]
async fn export_club_comparison(app: tauri::AppHandle, json: String) -> Result<Option<String>> {
    export_document(app, json, "json", "cm0304-comparacao-elenco.json").await
}

#[tauri::command]
async fn export_club_comparison_csv(app: tauri::AppHandle, csv: String) -> Result<Option<String>> {
    export_document(app, csv, "csv", "cm0304-comparacao-elenco.csv").await
}

#[derive(Clone, Copy, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
enum PlayerExportFormat {
    Csv,
    Json,
}

#[derive(Serialize)]
struct ExportPlayerRow<'a> {
    uid: i32,
    name: &'a str,
    club: Option<&'a str>,
    nation: Option<&'a str>,
    age: Option<u16>,
    ca: i16,
    pa: i16,
}
#[tauri::command]
async fn export_filtered_players(
    query: String,
    filters: Option<PlayerFilters>,
    format: Option<PlayerExportFormat>,
    app: tauri::AppHandle,
    state: State<'_, Shared>,
) -> Result<Option<String>> {
    let data = loaded(&state)?;
    let format = format.unwrap_or(PlayerExportFormat::Csv);
    let content = tauri::async_runtime::spawn_blocking(move || -> Result<String> {
        let rows = matching_players(&data, &query, filters.unwrap_or_default(), || true)?;
        if matches!(format, PlayerExportFormat::Json) {
            let players = rows
                .into_iter()
                .map(|(player, age)| ExportPlayerRow {
                    uid: player.uid,
                    name: &player.name,
                    club: player.club.as_deref(),
                    nation: player.nation.as_deref(),
                    age,
                    ca: player.current_ability,
                    pa: player.potential_ability,
                })
                .collect::<Vec<_>>();
            return serde_json::to_string_pretty(&players).map_err(|e| e.to_string());
        }
        let mut csv = String::from("\u{feff}UID;Jogador;Clube;Nacionalidade;Idade;CA;PA\r\n");
        for (player, age) in rows {
            use std::fmt::Write as _;
            let _ = write!(
                &mut csv,
                "{};{};{};{};{};{};{}\r\n",
                player.uid,
                csv_label(&player.name),
                csv_label(player.club.as_deref().unwrap_or_default()),
                csv_label(player.nation.as_deref().unwrap_or_default()),
                age.map(|value| value.to_string()).unwrap_or_default(),
                player.current_ability,
                player.potential_ability,
            );
        }
        Ok(csv)
    })
    .await
    .map_err(|e| e.to_string())??;
    let (extension, name) = match format {
        PlayerExportFormat::Csv => ("csv", "cm0304-jogadores.csv"),
        PlayerExportFormat::Json => ("json", "cm0304-jogadores.json"),
    };
    export_document(app, content, extension, name).await
}

fn csv_label(value: &str) -> String {
    let prefix = if value.trim_start().starts_with(['=', '+', '-', '@']) {
        "'"
    } else {
        ""
    };
    format!("\"{prefix}{}\"", value.replace('"', "\"\""))
}
async fn export_document(
    app: tauri::AppHandle,
    content: String,
    extension: &'static str,
    filename: &'static str,
) -> Result<Option<String>> {
    let max_bytes = if matches!(
        filename,
        "cm0304-jogadores.csv"
            | "cm0304-jogadores.json"
            | "cm0304-revisao.json"
            | "cm0304-plano.json"
    ) {
        16 * 1024 * 1024
    } else {
        256 * 1024
    };
    if content.len() > max_bytes {
        return Err("O arquivo excede o tamanho permitido para a exportação".into());
    }
    tauri::async_runtime::spawn_blocking(move || {
        let Some(file) = app
            .dialog()
            .file()
            .add_filter(extension.to_ascii_uppercase(), &[extension])
            .set_file_name(filename)
            .blocking_save_file()
        else {
            return Ok(None);
        };
        let path = file.into_path().map_err(|e| e.to_string())?;
        if path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_none_or(|ext| !ext.eq_ignore_ascii_case(extension))
        {
            return Err(format!("Escolha um arquivo com extensão .{extension}"));
        }
        std::fs::write(&path, content).map_err(|e| e.to_string())?;
        Ok(Some(path.display().to_string()))
    })
    .await
    .map_err(|e| e.to_string())?
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.unminimize();
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(Mutex::new(Session::default()))
        .manage(updates::DownloadedUpdate::default())
        .manage(PlayerSearch::default())
        .manage(SaveComparisonTask::default())
        .manage(OpenSaveTask::default())
        .manage(SaveCopyTask::default())
        .invoke_handler(tauri::generate_handler![
            updates::download_update,
            updates::install_update,
            pick_save_file,
            open_save,
            resume_editor,
            cancel_open_save,
            cancel_save_copy,
            open_copy_folder,
            search_players,
            cancel_player_search,
            select_filtered_players,
            search_clubs,
            club_detail,
            my_club::my_club_detail,
            my_club::stage_my_club_action,
            search_staff,
            staff_detail,
            player_detail,
            compare_player_save,
            stage_edits,
            stage_staff_edits,
            stage_club_balance,
            stage_club_infrastructure,
            stage_club_stadium,
            pending_edits,
            history_action,
            preview_edits,
            remove_player_from_plan,
            remove_review_operation,
            preview_player_batch,
            preview_health_batch,
            preview_wage_batch,
            preview_expiry_batch,
            preview_squad_status_batch,
            preview_staff_batch,
            stage_staff_batch,
            stage_player_batch,
            stage_wage_batch,
            export_batch_presets,
            export_player_comparison,
            export_player_comparison_json,
            compare_club_save,
            cancel_save_comparison,
            export_club_comparison,
            export_club_comparison_csv,
            export_plan_report,
            export_saved_report,
            export_session_plan,
            import_session_plan,
            export_filtered_players,
            save_copy,
            close_editor
        ])
        .on_window_event(|window, event| {
            let tauri::WindowEvent::CloseRequested { api, .. } = event else {
                return;
            };
            if window.label() != "main" {
                return;
            }
            let Some(webview) = window.get_webview_window("main") else {
                return;
            };
            api.prevent_close();
            // Let the save worker publish or clean up its temporary file before exiting.
            let saving = window.state::<SaveCopyTask>().is_active();
            let script = if saving {
                "window.dispatchEvent(new Event('save-close-blocked'))"
            } else {
                "window.dispatchEvent(new Event('editor-close-requested'))"
            };
            let _ = webview.eval(script);
        })
        .run(tauri::generate_context!())
        .expect("Falha ao iniciar o editor");
}

#[cfg(test)]
#[test]
fn canceling_a_copy_keeps_close_blocked_until_the_worker_finishes() {
    let task = SaveCopyTask::default();
    assert!(!task.is_active());

    let cancellation = Arc::new(AtomicBool::new(false));
    *task.0.lock().unwrap() = Some(Arc::clone(&cancellation));
    assert!(task.is_active());

    task.cancel().unwrap();
    assert!(cancellation.load(Ordering::Relaxed));
    assert!(task.is_active());

    *task.0.lock().unwrap() = None;
    assert!(!task.is_active());
}

#[cfg(test)]
mod club_expiry_tests {
    use super::*;

    #[test]
    fn analysis_uses_current_receiving_loans_and_preserves_unknown_expiry() {
        let date = |day| Date { year: 2003, day };
        let record = |uid, index, loan, start, expires| ClubContractDate {
            uid,
            index,
            name: String::new(),
            started: date(start),
            expires,
            loan,
            weekly_wage: 0,
            squad_status: -1,
            squad_number: -1,
        };
        let records = [
            record(42, 0, false, 1, date(110)),
            record(42, 1, true, 1, date(110)),
            record(43, 0, true, 1, date(120)),
            record(44, 0, true, 1, date(99)),
            record(45, 0, true, 101, date(120)),
            record(46, 0, true, 1, Date { year: 1900, day: 1 }),
        ];
        let loans = club_analysis_loans(&records, &date(100), &[42, 44, 45, 46]);
        assert_eq!(
            loans
                .iter()
                .map(|loan| (loan.uid, loan.index, loan.expires.as_ref().map(|d| d.day)))
                .collect::<Vec<_>>(),
            [(42, 1, Some(110)), (46, 0, None)]
        );
    }

    #[test]
    fn projects_only_the_selected_contract_across_a_leap_year_boundary() {
        let today = Date {
            year: 2003,
            day: 364,
        };
        let record = |index, year, day| ClubContractDate {
            uid: 42,
            name: "Jogador".into(),
            started: today.clone(),
            expires: Date { year, day },
            index,
            loan: index == 1,
            weekly_wage: 0,
            squad_status: -1,
            squad_number: -1,
        };
        let records = [
            record(0, 2004, 90),
            record(1, 2004, 89),
            record(2, 1900, 1),
            record(3, 2003, 363),
            record(4, 2003, 364),
        ];
        let original = project_club_expiries(&records, &today, &[]);
        assert_eq!(
            original.iter().map(|e| e.days).collect::<Vec<_>>(),
            [Some(-1), Some(0), Some(90), Some(91), None]
        );
        let edit = Edit {
            uid: 42,
            attribute: "contract.0.expires.day".into(),
            value: 0,
            policy: None,
        };
        let projected = project_club_expiries(&records, &today, &[edit]);
        assert_eq!(
            projected
                .iter()
                .map(|e| (e.days, e.loan))
                .collect::<Vec<_>>(),
            [
                (Some(-1), false),
                (Some(0), false),
                (Some(1), false),
                (Some(90), true),
                (None, false)
            ]
        );
    }
}

#[cfg(test)]
mod session_plan_tests {
    use super::*;

    #[test]
    fn session_plan_size_accepts_the_limit_and_rejects_the_next_byte() {
        let limit = MAX_PLAN_BYTES as usize;
        assert!(validate_session_plan_size(limit).is_ok());
        assert!(validate_session_plan_size(limit + 1).is_err());
    }

    #[test]
    fn session_plan_uses_compact_json_only_when_needed_to_fit() {
        let mut stored = SessionPlan {
            version: 1,
            source_sha256: "0".repeat(64),
            plan: Transaction::default(),
        };
        assert!(encode_session_plan(&stored).unwrap().contains('\n'));
        stored.plan.edits = (1_000_000..1_200_000)
            .map(|uid| Edit {
                uid,
                attribute: "personality.professionalism".into(),
                value: 20,
                policy: None,
            })
            .collect();
        let encoded = encode_session_plan(&stored).unwrap();
        assert!(!encoded.contains('\n'));
        let decoded: SessionPlan = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded.plan, stored.plan);
        stored.plan.edits.extend(decoded.plan.edits);
        assert!(encode_session_plan(&stored).unwrap_err().contains("16 MB"));
    }

    #[test]
    fn restores_only_against_the_original_save_hash() {
        let samples =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../game/user data/games");
        let initial = std::fs::read(samples.join("save_inicial.cm4")).unwrap();
        let advanced = std::fs::read(samples.join("save_modificado.cm4")).unwrap();
        let plan = Transaction {
            edits: vec![Edit {
                uid: 2001642,
                attribute: "passing".into(),
                value: 20,
                policy: None,
            }],
            ..Default::default()
        };
        let stored = SessionPlan {
            version: 1,
            source_sha256: hash(&initial),
            plan: plan.clone(),
        };
        let json = serde_json::to_vec(&stored).unwrap();
        assert!(
            restore_session_plan(&json, &advanced)
                .unwrap_err()
                .contains("outro save")
        );
        assert_eq!(restore_session_plan(&json, &initial).unwrap(), plan);
        let disk = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(disk.path(), &initial).unwrap();
        assert_eq!(
            restore_session_plan_from_source(&json, disk.path(), &initial).unwrap(),
            plan
        );
        std::fs::write(disk.path(), &advanced).unwrap();
        assert!(
            restore_session_plan_from_source(&json, disk.path(), &initial)
                .unwrap_err()
                .contains("mudou no disco")
        );
        let unknown = SessionPlan {
            version: 2,
            ..stored
        };
        assert!(
            restore_session_plan(&serde_json::to_vec(&unknown).unwrap(), &initial)
                .unwrap_err()
                .contains("Versão")
        );
    }
}
