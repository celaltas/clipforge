mod app;
mod config;
mod logging;
mod service;
mod storage;
mod ui;

use config::loader::load_settings;
use gpui::{AppContext, Focusable, KeyBinding, WindowOptions, actions};
use gpui_component::Root;
use std::sync::Arc;

use crate::{
    app::{
        clipboard::ClipboardListener,
        event::{AppEvent, UiAction},
        state::AppState,
    },
    service::clipboard_service::ClipboardService,
    storage::{clipboard_repository::ClipboardRepository, database::Database},
    ui::workspace::ClipboardWorkspace,
};

actions!(
    clipboard_workspace,
    [
        SelectNext,
        SelectPrevious,
        CopySelected,
        DeleteSelected,
        TogglePinSelected,
    ]
);

fn main() {
    let _ = logging::init();

    tracing::info!("application starting");

    let settings = load_settings().expect("Failed to load config");
    let listener_interval = std::time::Duration::from_millis(settings.poll_interval);

    tracing::info!("config loaded");

    let database = Arc::new(Database::new().expect("Failed to initialize database"));

    tracing::info!("database initialized");

    let clipboard_repository = ClipboardRepository::new(database.clone());

    let initial_items = clipboard_repository
        .get_latest(settings.max_history_items, 0)
        .unwrap_or_else(|e| {
            tracing::error!("Failed to load initial history: {}", e);
            Vec::new()
        });

    let (event_sender, event_receiver) = flume::unbounded::<AppEvent>();
    let (action_sender, action_receiver) = flume::unbounded::<UiAction>();

    let clipboard_service = Arc::new(ClipboardService::new(event_sender, clipboard_repository));
    let clipboard_listener = ClipboardListener::new(clipboard_service.clone(), listener_interval);

    clipboard_listener.start();

    gpui_platform::application().run(move |cx| {
        cx.bind_keys([
            KeyBinding::new("down", SelectNext, None),
            KeyBinding::new("up", SelectPrevious, None),
            KeyBinding::new("cmd-c", CopySelected, None),
            KeyBinding::new("backspace", DeleteSelected, None),
            KeyBinding::new("cmd-p", TogglePinSelected, None),
        ]);
        gpui_component::init(cx);

        let app_state = cx.new(|_| AppState::new(settings, initial_items));

        cx.open_window(WindowOptions::default(), |window, cx| {
            let workspace = cx.new(|cx| {
                ClipboardWorkspace::new(window, cx, app_state.clone(), action_sender.clone())
            });
            workspace.update(cx, |this, cx| {
                this.search_input.focus_handle(cx).focus(window, cx);
            });

            let service_clone = clipboard_service.clone();

            cx.spawn(async move |_cx| {
                while let Ok(action) = action_receiver.recv_async().await {
                    let service = service_clone.clone();
                    match action {
                        UiAction::TogglePin(id, is_pinned) => {
                            if let Err(e) = service.toggle_pin(id, is_pinned) {
                                tracing::error!("Failed to pin entry: {}", e);
                            }
                        }
                        UiAction::Delete(id) => {
                            if let Err(e) = service.delete_entry(id) {
                                tracing::error!("Failed to delete entry: {}", e);
                            }
                        }
                        UiAction::Search(query) => {
                            if let Err(e) = service.search(query) {
                                tracing::error!("Failed to execute search: {}", e);
                            }
                        }
                    }
                }
            })
            .detach();
            cx.spawn(async move |cx| {
                while let Ok(event) = event_receiver.recv_async().await {
                    match event {
                        AppEvent::HistoryUpdated(fresh_items) => {
                            app_state.update(cx, |state, cx| {
                                state.set_items(fresh_items, cx);
                            });
                        }
                    }
                }
            })
            .detach();

            cx.new(|cx| Root::new(workspace, window, cx))
        })
        .expect("Failed to open window");
    });
}
