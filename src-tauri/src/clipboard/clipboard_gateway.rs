
use arboard::{Clipboard, Error};
use once_cell::sync::Lazy;
use crate::clipboard::dao::{ClipboardContent, ClipboardItem};
use crate::clipboard::image_data::ClipboardImage;

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
    pub fn set(&mut self, item: ClipboardItem)->Result<(), Error> {
        match item.content{
            ClipboardContent::Text(text) => {
                self.clipboard.set_text(text)
            }
            ClipboardContent::Image(image_data) => {
                let data = image_data.to_image_data();
                self.clipboard.set_image(data)
            }
        }
    }

    pub fn get(&mut self) -> Result<ClipboardItem,Error>{
        self.clipboard.get_text().map(|text|{
            ClipboardItem::new(ClipboardContent::Text(text),text.chars().take(100).collect::<String>())
        }).or_else(|e|{
            self.clipboard.get_image().map(|image_data|{
                let clipboard_image = ClipboardImage::from(image_data);
                ClipboardItem::new(ClipboardContent::Image(clipboard_image), clipboard_image.to_base64_jpeg_thumbnail())
            })
        })
    }
}