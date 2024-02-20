use crate::api::clipboard_item_dto::ClipboardItemDTO;
use crate::app::app_context::ClipaAppContext;
use crate::dao::database::CLIPBOARD_DAO;

#[tauri::command]
pub fn wakeup() {
    ClipaAppContext::wakeup();
}
#[tauri::command]
pub fn paste(clipboard_item_id: String) {
    println!("paste:{}", clipboard_item_id);
    ClipaAppContext::paste_on_app(clipboard_item_id);
}

#[tauri::command]
pub fn query_clipboard_items() -> Vec<ClipboardItemDTO> {
    let vec = CLIPBOARD_DAO
        .read_all_clipboard_items()
        .unwrap_or(Vec::new());
    vec.into_iter()
        .map(|item| ClipboardItemDTO::new(item))
        .collect()
}
