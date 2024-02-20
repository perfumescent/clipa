// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod app;
mod clipboard;

use crate::clipboard::clipboard_listener::ClipboardListener;
use api::cmd::{paste, query_clipboard_items, wakeup};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// fn main() {
//     ClipboardListener::run();
//     // thread::spawn(|| loop {
//     //     let vec = CLIPBOARD_DAO.read_all_clipboard_items().unwrap();
//     //     println!("clipboard items:{:?}", vec);
//     //     thread::sleep(std::time::Duration::from_secs(5));
//     // });
//
//     tauri::Builder::default()
//         .setup(|app| Ok(()))
//         .invoke_handler(tauri::generate_handler![
//             greet,
//             wakeup,
//             paste,
//             query_clipboard_items
//         ])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }
use std::time::Duration;
use std::{mem, thread};
use windows::core::Result;
use windows::Win32::Foundation::{BOOL, HWND};
use windows::Win32::System::Threading::{AttachThreadInput, GetCurrentThreadId};
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetWindowThreadProcessId, SetForegroundWindow,
};

fn main() {
    unsafe {
        loop {
            // 获取当前前台窗口的句柄
            let hwnd: HWND = GetForegroundWindow();
            println!("hwnd:{:?}", hwnd);
            // sleep 3s
            thread::sleep(Duration::from_secs(3));
            // 获取当前前台窗口句柄和它的线程ID
            let hwnd_foreground = GetForegroundWindow();
            let foreground_thread_id = GetWindowThreadProcessId(hwnd_foreground, None);

            // 获取目标窗口的线程ID
            let target_thread_id = GetWindowThreadProcessId(hwnd, None);

            // 附加到目标窗口的线程，以便可以操作它
            if target_thread_id != foreground_thread_id
                && AttachThreadInput(foreground_thread_id, target_thread_id, BOOL::from(true))
                    .as_bool()
            {
                // 将目标窗口设置为前台
                let bool = SetForegroundWindow(hwnd);
                println!("bool:{:?}", bool);
                // SetFocus(hwnd);
                send_paste_command();
                thread::sleep(Duration::from_secs(3));
                // 在完成操作后，取消附加
                AttachThreadInput(foreground_thread_id, target_thread_id, BOOL::from(false));
            }
        }
    }
}

use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, VK_RETURN,
};
use windows::Win32::UI::Input::KeyboardAndMouse::{KEYEVENTF_KEYUP, VK_CONTROL, VK_V};
