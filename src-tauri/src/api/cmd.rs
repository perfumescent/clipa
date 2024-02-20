use crate::api::clipboard_item_dto::ClipboardItemDTO;
use crate::clipboard::clipboard_context::ClipboardContext;
use crate::clipboard::database::{CLIPBOARD_DAO};

#[tauri::command]
pub fn wakeup() {
    ClipboardContext::wakeup();
}
#[tauri::command]
pub fn paste(clipboard_item_id: String) {
    ClipboardContext::paste_on_app(clipboard_item_id);
}

#[tauri::command]
pub fn query_clipboard_items() -> Vec<ClipboardItemDTO> {
    let vec = CLIPBOARD_DAO.read_all_clipboard_items().unwrap_or(Vec::new());
    vec.into_iter().map(|item| ClipboardItemDTO::new(item)).collect()
}


