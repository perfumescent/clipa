use clipboard_master::{CallbackResult, ClipboardHandler, Master};

use crate::clipboard::clipboard_gateway::OsClipboardGateway;
use crate::clipboard::dao::CLIPBOARD_DAO;
use std::io::Error;
use std::{panic, thread};

pub struct ClipboardListener;

impl ClipboardListener {
    // 提供一个新的构造函数来创建实例
    pub fn new() -> Self {
        Self {}
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
        OsClipboardGateway::get()
            .map(|item| {
                CLIPBOARD_DAO.insert_clipboard_item(item).map_err(|e| {
                    println!("Error when inserting database: {}", e);
                })
            })
            .ok();

        println!("*********Clipboard updated**********");

        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: Error) -> CallbackResult {
        println!("Error: {}", error);
        CallbackResult::Next
    }
}
