#[cfg(test)]
mod tests {
    use tauri_app::clipboard::database::{ClipboardContent, ClipboardItem};


    #[test]
    fn test_clipboard_item_new() {
        let item = ClipboardItem::new(
            ClipboardContent::Text("test".to_string()),
            "summary".to_string(),
        );
        println!("{:?}", serde_json::to_string(&item).unwrap());
    }
}