use std::{io::Seek, path::Path};

use chrono::{DateTime, Local, NaiveDateTime};

pub fn read_taken_at_from_exif<P>(path: P) -> Result<DateTime<Local>, String>
where
    P: AsRef<Path>,
{
    let file =
        std::fs::File::open(path.as_ref()).map_err(|e| format!("Failed to open file: {}", e))?;
    let mut bufreader = std::io::BufReader::new(file);
    let exif_reader = exif::Reader::new();
    let exif = exif_reader
        .read_from_container(&mut bufreader)
        .map_err(|e| format!("Failed to read exif: {}", e))
        .or_else(|_| {
            bufreader
                .seek(std::io::SeekFrom::Start(0))
                .map_err(|e| format!("Failed to seek: {}", e))?;
            let buf = find_exif(&mut bufreader)?;
            exif_reader
                .read_raw(buf)
                .map_err(|e| format!("Failed to read exif: {}", e))
        })
        .map_err(|e| format!("Failed to read exif: {}", e))?;
    let taken_at = exif
        .get_field(exif::Tag::DateTimeOriginal, exif::In::PRIMARY)
        .or_else(|| exif.get_field(exif::Tag::DateTimeDigitized, exif::In::PRIMARY))
        .or_else(|| exif.get_field(exif::Tag::DateTime, exif::In::PRIMARY))
        .ok_or("Failed to find DateTimeOriginal")?;
    let parsed =
        NaiveDateTime::parse_from_str(&taken_at.display_value().to_string(), "%Y-%m-%d %H:%M:%S")
            .map_err(|e| format!("Failed to parse DateTimeOriginal: {}", e))
            .map(|naive| naive.and_local_timezone(Local))?;
    match parsed {
        chrono::LocalResult::None => Err("Failed to parse DateTimeOriginal".to_string()),
        chrono::LocalResult::Single(datetime) => Ok(datetime),
        chrono::LocalResult::Ambiguous(datetime, _) => Ok(datetime),
    }
}

mod marker {
    // The first byte of a marker.
    pub const P: u8 = 0xff;
    // Marker codes.
    pub const SOI: u8 = 0xd8;
    pub const APP1: u8 = 0xe1;
    // The EXIF identifier.
    pub const EXIF_ID: [u8; 6] = *b"Exif\0\0";

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
}
fn find_exif<R>(reader: &mut R) -> Result<Vec<u8>, String>
where
    R: std::io::BufRead,
{
    loop {
        reader
            .read_until(marker::P, &mut Vec::new())
            .map_err(|e| format!("Failed to read marker prefix: {}", e))?;
        let mut code;
        loop {
            code =
                marker::read8(reader).map_err(|e| format!("Failed to read marker code: {}", e))?;
            if code != marker::P {
                break;
            }
        }
        // 0xFFD8 is the start of the image data (SOI)
        if code != marker::SOI {
            continue;
        }
        // 0xFFE1 is the start of the EXIF data (APP1)
        code = marker::read8(reader).map_err(|e| format!("Failed to read marker code: {}", e))?;
        if code != marker::P {
            continue;
        }
        code = marker::read8(reader).map_err(|e| format!("Failed to read marker code: {}", e))?;
        if code != marker::APP1 {
            continue;
        }
        // The next 2 bytes are the length of the segment.
        let len = marker::read16(reader)
            .map_err(|e| format!("Failed to read segment length: {}", e))?
            .checked_sub(2)
            .ok_or("Invalid segment length".to_string())?;
        // Read the segment.
        let mut seg = vec![0; len.into()];
        reader
            .read_exact(&mut seg)
            .map_err(|e| format!("Failed to read segment: {}", e))?;
        if seg.starts_with(&marker::EXIF_ID) {
            // Skip the EXIF identifier.
            seg.drain(..marker::EXIF_ID.len());
            return Ok(seg);
        }
    }
}
