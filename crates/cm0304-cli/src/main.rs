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
        return Err("Usage: cm0304-cli inspect <save> | players <save> [query] | career <save> [query] | health <save> [query] | injury-types <save> | career-summary <save> | export <save> <json> | edit <save> <new-copy> <fields.json> | apply-plan <save> <new-copy> <transaction.json>".into());
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
    let mut players = db.players(base)?;
    match command {
        "players" | "career" | "health" => {
            if let Some(q) = a.get(2) {
                let q = search::key(q);
                players.retain(|p| {
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
                    Some(serde_json::json!({"uid":p.uid,"name":p.name,"club":p.club,"databaseBase":base,"records":health,"injuryCandidates":candidates}))
                }).collect::<Vec<_>>();
                println!(
                    "{}",
                    serde_json::to_string_pretty(&rows).map_err(|e| e.to_string())?
                );
            } else if command == "career" {
                let rows = players.iter().map(|p| {
                    let person = db.people.iter().find(|v| v.uid == p.uid).unwrap();
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
