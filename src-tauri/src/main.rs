// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod app;
mod clipboard;
mod dao;

use crate::clipboard::clipboard_listener::ClipboardListener;
use api::cmd::{paste, query_clipboard_items, wakeup};
use tauri::Manager;
use window_shadows::set_shadow;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    ClipboardListener::run();
    // thread::spawn(|| loop {
    //     let vec = CLIPBOARD_DAO.read_all_clipboard_items().unwrap();
    //     println!("clipboard items:{:?}", vec);
    //     thread::sleep(std::time::Duration::from_secs(5));
    // });

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
            greet,
            wakeup,
            paste,
            query_clipboard_items
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
