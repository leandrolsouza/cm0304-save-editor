use crate::{
    Result,
    attributes::{self, Attributes, FIELDS},
    binary::{Reader, check_type},
    container::read_container,
};
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize)]
pub struct Date {
    pub day: u16,
    pub year: u16,
}
impl Date {
    pub fn month_day(&self) -> Option<(u16, u16)> {
        let leap = self.year.is_multiple_of(4)
            && (!self.year.is_multiple_of(100) || self.year.is_multiple_of(400));
        let mut day = self.day;
        for (month, days) in [
            31,
            if leap { 29 } else { 28 },
            31,
            30,
            31,
            30,
            31,
            31,
            30,
            31,
            30,
            31,
        ]
        .into_iter()
        .enumerate()
        {
            if day < days {
                return Some((month as u16 + 1, day + 1));
            }
            day -= days;
        }
        None
    }
    pub fn age_on(&self, today: &Date) -> Option<u16> {
        let birthday = self.month_day()?;
        let now = today.month_day()?;
        let age = i32::from(today.year) - i32::from(self.year) - i32::from(now < birthday);
        (0..=150).contains(&age).then_some(age as u16)
    }
}
fn date(r: &mut Reader) -> Result<Date> {
    Ok(Date {
        day: r.u16()?,
        year: r.u16()?,
    })
}
#[derive(Debug, Serialize)]
pub struct Section {
    pub name: String,
    pub offset: usize,
    pub count: usize,
    pub end: usize,
}
#[derive(Debug, Clone, Serialize)]
pub struct Club {
    pub name: String,
    pub short_name: String,
    pub id: i32,
    pub uid: i32,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InjuryType {
    pub name: String,
    pub category: u8,
    pub sentence_name: String,
    pub chance: u8,
    pub minimum_days_out: i16,
    pub extra_days_out: i16,
    pub inactive_ratio: u8,
    pub recurring: bool,
    pub severity: u8,
    pub id: i32,
    pub uid: i32,
}
#[derive(Debug)]
pub struct Team {
    pub container_type: u8,
    pub container_index: i32,
}
#[derive(Debug)]
pub struct Playing {
    pub offset: usize,
    pub raw: [i8; 65],
}
#[derive(Debug)]
pub struct PlayingInfo {
    pub offset: usize,
    pub ca: i16,
    pub pa: i16,
    pub playing_index: i32,
    pub reputation: [u16; 3],
    pub weight: i16,
    pub height: i16,
    pub state: PlayerState,
    pub health_records: HealthRecords,
}
/// Structural diagnostics only: records are not classified as active or historical.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthRecords {
    pub injury_count_offset: Option<usize>,
    pub ban_count_offset: Option<usize>,
    pub injury_status_flag_offset: Option<usize>,
    pub injuries: Vec<RecordSpan>,
    pub bans: Vec<RecordSpan>,
    pub injury_status: Option<RecordSpan>,
    pub bookings: Vec<RecordSpan>,
}
#[derive(Debug, Serialize)]
pub struct RecordSpan {
    /// Offset relative to game_db.dat, not the save container.
    pub offset: usize,
    pub bytes: Vec<u8>,
}
fn record_span(r: &Reader, start: usize) -> RecordSpan {
    RecordSpan {
        offset: start,
        bytes: r.data[start..r.pos].to_vec(),
    }
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerState {
    pub estimated_value: i32,
    pub sale_value: i32,
    pub morale: u8,
    pub fitness: i16,
    pub jadedness: i16,
    pub condition: i16,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub kind: u8,
    pub offset: usize,
    pub common_offset: usize,
    pub entity_type: u8,
    pub entity_index: i32,
    pub weekly_wage: i32,
    pub expires: Date,
    pub started: Date,
    pub squad_status: i8,
    pub transfer_status: u8,
    pub squad_number: i8,
}
#[derive(Debug)]
pub struct Person {
    pub index: usize,
    pub offset: usize,
    pub kind: u8,
    pub id: i32,
    pub uid: i32,
    pub first: i32,
    pub surname: i32,
    pub common: i32,
    pub birth: Option<Date>,
    pub common_offset: usize,
    pub nation: i32,
    pub team: i32,
    pub personality: i32,
    pub playing: Option<PlayingInfo>,
    pub contracts: Vec<Contract>,
}
#[derive(Debug)]
pub struct Database {
    pub version: u16,
    pub game_date: Date,
    pub sections: Vec<Section>,
    pub clubs: Vec<Club>,
    pub nations: Vec<Club>,
    pub injury_types: Vec<InjuryType>,
    pub teams: Vec<Team>,
    pub names: [Vec<String>; 3],
    pub people: Vec<Person>,
    pub personalities: Vec<Attributes>,
    pub personality_offsets: Vec<usize>,
    pub playing: Vec<Playing>,
    pub parsed_until: usize,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Offsets {
    pub person: usize,
    pub current_ability: usize,
    pub potential_ability: usize,
    pub attributes: usize,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub uid: i32,
    pub id: i32,
    #[serde(rename = "type")]
    pub kind: u8,
    pub first_name: String,
    pub surname: String,
    pub common_name: String,
    pub name: String,
    pub club: Option<String>,
    pub nation: Option<String>,
    pub date_of_birth: Date,
    pub current_ability: i16,
    pub potential_ability: i16,
    pub raw_attributes: Attributes,
    pub attributes: Attributes,
    pub personality: Option<Attributes>,
    pub offsets: Offsets,
    pub fields: BTreeMap<String, Field>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Field {
    pub value: i32,
    pub offset: usize,
    pub width: u8,
    pub min: i32,
    pub max: i32,
}

pub const PERSONALITY_FIELDS: [&str; 8] = [
    "adaptability",
    "ambition",
    "loyalty",
    "pressure",
    "professionalism",
    "sportsmanship",
    "temperament",
    "controversy",
];

fn table<T>(
    r: &mut Reader,
    sections: &mut Vec<Section>,
    name: &str,
    mut read: impl FnMut(&mut Reader, u8, usize) -> Result<T>,
) -> Result<Vec<T>> {
    let offset = r.pos;
    let count = r.count()?;
    let mut rows = Vec::new();
    for i in 0..count {
        let start = r.pos;
        let t = r.u8()?;
        rows.push(read(r, t, i).map_err(|e| format!("{name}[{i}] at {start}: {e}"))?);
    }
    sections.push(Section {
        name: name.into(),
        offset,
        count,
        end: r.pos,
    });
    Ok(rows)
}
fn fixed(r: &mut Reader, s: &mut Vec<Section>, name: &str, width: usize) -> Result<()> {
    let offset = r.pos;
    let count = r.count()?;
    r.skip(count.checked_mul(width).ok_or("Size overflow")?)?;
    s.push(Section {
        name: name.into(),
        offset,
        count,
        end: r.pos,
    });
    Ok(())
}
fn team_container(r: &mut Reader) -> Result<Club> {
    r.kind(&[0, 1])?;
    r.skip(2)?;
    let name = r.string()?;
    let short_name = r.string()?;
    r.array(6, 1)?;
    r.array(4, 1)?;
    r.array(8, 1)?;
    let (id, uid) = r.identity()?;
    Ok(Club {
        name,
        short_name,
        id,
        uid,
    })
}
pub fn read_database(bytes: &[u8]) -> Result<Database> {
    let mut r = Reader::new(bytes);
    let mut s = Vec::new();
    if r.take(9)? != [0, 46, 0, 100, 0, 97, 0, 116, 0] {
        return Err("Invalid game_db signature".into());
    }
    let version = r.u16()?;
    if version != 147 {
        return Err(format!("Unsupported game_db version {version}"));
    }
    r.skip(6)?;
    let game_date = date(&mut r)?;
    r.skip(16)?;
    fixed(&mut r, &mut s, "awards", 45)?;
    table(&mut r, &mut s, "cities", |r, _, _| {
        r.skip(18)?;
        r.string()?;
        r.skip(11)
    })?;
    let clubs = table(&mut r, &mut s, "clubs", |r, t, _| {
        check_type(t, &[0, 12], r.pos - 1)?;
        if t == 12 {
            r.skip(9)?;
        }
        r.skip(5)?;
        r.string()?;
        r.skip(1)?;
        r.string()?;
        r.skip(23)?;
        r.array(4, 1)?;
        r.array(4, 1)?;
        let finance = r.kind(&[0, 1, 2])?;
        r.skip(match finance {
            0 => 4,
            1 => 438,
            _ => 0,
        })?;
        if r.i32()? != 0 {
            r.skip(13)?;
        }
        r.array(4, 2)?;
        for _ in 0..r.u16()? {
            match r.u8()? {
                0 | 2 => r.skip(5)?,
                1 | 8 => r.skip(1)?,
                3 => {
                    if r.boolean()? {
                        r.array(4, 1)?;
                    }
                    r.skip(5)?;
                }
                4 => r.skip(16)?,
                5 => r.skip(10)?,
                6 => r.skip(15)?,
                7 => r.skip(12)?,
                t => return Err(format!("Unknown transfer info {t}")),
            }
        }
        r.array(6, 1)?;
        r.skip(12)?;
        r.array(4, 2)?;
        r.array(5, 1)?;
        r.skip(5)?;
        team_container(r)
    })?;
    table(&mut r, &mut s, "competitions", |r, _, _| {
        r.skip(11)?;
        let t = r.u8()?;
        r.skip(1)?;
        if t == 4 {
            r.string()?;
            r.string()?;
        }
        r.skip(29)
    })?;
    fixed(&mut r, &mut s, "continents", 17)?;
    let injury_types = table(&mut r, &mut s, "injuries", |r, _, _| {
        let name = r.string()?;
        r.skip(1)?; // grammatical gender
        let category = r.u8()?;
        let sentence_name = r.string()?;
        r.skip(1)?; // grammatical gender
        let chance = r.u8()?;
        let minimum_days_out = r.i16()?;
        let extra_days_out = r.i16()?;
        let inactive_ratio = r.u8()?;
        let recurring = r.boolean()?;
        let severity = r.u8()?;
        let (id, uid) = r.identity()?;
        Ok(InjuryType {
            name,
            category,
            sentence_name,
            chance,
            minimum_days_out,
            extra_days_out,
            inactive_ratio,
            recurring,
            severity,
            id,
            uid,
        })
    })?;
    table(&mut r, &mut s, "media", |r, _, _| {
        r.string()?;
        r.skip(1)?;
        r.string()?;
        r.skip(2)?;
        r.array(4, 1)?;
        r.skip(14)
    })?;
    table(&mut r, &mut s, "languages", |r, _, _| {
        r.string()?;
        r.skip(21)
    })?;
    let nations = table(&mut r, &mut s, "nations", |r, t, _| {
        check_type(t, &[0], r.pos - 1)?;
        r.skip(51)?;
        r.array(2, 1)?;
        r.array(4, 1)?;
        r.array(4, 1)?;
        r.array(4, 2)?;
        for _ in 0..8 {
            r.skip(2)?;
            r.array(4, 1)?;
        }
        match r.u16()? {
            0 => (),
            1 => {
                r.skip(13)?;
                for _ in 0..4 {
                    r.string()?;
                }
            }
            v => return Err(format!("Unknown important player flag {v}")),
        }
        r.skip(21)?;
        r.array(4, 1)?;
        r.skip(2)?;
        team_container(r)
    })?;
    let mut names: [Vec<String>; 3] = Default::default();
    for (i, name) in ["firstNames", "surnames", "commonNames"].iter().enumerate() {
        names[i] = table(&mut r, &mut s, name, |r, t, _| {
            check_type(t, &[0], r.pos - 1)?;
            let value = r.string()?;
            r.skip(6)?;
            r.identity()?;
            Ok(value)
        })?;
    }
    fixed(&mut r, &mut s, "regions", 9)?;
    table(&mut r, &mut s, "sponsors", |r, _, _| {
        r.string()?;
        r.skip(1)?;
        r.string()?;
        r.skip(10)
    })?;
    fixed(&mut r, &mut s, "stadiums", 48)?;
    fixed(&mut r, &mut s, "stadiumChanges", 38)?;
    table(&mut r, &mut s, "stages", |r, _, _| {
        r.string()?;
        r.skip(1)?;
        r.string()?;
        r.skip(1)?;
        r.string()?;
        r.skip(9)
    })?;
    let teams = table(&mut r, &mut s, "teams", |r, t, _| {
        check_type(t, &[10, 11], r.pos - 1)?;
        if t == 11 {
            r.skip(14)?;
        }
        r.skip(42)?;
        r.array(4, 1)?;
        r.array(4, 2)?;
        r.array(4, 1)?;
        r.array(4, 1)?;
        r.skip(1)?;
        r.array(12, 2)?;
        r.skip(12)?;
        let squad = r.u8()?;
        check_type(squad, &(0..=18).collect::<Vec<_>>(), r.pos - 1)?;
        let container_type = r.kind(&[0, 1])?;
        r.skip(1)?;
        let container_index = r.i32()?;
        r.identity()?;
        Ok(Team {
            container_type,
            container_index,
        })
    })?;
    table(&mut r, &mut s, "weather", |r, _, _| {
        r.string()?;
        r.skip(87)
    })?;
    fixed(&mut r, &mut s, "playerTypes", 10)?;
    let people = table(&mut r, &mut s, "people", read_person)?;
    let mut personality_offsets = Vec::new();
    let personalities = table(&mut r, &mut s, "personData", |r, t, _| {
        check_type(t, &[0], r.pos - 1)?;
        personality_offsets.push(r.pos);
        let mut attrs = BTreeMap::new();
        for key in [
            "adaptability",
            "ambition",
            "loyalty",
            "pressure",
            "professionalism",
            "sportsmanship",
            "temperament",
            "controversy",
        ] {
            attrs.insert(key.into(), r.u8()? as i8);
        }
        r.identity()?;
        Ok(attrs)
    })?;
    let playing = table(&mut r, &mut s, "playingData", |r, t, _| {
        check_type(t, &[0], r.pos - 1)?;
        let offset = r.pos;
        let mut raw = [0i8; 65];
        for (i, b) in r.take(65)?.iter().enumerate() {
            raw[i] = *b as i8;
        }
        r.skip(1)?;
        r.identity()?;
        Ok(Playing { offset, raw })
    })?;
    Ok(Database {
        version,
        game_date,
        sections: s,
        clubs,
        nations,
        injury_types,
        teams,
        names,
        people,
        personalities,
        personality_offsets,
        playing,
        parsed_until: r.pos,
    })
}

fn read_player_info(r: &mut Reader) -> Result<PlayingInfo> {
    let offset = r.pos;
    let mut health_records = HealthRecords::default();
    let reputation = [r.u16()?, r.u16()?, r.u16()?];
    let ca = r.i16()?;
    let pa = r.i16()?;
    let playing_index = r.i32()?;
    r.skip(4)?; // playing history
    let estimated_value = r.i32()?;
    let morale = r.u8()?;
    r.skip(1)?; // nation choice factor
    let fitness = r.i16()?;
    let jadedness = r.i16()?;
    let condition = r.i16()?;
    r.skip(7)?;
    let weight = r.i16()?;
    let height = r.i16()?;
    r.skip(1)?; // ability offset
    let sale_value = r.i32()?;
    if r.boolean()? {
        r.skip(16)?;
    }
    r.skip(4)?;
    if r.boolean()? {
        health_records.injury_count_offset = Some(r.pos);
        for _ in 0..r.u8()? {
            let start = r.pos;
            r.skip(10)?;
            let n = match r.u8()? {
                0 => 1,
                1 => 3,
                3 | 5 => 5,
                7 => 7,
                t => return Err(format!("Unknown injury fixture {t}")),
            };
            r.skip(n + 26)?;
            health_records.injuries.push(record_span(r, start));
        }
        health_records.ban_count_offset = Some(r.pos);
        for _ in 0..r.u8()? {
            let start = r.pos;
            r.skip(18)?;
            if r.boolean()? {
                r.skip(39)?;
            }
            health_records.bans.push(record_span(r, start));
        }
        health_records.injury_status_flag_offset = Some(r.pos);
        if r.boolean()? {
            let start = r.pos;
            r.skip(26)?;
            health_records.injury_status = Some(record_span(r, start));
        }
    }
    for _ in 0..r.u8()? {
        let start = r.pos;
        let t = r.u8()?;
        let n = *[3, 5, 5, 8, 4]
            .get(t as usize)
            .ok_or_else(|| format!("Unknown booking {t}"))?;
        r.skip(n)?;
        health_records.bookings.push(record_span(r, start));
    }
    r.skip(38)?;
    if r.boolean()? {
        for _ in 0..r.u8()? {
            r.skip(5)?;
            for _ in 0..r.u8()? {
                r.skip(10)?;
                if r.boolean()? {
                    r.skip(29)?;
                }
            }
        }
    }
    Ok(PlayingInfo {
        offset,
        ca,
        pa,
        playing_index,
        reputation,
        weight,
        height,
        health_records,
        state: PlayerState {
            estimated_value,
            sale_value,
            morale,
            fitness,
            jadedness,
            condition,
        },
    })
}
fn contract(r: &mut Reader, t: u8) -> Result<Contract> {
    let offset = r.pos - 1;
    check_type(t, &[0, 1, 2, 5], r.pos - 1)?;
    if t == 0 {
        r.skip(20)?;
        let n = r.u8()? as usize;
        r.skip(6 + n * 5)?;
    }
    if t == 1 {
        r.skip(6)?;
    }
    if t == 5 {
        r.skip(7)?;
    }
    if t != 2 {
        r.skip(1)?;
    }
    let common_offset = r.pos;
    let entity_type = r.kind(&[0, 1])?;
    let entity_index = r.i32()?;
    let weekly_wage = r.i32()?;
    let expires = date(r)?;
    let squad_status = r.u8()? as i8;
    let transfer_status = r.u8()?;
    r.skip(8)?; // happiness flags
    let started = date(r)?;
    r.skip(3)?; // happiness level, perceived squad status, club choice
    let squad_number = r.u8()? as i8;
    r.skip(3)?; // offer options, unknown word
    Ok(Contract {
        kind: t,
        offset,
        common_offset,
        entity_type,
        entity_index,
        weekly_wage,
        expires,
        started,
        squad_status,
        transfer_status,
        squad_number,
    })
}
fn read_person(r: &mut Reader, t: u8, index: usize) -> Result<Person> {
    let offset = r.pos - 1;
    check_type(t, &[1, 2, 3, 4, 5, 6, 7, 8, 9], offset)?;
    let mut person = Person {
        index,
        offset,
        kind: t,
        id: 0,
        uid: 0,
        first: -1,
        surname: -1,
        common: -1,
        birth: None,
        common_offset: 0,
        nation: -1,
        team: -1,
        personality: -1,
        playing: None,
        contracts: Vec::new(),
    };
    if t == 6 || t == 7 {
        r.skip(if t == 6 { 1 } else { 6 })?;
        let (id, uid) = r.identity()?;
        person.id = id;
        person.uid = uid;
        return Ok(person);
    }
    if t == 5 {
        r.string()?;
        r.string()?;
        r.skip(2)?;
        let (id, uid) = r.identity()?;
        person.id = id;
        person.uid = uid;
        return Ok(person);
    }
    if t == 9 {
        human(r)?;
    }
    if t == 2 || t == 3 {
        person.playing = Some(read_player_info(r)?);
    }
    if [1, 3, 9].contains(&t) {
        r.skip(15)?;
    }
    if t == 4 {
        r.skip(24)?;
    }
    person.common_offset = r.pos;
    person.first = r.i32()?;
    person.surname = r.i32()?;
    person.common = r.i32()?;
    person.birth = Some(date(r)?);
    person.nation = r.i32()?;
    r.skip(13)?;
    person.team = r.i32()?;
    r.skip(6)?;
    person.personality = r.i32()?;
    r.skip(4)?;
    r.array(4, 1)?;
    r.skip(8)?;
    for _ in 0..r.u8()? {
        let ct = r.u8()?;
        person.contracts.push(contract(r, ct)?);
    }
    for _ in 0..r.u8()? {
        let ct = r.u8()?;
        r.skip(6)?;
        if ct != 255 {
            contract(r, ct)?;
        }
    }
    if r.boolean()? {
        r.array(8, 1)?;
    }
    r.array(4, 2)?;
    if r.boolean()? {
        r.string()?;
        r.skip(4)?;
    }
    r.skip(1)?;
    let (id, uid) = r.identity()?;
    person.id = id;
    person.uid = uid;
    if t == 3 {
        r.skip(2)?;
    }
    Ok(person)
}
fn ascii(r: &mut Reader, n: usize) -> Result<String> {
    Ok(std::str::from_utf8(r.take(n)?)
        .map_err(|e| e.to_string())?
        .to_owned())
}
fn human(r: &mut Reader) -> Result<()> {
    r.skip(16)?;
    r.string()?;
    r.skip(68)?;
    r.array(8, 1)?;
    r.skip(96)?;
    for _ in 0..r.count()? {
        let kind = ascii(r, 8)?;
        let expected = match kind.as_str() {
            "slfttslf" | "tslttslf" => "daqstlif",
            "tslntslf" => "swentlif",
            "ltontslf" => "etontlif",
            "tslhtslf" => "ntantlif",
            _ => return Err(format!("Unsupported filter {kind}")),
        };
        for _ in 0..r.count()? {
            if ascii(r, 8)? != expected {
                return Err("Unsupported human filter type".into());
            }
            r.string()?;
            r.skip(5)?;
            for _ in 0..r.count()? {
                let info = ascii(r, 4)?;
                if kind == "tslhtslf" && info == "rftn" {
                    r.skip(2)?;
                    r.array(8, 4)?;
                    r.skip(8)?;
                } else {
                    r.skip(match info.as_str() {
                        "rfqs" => 10,
                        "rfsp" | "rfcn" => 2,
                        "rfpv" => 3,
                        "rfsn" => 1,
                        "rftn" => 9,
                        _ => return Err(format!("Unsupported info {info}")),
                    })?;
                }
            }
        }
        if kind == "slfttslf" {
            r.skip(10)?;
        }
        if kind == "tslntslf" {
            r.skip(4)?;
        }
    }
    r.skip(16)?;
    r.array(32, 1)?;
    r.skip(1)
}
fn reference<'a, T>(table: &'a [T], index: i32, label: &str) -> Result<&'a T> {
    usize::try_from(index)
        .ok()
        .and_then(|i| table.get(i))
        .ok_or_else(|| format!("Invalid {label} reference {index}"))
}
fn optional_name(table: &[String], index: i32) -> Result<String> {
    if index == -1 {
        Ok(String::new())
    } else {
        Ok(reference(table, index, "name")?.clone())
    }
}
impl Database {
    pub fn players(&self, base: usize) -> Result<Vec<Player>> {
        self.people
            .iter()
            .filter_map(|p| p.playing.as_ref().map(|d| (p, d)))
            .map(|(p, d)| {
                let first_name = optional_name(&self.names[0], p.first)?;
                let surname = optional_name(&self.names[1], p.surname)?;
                let common_name = optional_name(&self.names[2], p.common)?;
                let name = if common_name.is_empty() {
                    format!("{first_name} {surname}").trim().into()
                } else {
                    common_name.clone()
                };
                let nation = if p.nation == -1 {
                    None
                } else {
                    Some(reference(&self.nations, p.nation, "nation")?.name.clone())
                };
                let club = if p.team == -1 {
                    None
                } else {
                    let team = reference(&self.teams, p.team, "team")?;
                    if team.container_type == 0 {
                        Some(
                            reference(&self.clubs, team.container_index, "club")?
                                .name
                                .clone(),
                        )
                    } else {
                        reference(&self.nations, team.container_index, "national team")?;
                        None
                    }
                };
                let playing = reference(&self.playing, d.playing_index, "playing data")?;
                let personality = if p.personality == -1 {
                    None
                } else {
                    Some(reference(&self.personalities, p.personality, "personality")?.clone())
                };
                let birth = p.birth.as_ref().ok_or("Missing birth date")?;
                let decoded = attributes::decoded(&playing.raw, d.ca)?;
                let mut fields = BTreeMap::new();
                for (i, key) in FIELDS.iter().enumerate() {
                    let name = if i < 12 {
                        format!("position.{key}")
                    } else {
                        key.to_string()
                    };
                    fields.insert(
                        name,
                        Field {
                            value: decoded[*key] as i32,
                            offset: base + playing.offset + i,
                            width: 1,
                            min: if i < 12 { 0 } else { 1 },
                            max: 20,
                        },
                    );
                }
                for (key, value, offset, width, min, max) in [
                    ("currentAbility", d.ca as i32, d.offset + 6, 2, 1, 200),
                    ("potentialAbility", d.pa as i32, d.offset + 8, 2, -2, 200),
                    (
                        "reputation.home",
                        d.reputation[0] as i32,
                        d.offset,
                        2,
                        0,
                        10000,
                    ),
                    (
                        "reputation.current",
                        d.reputation[1] as i32,
                        d.offset + 2,
                        2,
                        0,
                        10000,
                    ),
                    (
                        "reputation.world",
                        d.reputation[2] as i32,
                        d.offset + 4,
                        2,
                        0,
                        10000,
                    ),
                    ("weight", d.weight as i32, d.offset + 37, 2, 0, 300),
                    // CM4 stores height in whole inches; the public field uses
                    // rounded centimetres. Zero denotes an unspecified measure.
                    (
                        "height",
                        (d.height as f64 * 2.54).round() as i32,
                        d.offset + 39,
                        2,
                        0,
                        300,
                    ),
                    (
                        "birth.day",
                        birth.day as i32,
                        p.common_offset + 12,
                        2,
                        0,
                        365,
                    ),
                    (
                        "birth.year",
                        birth.year as i32,
                        p.common_offset + 14,
                        2,
                        1850,
                        self.game_date.year as i32,
                    ),
                    (
                        "nationality",
                        p.nation,
                        p.common_offset + 16,
                        4,
                        0,
                        self.nations.len() as i32 - 1,
                    ),
                ] {
                    fields.insert(
                        key.into(),
                        Field {
                            value,
                            offset: base + offset,
                            width,
                            min,
                            max,
                        },
                    );
                }
                if p.personality >= 0 {
                    let offset = *reference(
                        &self.personality_offsets,
                        p.personality,
                        "personality offset",
                    )?;
                    for (i, key) in PERSONALITY_FIELDS.iter().enumerate() {
                        fields.insert(
                            format!("personality.{key}"),
                            Field {
                                value: self.personalities[p.personality as usize][*key] as i32,
                                offset: base + offset + i,
                                width: 1,
                                min: 1,
                                max: 20,
                            },
                        );
                    }
                }
                crate::career::add_fields(p, base, &mut fields);
                Ok(Player {
                    uid: p.uid,
                    id: p.id,
                    kind: p.kind,
                    first_name,
                    surname,
                    common_name,
                    name,
                    club,
                    nation,
                    date_of_birth: Date {
                        day: birth.day,
                        year: birth.year,
                    },
                    current_ability: d.ca,
                    potential_ability: d.pa,
                    raw_attributes: FIELDS
                        .iter()
                        .zip(playing.raw)
                        .map(|(k, v)| (k.to_string(), v))
                        .collect(),
                    attributes: decoded,
                    personality,
                    offsets: Offsets {
                        person: base + p.offset,
                        current_ability: base + d.offset + 6,
                        potential_ability: base + d.offset + 8,
                        attributes: base + playing.offset,
                    },
                    fields,
                })
            })
            .collect()
    }
}
pub fn load(bytes: &[u8]) -> Result<(Database, usize)> {
    let container = read_container(bytes)?;
    let blocks = container
        .blocks
        .iter()
        .filter(|b| b.name == "game_db" && b.extension == ".dat")
        .collect::<Vec<_>>();
    if blocks.len() != 1 {
        return Err("Expected exactly one game_db.dat".into());
    }
    let b = blocks[0];
    Ok((
        read_database(&bytes[b.payload_offset..b.payload_offset + b.size])?,
        b.payload_offset,
    ))
}
