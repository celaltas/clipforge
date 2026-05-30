use crate::{app::clipboard::ClipboardEntry, config::settings::Settings};
use rusqlite::{Connection, params};
use std::sync::{Arc, Mutex};

pub type SharedAppState = Arc<AppState>;

pub struct AppState {
    pub settings: Settings,
    pub db: Mutex<Connection>,
    pub clipboard_items: Mutex<Vec<ClipboardEntry>>,
}

impl AppState {
    pub fn add_clipboard_entry(&self, entry: ClipboardEntry) -> anyhow::Result<()> {
        if entry.content.trim().is_empty() {
            return Ok(());
        }

        let conn = self.db.lock().unwrap();
        conn.execute(
            "INSERT INTO clipboard_entries (id, content, created_at) VALUES (?1, ?2, ?3)",
            params![entry.id, entry.content, entry.created_at],
        )?;

        {
            let mut items = self.clipboard_items.lock().unwrap();
            if items.len() >= self.settings.max_history_items {
                items.remove(0);
            }
            items.push(entry);
        }

        Ok(())
    }
}
