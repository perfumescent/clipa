
use clipboard_master::{CallbackResult, ClipboardHandler};
use std::io::Error;
use arboard::Clipboard;
use tauri::{AppHandle, Manager};
use base64::{encode};
use std::borrow::Cow;
use crate::clipboard::clipboard_dto::ClipboardDTO;

use image::{ImageBuffer, RgbaImage};

fn image_data_to_base64_jpeg(image_data: Cow<'_, [u8]>, width: usize, height: usize) -> String {
    let width_u32 = u32::try_from(width)
        .expect("width is too large to fit into a u32");
    let height_u32 = u32::try_from(height)
        .expect("height is too large to fit into a u32");

    // 使用 ImageBuffer 来创建一个 RgbaImage
    let img: RgbaImage = ImageBuffer::from_raw(width_u32, height_u32, image_data.to_vec()).unwrap();

    // 将图像编码为 JPEG
    let mut jpeg_bytes = Vec::new();
    image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg_bytes, 100) // 指定 JPEG 质量
        .encode_image(&img)
        .expect("Failed to encode image to JPEG");

    // 将 JPEG 字节编码为 Base64
    let base64_string = encode(&jpeg_bytes);

    // 创建 Data URL
    format!("data:image/jpeg;base64,{}", base64_string)
}

pub struct ClipboardListener{
    clipboard: Clipboard, // 添加 clipboard 字段
    app: AppHandle
}
impl ClipboardListener {
    // 提供一个新的构造函数来创建实例
    pub fn new(clipboard:Clipboard,app: AppHandle) -> Self {
        Self {
            clipboard, // 初始化 clipboard
            app
        }
    }
}
impl ClipboardHandler for ClipboardListener {

    fn on_clipboard_change(&mut self) -> CallbackResult {
        println!("*********Clipboard changed***********");
        // tokio::task::block_in_place(|| {
        //     tokio::runtime::Handle::current()
        //         .block_on(async { ClipboardHelper::upsert_clipboard().await })
        // });
        self.clipboard.get().text()
            .map(|text| {
                println!("Got text: {}", text); // 打印成功获取的文本
                let dto = ClipboardDTO::new("text", text);
                self.app.emit_all("clipboard-changed", dto).unwrap(); // 发送事件
            })
            .map_err(|e| {
                println!("Error getting text: {}", e); // 处理获取文本的错误
                self.clipboard.get().image() // 尝试获取图片
                    .map(|image| {
                        // println!("Got image: {:?}",image); // 打印成功获取的图片
                        let dto = ClipboardDTO::new("image", image_data_to_base64_jpeg(image.bytes, image.width, image.height));
                        self.app.emit_all("clipboard-changed",  dto).unwrap(); // 发送事件
                    })
                    .map_err(|e| {
                        println!("Error getting image: {}", e); // 处理获取图片的错误
                    }).ok() // 将内部Result转换为Option来忽略结果，仅为了错误处理
            }).ok(); // 将外部Result转换为Option，忽略最终结果，仅为了链式处理

        println!("*********Clipboard updated**********");

        // first copy doesnt work, so we do it twice
        // tauri::async_runtime::spawn(async { upsert_clipboard().await });

        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: Error) -> CallbackResult {
        println!("Error: {}", error);
        CallbackResult::Next
    }
}
