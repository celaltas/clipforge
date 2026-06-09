mod app;
mod config;
mod logging;
mod service;
mod storage;
mod ui;

use config::loader::load_settings;
use gpui::{AppContext, WindowOptions};
use gpui_component::Root;
use std::sync::Arc;

use crate::{
    app::{clipboard::ClipboardListener, event::AppEvent, state::AppState},
    service::clipboard_service::ClipboardService,
    storage::{clipboard_repository::ClipboardRepository, database::Database},
    ui::workspace::{ClipboardItemView, ClipboardWorkspace},
};

fn main() {
    let _ = logging::init();

    tracing::info!("application starting");

    let settings = load_settings().expect("Failed to load config");
    let listener_interval = std::time::Duration::from_millis(settings.poll_interval);

    tracing::info!("config loaded");

    let database = Arc::new(Database::new().expect("Failed to initialize database"));

    tracing::info!("database initialized");

    let clipboard_repository = ClipboardRepository::new(database.clone());

    let mut initial_items = clipboard_repository
        .get_latest(Some(settings.max_history_items))
        .unwrap_or_else(|e| {
            tracing::error!("Failed to load initial history: {}", e);
            Vec::new()
        });

    initial_items.reverse();

    let (event_sender, event_receiver) = flume::unbounded::<AppEvent>();

    let clipboard_service = Arc::new(ClipboardService::new(event_sender, clipboard_repository));
    let clipboard_listener = ClipboardListener::new(clipboard_service, listener_interval);

    clipboard_listener.start();

    gpui_platform::application().run(move |cx| {
        gpui_component::init(cx);

        let app_state = cx.new(|_| AppState::new(settings, initial_items));

        cx.open_window(WindowOptions::default(), |window, cx| {
            let workspace = cx.new(|cx| ClipboardWorkspace::new(window, cx, app_state.clone()));

            cx.spawn(async move |cx| {
                while let Ok(event) = event_receiver.recv_async().await {
                    match event {
                        AppEvent::ClipboardSaved(entry) => {
                            app_state.update(cx, |state, cx| {
                                state.add_item(entry, cx);
                            });
                        }
                        _ => (),
                    }
                }
            })
            .detach();

            cx.new(|cx| Root::new(workspace, window, cx))
        })
        .expect("Failed to open window");
    });
}

fn format_time_ago(created_at: i64) -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let diff = now - created_at;

    if diff < 60 {
        "Şimdi".to_string()
    } else if diff < 3600 {
        format!("{} dk önce", diff / 60)
    } else if diff < 86400 {
        format!("{} saat önce", diff / 3600)
    } else {
        format!("{} gün önce", diff / 86400)
    }
}

fn convert_to_ui_item(
    entry: crate::service::clipboard_service::ClipboardEntry,
) -> ClipboardItemView {
    let preview = if entry.content.len() > 120 {
        format!("{}...", &entry.content[..120])
    } else {
        entry.content.clone()
    };

    let item_type = if entry.content.starts_with("http") || entry.content.contains("://") {
        "text".to_string()
    } else if entry.content.contains("data:image") || entry.content.len() > 500 {
        "image".to_string()
    } else {
        "text".to_string()
    };

    ClipboardItemView {
        id: entry.id.parse::<i64>().unwrap_or(0),
        content_preview: preview,
        timestamp: format_time_ago(entry.created_at),
        item_type,
        full_content: entry.content,
    }
}
