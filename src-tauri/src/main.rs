// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use arboard::Clipboard;
mod clipboard;
use crate::clipboard::dao::CLIPBOARD_DAO;

use std::{thread};
use crate::clipboard::clipboard_listener::ClipboardListener;

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

    // tauri::Builder::default()
    //     .setup(|app| Ok(()))
    //     .invoke_handler(tauri::generate_handler![
    //         greet,
    //         snapshot_on_current_window,
    //         paste_on_window_snapshot
    //     ])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}
