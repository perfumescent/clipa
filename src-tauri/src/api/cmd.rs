use crate::api::clipboard_item_dto::ClipboardItemDTO;
use crate::app::app_context::ClipaAppContext;
use crate::dao::database::CLIPBOARD_DAO;

#[tauri::command]
pub fn wakeup(window: tauri::Window) {
    println!("wakeup");
    window.outer_position().map(|pos| {
        println!("window position: {:?}", pos);
    }).ok();
    ClipaAppContext::wakeup();
}
#[tauri::command]
pub fn paste(clipboard_item_id: String) {
    println!("paste:{}", clipboard_item_id);
    ClipaAppContext::paste_on_app(clipboard_item_id);
}

#[tauri::command]
pub fn query_clipboard_items(keyword: String) -> Vec<ClipboardItemDTO> {
    println!("query_clipboard_items");
    let param = if keyword.is_empty() {
        None
    } else {
        Some(keyword)
    };
    let vec = CLIPBOARD_DAO
        .read_all_clipboard_items(param)
        .unwrap_or(Vec::new());
    vec.into_iter()
        .map(|item| ClipboardItemDTO::new(item))
        .collect()
}

#[tauri::command]
pub fn delete_clipboard_item(clipboard_item_id: String) {
    println!("delete_clipboard_item:{}", clipboard_item_id);
    CLIPBOARD_DAO
        .delete_clipboard_item(&clipboard_item_id)
        .map_err(|e| {
            eprintln!(
                "Error when deleting clipboard item {}: {}",
                clipboard_item_id, e
            )
        })
        .ok();
}
