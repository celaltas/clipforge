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

        let last = self.repo.get_latest(Some(1))?;
        if let Some(prev) = last.first() {
            if prev.content == content {
                return Ok(());
            }
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

        match self.repo.insert(entry_to_insert.clone()) {
            Ok(generated_id) => {
                tracing::info!("Saved with ID: {}", generated_id);

                let final_entry = ClipboardEntry {
                    id: generated_id,
                    ..entry_to_insert
                };

                let _ = self
                    .event_sender
                    .send(AppEvent::ClipboardSaved(final_entry));
                Ok(())
            }
            Err(e) => {
                tracing::error!("DB error, skipped memory update: {}", e);
                Err(e)
            }
        }
    }
}

fn normalize(raw: String) -> String {
    let trimmed = raw.trim().to_string();
    let collapsed = trimmed.split_whitespace().collect::<Vec<_>>().join(" ");
    collapsed
}
