use crate::storage::models::ClipboardEntry;

#[derive(Debug, Clone)]
pub enum AppEvent {
    HistoryUpdated(Vec<ClipboardEntry>),
}

#[derive(Debug, Clone)]
pub enum UiAction {
    TogglePin(i64, bool),
    Delete(i64),
    Search(String)
}
