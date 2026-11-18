use crate::{
    Result,
    binary::{Reader, utf16},
};
use serde::Serialize;
use sha2::{Digest, Sha256};

pub fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub name: String,
    pub extension: String,
    pub offset: usize,
    pub payload_offset: usize,
    pub size: usize,
    pub sha256: String,
}
#[derive(Debug, Serialize)]
pub struct Container {
    pub size: usize,
    pub sha256: String,
    pub blocks: Vec<Block>,
}
pub fn read_container(bytes: &[u8]) -> Result<Container> {
    let mut r = Reader::new(bytes);
    if r.take(12)? != [0, 46, 0, 99, 0, 109, 0, 52, 0, 2, 0, 0] {
        return Err("Unsupported CM4 signature".into());
    }
    let mut blocks = Vec::new();
    while r.pos < bytes.len() {
        let offset = r.pos;
        if r.u32()? != 123456789 {
            return Err(format!("Invalid record marker at {offset}"));
        }
        match r.u8()? {
            0 => r.skip(54)?,
            1 => (),
            2 => {
                let name = utf16(r.take(54)?)?.trim_end_matches('\0').to_string();
                let extension = utf16(r.take(10)?)?.trim_end_matches('\0').to_string();
                let size = r.u32()? as usize;
                let payload_offset = r.pos;
                let sha256 = hash(r.take(size)?);
                blocks.push(Block {
                    name,
                    extension,
                    offset,
                    payload_offset,
                    size,
                    sha256,
                });
            }
            t => return Err(format!("Unsupported record type {t} at {offset}")),
        }
    }
    Ok(Container {
        size: bytes.len(),
        sha256: hash(bytes),
        blocks,
    })
}
