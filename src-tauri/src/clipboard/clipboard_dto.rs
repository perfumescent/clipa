use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ClipboardDTO {
    data_type: String, // "text" 或 "image"
    content: String,   // 文本数据或 Base64 编码的图片数据
}

impl ClipboardDTO {
    // 定义一个关联函数作为构造器
    pub fn new(data_type: &str, content: String) -> Self {
        Self {
            data_type: data_type.to_string(),
            content,
        }
    }
}
