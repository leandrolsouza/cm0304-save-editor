use crate::{Result, attributes, database::Player, editor::Edit};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Deserialize)]
pub struct Request {
    pub uids: Vec<i32>,
    #[serde(default)]
    pub attribute: String,
    pub operation: Operation,
}

#[derive(Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum Operation {
    Set(i32),
    Add(i32),
    Percent(i32),
    Floor(i32),
    Ceiling(i32),
    Recover,
    Copy(CopyAttributes),
    Adjust(Vec<FieldAdjustment>),
}

#[derive(Deserialize)]
pub struct FieldAdjustment {
    pub attribute: String,
    pub operation: Operation,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CopyAttributes {
    pub source_uid: i32,
    pub attributes: Vec<String>,
}

#[derive(Serialize)]
pub struct Target {
    pub uid: i32,
    pub name: String,
    pub attribute: String,
    pub before: i32,
    pub after: i32,
    pub limited: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub unavailable: bool,
}

pub struct HealthCounts<'a> {
    pub uid: i32,
    pub name: &'a str,
    pub injuries: usize,
    pub suspensions: usize,
}

pub fn project_health_removals(
    plan: &mut crate::transaction::Transaction,
    players: &[HealthCounts<'_>],
    heal_injuries: bool,
    clear_suspensions: bool,
) -> Vec<Target> {
    let mut targets = Vec::new();
    for player in players {
        for (count, selection, field) in [
            (
                heal_injuries.then_some(player.injuries),
                &mut plan.heal_injuries,
                "healInjuries",
            ),
            (
                clear_suspensions.then_some(player.suspensions),
                &mut plan.clear_suspensions,
                "clearSuspensions",
            ),
        ] {
            let Some(count) = count else { continue };
            let before = if count > 0 && selection.insert(player.uid) {
                count as i32
            } else {
                0
            };
            targets.push(Target {
                uid: player.uid,
                name: player.name.into(),
                attribute: field.into(),
                before,
                after: 0,
                limited: false,
                unavailable: false,
            });
        }
    }
    targets
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WageRequest {
    pub contracts: Vec<ContractSelection>,
    pub operation: Operation,
    pub ceiling: Option<i32>,
}

#[derive(Deserialize)]
pub struct ContractSelection {
    pub uid: i32,
    pub index: usize,
}

#[derive(Deserialize)]
pub struct ExpiryRequest {
    pub contracts: Vec<ContractSelection>,
    pub year: u16,
    pub day: u16,
}

#[derive(Deserialize)]
pub struct SquadStatusRequest {
    pub contracts: Vec<ContractSelection>,
    pub status: i32,
}

pub fn project_squad_status(
    players: &[Player],
    request: &SquadStatusRequest,
) -> Result<(Vec<Edit>, Vec<Target>)> {
    let mut edits = Vec::new();
    let mut targets = Vec::new();
    for (player, index) in selected_contracts(players, &request.contracts)? {
        let attribute = format!("contract.{index}.squadStatus");
        let field = player.fields.get(attribute.as_str()).ok_or_else(|| {
            format!(
                "Situação no elenco indisponível para {} no vínculo {index}",
                player.name
            )
        })?;
        let after = request.status;
        if !(field.min..=field.max).contains(&after) {
            return Err("Situação no elenco inválida".into());
        }
        if field.value != after {
            edits.push(Edit {
                uid: player.uid,
                attribute: attribute.clone(),
                value: after,
                policy: None,
            });
        }
        targets.push(Target {
            uid: player.uid,
            name: player.name.clone(),
            attribute,
            before: field.value,
            after,
            limited: false,
            unavailable: false,
        });
    }
    Ok((edits, targets))
}

pub fn project_expiries(
    players: &[Player],
    request: &ExpiryRequest,
) -> Result<(Vec<Edit>, Vec<Target>)> {
    let date = crate::database::Date {
        year: request.year,
        day: request.day,
    };
    if !(1900..=2200).contains(&date.year)
        || date.month_day().is_none()
        || crate::career::is_monthly(&date)
    {
        return Err("Informe um vencimento válido com data fixa".into());
    }
    let mut edits = Vec::new();
    let mut targets = Vec::new();
    for (player, index) in selected_contracts(players, &request.contracts)? {
        for (suffix, after) in [("year", date.year), ("day", date.day)] {
            let attribute = format!("contract.{index}.expires.{suffix}");
            let before = player
                .fields
                .get(attribute.as_str())
                .ok_or_else(|| {
                    format!(
                        "Vencimento indisponível para {} no vínculo {index}",
                        player.name
                    )
                })?
                .value;
            let after = i32::from(after);
            if before != after {
                edits.push(Edit {
                    uid: player.uid,
                    attribute: attribute.clone(),
                    value: after,
                    policy: None,
                });
            }
            targets.push(Target {
                uid: player.uid,
                name: player.name.clone(),
                attribute,
                before,
                after,
                limited: false,
                unavailable: false,
            });
        }
    }
    Ok((edits, targets))
}

fn selected_contracts<'a>(
    players: &'a [Player],
    contracts: &[ContractSelection],
) -> Result<Vec<(&'a Player, usize)>> {
    if contracts.is_empty() {
        return Err("Selecione pelo menos um vínculo".into());
    }
    let mut index = BTreeMap::new();
    for player in players {
        if index.insert(player.uid, player).is_some() {
            return Err("UID de jogador repetido no save".into());
        }
    }
    let mut selected = BTreeSet::new();
    contracts
        .iter()
        .map(|contract| {
            let uid = contract.uid;
            if !selected.insert((uid, contract.index)) {
                return Err(format!(
                    "Vínculo repetido: UID {uid}, índice {}",
                    contract.index
                ));
            }
            let player = index
                .get(&uid)
                .ok_or_else(|| format!("Jogador {uid} não encontrado"))?;
            Ok((*player, contract.index))
        })
        .collect()
}

pub fn project_wages(
    players: &[Player],
    request: &WageRequest,
) -> Result<(Vec<Edit>, Vec<Target>)> {
    let mut edits = Vec::new();
    let mut targets = Vec::new();
    for (player, index) in selected_contracts(players, &request.contracts)? {
        let uid = player.uid;
        let attribute = format!("contract.{index}.weeklyWage");
        let field = player.fields.get(attribute.as_str()).ok_or_else(|| {
            format!(
                "Salário indisponível para {} no vínculo {}",
                player.name, index
            )
        })?;
        let ceiling = request.ceiling.unwrap_or(field.max);
        if !(field.min..=field.max).contains(&ceiling) {
            return Err("Teto salarial fora do intervalo permitido".into());
        }
        let requested = numeric_target(field.value, &request.operation)
            .ok_or("Operação incompatível com edição de salários")?;
        let after = requested.clamp(i64::from(field.min), i64::from(ceiling)) as i32;
        if after != field.value {
            edits.push(Edit {
                uid,
                attribute: attribute.clone(),
                value: after,
                policy: None,
            });
        }
        targets.push(Target {
            uid,
            name: player.name.clone(),
            attribute,
            before: field.value,
            after,
            limited: i64::from(after) != requested,
            unavailable: false,
        });
    }
    Ok((edits, targets))
}

fn numeric_target(before: i32, operation: &Operation) -> Option<i64> {
    let before = i64::from(before);
    Some(match *operation {
        Operation::Recover | Operation::Copy(_) | Operation::Adjust(_) => return None,
        Operation::Set(v) => i64::from(v),
        Operation::Add(v) => before + i64::from(v),
        Operation::Percent(v) => {
            let scaled = before * (100 + i64::from(v));
            // Round to the nearest integer, with ties away from zero.
            (scaled + 50 * scaled.signum()) / 100
        }
        Operation::Floor(v) => before.max(i64::from(v)),
        Operation::Ceiling(v) => before.min(i64::from(v)),
    })
}

fn adjustable_player_field(key: &str) -> bool {
    attributes::FIELDS[12..].contains(&key)
        || key
            .strip_prefix("personality.")
            .is_some_and(|field| crate::database::PERSONALITY_FIELDS.contains(&field))
        || matches!(
            key,
            "state.condition" | "state.fitness" | "state.jadedness" | "state.morale"
        )
}

pub fn project(players: &[Player], request: &Request) -> Result<(Vec<Edit>, Vec<Target>)> {
    if request.uids.is_empty() {
        return Err("Selecione pelo menos um jogador".into());
    }
    let index = players
        .iter()
        .map(|p| (p.uid, p))
        .collect::<BTreeMap<_, _>>();
    if index.len() != players.len() {
        return Err("UID de jogador repetido no save".into());
    }
    let full_fitness = Operation::Set(10000);
    let rested = Operation::Ceiling(0);
    let high_morale = Operation::Floor(18);
    let mut copy_values = Vec::new();
    let fields = match &request.operation {
        Operation::Adjust(adjustments) => {
            if adjustments.is_empty() {
                return Err("Adicione pelo menos um ajuste ao lote".into());
            }
            let mut seen = BTreeSet::new();
            adjustments
                .iter()
                .map(|adjustment| {
                    let key = adjustment.attribute.as_str();
                    if !adjustable_player_field(key) || !seen.insert(key) {
                        return Err(format!("Campo repetido ou incompatível com o lote: {key}"));
                    }
                    if matches!(
                        adjustment.operation,
                        Operation::Recover | Operation::Copy(_) | Operation::Adjust(_)
                    ) {
                        return Err(format!("Escolha uma operação numérica para {key}"));
                    }
                    Ok((key, &adjustment.operation))
                })
                .collect::<Result<Vec<_>>>()?
        }
        Operation::Recover => vec![
            ("state.condition", &full_fitness),
            ("state.fitness", &full_fitness),
            ("state.jadedness", &rested),
            ("state.morale", &high_morale),
        ],
        Operation::Copy(copy) => {
            let source = index
                .get(&copy.source_uid)
                .ok_or("Jogador de origem não encontrado")?;
            if copy.attributes.is_empty() {
                return Err("Selecione pelo menos um atributo para copiar".into());
            }
            let mut seen = BTreeSet::new();
            for key in &copy.attributes {
                if !attributes::FIELDS[12..].contains(&key.as_str()) || !seen.insert(key) {
                    return Err(format!(
                        "Atributo repetido ou incompatível com cópia: {key}"
                    ));
                }
                let value = source
                    .fields
                    .get(key.as_str())
                    .ok_or_else(|| format!("Atributo {key} indisponível na origem"))?
                    .value;
                copy_values.push((key.as_str(), Operation::Set(value)));
            }
            copy_values
                .iter()
                .map(|(key, operation)| (*key, operation))
                .collect()
        }
        operation if adjustable_player_field(&request.attribute) => {
            vec![(request.attribute.as_str(), operation)]
        }
        _ => return Err("Selecione um campo de jogador compatível com edição em lote".into()),
    };
    let mut selected = BTreeSet::new();
    let mut edits = Vec::new();
    let mut targets = Vec::new();
    for &uid in &request.uids {
        if !selected.insert(uid) {
            return Err(format!("Jogador {uid} selecionado mais de uma vez"));
        }
        let player = index
            .get(&uid)
            .ok_or_else(|| format!("Jogador {uid} não encontrado"))?;
        for &(attribute, operation) in &fields {
            let Some(field) = player.fields.get(attribute) else {
                targets.push(Target {
                    uid,
                    name: player.name.clone(),
                    attribute: attribute.to_owned(),
                    before: 0,
                    after: 0,
                    limited: false,
                    unavailable: true,
                });
                continue;
            };
            let requested = numeric_target(field.value, operation)
                .ok_or("Operação incompatível com o campo selecionado")?;
            let after = requested.clamp(i64::from(field.min), i64::from(field.max)) as i32;
            if matches!(request.operation, Operation::Copy(_)) && i64::from(after) != requested {
                return Err(format!(
                    "{}: valor {requested} de {attribute} fora do intervalo do destino",
                    player.name
                ));
            }
            if after != field.value {
                edits.push(Edit {
                    uid,
                    attribute: attribute.to_owned(),
                    value: after,
                    policy: None,
                });
            }
            targets.push(Target {
                uid,
                name: player.name.clone(),
                attribute: attribute.to_owned(),
                before: field.value,
                after,
                limited: i64::from(after) != requested,
                unavailable: false,
            });
        }
    }
    if targets.iter().any(|target| target.unavailable) {
        edits.clear();
    }
    Ok((edits, targets))
}
