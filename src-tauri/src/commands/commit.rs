use std::path::Path;

use chrono::{DateTime, Datelike, Utc};
use ts_rs::TS;
use walkdir::WalkDir;

#[derive(serde::Serialize, Clone, TS)]
#[ts(export)]
pub struct PreviewResult {
    pub entries: Vec<(String, String)>,
}

#[derive(serde::Serialize, Clone, TS)]
#[ts(export)]
pub struct Progress {
    current: usize,
    total: usize,
}

#[tauri::command]
pub fn preview(source: String, target: String, pattern: String) -> Result<PreviewResult, String> {
    let mut target_files = vec![];
    for entry in WalkDir::new(source).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            target_files.push(entry);
        }
    }
    let mut entries = vec![];
    for entry in target_files {
        let source = entry.path().to_string_lossy().to_string();
        let meta = entry
            .metadata()
            .map_err(|e| format!("Failed to load metadata: {}", e))?;
        let created: DateTime<Utc> = meta
            .created()
            .map_err(|e| format!("Failed to load metadata(created): {}", e))?
            .into();
        let resolved_pattern = pattern
            .replace("{CREATED_YYYY}", format!("{:04}", created.year()).as_str())
            .replace("{CREATED_MM}", format!("{:02}", created.month()).as_str())
            .replace("{CREATED_DD}", format!("{:02}", created.day()).as_str())
            .replace("{FILE_NAME}", &entry.file_name().to_string_lossy());
        let target = Path::new(&target).join(resolved_pattern);
        entries.push((source, target.to_string_lossy().to_string()));
    }
    Ok(PreviewResult { entries })
}

#[tauri::command]
pub fn commit<R: tauri::Runtime>(
    window: tauri::Window<R>,
    source: String,
    target: String,
    pattern: String,
) -> Result<(), String> {
    let preview = preview(source, target, pattern)?;
    let total = preview.entries.len();
    for (index, (source, target)) in preview.entries.into_iter().enumerate() {
        std::fs::create_dir_all(Path::new(&target).parent().unwrap())
            .map_err(|e| format!("Failed to create missing directory: {}", e))?;
        std::fs::rename(&source, &target).map_err(|e| format!("Failed to move file: {}", e))?;
        let _ = window.emit(
            "move-progress",
            Progress {
                current: index,
                total,
            },
        );
    }
    Ok(())
}
