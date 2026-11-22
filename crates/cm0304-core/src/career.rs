//! Editable fixed-size career fields. Contract identity and structure stay intact.
use crate::{
    Result,
    database::{Contract, Database, Date, Field, Person, Player},
    editor::Edit,
};
use serde::Serialize;
use std::{borrow::Cow, collections::BTreeMap};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractView {
    pub index: usize,
    pub kind: u8,
    pub kind_label: &'static str,
    pub entity_type: u8,
    pub entity: Option<String>,
    pub started: Date,
    pub monthly: bool,
    pub transfer_status: u8,
}

pub fn contract_entity<'a>(
    db: &'a Database,
    contract: &Contract,
) -> Option<&'a crate::database::Club> {
    let index = usize::try_from(contract.entity_index).ok()?;
    match contract.entity_type {
        0 => db.clubs.get(index),
        1 => db.nations.get(index),
        _ => None,
    }
}

pub fn contracts(db: &Database, person: &Person) -> Vec<ContractView> {
    person
        .contracts
        .iter()
        .enumerate()
        .map(|(index, c)| ContractView {
            index,
            kind: c.kind,
            kind_label: match c.kind {
                0 => "Completo",
                1 => "Empréstimo",
                2 => "Teste",
                5 => "Liga",
                _ => "Desconhecido",
            },
            entity_type: c.entity_type,
            entity: contract_entity(db, c).map(|entity| entity.name.clone()),
            started: c.started.clone(),
            monthly: is_monthly(&c.expires),
            transfer_status: c.transfer_status,
        })
        .collect()
}
pub fn is_monthly(date: &Date) -> bool {
    date.year == 1900 && date.day == 1
}

pub fn add_fields(person: &Person, base: usize, fields: &mut BTreeMap<Cow<'static, str>, Field>) {
    let Some(d) = &person.playing else { return };
    let s = &d.state;
    let mut add = |key: Cow<'static, str>, value, offset, width, min, max| {
        fields.insert(
            key,
            Field {
                value,
                offset: base + offset,
                width,
                min,
                max,
            },
        );
    };
    for (name, value, offset, width, min, max) in [
        ("state.condition", s.condition as i32, 28, 2, 0, 10000),
        ("state.fitness", s.fitness as i32, 24, 2, 0, 10000),
        ("state.jadedness", s.jadedness as i32, 26, 2, -1500, 1000),
        ("state.morale", s.morale as i32, 22, 1, 0, 20),
        (
            "state.estimatedValue",
            s.estimated_value,
            18,
            4,
            0,
            1_000_000_000,
        ),
        ("state.saleValue", s.sale_value, 42, 4, -1, 1_000_000_000),
    ] {
        add(
            Cow::Borrowed(name),
            value,
            d.offset + offset,
            width,
            min,
            max,
        );
    }
    for (i, c) in person.contracts.iter().enumerate() {
        for (name, value, offset) in [
            ("appearanceBonus", c.appearance_bonus, c.offset + 5),
            ("goalBonus", c.goal_bonus, c.offset + 9),
            ("assistBonus", c.assist_bonus, c.offset + 13),
        ] {
            if let Some(value) = value.filter(|value| *value >= 0) {
                add(
                    format!("contract.{i}.{name}").into(),
                    value,
                    offset,
                    4,
                    0,
                    1_000_000,
                );
            }
        }
        if let Some(clause) = &c.release_clause {
            add(
                format!("contract.{i}.releaseClause").into(),
                clause.amount,
                clause.offset,
                4,
                1,
                1_000_000_000,
            );
        }
        for (name, value, offset, width, min, max) in [
            ("weeklyWage", c.weekly_wage, 5, 4, 0, 1_000_000),
            ("expires.day", c.expires.day as i32, 9, 2, 0, 365),
            ("expires.year", c.expires.year as i32, 11, 2, 1900, 2200),
            ("squadStatus", c.squad_status as i32, 13, 1, -1, 7),
            ("squadNumber", c.squad_number as i32, 30, 1, -1, 99),
        ] {
            add(
                format!("contract.{i}.{name}").into(),
                value,
                c.common_offset + offset,
                width,
                min,
                max,
            );
        }
    }
}

