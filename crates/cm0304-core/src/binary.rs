use crate::Result;

pub struct Reader<'a> {
    pub data: &'a [u8],
    pub pos: usize,
}
impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }
    pub fn take(&mut self, size: usize) -> Result<&'a [u8]> {
        let end = self.pos.checked_add(size).ok_or("Offset overflow")?;
        let bytes = self
            .data
            .get(self.pos..end)
            .ok_or_else(|| format!("Truncated data at {} (requires {size} bytes)", self.pos))?;
        self.pos = end;
        Ok(bytes)
    }
    pub fn skip(&mut self, size: usize) -> Result<()> {
        self.take(size).map(|_| ())
    }
    pub fn u8(&mut self) -> Result<u8> {
        Ok(self.take(1)?[0])
    }
    pub fn u16(&mut self) -> Result<u16> {
        Ok(u16::from_le_bytes(self.take(2)?.try_into().unwrap()))
    }
    pub fn i16(&mut self) -> Result<i16> {
        Ok(self.u16()? as i16)
    }
    pub fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }
    pub fn i32(&mut self) -> Result<i32> {
        Ok(self.u32()? as i32)
    }
    pub fn boolean(&mut self) -> Result<bool> {
        match self.u8()? {
            0 => Ok(false),
            1 => Ok(true),
            v => Err(format!("Invalid boolean {v} at {}", self.pos - 1)),
        }
    }
    pub fn count(&mut self) -> Result<usize> {
        let n = self.i32()?;
        if !(0..=1_000_000).contains(&n) {
            return Err(format!("Invalid count {n} at {}", self.pos - 4));
        }
        Ok(n as usize)
    }
    pub fn array(&mut self, width: usize, count_bytes: usize) -> Result<()> {
        let n = match count_bytes {
            1 => self.u8()? as usize,
            2 => self.u16()? as usize,
            4 => self.count()?,
            _ => return Err("Invalid count width".into()),
        };
        self.skip(n.checked_mul(width).ok_or("Array size overflow")?)
    }
    pub fn string(&mut self) -> Result<String> {
        let n = self.count()?;
        if n > 100_000 {
            return Err("String too large".into());
        }
        let text = utf16(self.take(n * 2)?)?;
        if n > 0 && self.u16()? != 0 {
            return Err("Missing UTF-16 terminator".into());
        }
        Ok(text)
    }
    pub fn identity(&mut self) -> Result<(i32, i32)> {
        Ok((self.i32()?, self.i32()?))
    }
    pub fn kind(&mut self, allowed: &[u8]) -> Result<u8> {
        let t = self.u8()?;
        check_type(t, allowed, self.pos - 1)?;
        Ok(t)
    }
}
pub fn utf16(bytes: &[u8]) -> Result<String> {
    if !bytes.len().is_multiple_of(2) {
        return Err("Odd UTF-16 byte length".into());
    }
    String::from_utf16(
        &bytes
            .as_chunks::<2>()
            .0
            .iter()
            .map(|v| u16::from_le_bytes([v[0], v[1]]))
            .collect::<Vec<_>>(),
    )
    .map_err(|e| e.to_string())
}
pub fn check_type(t: u8, allowed: &[u8], pos: usize) -> Result<()> {
    if allowed.contains(&t) {
        Ok(())
    } else {
        Err(format!("Unsupported type {t} at {pos}"))
    }
}
