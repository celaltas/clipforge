use crate::{config::settings::Settings, service::clipboard_service::ClipboardEntry};
use gpui::*;

pub struct AppState {
    pub settings: Settings,
    pub clipboard_items: Vec<ClipboardEntry>,
}

impl AppState {
    pub fn new(settings: Settings, initial_items: Vec<ClipboardEntry>) -> Self {
        Self {
            settings,
            clipboard_items: initial_items,
        }
    }

    pub fn add_item(&mut self, entry: ClipboardEntry, cx: &mut Context<Self>) {
        if self.clipboard_items.len() >= self.settings.max_history_items {
            self.clipboard_items.remove(0);
        }
        self.clipboard_items.push(entry);
        cx.notify();
    }

    pub fn get_items(&self) -> &[ClipboardEntry] {
        &self.clipboard_items
    }
}
