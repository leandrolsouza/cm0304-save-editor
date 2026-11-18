use cm0304_core::{
    attributes,
    binary::Reader,
    container::{hash, read_container},
    database,
    editor::{self, Edit},
};
use std::{fs, path::PathBuf, sync::atomic::AtomicBool};

fn sample(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../game/user data/games")
        .join(name)
}
fn edits() -> Vec<Edit> {
    vec![
        Edit {
            uid: 2001642,
            attribute: "passing".into(),
            value: 20,
        },
        Edit {
            uid: 2001642,
            attribute: "flair".into(),
            value: 20,
        },
        Edit {
            uid: 9888,
            attribute: "pace".into(),
            value: 19,
        },
    ]
}

#[test]
fn rejects_truncated_and_unknown_formats() {
    for n in 0..12 {
        assert!(read_container(&vec![0; n]).is_err());
    }
    assert!(read_container(&[0; 12]).is_err());
    assert!(Reader::new(&[2]).boolean().is_err());
    let mut r = Reader::new(&[255, 255, 255, 255]);
    assert!(r.string().is_err());
    assert!(database::read_database(&[0; 11]).is_err());
}
#[test]
fn inverse_roundtrips_and_rejects_unreachable_targets() {
    for ca in 0..=200 {
        for low in [true, false] {
            for value in 1..=20 {
                let possible = (-128i16..=127)
                    .any(|raw| attributes::convert(raw as i8, ca, low).unwrap() == value);
                let encoded = attributes::encode(value, ca, low, -30);
                if possible {
                    assert_eq!(
                        attributes::convert(encoded.unwrap(), ca, low).unwrap(),
                        value
                    );
                } else {
                    assert!(encoded.is_err());
                }
            }
        }
    }
}
#[test]
fn real_saves_parse_and_resolve_every_player() {
    for (name, blocks, people, end) in [
        ("save_inicial.cm4", 87, 32788, 19504119),
        ("save_modificado.cm4", 267, 81528, 20664639),
    ] {
        let bytes = fs::read(sample(name)).unwrap();
        let original = hash(&bytes);
        let container = read_container(&bytes).unwrap();
        assert_eq!(container.blocks.len(), blocks);
        let (db, base) = database::load(&bytes).unwrap();
        assert_eq!(db.people.len(), people);
        assert_eq!(db.sections.len(), 23);
        assert_eq!(db.parsed_until, end);
        let players = db.players(base).unwrap();
        assert_eq!(players.len(), 22062);
        let zidane = players.iter().find(|p| p.uid == 8076).unwrap();
        assert_eq!(zidane.name, "Zinedine Zidane");
        assert_eq!(zidane.club.as_deref(), Some("Real Madrid C.F."));
        for p in &players {
            assert_eq!(p.raw_attributes.len(), 65);
            assert_eq!(
                i16::from_le_bytes(
                    bytes[p.offsets.current_ability..p.offsets.current_ability + 2]
                        .try_into()
                        .unwrap()
                ),
                p.current_ability
            );
        }
        assert_eq!(hash(&bytes), original);
        let b = container
            .blocks
            .iter()
            .find(|b| b.name == "game_db")
            .unwrap();
        assert!(
            database::read_database(&bytes[b.payload_offset..b.payload_offset + end - 1]).is_err()
        );
    }
}
#[test]
fn multi_player_transaction_changes_only_planned_bytes() {
    let input = fs::read(sample("save_inicial.cm4")).unwrap();
    let original = hash(&input);
    let (output, report) = editor::apply(&input, &edits()).unwrap();
    assert_eq!(report.changed_bytes, 3);
    assert_eq!(report.changes.len(), 3);
    assert_eq!(input.len(), output.len());
    assert_eq!(hash(&input), original);
    let differences = input
        .iter()
        .zip(&output)
        .enumerate()
        .filter_map(|(i, (a, b))| (a != b).then_some(i))
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        differences,
        report.changes.iter().map(|c| c.offset).collect()
    );
    let (db, base) = database::load(&output).unwrap();
    let players = db.players(base).unwrap();
    for e in edits() {
        assert_eq!(
            players.iter().find(|p| p.uid == e.uid).unwrap().attributes[&e.attribute],
            e.value
        );
    }
    assert_eq!(
        players
            .iter()
            .find(|p| p.uid == 2001642)
            .unwrap()
            .current_ability,
        142
    );
}
#[test]
fn invalid_plan_is_rejected_as_a_whole() {
    let bytes = fs::read(sample("save_inicial.cm4")).unwrap();
    let mut plan = edits();
    plan.push(plan[0].clone());
    assert!(editor::apply(&bytes, &plan).is_err());
    let mut plan = edits();
    plan[2].uid = i32::MAX;
    assert!(editor::apply(&bytes, &plan).is_err());
    let mut plan = edits();
    plan[2].attribute = "goalkeeper".into();
    assert!(editor::apply(&bytes, &plan).is_err());
    assert!(editor::apply(&bytes, &[]).is_err());
}
#[test]
fn copy_is_exclusive_cancellable_and_cleans_up_temporary_files() {
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("source.cm4");
    fs::copy(sample("save_inicial.cm4"), &source).unwrap();
    let before = fs::read(&source).unwrap();
    let dest = temp.path().join("copy.cm4");
    assert!(editor::save_copy(&source, &dest, &edits(), &AtomicBool::new(true)).is_err());
    assert!(!dest.exists());
    assert!(
        editor::save_copy(
            &source,
            &temp.path().join("missing/copy.cm4"),
            &edits(),
            &AtomicBool::new(false)
        )
        .is_err()
    );
    editor::save_copy(&source, &dest, &edits(), &AtomicBool::new(false)).unwrap();
    let copy = fs::read(&dest).unwrap();
    assert!(editor::save_copy(&source, &dest, &edits(), &AtomicBool::new(false)).is_err());
    assert_eq!(fs::read(&dest).unwrap(), copy);
    assert_eq!(fs::read(&source).unwrap(), before);
    assert_eq!(fs::read_dir(temp.path()).unwrap().count(), 2);
    assert!(editor::save_copy(&source, &source, &edits(), &AtomicBool::new(false)).is_err());
    fs::write(&source, b"changed while the editing session was open").unwrap();
    let another = temp.path().join("stale.cm4");
    assert!(
        editor::save_snapshot(
            &source,
            &another,
            &edits(),
            &before,
            &AtomicBool::new(false)
        )
        .is_err()
    );
    assert!(!another.exists());
}
