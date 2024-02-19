
use arboard::{Clipboard, Error};
use once_cell::sync::Lazy;
use crate::clipboard::dao::{ClipboardContent, ClipboardItem};
use crate::clipboard::clipboard_image::ClipboardImage;

use std::sync::{Mutex};

static CLIPBOARD_GATEWAY_LOCK: Lazy<Mutex<OsClipboardGateway>> = Lazy::new(|| Mutex::new(OsClipboardGateway::new()));

pub(crate) struct OsClipboardGateway {
    clipboard: Clipboard,
}

impl OsClipboardGateway {
    fn new() -> Self {
        Self {
            clipboard: Clipboard::new().unwrap(),
        }
    }
    pub fn set(item: ClipboardItem)->Result<(), Error> {
        let mut gateway = CLIPBOARD_GATEWAY_LOCK.lock().unwrap();
        match item.content{
            ClipboardContent::Text(text) => {
                gateway.clipboard.set_text(text)
            }
            ClipboardContent::Image(image_data) => {
                let data = image_data.to_image_data();
                gateway.clipboard.set_image(data)
            }
        }
    }

    pub fn get() -> Result<ClipboardItem,Error>{
        let mut gateway = CLIPBOARD_GATEWAY_LOCK.lock().unwrap();
        gateway.clipboard.get_text().map(|text|{
            ClipboardItem::new(ClipboardContent::Text(text.clone()),text.chars().take(100).collect::<String>())
        }).or_else(|e|{
            gateway.clipboard.get_image().map(|image_data|{
                let clipboard_image = ClipboardImage::from(image_data);
                ClipboardItem::new(ClipboardContent::Image(clipboard_image.clone()), clipboard_image.to_base64_jpeg_thumbnail())
            })
        })
    }
}