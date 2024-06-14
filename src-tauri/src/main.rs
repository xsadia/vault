// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    CustomMenuItem, GlobalWindowEvent, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, WindowEvent,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn window_event_handler(event: GlobalWindowEvent) {
    match event.event() {
        WindowEvent::CloseRequested { api, .. } => {
            event.window().hide().unwrap();
            api.prevent_close();
        }
        _ => {}
    }
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);

    let tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .on_window_event(window_event_handler)
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { tray_id, id, .. } => match id.as_str() {
                "quit" => std::process::exit(0),
                _ => {}
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
