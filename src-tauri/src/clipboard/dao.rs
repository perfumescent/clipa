use serde::{Serialize, Deserialize};
use sled::{Db, IVec};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug)]
struct ClipboardItem {
    id: u64,
    content: Content,
    timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug)]
enum Content {
    Text(String),
    Image(String), // 图片以文件路径或Base64编码存储
}

struct ClipboardDao {
    db: Db,
}

impl ClipboardDao {
    // 初始化数据库并创建 ClipboardManager 实例
    fn new(db_path: &str) -> Result<Self, sled::Error> {
        let db = sled::open(db_path)?;
        Ok(Self { db })
    }

    // 插入新的剪贴项
    fn insert_clipboard_item(&self, item: ClipboardItem) -> Result<(), Box<dyn std::error::Error>> {
        let key = item.id.to_be_bytes();
        let value = serde_json::to_vec(&item)?;
        self.db.insert(key, value)?;
        Ok(())
    }

    // 读取所有剪贴项
    fn read_all_clipboard_items(&self) -> Result<Vec<ClipboardItem>, Box<dyn std::error::Error>> {
        let mut items = Vec::new();
        for result in self.db.iter() {
            let (_key, value) = result?;
            let item: ClipboardItem = serde_json::from_slice(&value)?;
            items.push(item);
        }

        // 按时间戳排序，确保最新的剪贴项在前
        items.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        Ok(items)
    }
}
