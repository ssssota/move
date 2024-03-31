mod exif;
mod move_file;

use std::path::Path;

use chrono::{DateTime, Datelike, Local};
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
) -> Result<Commit, String> {
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
                .map(|e| e.to_lowercase());
            let parser = extension.as_deref().and_then(|ext| match ext {
                "jpg" | "jpeg" | "heif" | "heic" | "avif" | "webp" | "png" | "nef" | "cr2"
                | "dng" | "orf" | "arw" | "rw2" | "raf" | "mrw" | "tif" | "tiff" => {
                    Some(exif::read_taken_at_from_exif)
                }
                _ => None,
            });
            let created = parser
                .map(|parse| parse(path))
                .unwrap_or_else(|| get_created_at(&entry))?;
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
        .collect::<Result<Vec<(String, String)>, String>>()?;
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
    Ok(Commit { entries })
}

fn get_created_at(entry: &DirEntry) -> Result<DateTime<Local>, String> {
    let meta = entry
        .metadata()
        .map_err(|e| format!("Failed to load metadata: {}", e))?;
    let created: DateTime<Local> = meta
        .created()
        .map_err(|e| format!("Failed to load metadata(created): {}", e))?
        .into();
    Ok::<chrono::DateTime<Local>, String>(created)
}
