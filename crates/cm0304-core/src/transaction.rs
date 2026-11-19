//! One reviewable transaction combining field edits and structural operations.
use crate::{
    Result,
    container::hash,
    database::Player,
    editor::{self, Edit},
    health,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
    sync::atomic::AtomicBool,
};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    #[serde(default)]
    pub club_balances: BTreeMap<i32, i32>,
    #[serde(default)]
    pub club_transfer_budgets: BTreeMap<i32, TransferBudgets>,
    #[serde(default)]
    pub club_training: BTreeMap<i32, u8>,
    #[serde(default)]
    pub club_reputation: BTreeMap<i32, i16>,
    #[serde(default)]
    pub club_youth_setup: BTreeMap<i32, u8>,
    #[serde(default)]
    pub edits: Vec<Edit>,
    #[serde(default)]
    pub staff_edits: Vec<crate::staff_editor::Edit>,
    #[serde(default)]
    pub staff_header_edits: Vec<crate::staff_header::Edit>,
    #[serde(default)]
    pub heal_injuries: BTreeSet<i32>,
    #[serde(default)]
    pub clear_suspensions: BTreeSet<i32>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferBudgets {
    pub season: i32,
    pub remaining: i32,
}

impl Transaction {
    /// Merge a staff form atomically against the source snapshot. Final byte validation
    /// remains in apply(), where CA and explicit attribute targets are resolved together.
    pub fn merge_staff(
        &mut self,
        profiles: &[crate::staff::Staff],
        edits: Vec<crate::staff_editor::Edit>,
        header: Option<crate::staff_header::Edit>,
    ) -> Result<()> {
        let mut next = self.clone();
        if let Some(update) = header {
            let p = profiles
                .iter()
                .find(|p| p.uid == update.uid)
                .ok_or("Staff não encontrado")?;
            let mut combined = next
                .staff_header_edits
                .iter()
                .find(|e| e.uid == update.uid)
                .cloned()
                .unwrap_or(crate::staff_header::Edit {
                    uid: update.uid,
                    ..Default::default()
                });
            if let Some(ca) = update.current_ability {
                combined.current_ability = (ca != p.current_ability).then_some(ca);
                combined.policy = if combined.current_ability.is_some() {
                    Some(
                        update
                            .policy
                            .ok_or("Escolha uma política para a CA de staff")?,
                    )
                } else {
                    None
                };
            } else if update.policy.is_some() {
                if combined.current_ability.is_none() {
                    return Err("Política de CA sem alvo de CA".into());
                }
                combined.policy = update.policy;
            }
            if let Some(pa) = update.potential_ability {
                combined.potential_ability = (pa != p.potential_ability).then_some(pa);
            }
            if let Some(rep) = update.reputation {
                combined.reputation = (rep != p.reputation).then_some(rep);
            }
            next.staff_header_edits.retain(|e| e.uid != update.uid);
            if combined.current_ability.is_some()
                || combined.potential_ability.is_some()
                || combined.reputation.is_some()
            {
                next.staff_header_edits.push(combined);
            }
        }
        let dependencies = profiles
            .iter()
            .filter(|p| {
                if p.is_player {
                    next.edits
                        .iter()
                        .any(|e| e.uid == p.uid && e.attribute == "currentAbility")
                } else {
                    next.staff_header_edits
                        .iter()
                        .any(|h| h.uid == p.uid && h.current_ability.is_some())
                }
            })
            .map(|p| p.uid)
            .collect::<BTreeSet<_>>();
        let mut fields = next
            .staff_edits
            .iter()
            .cloned()
            .map(|e| ((e.uid, e.attribute.clone()), e))
            .collect::<BTreeMap<_, _>>();
        let mut seen = BTreeSet::new();
        for edit in edits {
            if edit.attribute.starts_with("personality.")
                && next.edits.iter().any(|p| {
                    p.uid == edit.uid
                        && p.attribute == edit.attribute
                        && p.value != i32::from(edit.value)
                })
            {
                return Err("Alvos conflitantes de personalidade entre jogador e staff".into());
            }
            if !seen.insert((edit.uid, edit.attribute.clone())) {
                return Err("Campo de staff repetido".into());
            }
            if !crate::staff_editor::is_editable(&edit.attribute) || !(1..=20).contains(&edit.value)
            {
                return Err("Campo ou valor de staff inválido".into());
            }
            let p = profiles
                .iter()
                .find(|p| p.uid == edit.uid)
                .ok_or("Staff não encontrado")?;
            let original = crate::staff_editor::value(p, &edit.attribute)?;
            let dependent = crate::staff::RELATIVE.contains(&edit.attribute.as_str())
                && dependencies.contains(&edit.uid);
            let key = (edit.uid, edit.attribute.clone());
            if edit.value == original && !dependent {
                fields.remove(&key);
            } else {
                fields.insert(key, edit);
            }
        }
        fields.retain(|_, e| {
            profiles
                .iter()
                .find(|p| p.uid == e.uid)
                .and_then(|p| crate::staff_editor::value(p, &e.attribute).ok())
                .is_none_or(|v| v != e.value)
                || (crate::staff::RELATIVE.contains(&e.attribute.as_str())
                    && dependencies.contains(&e.uid))
        });
        next.staff_edits = fields.into_values().collect();
        next.staff_header_edits.sort_by_key(|h| h.uid);
        *self = next;
        Ok(())
    }
    pub fn is_empty(&self) -> bool {
        self.edits.is_empty()
            && self.staff_edits.is_empty()
            && self.staff_header_edits.is_empty()
            && self.heal_injuries.is_empty()
            && self.clear_suspensions.is_empty()
            && self.club_balances.is_empty()
            && self.club_transfer_budgets.is_empty()
            && self.club_training.is_empty()
            && self.club_reputation.is_empty()
            && self.club_youth_setup.is_empty()
    }
    pub fn merge_fields(&mut self, players: &[Player], edits: Vec<Edit>) -> Result<()> {
        for edit in &edits {
            if edit.attribute.starts_with("personality.")
                && self.staff_edits.iter().any(|s| {
                    s.uid == edit.uid
                        && s.attribute == edit.attribute
                        && i32::from(s.value) != edit.value
                })
            {
                return Err("Alvos conflitantes de personalidade entre jogador e staff".into());
            }
        }
        let previous = self
            .edits
            .iter()
            .cloned()
            .map(|e| ((e.uid, e.attribute.clone()), e))
            .collect::<BTreeMap<_, _>>();
        self.edits = editor::merge_plan(players, &previous, edits)?
            .into_values()
            .collect();
        Ok(())
    }
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    pub source_sha256: String,
    pub copy_sha256: String,
    pub size_before: usize,
    pub size_after: usize,
    pub field_changed_bytes: usize,
    pub removed_bytes: usize,
    pub changes: Vec<editor::Change>,
    pub staff_changes: Vec<crate::staff_editor::Change>,
    pub removals: Vec<health::Removal>,
}
pub fn apply(input: &[u8], plan: &Transaction) -> Result<(Vec<u8>, Report)> {
    if plan.is_empty() {
        return Err("Plano de alterações vazio".into());
    }
    for e in &plan.staff_edits {
        if e.attribute.starts_with("personality.")
            && plan.edits.iter().any(|p| {
                p.uid == e.uid && p.attribute == e.attribute && p.value != i32::from(e.value)
            })
        {
            return Err("Alvos conflitantes de personalidade entre jogador e staff".into());
        }
    }
    // Fixed fields are applied first, while their source offsets still hold.
    let (fields_output, _, mut changes) = if plan.edits.is_empty() {
        (input.to_vec(), 0, Vec::new())
    } else {
        let (bytes, report) = editor::apply(input, &plan.edits)?;
        (bytes, report.changed_bytes, report.changes)
    };
    let (fields_output, header_changes) = if plan.staff_header_edits.is_empty() {
        (fields_output, Vec::new())
    } else {
        crate::staff_header::apply(&fields_output, &plan.staff_header_edits, &plan.staff_edits)?
    };
    changes.extend(header_changes);
    let remaining_staff = plan
        .staff_edits
        .iter()
        .filter(|e| {
            !plan.staff_header_edits.iter().any(|h| {
                h.uid == e.uid
                    && h.current_ability.is_some()
                    && crate::staff_editor::EDITABLE.contains(&e.attribute.as_str())
            })
        })
        .cloned()
        .collect::<Vec<_>>();
    let (fields_output, mut staff_changes) = if remaining_staff.is_empty() {
        (fields_output, Vec::new())
    } else {
        crate::staff_editor::apply(&fields_output, &remaining_staff)?
    };
    let playing_ca = plan
        .edits
        .iter()
        .filter(|e| e.attribute == "currentAbility")
        .map(|e| e.uid)
        .collect::<BTreeSet<_>>();
    if !playing_ca.is_empty() {
        // Reconcile against the source, not intermediate headers: a staff target may
        // override or cancel a conversion effect caused by the playing-role CA.
        let (before_db, before_base) = crate::database::load(input)?;
        let before = crate::staff::profiles(&before_db, input, before_base)?;
        let (after_db, after_base) = crate::database::load(&fields_output)?;
        let after = crate::staff::profiles(&after_db, &fields_output, after_base)?;
        for p in before
            .iter()
            .filter(|p| p.is_player && playing_ca.contains(&p.uid))
        {
            let q = after
                .iter()
                .find(|q| q.uid == p.uid)
                .ok_or("Jogador/staff ausente após projeção")?;
            let (
                Some(before_values),
                Some(after_values),
                Some(before_raw),
                Some(after_raw),
                Some(offset),
            ) = (
                &p.attributes,
                &q.attributes,
                &p.raw_attributes,
                &q.raw_attributes,
                p.attributes_offset,
            )
            else {
                continue;
            };
            for (i, key) in crate::staff::FIELDS.iter().enumerate() {
                changes.retain(|c| {
                    !(c.uid == p.uid
                        && c.attribute == *key
                        && c.context.as_deref().is_some_and(|s| s.starts_with("Staff")))
                });
                staff_changes.retain(|c| !(c.uid == p.uid && c.attribute == *key));
                if before_values[*key] == after_values[*key] && before_raw[*key] == after_raw[*key]
                {
                    continue;
                }
                let explicit = plan
                    .staff_edits
                    .iter()
                    .any(|e| e.uid == p.uid && e.attribute == *key);
                changes.push(editor::Change {
                    uid: p.uid,
                    name: p.name.clone(),
                    attribute: key.to_string(),
                    before: i32::from(before_values[*key]),
                    after: i32::from(after_values[*key]),
                    raw_before: i32::from(before_raw[*key]),
                    raw_after: i32::from(after_raw[*key]),
                    offset: offset + i,
                    width: 1,
                    derived: !explicit,
                    context: Some(
                        if explicit {
                            "Staff · alvo explícito"
                        } else {
                            "Staff · efeito da CA de jogador"
                        }
                        .into(),
                    ),
                    display_before: None,
                    display_after: None,
                });
            }
        }
    }
    let mut fields_output = fields_output;
    if !plan.club_balances.is_empty()
        || !plan.club_transfer_budgets.is_empty()
        || !plan.club_training.is_empty()
        || !plan.club_reputation.is_empty()
        || !plan.club_youth_setup.is_empty()
    {
        let (db, base) = crate::database::load(&fields_output)?;
        let club_uids = plan
            .club_balances
            .keys()
            .chain(plan.club_transfer_budgets.keys())
            .chain(plan.club_training.keys())
            .chain(plan.club_reputation.keys())
            .chain(plan.club_youth_setup.keys())
            .copied()
            .collect::<BTreeSet<_>>();
        for uid in club_uids {
            let mut matches = db.clubs.iter().enumerate().filter(|(_, c)| c.uid == uid);
            let (club_index, club) = matches
                .next()
                .ok_or_else(|| format!("Clube {uid} não encontrado"))?;
            if matches.next().is_some() {
                return Err(format!("UID de clube repetido: {uid}"));
            }
            let details = club
                .details
                .as_ref()
                .ok_or_else(|| format!("Clube {uid} sem registro financeiro completo"))?;
            if details.finance_kind != 1
                && (plan.club_balances.contains_key(&uid)
                    || plan.club_transfer_budgets.contains_key(&uid))
            {
                return Err(format!("Clube {uid} sem registro financeiro completo"));
            }
            let mut fields = Vec::with_capacity(6);
            if let Some(&target) = plan.club_reputation.get(&uid) {
                if !(0..=10_000).contains(&target) {
                    return Err(format!(
                        "Reputação do clube {uid} deve ficar entre 0 e 10000"
                    ));
                }
                let mut teams = db.teams.iter().filter(|team| {
                    team.container_type == 0
                        && team.container_index == club_index as i32
                        && team.squad == 0
                });
                let team = teams
                    .next()
                    .ok_or_else(|| format!("Clube {uid} sem equipe principal"))?;
                if teams.next().is_some() {
                    return Err(format!("Clube {uid} com mais de uma equipe principal"));
                }
                fields.push((
                    "club.reputation",
                    i32::from(team.reputation),
                    team.reputation_offset,
                    i32::from(target),
                    2,
                ));
            }
            if let Some(&target) = plan.club_youth_setup.get(&uid) {
                if !(1..=20).contains(&target) {
                    return Err(format!(
                        "Formação de jovens do clube {uid} deve ficar entre 1 e 20"
                    ));
                }
                fields.push((
                    "club.youthSetup",
                    i32::from(details.youth_setup_raw),
                    details.infrastructure_offset + 1,
                    i32::from(target),
                    1,
                ));
            }
            if let Some(&target) = plan.club_training.get(&uid) {
                if !(1..=20).contains(&target) {
                    return Err(format!(
                        "Instalações de treino do clube {uid} devem ficar entre 1 e 20"
                    ));
                }
                fields.push((
                    "club.training",
                    i32::from(details.training_raw),
                    details.infrastructure_offset + 15,
                    i32::from(target),
                    1,
                ));
            }
            if let Some(&target) = plan.club_balances.get(&uid) {
                let (before, offset) = details
                    .balance
                    .zip(details.balance_offset)
                    .ok_or("Registro financeiro sem balanço")?;
                fields.push(("club.balance", before, offset, target, 4));
            }
            if let Some(target) = plan.club_transfer_budgets.get(&uid) {
                if target.season < 0 || target.remaining < 0 {
                    return Err(format!(
                        "Orçamentos de transferências do clube {uid} devem ser não negativos"
                    ));
                }
                let season = details
                    .transfer_budget_season
                    .ok_or("Orçamento da temporada indisponível")?;
                let remaining = details
                    .transfer_budget_remaining
                    .ok_or("Orçamento restante indisponível")?;
                fields.push((
                    "club.transferBudgetSeason",
                    season,
                    details.finance_offset,
                    target.season,
                    4,
                ));
                fields.push((
                    "club.transferBudgetRemaining",
                    remaining,
                    details.finance_offset + 4,
                    target.remaining,
                    4,
                ));
            }
            for (attribute, before, offset, target, width) in fields {
                if before == target {
                    continue;
                }
                let offset = base + offset;
                fields_output[offset..offset + width]
                    .copy_from_slice(&target.to_le_bytes()[..width]);
                changes.push(crate::editor::Change {
                    uid,
                    name: club.name.clone(),
                    attribute: attribute.into(),
                    before,
                    after: target,
                    raw_before: before,
                    raw_after: target,
                    offset,
                    width: width as u8,
                    derived: false,
                    context: Some(
                        if width == 2 {
                            "Clube · reputação"
                        } else if width == 1 {
                            "Clube · infraestrutura"
                        } else {
                            "Clube · finanças"
                        }
                        .into(),
                    ),
                    display_before: None,
                    display_after: None,
                });
            }
        }
    }
    let field_changed_bytes = input
        .iter()
        .zip(&fields_output)
        .filter(|(a, b)| a != b)
        .count();
    let selections = plan
        .heal_injuries
        .iter()
        .map(|uid| (*uid, health::RemovalKind::Injuries))
        .chain(
            plan.clear_suspensions
                .iter()
                .map(|uid| (*uid, health::RemovalKind::Suspensions)),
        )
        .collect::<Vec<_>>();
    let (output, removals) = if selections.is_empty() {
        (fields_output, Vec::new())
    } else {
        let (bytes, report) = health::remove_records(&fields_output, &selections)?;
        (bytes, report.removals)
    };
    let report = Report {
        source_sha256: hash(input),
        copy_sha256: hash(&output),
        size_before: input.len(),
        size_after: output.len(),
        field_changed_bytes,
        removed_bytes: removals.iter().map(|r| r.removed_bytes).sum(),
        changes,
        staff_changes,
        removals,
    };
    Ok((output, report))
}
pub fn save_snapshot(
    source: &Path,
    destination: &Path,
    plan: &Transaction,
    input: &[u8],
    cancel: &AtomicBool,
) -> Result<Report> {
    editor::save_transformed_snapshot(source, destination, input, cancel, |bytes| {
        apply(bytes, plan)
    })
}
