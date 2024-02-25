use crate::clipboard::clipboard_image::ClipboardImage;
use arboard::{Clipboard, Error};
use once_cell::sync::Lazy;
use crate::dao::clipboard_item::{ClipboardContent, ClipboardItem};
use std::sync::Mutex;

static CLIPBOARD_GATEWAY_LOCK: Lazy<Mutex<OsClipboardGateway>> =
    Lazy::new(|| Mutex::new(OsClipboardGateway::new()));

pub(crate) struct OsClipboardGateway {
    clipboard: Clipboard,
}

impl OsClipboardGateway {
    fn new() -> Self {
        Self {
            clipboard: Clipboard::new().unwrap(),
        }
    }
    pub(crate) fn set(item: ClipboardItem) -> Result<(), Error> {
        let mut gateway = CLIPBOARD_GATEWAY_LOCK.lock().unwrap();
        match item.content {
            ClipboardContent::Text(text) => gateway.clipboard.set_text(text),
            ClipboardContent::Image(image_data) => {
                let data = image_data.to_image_data();
                gateway.clipboard.set_image(data)
            }
        }
    }

    pub(crate) fn get() -> Result<ClipboardItem, Error> {
        let mut gateway = CLIPBOARD_GATEWAY_LOCK.lock().unwrap();
        gateway
            .clipboard
            .get_text()
            .map(|text| {
                let summary = text.chars().take(100).collect::<String>();
                ClipboardItem::new(
                    ClipboardContent::Text(text),
                    summary,
                )
            })
            .or_else(|_e| {
                gateway.clipboard.get_image().map(|image_data| {
                    let clipboard_image = ClipboardImage::from(image_data);
                    let base64 = clipboard_image.to_base64_jpeg_thumbnail();
                    ClipboardItem::new(
                        ClipboardContent::Image(clipboard_image),
                        base64,
                    )
                })
            })
    }
}

