use super::utils;
use crate::commands::result::Result;
use chrono::{offset::TimeZone, DateTime, Local, TimeDelta, Utc};
use std::{
    io::{Read, Seek, SeekFrom},
    path::Path,
    sync::OnceLock,
};

pub fn read_taken_at_from_atom<P>(path: P) -> Result<DateTime<Local>>
where
    P: AsRef<Path>,
{
    let file =
        std::fs::File::open(path.as_ref()).map_err(|e| format!("Failed to open file: {}", e))?;
    let mut bufreader = std::io::BufReader::new(file);
    seek_to(&mut bufreader, *b"moov")?;
    seek_to(&mut bufreader, *b"mvhd")?;
    let created_at = read_created_at_from_mvhd(&mut bufreader)?;
    Ok(created_at.with_timezone(&Local))
}

fn seek_to(reader: &mut (impl Read + Seek), fourcc: [u8; 4]) -> Result<Head> {
    loop {
        let head = read_head(reader)?;
        if head.fourcc == fourcc {
            break Ok(head);
        }
        reader
            .seek(SeekFrom::Current(head.length as i64))
            .map_err(|e| format!("Failed to seek: {}", e.to_string()))?;
    }
}

static START_OF_TIME: OnceLock<DateTime<Utc>> = OnceLock::new();
fn get_created_at_start_of_time() -> DateTime<Utc> {
    *START_OF_TIME.get_or_init(|| Utc.with_ymd_and_hms(1904, 1, 1, 0, 0, 0).latest().unwrap())
}
fn read_created_at_from_mvhd(reader: &mut (impl Read + Seek)) -> Result<DateTime<Utc>> {
    // Version 0: | 1 byte version | 3 bytes flags | 4 bytes creation time |...
    // Version 1: | 1 byte version | 3 bytes flags | 8 bytes creation time |...
    let version = utils::read8(reader).map_err(|e| format!("Failed to read version: {}", e))?;
    let _flag1 = utils::read8(reader).map_err(|e| format!("Failed to read flag: {}", e))?;
    let _flag2 = utils::read8(reader).map_err(|e| format!("Failed to read flag: {}", e))?;
    let _flag3 = utils::read8(reader).map_err(|e| format!("Failed to read flag: {}", e))?;

    let created = if version == 0 {
        let seconds =
            utils::read32(reader).map_err(|e| format!("Failed to read creation time: {}", e))?;
        let delta = TimeDelta::try_seconds(seconds as i64)
            .ok_or("Failed to convert seconds to DateTime<Utc>".to_string())?;
        get_created_at_start_of_time() + delta
    } else {
        let seconds =
            utils::read64(reader).map_err(|e| format!("Failed to read creation time: {}", e))?;
        let delta = TimeDelta::try_seconds(seconds as i64)
            .ok_or("Failed to convert seconds to DateTime<Utc>".to_string())?;
        get_created_at_start_of_time() + delta
    };
    Ok(created)
}

struct Head {
    length: u64,
    fourcc: [u8; 4],
}
fn read_head(reader: &mut (impl Read + Seek)) -> Result<Head> {
    let length = utils::read32(reader)
        .map_err(|e| format!("Failed to read atom head: {}", e.to_string()))?
        as u64;
    let mut fourcc = [0u8; 4];
    reader
        .read_exact(&mut fourcc)
        .map_err(|e| format!("Failed to read atom head: {}", e.to_string()))?;
    if length == 1 {
        // Extended size
        let length = utils::read64(reader)
            .map_err(|e| format!("Failed to read atom head: {}", e.to_string()))?;
        return Ok(Head {
            length: length - 16,
            fourcc,
        });
    }
    if length < 8 {
        return Err("Invalid atom length".to_string());
    }
    Ok(Head {
        length: length - 8,
        fourcc,
    })
}
