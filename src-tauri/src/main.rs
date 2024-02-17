// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use arboard::Clipboard;
mod clipboard;
use crate::clipboard::clipboard_listener::ClipboardListener;
use crate::clipboard::cmd::paste_on_window_snapshot;
use crate::clipboard::cmd::snapshot_on_current_window;
use crate::clipboard::dao::CLIPBOARD_DAO;
use clipboard_master::Master;
use std::{panic, thread};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    thread::spawn(|| loop {
        let vec = CLIPBOARD_DAO.read_all_clipboard_items().unwrap();
        println!("clipboard items:{:?}", vec);
        thread::sleep(std::time::Duration::from_secs(5));
    });

    tauri::Builder::default()
        .setup(|app| Ok(()))
        .invoke_handler(tauri::generate_handler![
            greet,
            snapshot_on_current_window,
            paste_on_window_snapshot
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
