use chrono::Local;
use serde::{Deserialize, Serialize};
use crate::clipboard::clipboard_image::ClipboardImage;

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
    pub(crate) timestamp: i64,
}

impl ClipboardItem {
    pub fn new(content: ClipboardContent, summary: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            content,
            summary,
            timestamp: Local::now().timestamp_millis(),
        }
    }
}

impl ClipboardContent{

    pub(crate) fn contain(&self, keyword: &String) -> bool {
        match self {
            ClipboardContent::Text(text) => text.contains(keyword),
            ClipboardContent::Image(_) => false
        }
    }
}