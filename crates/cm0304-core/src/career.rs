//! Editable fixed-size career fields. Contract identity and structure stay intact.
use crate::{
    Result,
    database::{Contract, Database, Date, Field, Person, Player},
    editor::Edit,
};
use serde::Serialize;
use std::collections::BTreeMap;

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
            entity: if c.entity_type == 0 {
                db.clubs.get(c.entity_index as usize)
            } else {
                db.nations.get(c.entity_index as usize)
            }
            .map(|v| v.name.clone()),
            started: c.started.clone(),
            monthly: is_monthly(&c.expires),
        })
        .collect()
}
pub fn is_monthly(date: &Date) -> bool {
    date.year == 1900 && date.day == 1
}

pub fn add_fields(person: &Person, base: usize, fields: &mut BTreeMap<String, Field>) {
    let Some(d) = &person.playing else { return };
    let s = &d.state;
    let mut add = |key: String, value, offset, width, min, max| {
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
        ("condition", s.condition as i32, 28, 2, 0, 10000),
        ("fitness", s.fitness as i32, 24, 2, 0, 10000),
        ("jadedness", s.jadedness as i32, 26, 2, -1500, 1000),
        ("morale", s.morale as i32, 22, 1, 0, 20),
        ("estimatedValue", s.estimated_value, 18, 4, 0, 1_000_000_000),
        ("saleValue", s.sale_value, 42, 4, -1, 1_000_000_000),
    ] {
        add(
            format!("state.{name}"),
            value,
            d.offset + offset,
            width,
            min,
            max,
        );
    }
    for (i, c) in person.contracts.iter().enumerate() {
        for (name, value, offset, width, min, max) in [
            ("weeklyWage", c.weekly_wage, 5, 4, 0, 1_000_000),
            ("expires.day", c.expires.day as i32, 9, 2, 0, 365),
            ("expires.year", c.expires.year as i32, 11, 2, 1900, 2200),
            ("squadStatus", c.squad_status as i32, 13, 1, -1, 7),
            ("squadNumber", c.squad_number as i32, 30, 1, -1, 99),
        ] {
            add(
                format!("contract.{i}.{name}"),
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
                .unwrap_or(p.fields[&k].value)
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
            for other in db
                .people
                .iter()
                .filter(|v| v.uid != p.uid && v.team == person.team && person.team >= 0)
            {
                if other.contracts.iter().enumerate().any(|(j, v)| {
                    let key = format!("contract.{j}.squadNumber");
                    let number = plan
                        .iter()
                        .find(|e| e.uid == other.uid && e.attribute == key)
                        .map(|e| e.value)
                        .unwrap_or(v.squad_number as i32);
                    same_entity(c, v) && number == value("squadNumber")
                }) {
                    return Err(format!(
                        "Camisa {} já usada por UID {} nesta equipe",
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
            year: p.fields[&format!("{prefix}.year")].value as u16,
            day: p.fields[&format!("{prefix}.day")].value as u16,
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
    if key == "state.estimatedValue" || key == "state.saleValue" {
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
