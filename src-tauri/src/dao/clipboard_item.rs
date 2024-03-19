use crate::clipboard::clipboard_image::ClipboardImage;
use serde::{Deserialize, Serialize};
use chrono::Local;
#[derive(Serialize, Deserialize, Debug)]
pub enum ClipboardContent {
    Text(String),
    Image(ClipboardImage),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClipboardItem {
    pub(crate) id: String,
    pub(crate) content: ClipboardContent,
    pub(crate) summary: String,
}

impl ClipboardItem {
    pub fn new(content: ClipboardContent, summary: String) -> Self {
        Self {
            id: Local::now().timestamp_millis().to_string(),
            content,
            summary,
        }
    }
}

impl ClipboardContent {
    pub(crate) fn contain(&self, keyword: &String) -> bool {
        match self {
            ClipboardContent::Text(text) => text.contains(keyword),
            ClipboardContent::Image(_) => false,
        }
    }
}
