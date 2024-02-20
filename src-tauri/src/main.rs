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
use std::{mem, thread};
use std::time::Duration;
use winapi::shared::windef::HWND;
use winapi::um::winuser::{AllowSetForegroundWindow, ASFW_ANY, GetFocus, GetWindowTextW, INPUT_u, LPINPUT, SendMessageW, SPI_GETFOREGROUNDLOCKTIMEOUT, SW_SHOW, VK_CONTROL};
use winapi::um::winuser::{
    GetForegroundWindow, SetForegroundWindow, ShowWindow, SW_HIDE, SW_MINIMIZE, SW_RESTORE,
};

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use winapi::um::winnt::{PVOID, WCHAR};

use winapi::um::winuser::{keybd_event};
use winapi::um::winuser::{SystemParametersInfoW, GetWindowThreadProcessId, AttachThreadInput, SetFocus, BringWindowToTop , SPI_SETFOREGROUNDLOCKTIMEOUT, KEYBDINPUT, INPUT_KEYBOARD, KEYEVENTF_KEYUP};
use winapi::um::processthreadsapi::GetCurrentThreadId;
use std::ptr::null_mut;use winapi::um::winuser::{ WM_SETTEXT};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::minwindef::{LPARAM, WPARAM};
fn win_title(hwnd: HWND){
    unsafe{
        // 准备缓冲区接收窗口标题
        let mut buffer: [WCHAR; 256] = [0; 256];
        let len = GetWindowTextW(hwnd, buffer.as_mut_ptr(), buffer.len() as i32);

        // 将宽字符串转换为Rust字符串
        let window_title = OsString::from_wide(&buffer[..len as usize])
            .to_string_lossy()
            .into_owned();

        println!("Current window {:?} title: {}", hwnd, window_title);
    }
}
fn main() {
    unsafe {
        loop {
            // 获取当前前台窗口的句柄
            let hwnd: HWND = GetForegroundWindow();
            if hwnd.is_null() {
                eprintln!("No foreground window.");
                continue;
            }
            win_title(hwnd);
            // 模拟做一些工作...
            thread::sleep(Duration::from_secs(5));

            // 将焦点切换回之前保存的窗口
            // 如果窗口被最小化，先恢复它
            // ShowWindow(GetForegroundWindow(), SW_MINIMIZE);
            // thread::sleep(Duration::from_secs(1));
            // 获取程序 A 和程序 B 的线程 ID
            let clipa=GetForegroundWindow();

            let target_thread_id = GetCurrentThreadId();
            let current_thread_id  = GetWindowThreadProcessId(hwnd, null_mut());

            // let current_thread_id = GetWindowThreadProcessId(clipa, null_mut());
            // let target_thread_id  = GetWindowThreadProcessId(hwnd, null_mut());
            println!("target_thread_id:{}, current_thread_id:{}", target_thread_id, current_thread_id);


            // 附加程序 A 的输入线程到程序 B
            if current_thread_id != target_thread_id{
                println!("GetFocus:{:?}",GetFocus());
                if AttachThreadInput(current_thread_id, target_thread_id, true as i32) == 0 {
                    println!("附加输入线程失败。");
                    return;
                }

                let timeout: u32 = 0;
                // SystemParametersInfoW(
                //     SPI_SETFOREGROUNDLOCKTIMEOUT,
                //     0,
                //     &timeout as *const _ as PVOID,
                //     0
                // );
                BringWindowToTop(hwnd);
                SetForegroundWindow(hwnd);

                let set_focus = SetFocus(hwnd);
                println!("SetFocus:{:?}",set_focus);
                println!("GetFocus:{:?}",GetFocus());
                win_title(hwnd);
                win_title(GetFocus());
                send_key_input(hwnd, 0x51);
                // 分离输入线程
                let detach = AttachThreadInput(current_thread_id, target_thread_id, false as i32);
                if detach == 0 {
                    println!("分离输入线程失败。");
                    return;
                }

            }

        }
    }
}
use winapi::um::winuser::{ WM_KEYDOWN, WM_KEYUP};
fn send_key_input(hwnd: winapi::shared::windef::HWND, key: u16) {
    unsafe {
        // 模拟按键按下
        SendMessageW(hwnd, WM_KEYDOWN, key as WPARAM, 0);
        // 模拟字符输入，如果需要的话
        // SendMessageW(hwnd, WM_CHAR, key as WPARAM, 0);
        // 模拟按键释放
        SendMessageW(hwnd, WM_KEYUP, key as WPARAM, 0);
    }
}