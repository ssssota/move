// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::read_config::read_config,
            commands::save_config::save_config,
            commands::commit::preview,
            commands::commit::commit,
            commands::select_directory::select_directory,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
