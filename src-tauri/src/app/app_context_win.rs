
use crate::clipboard::clipboard_os_gateway::OsClipboardGateway;
use crate::clipboard::database::CLIPBOARD_DAO;
use std::process::Command;
use winapi::shared::windef::HWND;
use winapi::um::winuser::GetForegroundWindow;
use crate::app::app_context::{ClipaAppContext, CLIPBOARD_CONTEXT_LOCK};

impl ClipaAppContext {
    pub fn new() -> Self {
        Self {
            invoking_app_id: "".to_string(),
        }
    }
    pub fn wakeup() {
        get_front_most_window_application_id()
            .map(|app_id| {
                println!("wakeup:{}", app_id);
                let mut context = CLIPBOARD_CONTEXT_LOCK.lock().unwrap();
                context.invoking_app_id = app_id;
            })
            .ok();
    }

    pub fn paste_on_app(clipboard_item_id: String) {
        CLIPBOARD_DAO
            .query_clipboard_item(clipboard_item_id)
            .map(|item| {
                OsClipboardGateway::set(item)
                    .map(|()| {
                        let context = CLIPBOARD_CONTEXT_LOCK.lock().unwrap();
                        focus_on_window(context.invoking_app_id.clone());
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
    let script = r#"
        tell application "System Events"
            keystroke "v" using {command down}
        end tell
    "#;

    if let Err(e) = Command::new("osascript").arg("-e").arg(script).output() {
        eprintln!("Failed to simulate paste: {}", e);
    }
}
fn get_front_most_window_application_id() -> Result<String, std::io::Error> {
    // 获取当前前台窗口的句柄
    unsafe {
        let hwnd: HWND = GetForegroundWindow();
        if hwnd.is_null() {
            eprintln!("No foreground window.");
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "No foreground window."));
        }
    }
    Ok("".to_string())
}
fn focus_on_window(application_id: String) {
    let script = format!(
        r#"
        tell application id {} to activate
    "#,
        application_id
    );

    Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| {
            eprintln!("Failed to focus_on_window {}: {}", application_id, e);
        })
        .ok();
}
