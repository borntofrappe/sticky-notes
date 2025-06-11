use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::{LogicalPosition, LogicalSize, Manager};
use tauri_plugin_sql::{Migration, MigrationKind};
use tauri_plugin_store::StoreExt;

#[derive(Serialize, Deserialize, Debug)]
struct View {
    label: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    visible: bool,
}

struct Rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

const STORE_PATH: &str = "store.bin";
const DB_PATH: &str = "sqlite:notes.db";
const STORE_KEY: &str = "views";
const NOTES_LIST_LABEL: &str = "notes-list";
const TRANSPARENT: bool = true;
const DECORATIONS: bool = false;

const NOTE_RECT: Rect = Rect {
    x: 150.0,
    y: 150.0,
    width: 320.0,
    height: 320.0,
};

const NOTES_LIST_RECT: Rect = Rect {
    x: 120.0,
    y: 120.0,
    width: 480.0,
    height: 680.0,
};

#[tauri::command]
async fn create_note(window: tauri::Window, label: String) {
    let mut x: f64 = NOTE_RECT.x;
    let mut y: f64 = NOTE_RECT.y;
    let mut width: f64 = NOTE_RECT.width;
    let mut height: f64 = NOTE_RECT.height;

    if window.label() != NOTES_LIST_LABEL {
        let webview_window = window.get_webview_window(window.label()).unwrap();
        let inner_position = webview_window.inner_position().unwrap();
        let inner_size = webview_window.inner_size().unwrap();

        let current_monitor = webview_window.current_monitor().unwrap().unwrap();
        let monitor_position = current_monitor.position();
        let monitor_size = current_monitor.size();
        let monitor_width: i32 = monitor_size.width.try_into().unwrap();
        let scale_factor = current_monitor.scale_factor();

        let position: LogicalPosition<u32> = inner_position.to_logical(scale_factor);
        let size: LogicalSize<u32> = inner_size.to_logical(scale_factor);

        if inner_position.x > monitor_position.x + monitor_width / 2 {
            x = (position.x - size.width).into();
        } else {
            x = (position.x + size.width).into();
        }
        y = position.y.into();
        width = size.width.into();
        height = size.height.into();
    }

    let store = window.store(STORE_PATH).unwrap();
    let value = store.get(STORE_KEY).expect("");
    let mut views: Vec<View> = serde_json::from_value(value).unwrap();
    views.push(View {
        label: label.clone(),
        x,
        y,
        width,
        height,
        visible: true,
    });

    store.set(STORE_KEY, json!(views));

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
async fn delete_note(window: tauri::Window) {
    let label = window.label();

    let store = window.store(STORE_PATH).unwrap();
    let value = store.get(STORE_KEY).expect("");
    let mut views: Vec<View> = serde_json::from_value(value).unwrap();

    if let Some(index) = views
        .iter()
        .position(|view| *view.label == label.to_string())
    {
        views.remove(index);
    }

    store.set(STORE_KEY, json!(views));

    let _ = store.save();
    store.close_resource();

    let _ = window.get_webview_window(&label).unwrap().close();
}

#[tauri::command]
async fn close_window(window: tauri::Window) {
    let label = window.label();

    let store = window.store(STORE_PATH).unwrap();
    let value = store.get(STORE_KEY).expect("");
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

    let visible_views = views.iter().filter(|view| view.visible == true).count();

    if let Some(index) = views
        .iter()
        .position(|view| *view.label == label.to_string())
    {
        views[index].x = x;
        views[index].y = y;
        views[index].width = width;
        views[index].height = height;
        if visible_views > 1 {
            views[index].visible = false
        }
    }

    store.set(STORE_KEY, json!(views));
    let _ = store.save();
    store.close_resource();

    let _ = window.get_webview_window(&label).unwrap().close();
}

#[tauri::command]
async fn show_window(window: tauri::Window, label: String) {
    let notes_list = window.get_webview_window(&label);

    match notes_list {
        Some(window) => {
            let _ = window.set_focus();
        }
        None => {
            // get and show instead of adding anew
            let store = window.store(STORE_PATH).unwrap();
            let value = store.get(STORE_KEY).expect("");
            let mut views: Vec<View> = serde_json::from_value(value).unwrap();

            if let Some(view) = views.iter_mut().find(|v| *v.label == label.to_string()) {
                let _ = tauri::WebviewWindowBuilder::new(
                    &window,
                    view.label.clone(),
                    tauri::WebviewUrl::App("index.html".into()),
                )
                .position(view.x, view.y)
                .inner_size(view.width, view.height)
                .transparent(TRANSPARENT)
                .decorations(DECORATIONS)
                .build()
                .unwrap();

                view.visible = true;
                store.set(STORE_KEY, json!(views));

                let _ = store.save();
                store.close_resource();
            };
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![
        Migration {
            version: 1,
            description: "create_initial_table",
            sql: "CREATE TABLE notes (id INTEGER PRIMARY KEY, label TEXT UNIQUE, lastModified TEXT, highlight TEXT, text TEXT, open INTEGER DEFAULT true);",
            kind: MigrationKind::Up,
        }
    ];

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let store = tauri_plugin_store::StoreBuilder::new(app, STORE_PATH).build()?;

            if !store.has(STORE_KEY) {
                let default_view = View {
                    label: String::from("main"),
                    x: NOTE_RECT.x,
                    y: NOTE_RECT.y,
                    width: NOTE_RECT.width,
                    height: NOTE_RECT.height,
                    visible: true,
                };
                let notes_list_view: View = View {
                    label: String::from(NOTES_LIST_LABEL),
                    x: NOTES_LIST_RECT.x,
                    y: NOTES_LIST_RECT.y,
                    width: NOTES_LIST_RECT.width,
                    height: NOTES_LIST_RECT.height,
                    visible: false,
                };
                store.set(STORE_KEY, json!(vec![default_view, notes_list_view]));
            }

            let _ = store.save();

            let value = store.get(STORE_KEY).expect("");
            let views: Vec<View> = serde_json::from_value(value).unwrap();

            for view in views {
                let View {
                    label,
                    x,
                    y,
                    width,
                    height,
                    visible,
                } = view;
                if visible {
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
            }

            store.close_resource();

            Ok(())
        })
        .plugin(
            tauri_plugin_sql::Builder::new()
                .add_migrations(DB_PATH, migrations)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            create_note,
            delete_note,
            close_window,
            show_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
