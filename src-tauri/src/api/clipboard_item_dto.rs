use crate::dao::clipboard_item::{ClipboardContent, ClipboardItem};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ClipboardItemDTO {
    id: String,
    content_type: String,
    summary: String,
}

impl ClipboardItemDTO {
    pub fn new(item: ClipboardItem) -> Self {
        Self {
            id: item.id,
            content_type: match item.content {
                ClipboardContent::Text(_) => "Text".to_string(),
                ClipboardContent::Image(_) => "Image".to_string(),
            },
            summary: item.summary,
        }
    }
}
