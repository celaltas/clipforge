use crate::{app::event::AppEvent, storage::clipboard_repository::ClipboardRepository};
use flume::Sender;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ClipboardEntry {
    pub id: String,
    pub content: String,
    pub created_at: i64,
}

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
        let entry = ClipboardEntry {
            id: Uuid::now_v7().to_string(),
            content: content.clone(),
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64,
        };

        match self.repo.insert(entry.clone()) {
            Ok(_) => {
                tracing::info!("Saved: {}", content);
                let _ = self.event_sender.send(AppEvent::ClipboardSaved(entry));
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
