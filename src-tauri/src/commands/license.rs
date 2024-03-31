use crate::commands::result::Result;

// ref. https://docs.rs/tauri/1.6.1/tauri/window/struct.WindowBuilder.html#known-issues
#[tauri::command]
pub async fn open_licenses<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> Result<()> {
    tauri::WindowBuilder::new(
        &app,
        "Licenses",
        tauri::WindowUrl::App("licenses.html".into()),
    )
    .title("Licenses")
    .build()
    .map(|_| ())
    .map_err(|e| e.to_string())
}
