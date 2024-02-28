// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod app;
mod clipboard;
mod dao;

use crate::clipboard::clipboard_listener::ClipboardListener;
use api::cmd::{delete_clipboard_item, paste, query_clipboard_items, wakeup};
use tauri::Manager;
use window_shadows::set_shadow;


fn main() {
    ClipboardListener::run();

    tauri::Builder::default()
        .setup(|app| {
            app.windows().into_values().for_each(|window| {
                set_shadow(&window, true)
                    .map_err(|e| println!("Failed to set {} shadow: {}", window.label(), e))
                    .ok();
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            wakeup,
            paste,
            query_clipboard_items,
            delete_clipboard_item
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
