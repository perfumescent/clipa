#[tauri::command]
pub fn snapshot_on_current_window() -> String {
    return get_frontmost_window_application_id().unwrap();
}
#[tauri::command]
pub fn paste_on_window_snapshot(id:String) {
    println!("paste_on_window_snapshot:{}",id);
    focus_on_window(id).unwrap();
    simulate_paste();
}



use std::process::Command;
fn simulate_paste() {
    let script = r#"
    tell application "System Events"
        keystroke "v" using {command down}
    end tell
    "#;

    if let Err(e) = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output() {
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

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()?;

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        println!("frontmost_window_application_id:{}", result.to_string());
        Ok(result.to_string())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to execute AppleScript"))
    }
}
fn focus_on_window(application_id:String) -> std::io::Result<()> {
    let script = format!(r#"
    tell application id {} to activate
    "#,application_id);

    Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()?;

    Ok(())
}