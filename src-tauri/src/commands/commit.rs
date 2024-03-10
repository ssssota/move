use std::path::Path;

use chrono::{DateTime, Datelike, Local};
use ts_rs::TS;
use walkdir::WalkDir;

#[derive(serde::Serialize, Clone, TS)]
#[ts(export)]
pub struct Commit {
    pub entries: Vec<(String, String)>,
}

#[derive(serde::Serialize, Clone, TS)]
#[ts(export)]
pub struct Progress {
    current: usize,
    total: usize,
}

#[tauri::command]
pub fn commit<R: tauri::Runtime>(
    window: tauri::Window<R>,
    source: String,
    target: String,
    pattern: String,
    dry_run: bool,
) -> Result<Commit, String> {
    let source = Path::new(&source);
    let target = Path::new(&target);
    if !source.is_absolute() {
        return Err("Source path must be absolute".to_string());
    }
    if !source.exists() {
        return Err(format!("Source path does not exist: {}", source.display()));
    }
    if !target.is_absolute() {
        return Err("Target path must be absolute".to_string());
    }
    let mut target_files = vec![];
    for entry in WalkDir::new(source)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        target_files.push(entry);
    }
    let mut entries = vec![];
    for entry in target_files {
        let source = entry.path().to_string_lossy().to_string();
        let meta = entry
            .metadata()
            .map_err(|e| format!("Failed to load metadata: {}", e))?;
        let created: DateTime<Local> = meta
            .created()
            .map_err(|e| format!("Failed to load metadata(created): {}", e))?
            .into();
        let resolved_pattern = pattern
            .replace("{CREATED_YYYY}", format!("{:04}", created.year()).as_str())
            .replace("{CREATED_MM}", format!("{:02}", created.month()).as_str())
            .replace("{CREATED_DD}", format!("{:02}", created.day()).as_str())
            .replace("{FILE_NAME}", &entry.file_name().to_string_lossy());
        entries.push((
            source,
            target.join(resolved_pattern).to_string_lossy().to_string(),
        ));
    }
    if dry_run {
        return Ok(Commit { entries });
    }
    let total = entries.len();
    for (index, (source, target)) in entries.iter().enumerate() {
        let _ = window.emit(
            "move-progress",
            Progress {
                current: index,
                total,
            },
        );
        move_file(source, target)?;
    }
    Ok(Commit { entries })
}

#[cfg(not(target_os = "windows"))]
fn move_file(source: &str, target: &str) -> Result<(), String> {
    std::fs::create_dir_all(Path::new(target).parent().unwrap())
        .map_err(|e| format!("Failed to create missing directory: {}", e))?;
    std::fs::rename(source, target).map_err(|e| format!("Failed to move file: {}", e))
}
#[cfg(target_os = "windows")]
fn move_file(source: &str, target: &str) -> Result<(), String> {
    std::fs::create_dir_all(Path::new(target).parent().unwrap())
        .map_err(|e| format!("Failed to create missing directory: {}", e))?;
    let is_cross_drive =
        source.chars().take(3).collect::<String>() != target.chars().take(3).collect::<String>();
    if is_cross_drive {
        std::fs::copy(source, target).map_err(|e| format!("Failed to copy file: {}", e))?;
        std::fs::remove_file(source).map_err(|e| format!("Failed to remove file: {}", e))
    } else {
        std::fs::rename(source, target).map_err(|e| format!("Failed to move file: {}", e))
    }
}
