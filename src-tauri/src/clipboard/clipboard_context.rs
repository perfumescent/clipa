use once_cell::sync::Lazy;
use std::sync::Mutex;
static CLIPBOARD_CONTEXT_LOCK: Lazy<Mutex<ClipboardContext>> =
    Lazy::new(|| Mutex::new(ClipboardContext::new()));
pub(crate) struct ClipboardContext {
    invoking_app_id: String,
}

impl ClipboardContext {
    pub(crate) fn new() -> Self {
        Self {
            invoking_app_id: "".to_string(),
        }
    }
    pub(crate) fn wakeup() {
        get_front_most_window_application_id()
            .map(|app_id| {
                println!("wakeup:{}", app_id);
                let mut context = CLIPBOARD_CONTEXT_LOCK.lock().unwrap();
                context.invoking_app_id = app_id;
            })
            .ok();
    }

    pub(crate) fn paste_on_app(clipboard_item_id: String) {
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

use crate::clipboard::clipboard_gateway::OsClipboardGateway;
use crate::clipboard::dao::CLIPBOARD_DAO;
use std::process::Command;

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
    let script = r#"
        tell application "System Events"
            set frontmostApp to first application process whose frontmost is true
            set appId to bundle identifier of frontmostApp
            if appId is missing value then
                set appId to name of frontmostApp
            end if
            return appId
        end tell
    "#;

    let output = Command::new("osascript").arg("-e").arg(script).output()?;

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        println!("front_most_window_application_id:{}", result.to_string());
        Ok(result.to_string())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to get front most window application id",
        ))
    }
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
