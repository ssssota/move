use std::fs;
use std::path::Path;

#[cfg(not(target_os = "windows"))]
pub fn move_file(source: &str, target: &str) -> Result<(), String> {
    fs::create_dir_all(Path::new(target).parent().unwrap())
        .map_err(|e| format!("Failed to create missing directory: {}", e))?;
    fs::rename(source, target).map_err(|e| format!("Failed to move file: {}", e))
}
#[cfg(target_os = "windows")]
pub fn move_file(source: &str, target: &str) -> Result<(), String> {
    fs::create_dir_all(Path::new(target).parent().unwrap())
        .map_err(|e| format!("Failed to create missing directory: {}", e))?;
    let is_cross_drive =
        source.chars().take(3).collect::<String>() != target.chars().take(3).collect::<String>();
    if is_cross_drive {
        fs::copy(source, target).map_err(|e| format!("Failed to copy file: {}", e))?;
        fs::remove_file(source).map_err(|e| format!("Failed to remove file: {}", e))
    } else {
        fs::rename(source, target).map_err(|e| format!("Failed to move file: {}", e))
    }
}
