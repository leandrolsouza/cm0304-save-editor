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
    let mut blocks = Vec::new();
    visit_blocks(bytes, |name, extension, offset, payload_offset, payload| {
        blocks.push(Block {
            name,
            extension,
            offset,
            payload_offset,
            size: payload.len(),
            sha256: hash(payload),
        });
    })?;
    Ok(Container {
        size: bytes.len(),
        sha256: hash(bytes),
        blocks,
    })
}

/// Finds one payload while validating all container record boundaries.
pub(crate) fn find_block<'a>(
    bytes: &'a [u8],
    name: &str,
    extension: &str,
) -> Result<(&'a [u8], usize)> {
    find_optional_block(bytes, name, extension)?
        .ok_or_else(|| format!("Expected exactly one {name}{extension} block"))
}

pub(crate) fn find_optional_block<'a>(
    bytes: &'a [u8],
    name: &str,
    extension: &str,
) -> Result<Option<(&'a [u8], usize)>> {
    let mut matches = 0;
    let mut block = None;
    visit_blocks(
        bytes,
        |block_name, block_extension, _, payload_offset, payload| {
            if block_name == name && block_extension == extension {
                matches += 1;
                block = Some((payload, payload_offset));
            }
        },
    )?;
    if matches > 1 {
        return Err(format!("Expected exactly one {name}{extension} block"));
    }
    Ok(block)
}

fn visit_blocks<'a>(
    bytes: &'a [u8],
    mut visit: impl FnMut(String, String, usize, usize, &'a [u8]),
) -> Result<()> {
    let mut r = Reader::new(bytes);
    if r.take(12)? != [0, 46, 0, 99, 0, 109, 0, 52, 0, 2, 0, 0] {
        return Err("Unsupported CM4 signature".into());
    }
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
                visit(name, extension, offset, payload_offset, r.take(size)?);
            }
            t => return Err(format!("Unsupported record type {t} at {offset}")),
        }
    }
    Ok(())
}

/// Replace exactly one payload, preserving record headers, ordering and every
/// unrelated payload. Used when a validated edit changes a database record size.
pub fn replace_block(input: &[u8], name: &str, extension: &str, payload: &[u8]) -> Result<Vec<u8>> {
    let (original, offset) = find_block(input, name, extension)?;
    let size = u32::try_from(payload.len()).map_err(|_| "Block exceeds container size limit")?;
    let mut output = Vec::with_capacity(input.len() - original.len() + payload.len());
    output.extend_from_slice(&input[..offset - 4]);
    output.extend_from_slice(&size.to_le_bytes());
    output.extend_from_slice(payload);
    output.extend_from_slice(&input[offset + original.len()..]);
    let (checked, checked_offset) = find_block(&output, name, extension)?;
    if checked_offset != offset || checked != payload {
        return Err("Block verification failed during replacement".into());
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> Vec<u8> {
        let mut bytes = vec![0, 46, 0, 99, 0, 109, 0, 52, 0, 2, 0, 0];
        for (name, data) in [("a", &[1u8, 2, 3][..]), ("b", &[7u8, 8][..])] {
            bytes.extend_from_slice(&123456789u32.to_le_bytes());
            bytes.push(2);
            for (text, width) in [(name, 27), (".dat", 5)] {
                let mut units = text.encode_utf16().collect::<Vec<_>>();
                units.resize(width, 0);
                for unit in units {
                    bytes.extend_from_slice(&unit.to_le_bytes());
                }
            }
            bytes.extend_from_slice(&(data.len() as u32).to_le_bytes());
            bytes.extend_from_slice(data);
        }
        bytes
    }
    #[test]
    fn resizing_a_payload_preserves_other_records_and_rejects_ambiguous_targets() {
        let original = fixture();
        for payload in [&[][..], &[4u8][..], &[4u8, 5, 6, 7, 8, 9][..]] {
            let out = replace_block(&original, "a", ".dat", payload).unwrap();
            let parsed = read_container(&out).unwrap();
            assert_eq!(
                &out[parsed.blocks[0].payload_offset
                    ..parsed.blocks[0].payload_offset + payload.len()],
                payload
            );
            assert_eq!(
                &out[parsed.blocks[1].offset..],
                &original[read_container(&original).unwrap().blocks[1].offset..]
            );
        }
        assert!(replace_block(&original, "missing", ".dat", &[]).is_err());
        let mut duplicate = original.clone();
        let first_end = read_container(&original).unwrap().blocks[1].offset;
        duplicate.extend_from_slice(&original[12..first_end]);
        let (payload, offset) = find_block(&original, "a", ".dat").unwrap();
        assert_eq!(payload, &[1, 2, 3]);
        assert_eq!(
            offset,
            read_container(&original).unwrap().blocks[0].payload_offset
        );
        assert!(find_block(&duplicate, "a", ".dat").is_err());
        assert!(find_block(&original[..original.len() - 1], "a", ".dat").is_err());
        assert!(replace_block(&duplicate, "a", ".dat", &[]).is_err());
        assert!(replace_block(&original[..original.len() - 1], "a", ".dat", &[]).is_err());
    }
}
