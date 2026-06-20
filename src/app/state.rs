use crate::{config::settings::Settings, storage::models::ClipboardEntry};
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

    pub fn set_items(&mut self, items: Vec<ClipboardEntry>, cx: &mut Context<Self>) {
        self.clipboard_items = items;
        cx.notify();
    }

    pub fn get_items(&self) -> &[ClipboardEntry] {
        &self.clipboard_items
    }
}
