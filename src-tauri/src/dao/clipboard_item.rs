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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ContentType {
    Text,
    Image, // 图片以文件路径或Base64编码存储
}
