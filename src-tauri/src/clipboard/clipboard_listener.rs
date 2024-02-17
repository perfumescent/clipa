use crate::clipboard::dao::ContentType::{Image, Text};
use crate::clipboard::dao::{ClipboardItem, CLIPBOARD_DAO};

use clipboard_master::{CallbackResult, ClipboardHandler, Master};

use std::io::Error;
use std::{panic, thread};
use crate::clipboard::clipboard_gateway::CLIPBOARD_GATEWAY;


pub struct ClipboardListener;

impl ClipboardListener {
    // 提供一个新的构造函数来创建实例
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn run() {
        // 如果遇到异常跑出，不允许panic，应该继续运行监听线程
        panic::catch_unwind(|| {
            // 在后台线程中执行剪贴板监听
            thread::spawn(move || {
                let listener = ClipboardListener::new();
                // 这里的代码可能会panic
                Master::new(listener).run()
            });
        })
        .map_err(|e| {
            // 如果有异常，打印异常信息
            println!("Clipboard listener panicked: {:?}", e);
            println!("Restarting the listener...");
            Self::run();
        })
        .ok(); // 将外部Result转换为Option，忽略最终结果，仅为了链式处理
    }
}

impl ClipboardHandler for ClipboardListener {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        println!("*********Clipboard changed***********");
        CLIPBOARD_GATEWAY
            .get()
            .text()
            .map(|text| {
                println!("Got text: {}", text); // 打印成功获取的文本
                let summary = text.chars().take(20).collect::<String>();
                CLIPBOARD_DAO
                    .insert_clipboard_item(ClipboardItem::new(Text, text, summary))
                    .unwrap(); // 插入到数据库
            })
            .map_err(|e| {
                println!("Error getting text: {}", e); // 处理获取文本的错误
                let image_data = CLIPBOARD_GATEWAY.get().image().unwrap();
                let base64_jpeg = image_data_to_base64_jpeg(
                    image_data.bytes.clone(),
                    image_data.width,
                    image_data.height,
                );
                let base64_jpeg_thumbnail = image_data_to_base64_jpeg_thumbnail(
                    image_data.bytes,
                    image_data.width,
                    image_data.height,
                );
                CLIPBOARD_DAO
                    .insert_clipboard_item(ClipboardItem::new(
                        Image,
                        base64_jpeg,
                        base64_jpeg_thumbnail,
                    ))
                    .ok(); // 插入到数据库
            })
            .ok(); // 将外部Result转换为Option，忽略最终结果，仅为了链式处理

        println!("*********Clipboard updated**********");

        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: Error) -> CallbackResult {
        println!("Error: {}", error);
        CallbackResult::Next
    }
}
