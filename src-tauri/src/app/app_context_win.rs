
use crate::clipboard::clipboard_os_gateway::OsClipboardGateway;

use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::app::app_context::{ClipaAppContext};
use windows::Win32::Foundation::{BOOL, HWND};
use windows::Win32::System::Threading::{AttachThreadInput};
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetWindowThreadProcessId, SetForegroundWindow,
};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS
};
use windows::Win32::UI::Input::KeyboardAndMouse::{KEYEVENTF_KEYUP, VK_CONTROL, VK_V};
use crate::dao::database::CLIPBOARD_DAO;


static HWND_LOCK: Lazy<Mutex<HWND>> = Lazy::new(|| Mutex::new(HWND::default()));
impl ClipaAppContext {
    pub fn wakeup() {
        unsafe {
            // 获取当前前台窗口的句柄
            let hwnd: HWND = GetForegroundWindow();
            println!("hwnd:{:?}", hwnd);
            HWND_LOCK.lock().map(|mut hwnd_lock| {
                *hwnd_lock = hwnd;
            }).ok();
        }

    }

    pub fn paste_on_app(clipboard_item_id: String) {
        CLIPBOARD_DAO
            .query_clipboard_item(clipboard_item_id)
            .map(|item| {
                OsClipboardGateway::set(item)
                    .map(|()| {
                        simulate_paste();
                    })
                    .map_err(|e| {
                        eprintln!("Failed to set OsClipboard: {}", e);
                    })
            })
            .map_err(|e| {
                eprintln!("Failed to query_clipboard_item: {}", e);
            })
            .ok();
    }
}


fn simulate_paste() {
    unsafe {
        HWND_LOCK.lock().map(|hwnd_lock| {
            let hwnd = *hwnd_lock;
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
                // 在完成操作后，取消附加
                AttachThreadInput(foreground_thread_id, target_thread_id, BOOL::from(false));
            }
        }).ok();
    }
}

unsafe fn send_paste_command() {
    let inputs = [
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_CONTROL,
                    wScan: 0,
                    dwFlags: KEYBD_EVENT_FLAGS(0), // 0 for key press
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_V,
                    wScan: 0,
                    dwFlags: KEYBD_EVENT_FLAGS(0), // 0 for key press
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_V,
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP, // Key up
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_CONTROL,
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP, // Key up
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
    ];

    let inputs_sent = SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);

    if inputs_sent == 0 {
        println!("Failed to send_paste_command");
    };
}

