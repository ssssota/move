mod atom;
mod exif;
mod move_file;
mod utils;

use crate::commands::result::Result;
use chrono::{DateTime, Datelike, Local};
use std::path::Path;
use ts_rs::TS;
use walkdir::{DirEntry, WalkDir};

#[derive(serde::Serialize, Clone, TS)]
#[ts(export)]
pub struct Commit {
    pub entries: Vec<(String, String)>,
}

#[derive(serde::Serialize, Clone, TS)]
#[ts(export)]
pub struct Progress {
    complete: usize,
    total: usize,
}

#[tauri::command(async)]
pub fn commit<R: tauri::Runtime>(
    window: tauri::Window<R>,
    source: String,
    target: String,
    pattern: String,
    dry_run: bool,
) -> Result<Commit> {
    let source_dir = Path::new(&source);
    let target_dir = Path::new(&target);
    if !source_dir.is_absolute() {
        return Err("Source path must be absolute".to_string());
    }
    if !source_dir.exists() {
        return Err(format!(
            "Source path does not exist: {}",
            source_dir.display()
        ));
    }
    if !target_dir.is_absolute() {
        return Err("Target path must be absolute".to_string());
    }
    if source_dir == target_dir {
        return Err("Source and target path must be different".to_string());
    }
    let entries = WalkDir::new(source_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|entry| {
            let path = entry.path();
            let filename = path.file_name().unwrap().to_string_lossy();
            let extension = path
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e.to_lowercase())
                .unwrap_or("".to_string());
            let created = match &extension[..] {
                "jpg" | "jpeg" | "heif" | "heic" | "avif" | "webp" | "png" | "nef" | "nrw"
                | "cr2" | "dng" | "arw" | "sr2" | "srf" | "rw2" | "raf" | "pef" | "mos" | "3fr"
                | "erf" | "mef" | "dcr" | "srw" | "tif" | "tiff" => {
                    exif::read_taken_at_from_exif(path)
                }
                "mp4" | "m4v" | "mov" | "qt" => atom::read_taken_at_from_atom(path),
                _ => Err("Unsupported file type".to_string()),
            }
            .unwrap_or_else(|_| get_created_at(&entry).unwrap_or_else(|_| Local::now()));
            let resolved_pattern = pattern
                .replace("{CREATED_YYYY}", format!("{:04}", created.year()).as_str())
                .replace("{CREATED_MM}", format!("{:02}", created.month()).as_str())
                .replace("{CREATED_DD}", format!("{:02}", created.day()).as_str())
                .replace("{FILE_NAME}", &filename);

            let target = target_dir.join(resolved_pattern);
            Ok((
                path.to_string_lossy().to_string(),
                target.to_string_lossy().to_string(),
            ))
        })
        .collect::<Result<Vec<(String, String)>>>()?;
    if dry_run {
        return Ok(Commit { entries });
    }
    let total = entries.len();
    for (index, (source, target)) in entries.iter().enumerate() {
        if !Path::new(target).exists() {
            move_file::move_file(source, target)?;
        }
        let _ = window.emit(
            "commit-progress",
            Progress {
                complete: index + 1,
                total,
            },
        );
    }
    let _ = remove_blank_dir_recursive(&source_dir);
    Ok(Commit { entries })
}

fn get_created_at(entry: &DirEntry) -> Result<DateTime<Local>> {
    let meta = entry
        .metadata()
        .map_err(|e| format!("Failed to load metadata: {}", e))?;
    let created: DateTime<Local> = meta
        .created()
        .map_err(|e| format!("Failed to load metadata(created): {}", e))?
        .into();
    Ok::<chrono::DateTime<Local>, String>(created)
}

fn remove_blank_dir_recursive<P>(path: &P) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    if path.is_dir() {
        for entry in std::fs::read_dir(path)?.filter_map(|e| e.ok()) {
            remove_blank_dir_recursive(&entry.path())?;
        }
        if is_blank_dir(path)? {
            std::fs::remove_dir(path)?;
        }
    }
    Ok(())
}

fn is_blank_dir<P>(path: &P) -> std::io::Result<bool>
where
    P: AsRef<Path> + ?Sized,
{
    let path = path.as_ref();
    if path.is_dir() {
        Ok(std::fs::read_dir(path)?
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name() != ".DS_Store")
            .next()
            .is_none())
    } else {
        Ok(false)
    }
}