pub fn validate(
    db: &Database,
    p: &Player,
    person: &Person,
    edits: &BTreeMap<&str, &Edit>,
    plan: &[Edit],
) -> Result<()> {
    let expiry_changed = edits
        .keys()
        .any(|key| key.starts_with("contract.") && key.contains(".expires."));
    if expiry_changed {
        let final_date = |i: usize, c: &Contract| Date {
            year: edits
                .get(format!("contract.{i}.expires.year").as_str())
                .map(|e| e.value as u16)
                .unwrap_or(c.expires.year),
            day: edits
                .get(format!("contract.{i}.expires.day").as_str())
                .map(|e| e.value as u16)
                .unwrap_or(c.expires.day),
        };
        for (i, loan) in person
            .contracts
            .iter()
            .enumerate()
            .filter(|(_, c)| c.kind == 1 && c.entity_type == 0)
        {
            let end = final_date(i, loan);
            for (j, owner) in person
                .contracts
                .iter()
                .enumerate()
                .filter(|(_, c)| c.kind == 0 && c.entity_type == 0)
            {
                let owned_end = final_date(j, owner);
                if !is_monthly(&owned_end) && (end.year, end.day) > (owned_end.year, owned_end.day)
                {
                    return Err(
                        "Empréstimo não pode terminar após o contrato com o clube proprietário"
                            .into(),
                    );
                }
            }
        }
    }
    for (i, c) in person.contracts.iter().enumerate() {
        let key = |name: &str| format!("contract.{i}.{name}");
        let value = |name: &str| {
            let k = key(name);
            edits
                .get(k.as_str())
                .map(|e| e.value)
                .unwrap_or(p.fields[k.as_str()].value)
        };
        if edits.contains_key(key("expires.year").as_str())
            || edits.contains_key(key("expires.day").as_str())
        {
            let end = Date {
                year: value("expires.year") as u16,
                day: value("expires.day") as u16,
            };
            if is_monthly(&end) {
                if c.kind != 0 {
                    return Err("Contrato mensal só é aceito no contrato completo existente".into());
                }
            } else if end.month_day().is_none()
                || (end.year, end.day) < (db.game_date.year, db.game_date.day)
                || (!is_monthly(&c.started)
                    && (end.year, end.day) < (c.started.year, c.started.day))
            {
                return Err(
                    "Término contratual inválido, anterior ao início ou à data do save".into(),
                );
            }
        }
        if edits.contains_key(key("squadNumber").as_str()) && value("squadNumber") > 0 {
            for other in db.people.iter().filter(|v| v.uid != p.uid) {
                if other.contracts.iter().enumerate().any(|(j, v)| {
                    if !same_entity(c, v) {
                        return false;
                    }
                    let key = format!("contract.{j}.squadNumber");
                    let number = plan
                        .iter()
                        .find(|e| e.uid == other.uid && e.attribute == key)
                        .map(|e| e.value)
                        .unwrap_or(v.squad_number as i32);
                    number == value("squadNumber")
                }) {
                    return Err(format!(
                        "Camisa {} já usada por UID {} neste vínculo",
                        value("squadNumber"),
                        other.uid
                    ));
                }
            }
        }
    }
    Ok(())
}
fn same_entity(a: &Contract, b: &Contract) -> bool {
    a.entity_type == b.entity_type && a.entity_index == b.entity_index
}

