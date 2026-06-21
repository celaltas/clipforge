use crate::{
    app::event::AppEvent,
    storage::{
        clipboard_repository::ClipboardRepository,
        models::{ClipboardContentType, ClipboardEntry},
    },
};
use flume::Sender;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct ClipboardService {
    repo: ClipboardRepository,
    event_sender: Sender<AppEvent>,
}

impl ClipboardService {
    pub fn new(event_sender: Sender<AppEvent>, repo: ClipboardRepository) -> Self {
        Self { repo, event_sender }
    }

    pub fn handle_clipboard_change(&self, raw: String) -> anyhow::Result<()> {
        let content = normalize(raw);

        if content.is_empty() {
            return Ok(());
        }

        let last = self.repo.get_latest(1,0)?;
        if let Some(prev) = last.first()
            && prev.content == content
        {
            return Ok(());
        }
        let entry_to_insert = ClipboardEntry {
            id: 0,
            content: content.clone(),
            content_type: ClipboardContentType::Text,
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64,
            pinned: false,
        };

        self.repo.insert(entry_to_insert)?;

        let fresh_items = self.repo.get_latest(100, 0)?;
        let _ = self
            .event_sender
            .send(AppEvent::HistoryUpdated(fresh_items));
        Ok(())
    }

    pub fn toggle_pin(&self, id: i64, is_pinned: bool) -> anyhow::Result<()> {
        self.repo.toggle_pin(id, is_pinned)?;
        let fresh_items = self.repo.get_latest(100, 0)?;
        let _ = self
            .event_sender
            .send(AppEvent::HistoryUpdated(fresh_items));
        Ok(())
    }

    pub fn delete_entry(&self, id: i64) -> anyhow::Result<()> {
        self.repo.delete_entry(id)?;
        let fresh_items = self.repo.get_latest(100, 0)?;
        let _ = self
            .event_sender
            .send(AppEvent::HistoryUpdated(fresh_items));
        Ok(())
    }

    pub fn search(&self, query: String) -> anyhow::Result<()> {
        let fresh_items = if query.trim().is_empty() {
            self.repo.get_latest(100, 0)?
        } else {
            self.repo.search_entries(&query, 100, 0)?
        };

        let _ = self
            .event_sender
            .send(AppEvent::HistoryUpdated(fresh_items));
        Ok(())
    }
}

fn normalize(raw: String) -> String {
    let trimmed = raw.trim().to_string();
    trimmed.split_whitespace().collect::<Vec<_>>().join(" ")
}
