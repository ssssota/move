use crate::commands::result::Result;
use crate::config::{Config, CONFIG_FILE};

#[tauri::command]
pub fn read_config<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> Result<Config> {
    let config_path = tauri::api::path::config_dir()
        .ok_or_else(|| "Not found config directory".to_string())?
        .join(app.config().tauri.bundle.identifier.clone())
        .join(CONFIG_FILE);
    let config = std::fs::read_to_string(config_path).map_err(|e| e.to_string())?;
    let config: Config = serde_json::from_str(&config).map_err(|e| e.to_string())?;
    Ok(config)
}
