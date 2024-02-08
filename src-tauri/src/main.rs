// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use arboard::Clipboard;
mod clipboard;
use clipboard_master::Master;
use crate::clipboard::clipboard_listener::ClipboardListener;
use std::thread;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


fn main() {

    tauri::Builder::default()
        .setup(|app| {
            let clipboard = Clipboard::new().unwrap();
            let listener = ClipboardListener::new(clipboard,app.handle());
            // 在后台线程中执行剪贴板监听
            thread::spawn(move || {
                Master::new(listener).run().expect("Clipboard listener task run failed");
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
