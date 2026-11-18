use crate::Result;
use std::collections::BTreeMap;

pub const FIELDS: [&str; 65] = [
    "goalkeeper",
    "sweeper",
    "defender",
    "defensiveMidfielder",
    "midfielder",
    "attackingMidfielder",
    "attacker",
    "wingBack",
    "freeRole",
    "rightSide",
    "leftSide",
    "central",
    "crossing",
    "dribbling",
    "finishing",
    "heading",
    "longShots",
    "marking",
    "offTheBall",
    "passing",
    "penalties",
    "tackling",
    "creativity",
    "handling",
    "aerialAbility",
    "commandOfArea",
    "communication",
    "kicking",
    "throwing",
    "anticipation",
    "decisions",
    "oneOnOnes",
    "positioning",
    "reflexes",
    "throwIns",
    "firstTouch",
    "technique",
    "leftFoot",
    "rightFoot",
    "flair",
    "corners",
    "teamwork",
    "workRate",
    "longThrows",
    "eccentricity",
    "rushingOut",
    "tendencyToPunch",
    "acceleration",
    "setPieces",
    "strength",
    "stamina",
    "pace",
    "jumping",
    "influence",
    "dirtiness",
    "balance",
    "bravery",
    "consistency",
    "aggression",
    "agility",
    "importantMatches",
    "injuryProneness",
    "versatility",
    "naturalFitness",
    "determination",
];
pub const DIRECT: [&str; 18] = [
    "flair",
    "corners",
    "teamwork",
    "workRate",
    "longThrows",
    "acceleration",
    "setPieces",
    "strength",
    "stamina",
    "pace",
    "jumping",
    "influence",
    "balance",
    "bravery",
    "aggression",
    "agility",
    "naturalFitness",
    "determination",
];
pub type Attributes = BTreeMap<String, i8>;
pub fn mode(index: usize, goalkeeper: i8) -> Option<bool> {
    match index {
        12..=22 => Some(goalkeeper >= 15),
        23..=28 => Some(goalkeeper < 15),
        29..=36 => Some(false),
        _ => None,
    }
}
pub fn convert(raw: i8, ca: i16, low: bool) -> Result<i8> {
    if !(0..=200).contains(&ca) {
        return Err(format!("Invalid CA {ca}"));
    }
    let t = ((raw as f64 * 2.0 + ca as f64 * if low { 0.1 } else { 1.0 }) * 0.05) + 10.0;
    Ok((((t * t * 0.1 + t) / 3.0 + 0.5).trunc() as i32).clamp(1, 20) as i8)
}
pub fn encode(value: i8, ca: i16, low: bool, original: i8) -> Result<i8> {
    if !(1..=20).contains(&value) {
        return Err("Attribute must be 1–20".into());
    }
    convert(original, ca, low)?;
    (-128i16..=127)
        .filter_map(|v| {
            let raw = v as i8;
            (convert(raw, ca, low).ok() == Some(value)).then_some(raw)
        })
        .min_by_key(|v| (*v as i16 - original as i16).abs())
        .ok_or_else(|| format!("Target {value} not representable at CA {ca}"))
}
pub fn decoded(raw: &[i8; 65], ca: i16) -> Result<Attributes> {
    FIELDS
        .iter()
        .enumerate()
        .map(|(i, key)| {
            Ok((
                key.to_string(),
                match mode(i, raw[0]) {
                    Some(low) => convert(raw[i], ca, low)?,
                    None => raw[i],
                },
            ))
        })
        .collect()
}
