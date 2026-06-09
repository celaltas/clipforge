
use crate::{service::clipboard_service::ClipboardEntry};

#[derive(Debug, Clone)]
pub enum AppEvent {
    ClipboardSaved(ClipboardEntry),
}


