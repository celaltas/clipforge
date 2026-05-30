use crate::app::state::SharedAppState;
use arboard::Clipboard;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub struct ClipboardEntry {
    pub id: String,
    pub content: String,
    pub created_at: i64,
}

pub fn start_clipboard_listener(state: SharedAppState) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut clipboard = Clipboard::new().expect("Failed to initialize clipboard");
        let mut last_content = String::new();
        tracing::info!("clipboard listener started");

        loop {
            match clipboard.get_text() {
                Ok(new_content) => {
                    if !new_content.is_empty() && new_content != last_content {
                        let new_entry = ClipboardEntry {
                            id: Uuid::now_v7().to_string(),
                            content: new_content.clone(),
                            created_at: SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_secs() as i64,
                        };
                        

                        if let Err(e) = state.add_clipboard_entry(new_entry) {
                            eprintln!("Failed to save clipboard entry: {}", e);
                        }
                         tracing::info!("new content saved!");

                        last_content = new_content;
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read clipboard: {}", e);
                }
            }

            thread::sleep(Duration::from_millis(500));
        }
    })
}
