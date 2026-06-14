use crate::storage::models::ClipboardEntry;

#[derive(Debug, Clone)]
pub enum AppEvent {
    ClipboardSaved(ClipboardEntry),
    ClipboardDeleted(i64),
    ClipboardPinned(i64, bool),
}
