use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::{LogicalPosition, LogicalSize, Manager};
use tauri_plugin_sql::{Migration, MigrationKind};
use tauri_plugin_store::StoreExt;

const STORE_PATH: &str = "store.bin";
const TRANSPARENT: bool = true;
const DECORATIONS: bool = false;

#[derive(Serialize, Deserialize, Debug)]
struct View {
    label: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

#[tauri::command]
async fn create_note(window: tauri::Window, label: String) {
    let webview_window = window.get_webview_window(window.label()).unwrap();
    let inner_position = webview_window.inner_position().unwrap();
    let inner_size = webview_window.inner_size().unwrap();
    let scale_factor = webview_window
        .current_monitor()
        .unwrap()
        .map(|m| m.scale_factor())
        .unwrap_or(1.);
    let position: LogicalPosition<u32> = inner_position.to_logical(scale_factor);
    let size: LogicalSize<u32> = inner_size.to_logical(scale_factor);
    let x: f64 = position.x.into();
    let y: f64 = position.y.into();
    let width: f64 = size.width.into();
    let height: f64 = size.height.into();

    let store = webview_window.store(STORE_PATH).unwrap();
    let value = store.get("views").expect("");
    let mut views: Vec<View> = serde_json::from_value(value).unwrap();
    views.push(View {
        label: label.clone(),
        x,
        y,
        width,
        height,
    });

    store.set("views", json!(views));

    let _ = store.save();
    store.close_resource();

    let _ = tauri::WebviewWindowBuilder::new(
        &window,
        label,
        tauri::WebviewUrl::App("index.html".into()),
    )
    .position(x, y)
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
    let value = store.get("views").expect("");
    let mut views: Vec<View> = serde_json::from_value(value).unwrap();

    let webview_window = window.get_webview_window(&label).unwrap();
    let inner_position = webview_window.inner_position().unwrap();
    let inner_size = webview_window.inner_size().unwrap();
    let scale_factor = webview_window
        .current_monitor()
        .unwrap()
        .map(|m| m.scale_factor())
        .unwrap_or(1.);
    let position: LogicalPosition<u32> = inner_position.to_logical(scale_factor);
    let size: LogicalSize<u32> = inner_size.to_logical(scale_factor);
    let x: f64 = position.x.into();
    let y: f64 = position.y.into();
    let width: f64 = size.width.into();
    let height: f64 = size.height.into();

    if let Some(index) = views
        .iter()
        .position(|view| *view.label == label.to_string())
    {
        views[index].x = x;
        views[index].y = y;
        views[index].width = width;
        views[index].height = height;
    }

    store.set("views", json!(views));
    let _ = store.save();
    store.close_resource();

    let _ = window.get_webview_window(&label).unwrap().close();
}

#[tauri::command]
fn delete_note(window: tauri::Window) {
    let label = window.label();

    let store = window.store(STORE_PATH).unwrap();
    let value = store.get("views").expect("");
    let mut views: Vec<View> = serde_json::from_value(value).unwrap();

    if let Some(index) = views
        .iter()
        .position(|view| *view.label == label.to_string())
    {
        views.remove(index);
    }

    if views.len() == 0 {
        let _ = store.delete("views");
    } else {
        store.set("views", json!(views));
    }

    let _ = store.save();
    store.close_resource();

    let _ = window.get_webview_window(&label).unwrap().close();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![
        Migration {
            version: 1,
            description: "create_initial_table",
            sql: "CREATE TABLE notes (id INTEGER PRIMARY KEY, label TEXT UNIQUE, lastModified TEXT, highlight TEXT, text TEXT);",
            kind: MigrationKind::Up,
        }
    ];

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let store = tauri_plugin_store::StoreBuilder::new(app, STORE_PATH).build()?;

            if !store.has("views") {
                let view = View {
                    label: String::from("note-0"),
                    x: 180.0,
                    y: 180.0,
                    width: 385.0,
                    height: 400.0,
                };
                store.set("views", json!(vec![view]));
            }

            let _ = store.save();

            let value = store.get("views").expect("");
            let views: Vec<View> = serde_json::from_value(value).unwrap();

            for view in views {
                let View {
                    label,
                    x,
                    y,
                    width,
                    height,
                } = view;
                let _ = tauri::WebviewWindowBuilder::new(
                    app,
                    label,
                    tauri::WebviewUrl::App("index.html".into()),
                )
                .position(x, y)
                .inner_size(width, height)
                .transparent(TRANSPARENT)
                .decorations(DECORATIONS)
                .build()?;
            }

            store.close_resource();

            Ok(())
        })
        .plugin(
            tauri_plugin_sql::Builder::new()
                .add_migrations("sqlite:notes.db", migrations)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            create_note,
            close_note,
            delete_note
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
