use tauri::Manager;

#[tauri::command]
fn close_note(window: tauri::Window) {
    let label = window.label();
    let _ = window.get_webview_window(&label).unwrap().close();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![close_note])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
