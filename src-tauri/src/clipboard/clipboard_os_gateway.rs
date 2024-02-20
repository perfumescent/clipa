
use arboard::{Clipboard, Error};
use once_cell::sync::Lazy;
use crate::clipboard::database::{ClipboardContent, ClipboardItem};
use crate::clipboard::clipboard_image::ClipboardImage;

use std::sync::{Mutex};


static CLIPBOARD_GATEWAY_LOCK: Lazy<Mutex<OsClipboardGateway>> = Lazy::new(|| Mutex::new(OsClipboardGateway::new()));

pub(crate) struct OsClipboardGateway {
    clipboard: Clipboard,
}

impl OsClipboardGateway {
    fn new() -> Self {
        Self {
            clipboard: Clipboard::new().unwrap(),
        }
    }
    pub(crate) fn set(item: ClipboardItem)->Result<(), Error> {
        let mut gateway = CLIPBOARD_GATEWAY_LOCK.lock().unwrap();
        match item.content{
            ClipboardContent::Text(text) => {
                gateway.clipboard.set_text(text)
            }
            ClipboardContent::Image(image_data) => {
                let data = image_data.to_image_data();
                gateway.clipboard.set_image(data)
            }
        }
    }

    pub(crate)  fn get() -> Result<ClipboardItem,Error>{
        let mut gateway = CLIPBOARD_GATEWAY_LOCK.lock().unwrap();
        gateway.clipboard.get_text().map(|text|{
            ClipboardItem::new(ClipboardContent::Text(text.clone()),text.chars().take(100).collect::<String>())
        }).or_else(|e|{
            gateway.clipboard.get_image().map(|image_data|{
                let clipboard_image = ClipboardImage::from(image_data);
                ClipboardItem::new(ClipboardContent::Image(clipboard_image.clone()), clipboard_image.to_base64_jpeg_thumbnail())
            })
        })
    }

}

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
fn get_frontmost_window_application_id() -> std::io::Result<String> {
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
        println!("frontmost_window_application_id:{}", result.to_string());
        Ok(result.to_string())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to execute AppleScript",
        ))
    }
}
fn focus_on_window(application_id: String) -> std::io::Result<()> {
    let script = format!(
        r#"
    tell application id {} to activate
    "#,
        application_id
    );

    Command::new("osascript").arg("-e").arg(script).output()?;

    Ok(())
}