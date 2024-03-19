use crate::dao::clipboard_item::ClipboardItem;
use once_cell::sync::Lazy;
use sled::Db;

pub(crate) struct ClipboardDao {
    db: Db,
}

pub static CLIPBOARD_DAO: Lazy<ClipboardDao> = Lazy::new(|| ClipboardDao::new().unwrap());

impl ClipboardDao {
    // 初始化数据库并创建 ClipboardManager 实例
    fn new() -> Result<Self, sled::Error> {
        let db = Self::initialize_db()?;
        Ok(Self { db })
    }

    fn initialize_db() -> Result<Db, sled::Error> {
        // 尝试打开数据库，如果数据库不存在，`sled` 将会创建它
        let db = sled::open("clipboard_db")?;
        Ok(db)
    }

    // query by id
    pub(crate) fn query_clipboard_item(
        &self,
        id: String,
    ) -> Result<ClipboardItem, Box<dyn std::error::Error>> {
        let result = self.db.get(id)?;
        match result {
            Some(value) => {
                let item: ClipboardItem = serde_json::from_slice(&value)?;
                Ok(item)
            }
            None => Err("Not found".into()),
        }
    }

    // 插入新的剪贴项
    pub(crate) fn insert_clipboard_item(
        &self,
        item: ClipboardItem,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let value = serde_json::to_vec(&item)?;
        if self.db.len() > 500 {
            // 如果剪贴板中的剪贴项超过 500 个，删除最旧的一个
            self.db
                .pop_min()
                .map_err(|e| eprintln!("Error when removing the oldest: {}", e))
                .ok();
        }
        println!("insert_clipboard_item: {:?}", item);
        self.db.insert(item.id, value)?;
        Ok(())
    }

    // 删除指定的剪贴项
    pub(crate) fn delete_clipboard_item(
        &self,
        id: &String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.db.remove(id)?;
        Ok(())
    }
    // 读取所有剪贴项
    pub(crate) fn read_all_clipboard_items(
        &self,
        keyword: Option<String>,
    ) -> Result<Vec<ClipboardItem>, Box<dyn std::error::Error>> {
        let mut items = Vec::new();

        for result in self.db.iter() {
            let (_key, value) = result?;
            if let Ok(item) = serde_json::from_slice::<ClipboardItem>(&value) {
                match &keyword {
                    Some(k) if item.content.contain(k) => items.push(item),
                    None => items.push(item),
                    _ => {}
                }
            }
        }

        // 按时间戳排序，确保最新的剪贴项在前
        items.sort_by(|a, b| b.id.cmp(&a.id));
        Ok(items)
    }
}