pub fn display_field(p: &Player, key: &str) -> Option<String> {
    let value = p.fields.get(key)?.value;
    if let Some(prefix) = key
        .strip_suffix(".year")
        .or_else(|| key.strip_suffix(".day"))
        && (prefix == "birth" || (prefix.starts_with("contract.") && prefix.ends_with(".expires")))
    {
        let date = Date {
            year: p.fields[format!("{prefix}.year").as_str()].value as u16,
            day: p.fields[format!("{prefix}.day").as_str()].value as u16,
        };
        if prefix != "birth" && is_monthly(&date) {
            return Some("Mensal".into());
        }
        return Some(
            date.month_day()
                .map(|(m, d)| format!("{d:02}/{m:02}/{:04}", date.year))
                .unwrap_or_else(|| "Não informado".into()),
        );
    }
    if key == "state.condition" || key == "state.fitness" {
        return Some(format!("{:.2}%", value as f64 / 100.0));
    }
    if key.ends_with(".weeklyWage") {
        return Some(format!("£{value} / semana"));
    }
    if key.starts_with("contract.")
        && [
            ".appearanceBonus",
            ".goalBonus",
            ".assistBonus",
            ".releaseClause",
        ]
        .iter()
        .any(|suffix| key.ends_with(suffix))
    {
        return Some(format!("£{value}"));
    }
    if key == "state.estimatedValue" || key == "state.saleValue" {
        if key == "state.saleValue" && value == -1 {
            return Some("Não especificado".into());
        }
        return Some(if value == -1 {
            "Automático".into()
        } else {
            format!("£{value}")
        });
    }
    if key.ends_with(".squadNumber") && value <= 0 {
        return Some("Sem número".into());
    }
    if key.ends_with(".squadStatus") {
        return Some(
            match value {
                -1 | 0 => "Não especificado",
                1 => "Indispensável",
                2 => "Importante",
                3 => "Rodízio",
                4 => "Apoio à equipe principal",
                5 => "Grande promessa",
                6 => "Jovem de qualidade",
                7 => "Dispensável",
                _ => "Desconhecido",
            }
            .into(),
        );
    }
    None
}

/// Gross weekly wages recorded on player contracts, keyed by contracting club index.
pub fn player_wages_by_club(db: &Database) -> std::collections::BTreeMap<usize, i64> {
    let mut wages = std::collections::BTreeMap::new();
    for person in db.people.iter().filter(|p| p.playing.is_some()) {
        for contract in &person.contracts {
            if contract.entity_type == 0
                && matches!(contract.kind, 0 | 1)
                && let Ok(index) = usize::try_from(contract.entity_index)
                && index < db.clubs.len()
            {
                *wages.entry(index).or_default() += i64::from(contract.weekly_wage);
            }
        }
    }
    wages
}

/// Weekly loan contributions that offset the owning club's wage obligation.
pub fn loan_wage_offsets_by_club(db: &Database) -> std::collections::BTreeMap<usize, i64> {
    let mut offsets = std::collections::BTreeMap::new();
    for (club, _, (_, owner), (_, loan)) in player_loan_wage_pairs(db) {
        *offsets.entry(club).or_default() +=
            loan_wage_contribution(owner.weekly_wage, loan.weekly_wage);
    }
    offsets
}

pub fn loan_wage_contribution(owner: i32, loan: i32) -> i64 {
    i64::from(loan.min(owner).max(0))
}

/// Each unambiguous player loan with the owning club index and both contract indices.
pub fn player_loan_wage_pairs(
    db: &Database,
) -> impl Iterator<Item = (usize, i32, (usize, &Contract), (usize, &Contract))> {
    db.people
        .iter()
        .filter(|p| p.playing.is_some())
        .filter_map(|person| {
            let mut owners = person
                .contracts
                .iter()
                .enumerate()
                .filter(|(_, c)| c.kind == 0 && c.entity_type == 0);
            let mut loans = person
                .contracts
                .iter()
                .enumerate()
                .filter(|(_, c)| c.kind == 1 && c.entity_type == 0);
            let owner = owners.next()?;
            let loan = loans.next()?;
            if owners.next().is_some()
                || loans.next().is_some()
                || owner.1.entity_index == loan.1.entity_index
            {
                return None;
            }
            let owner_index = usize::try_from(owner.1.entity_index).ok()?;
            let loan_index = usize::try_from(loan.1.entity_index).ok()?;
            (owner_index < db.clubs.len() && loan_index < db.clubs.len()).then_some((
                owner_index,
                person.uid,
                owner,
                loan,
            ))
        })
}
