use tauri::{LogicalSize, Manager};
use uuid::Uuid;

#[tauri::command]
fn close_note(window: tauri::Window) {
    let label = window.label();
    let _ = window.get_webview_window(&label).unwrap().close();
}

#[tauri::command]
async fn new_note(window: tauri::Window) {
    let label = window.label();
    let webview_window = window.get_webview_window(&label).unwrap();
    let inner_size = webview_window.inner_size().unwrap();
    let scale_factor = webview_window
        .current_monitor()
        .unwrap()
        .map(|m| m.scale_factor())
        .unwrap_or(1.);
    let size: LogicalSize<u32> = inner_size.to_logical(scale_factor);

    let width: f64 = size.width.into();
    let height: f64 = size.height.into();
    let new_label = format!("note-{}", Uuid::new_v4());

    let _ = tauri::WebviewWindowBuilder::new(
        &window,
        new_label,
        tauri::WebviewUrl::App("index.html".into()),
    )
    .inner_size(width, height)
    .transparent(true)
    .decorations(false)
    .build()
    .unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let _ = tauri::WebviewWindowBuilder::new(
                app,
                "main",
                tauri::WebviewUrl::App("index.html".into()),
            )
            .title("note-window")
            .inner_size(385.0, 400.0)
            .transparent(true)
            .decorations(false)
            .build()?;

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![close_note, new_note])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
