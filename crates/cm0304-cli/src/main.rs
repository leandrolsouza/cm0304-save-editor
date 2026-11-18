use cm0304_core::{
    Result,
    container::read_container,
    database,
    editor::{Edit, save_copy},
    search,
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
        return Err("Usage: cm0304-cli inspect <save> | players <save> [query] | career <save> [query] | export <save> <json> | edit <save> <new-copy> <plan.json>".into());
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
    let mut players = db.players(base)?;
    match command {
        "players" | "career" => {
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
            if command == "career" {
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
