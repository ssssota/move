#[tauri::command]
pub fn select_directory<R: tauri::Runtime>(window: tauri::Window<R>) {
    tauri::api::dialog::FileDialogBuilder::new().pick_folder(move |f| {
        let _ = window.emit(
            "directory-select",
            f.map(|f| f.to_string_lossy().to_string()),
        );
    });
}
