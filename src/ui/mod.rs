use crate::storage::models::{ClipboardContentType, ClipboardEntry};

pub mod workspace;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemType {
    Text,
    Link,
    Image,
    File,
}

#[derive(Clone, Debug)]
pub struct ClipboardItemView {
    pub id: i64,
    pub content_preview: String,
    pub timestamp: String,
    pub item_type: ItemType,
    pub full_content: String,
    pub pinned: bool,
}

impl From<&ClipboardEntry> for ClipboardItemView {
    fn from(entry: &ClipboardEntry) -> Self {
        let preview = if entry.content.len() > 120 {
            format!("{}...", &entry.content[..120])
        } else {
            entry.content.clone()
        };

        let item_type = match entry.content_type {
            ClipboardContentType::Image => ItemType::Image,
            ClipboardContentType::File => ItemType::File,
            ClipboardContentType::Text => {
                if entry.content.starts_with("http://") || entry.content.starts_with("https://") {
                    ItemType::Link
                } else if entry.content.contains("data:image") {
                    ItemType::Image
                } else {
                    ItemType::Text
                }
            }
        };

        Self {
            id: entry.id,
            content_preview: preview,
            timestamp: format_time_ago(entry.created_at),
            item_type,
            full_content: entry.content.clone(),
            pinned: entry.pinned,
        }
    }
}

fn format_time_ago(created_at: i64) -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    let diff = now - created_at;

    if diff < 0 {
        "Just now".to_string()
    } else if diff < 60 {
        "Just now".to_string()
    } else if diff < 3600 {
        let mins = diff / 60;
        format!("{}m ago", mins)
    } else if diff < 86400 {
        let hours = diff / 3600;
        format!("{}h ago", hours)
    } else {
        let days = diff / 86400;
        format!("{}d ago", days)
    }
}
