use serde_json::json;
use tauri::{LogicalSize, Manager};
use tauri_plugin_store::StoreExt;
use uuid::Uuid;

const STORE_PATH: &str = "store.bin";
const TRANSPARENT: bool = true;
const DECORATIONS: bool = false;

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

    let store = webview_window.store(STORE_PATH).unwrap();
    store.set(label, json!({ "width": width, "height": height }));
    store.set(&new_label, json!({ "width": width, "height": height }));

    let _ = store.save();

    let _ = tauri::WebviewWindowBuilder::new(
        &window,
        new_label,
        tauri::WebviewUrl::App("index.html".into()),
    )
    .inner_size(width, height)
    .transparent(TRANSPARENT)
    .decorations(DECORATIONS)
    .build()
    .unwrap();
}

#[tauri::command]
fn close_note(window: tauri::Window) {
    let label = window.label();

    let store = window.store(STORE_PATH).unwrap();
    if store.entries().len() > 1 {
        store.delete(&label); // temp solution to remove too many windows
    } else {
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
        store.set(label, json!({ "width": width, "height": height }));
    }

    let _ = store.save();
    store.close_resource();

    let _ = window.get_webview_window(&label).unwrap().close();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let store = tauri_plugin_store::StoreBuilder::new(app, STORE_PATH).build()?;

            if store.entries().len() == 0 {
                let label = "note-0";
                store.set(label, json!({ "width": 385.0, "height": 400.0 }));
                let _ = store.save();
            }

            for (label, value) in store.entries() {
                let width = value["width"].as_f64().unwrap();
                let height = value["height"].as_f64().unwrap();

                let _ = tauri::WebviewWindowBuilder::new(
                    app,
                    label,
                    tauri::WebviewUrl::App("index.html".into()),
                )
                .inner_size(width, height)
                .transparent(TRANSPARENT)
                .decorations(DECORATIONS)
                .build()?;
            }

            store.close_resource();

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![close_note, new_note])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
