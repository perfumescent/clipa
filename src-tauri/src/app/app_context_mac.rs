
use crate::clipboard::clipboard_os_gateway::OsClipboardGateway;
use crate::clipboard::database::CLIPBOARD_DAO;
use std::process::Command;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::app::app_context::{ClipaAppContext, CLIPBOARD_CONTEXT_LOCK};

static WINDOW_APP_ID_LOCK: Lazy<Mutex<String>> =
    Lazy::new(|| Mutex::new(String::new()));
impl ClipaAppContext {
    pub fn wakeup() {
        get_front_most_window_application_id()
            .map(|app_id| {
                println!("wakeup:{}", app_id);
                WINDOW_APP_ID_LOCK.lock().map(|mut lock| {
                    *lock = app_id;
                })
            })
            .ok();
    }

    pub fn paste_on_app(clipboard_item_id: String) {
        CLIPBOARD_DAO
            .query_clipboard_item(clipboard_item_id)
            .map(|item| {
                OsClipboardGateway::set(item)
                    .map(|| {
                        WINDOW_APP_ID_LOCK.lock().map(|lock| {
                            focus_on_window(lock.clone());
                            simulate_paste();
                        })
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
