use arboard::{Clipboard, Get};
use once_cell::sync::Lazy;
use crate::clipboard::dao::{ClipboardItem, ContentType};

pub(crate) static CLIPBOARD_GATEWAY: Lazy<OsClipboardGateway> = Lazy::new(|| OsClipboardGateway::new());


pub(crate) struct OsClipboardGateway {
    clipboard: Clipboard,
}

impl OsClipboardGateway {
    pub fn new() -> Self {
        Self {
            clipboard: Clipboard::new().unwrap(),
        }
    }
    pub fn set(&mut self, item: ClipboardItem) {
        match item.content_type {
            ContentType::Text => {
                // 类型转换
                item.content.downcast_ref::<String>().map(|text| {
                    self.clipboard.set_text(text).ok();
                });
            }
            ContentType::Image => {
                
            }
        }
    }

    pub fn get(&mut self) -> Get<'_> {
        self.clipboard.get()
    }
}