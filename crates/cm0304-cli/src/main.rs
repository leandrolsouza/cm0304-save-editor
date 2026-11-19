use cm0304_core::{
    Result,
    container::read_container,
    database,
    editor::{Edit, save_copy},
    search, transaction,
};
use std::{env, fs, path::Path, sync::atomic::AtomicBool};

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
fn run() -> Result<()> {
    let a = env::args().skip(1).collect::<Vec<_>>();
    let command = a.first().map(String::as_str).unwrap_or("");
    if command == "staff-ability" && a.len() == 5 {
        let uid = a[2]
            .parse::<i32>()
            .map_err(|_| "UID inválido".to_string())?;
        let ca = a[3].parse::<i16>().map_err(|_| "CA inválida".to_string())?;
        let policy = match a[4].as_str() {
            "preserveValues" => cm0304_core::editor::AbilityPolicy::PreserveValues,
            "preserveRelative" => cm0304_core::editor::AbilityPolicy::PreserveRelative,
            _ => return Err("Escolha preserveValues ou preserveRelative".into()),
        };
        let input = fs::read(&a[1]).map_err(|e| e.to_string())?;
        let (db, base) = database::load(&input)?;
        let profiles = cm0304_core::staff::profiles(&db, &input, base)?;
        let profile = profiles
            .iter()
            .find(|p| p.uid == uid)
            .ok_or("Staff não encontrado")?;
        let projection =
            cm0304_core::staff_editor::project_ability(profile, ca, policy, &Default::default())?;
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "sourceSha256":cm0304_core::container::hash(&input),"uid":uid,"name":profile.name,
                "policy":policy,"projection":projection,"writesSave":false
            }))
            .map_err(|e| e.to_string())?
        );
        return Ok(());
    }
    if command == "apply-plan" && a.len() == 4 {
        let plan: transaction::Transaction =
            serde_json::from_slice(&fs::read(&a[3]).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        let input = fs::read(&a[1]).map_err(|e| e.to_string())?;
        let report = transaction::save_snapshot(
            Path::new(&a[1]),
            Path::new(&a[2]),
            &plan,
            &input,
            &AtomicBool::new(false),
        )?;
        println!(
            "{}",
            serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?
        );
        return Ok(());
    }
    if command == "edit" && a.len() == 4 {
        let edits: Vec<Edit> = serde_json::from_slice(&fs::read(&a[3]).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        let report = save_copy(
            Path::new(&a[1]),
            Path::new(&a[2]),
            &edits,
            &AtomicBool::new(false),
        )?;
        println!(
            "{}",
            serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?
        );
        return Ok(());
    }
    if a.len() < 2 {
        return Err("Usage: cm0304-cli staff-ability <save> <uid> <ca> <preserveValues|preserveRelative> | inspect <save> | clubs <save> [query or uid:number] | club-data <save> [query or uid:number] | staff <save> [query or uid:number] | players <save> [query] | career <save> [query] | health <save> [query] | health-cases <save> | injury-types <save> | career-summary <save> | export <save> <json> | edit <save> <new-copy> <fields.json> | apply-plan <save> <new-copy> <transaction.json>".into());
    }
    let bytes = fs::read(&a[1]).map_err(|e| e.to_string())?;
    if command == "inspect" {
        println!(
            "{}",
            serde_json::to_string_pretty(&read_container(&bytes)?).map_err(|e| e.to_string())?
        );
        return Ok(());
    }
    let (db, base) = database::load(&bytes)?;
    if command == "club-data" {
        let q = a.get(2).map(|q| search::key(q));
        let uid = q
            .as_deref()
            .and_then(|q| q.strip_prefix("uid:"))
            .map(|n| n.parse::<i32>().map_err(|_| "UID de clube inválido"))
            .transpose()?;
        let wages = cm0304_core::career::player_wages_by_club(&db);
        let loan_offsets = cm0304_core::career::loan_wage_offsets_by_club(&db);
        let rows = db
            .clubs
            .iter()
            .enumerate()
            .filter(|(_, c)| {
                uid.map_or_else(
                    || {
                        q.as_ref().is_none_or(|q| {
                            search::key(&format!("{} {} {}", c.uid, c.name, c.short_name))
                                .contains(q)
                        })
                    },
                    |uid| c.uid == uid,
                )
            })
            .map(|(index, c)| {
                let gross = wages.get(&index).copied().unwrap_or_default();
                let loan_offset = loan_offsets.get(&index).copied().unwrap_or_default();
                serde_json::json!({
                    "uid": c.uid,
                    "name": c.name,
                    "details": c.details,
                    "playerContractWagesWeekly": gross,
                    "loanWageOffsetWeekly": loan_offset,
                    "estimatedPlayerWagesWeekly": gross - loan_offset,
                })
            })
            .collect::<Vec<_>>();
        println!("{}",serde_json::to_string_pretty(&serde_json::json!({"sourceSha256":cm0304_core::container::hash(&bytes),"databaseBase":base,"gameDate":db.game_date,"clubs":rows})).map_err(|e|e.to_string())?);
        return Ok(());
    }
    if command == "clubs" {
        let mut rows = cm0304_core::clubs::profiles(&db)?;
        if let Some(query) = a.get(2) {
            if let Some(uid) = query.strip_prefix("uid:") {
                let uid = uid.parse::<i32>().map_err(|_| "UID de clube inválido")?;
                rows.retain(|c| c.uid == uid);
            } else {
                let q = search::key(query);
                rows.retain(|c| {
                    search::key(&format!("{} {} {}", c.uid, c.name, c.short_name)).contains(&q)
                });
            }
        }
        println!(
            "{}",
            serde_json::to_string_pretty(&rows).map_err(|e| e.to_string())?
        );
        return Ok(());
    }
    if command == "injury-types" {
        println!(
            "{}",
            serde_json::to_string_pretty(&db.injury_types).map_err(|e| e.to_string())?
        );
        return Ok(());
    }
    if command == "career-summary" {
        let mut histograms: std::collections::BTreeMap<
            &str,
            std::collections::BTreeMap<i32, usize>,
        > = Default::default();
        for p in &db.people {
            if let Some(d) = &p.playing {
                for (key, value) in [
                    ("morale", d.state.morale as i32),
                    ("jadedness", d.state.jadedness as i32),
                    ("condition", d.state.condition as i32),
                    ("fitness", d.state.fitness as i32),
                ] {
                    *histograms.entry(key).or_default().entry(value).or_default() += 1;
                }
            }
        }
        let summary = histograms.iter().map(|(key, values)| (*key, serde_json::json!({"min":values.keys().next(),"max":values.keys().next_back(),"distinct":values.len(),"values":if values.len()<=30 {Some(values)} else {None}}))).collect::<std::collections::BTreeMap<_, _>>();
        println!(
            "{}",
            serde_json::to_string_pretty(&summary).map_err(|e| e.to_string())?
        );
        return Ok(());
    }
    if command == "staff" {
        let mut rows = cm0304_core::staff::profiles(&db, &bytes, base)?;
        if let Some(query) = a.get(2) {
            if let Some(uid) = query.strip_prefix("uid:") {
                let uid = uid.parse::<i32>().map_err(|_| "UID inválido")?;
                rows.retain(|p| p.uid == uid);
            } else {
                let query = search::key(query);
                rows.retain(|p| {
                    search::key(&format!(
                        "{} {} {} {} {} {}",
                        p.uid,
                        p.name,
                        p.team.as_deref().unwrap_or(""),
                        p.club_job,
                        p.national_team.as_deref().unwrap_or(""),
                        p.national_job,
                    ))
                    .contains(&query)
                });
            }
        }
        println!(
            "{}",
            serde_json::to_string_pretty(&rows).map_err(|e| e.to_string())?
        );
        return Ok(());
    }
    let mut players = db.players(base)?;
    if command == "health-cases" {
        // Group by observed structure only; raw discriminator semantics remain unconfirmed.
        let index = players
            .iter()
            .map(|p| (p.uid, p))
            .collect::<std::collections::BTreeMap<_, _>>();
        let mut groups = std::collections::BTreeMap::<
            (String, usize, u8),
            (usize, Vec<serde_json::Value>),
        >::new();
        for person in &db.people {
            let Some(playing) = &person.playing else {
                continue;
            };
            let Some(player) = index.get(&person.uid) else {
                continue;
            };
            for (kind, records) in [
                ("injury", &playing.health_records.injuries),
                ("suspension", &playing.health_records.bans),
            ] {
                for record in records {
                    let discriminator_offset = if kind == "injury" { 10 } else { 0 };
                    let key = (
                        kind.to_owned(),
                        record.bytes.len(),
                        record.bytes[discriminator_offset],
                    );
                    let group = groups.entry(key).or_default();
                    group.0 += 1;
                    if group.1.len() < 3 {
                        group.1.push(serde_json::json!({"uid":player.uid,"name":player.name,"club":player.club,"record":record}));
                    }
                }
            }
        }
        let variants = groups.into_iter().map(|((kind, length, discriminator), (count, examples))| serde_json::json!({"discriminatorOffset":if kind == "injury" {10} else {0},"kind":kind,"recordBytes":length,"rawDiscriminator":discriminator,"count":count,"examples":examples})).collect::<Vec<_>>();
        println!("{}", serde_json::to_string_pretty(&serde_json::json!({"sourceSha256":cm0304_core::container::hash(&bytes),"databaseBase":base,"variants":variants})).map_err(|e| e.to_string())?);
        return Ok(());
    }
    match command {
        "players" | "career" | "health" => {
            if let Some(q) = a.get(2) {
                let exact_uid = q
                    .strip_prefix("uid:")
                    .map(|value| {
                        value
                            .parse::<i32>()
                            .map_err(|_| "UID inválido; use uid:1873".to_string())
                    })
                    .transpose()?;
                let q = search::key(q);
                players.retain(|p| {
                    if let Some(uid) = exact_uid {
                        return p.uid == uid;
                    }
                    search::key(&format!(
                        "{} {} {} {} {}",
                        p.uid,
                        p.name,
                        p.first_name,
                        p.surname,
                        p.club.as_deref().unwrap_or("")
                    ))
                    .contains(&q)
                });
            }
            if command == "health" {
                let rows = players.iter().filter_map(|p| {
                    let person = db.people.iter().find(|v| v.uid == p.uid)?;
                    let health = &person.playing.as_ref()?.health_records;
                    if a.get(2).is_none() && health.injuries.is_empty() && health.bans.is_empty() && health.injury_status.is_none() && health.bookings.is_empty() { return None; }
                    let candidates = health.injuries.iter().filter_map(|record| {
                        let b = &record.bytes;
                        let index = i32::from_le_bytes(b.get(..4)?.try_into().ok()?);
                        let injury = db.injury_types.get(usize::try_from(index).ok()?);
                        let tail = b.get(b.len().checked_sub(10)?..)?;
                        let word = |i| u16::from_le_bytes([tail[i], tail[i+1]]);
                        Some(serde_json::json!({"offset":record.offset,"injuryIndexCandidate":index,
                            "typeCandidate":injury,"dateCandidate":{"day":word(0),"year":word(2)},
                            "durationWordsUnconfirmed":[word(4),word(6)],"flagsUnconfirmed":[tail[8],tail[9]]}))
                    }).collect::<Vec<_>>();
                    Some(serde_json::json!({"uid":p.uid,"name":p.name,"club":p.club,"databaseBase":base,"records":health,"injuryViews":cm0304_core::health::injuries(&db,person),"injuryCandidates":candidates}))
                }).collect::<Vec<_>>();
                println!(
                    "{}",
                    serde_json::to_string_pretty(&rows).map_err(|e| e.to_string())?
                );
            } else if command == "career" {
                let people_by_uid = db
                    .people
                    .iter()
                    .map(|person| (person.uid, person))
                    .collect::<std::collections::BTreeMap<_, _>>();
                let rows = players.iter().map(|p| {
                    let person = people_by_uid[&p.uid];
                    let state = &person.playing.as_ref().unwrap().state;
                    let contracts = person.contracts.iter().map(|c| {
                        let entity = if c.entity_type == 0 { db.clubs.get(c.entity_index as usize) } else { db.nations.get(c.entity_index as usize) };
                        serde_json::json!({"contract":c, "entity":entity.map(|e| &e.name)})
                    }).collect::<Vec<_>>();
                    serde_json::json!({"uid":p.uid,"name":p.name,"club":p.club,"databaseBase":base,"state":state,"contracts":contracts})
                }).collect::<Vec<_>>();
                println!(
                    "{}",
                    serde_json::to_string_pretty(&rows).map_err(|e| e.to_string())?
                );
            } else {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&players).map_err(|e| e.to_string())?
                );
            }
        }
        "export" if a.len() == 3 => {
            fs::write(
                &a[2],
                serde_json::to_vec(&players).map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
            println!("Exported {} players", players.len());
        }
        _ => return Err("Unknown command or invalid arguments".into()),
    }
    Ok(())
}
