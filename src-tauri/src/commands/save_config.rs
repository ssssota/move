use crate::commands::result::Result;
use crate::config::{Config, CONFIG_FILE};

#[tauri::command]
pub fn save_config<R: tauri::Runtime>(app: tauri::AppHandle<R>, config: Config) -> Result<()> {
    let config_path = tauri::api::path::config_dir()
        .ok_or_else(|| "Not found config directory".to_string())?
        .join(app.config().tauri.bundle.identifier.clone())
        .join(CONFIG_FILE);
    let config = serde_json::to_string(&config).map_err(|e| e.to_string())?;
    std::fs::create_dir_all(config_path.parent().unwrap()).map_err(|e| e.to_string())?;
    std::fs::write(config_path, config).map_err(|e| e.to_string())?;
    Ok(())
}
