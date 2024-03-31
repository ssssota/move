pub fn read8<R>(reader: &mut R) -> Result<u8, std::io::Error>
where
    R: std::io::Read,
{
    let mut buf = [0u8; 1];
    reader.read_exact(&mut buf).and(Ok(buf[0]))
}

pub fn read16<R>(reader: &mut R) -> Result<u16, std::io::Error>
where
    R: std::io::Read,
{
    let mut buf = [0u8; 2];
    reader.read_exact(&mut buf)?;
    Ok(u16::from_be_bytes(buf))
}

pub fn read32<R>(reader: &mut R) -> Result<u32, std::io::Error>
where
    R: std::io::Read,
{
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(u32::from_be_bytes(buf))
}

pub fn read64<R>(reader: &mut R) -> Result<u64, std::io::Error>
where
    R: std::io::Read,
{
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    Ok(u64::from_be_bytes(buf))
}
