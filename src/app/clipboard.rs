use crate::service::clipboard_service::ClipboardService;
use arboard::Clipboard;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct ClipboardListener {
    service: Arc<ClipboardService>,
    poll_interval: Duration,
}

impl ClipboardListener {
    pub fn new(service: Arc<ClipboardService>, poll_interval: Duration) -> Self {
        Self {
            service,
            poll_interval,
        }
    }

    pub fn start(self) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            let mut clipboard = Clipboard::new().expect("Failed to initialize clipboard");
            let mut last_content = String::new();
            tracing::info!("clipboard listener started");
            loop {
                match clipboard.get_text() {
                    Ok(new_content) => {
                        if !new_content.is_empty() && new_content != last_content {
                            let res = self.service.handle_clipboard_change(new_content.clone());
                            if let Err(e) = res {
                                eprintln!("Failed to handle clipboard change: {}", e);
                            } else {
                                tracing::info!("new content saved!");
                                last_content = new_content;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to read clipboard: {}", e);
                    }
                }
                thread::sleep(self.poll_interval);
            }
        })
    }
}
