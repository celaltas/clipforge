mod app;
mod config;
mod logging;
mod storage;
mod ui;

use std::sync::{Arc, Mutex};

use config::loader::load_settings;
use gpui::{AppContext, WindowOptions};
use gpui_component::Root;
use storage::migration::run_migrations;
use storage::sqlite::initialize_database;

use crate::{
    app::{clipboard::start_clipboard_listener, state::AppState},
    ui::main_window::ClipboardList,
};

fn main() {
    let _ = logging::init();

    tracing::info!("application starting");

    let settings = load_settings().expect("Failed to load config");

    tracing::info!("config loaded");

    let mut connection = initialize_database().expect("Failed to initialize database");

    tracing::info!("database initialized");

    run_migrations(&mut connection).expect("Failed to run database migrations");

    let state = Arc::new(AppState {
        settings,
        db: Mutex::new(connection),
        clipboard_items: Mutex::new(Vec::new()),
    });

    let _clipboard_thread = start_clipboard_listener(state.clone());

    let app = gpui_platform::application().with_assets(gpui_component_assets::Assets);

    let ui_state = state.clone();

    app.run(move |cx| {
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| ClipboardList::new(ui_state.clone()));

                cx.new(|cx| Root::new(view, window, cx))
            })
            .expect("Failed to open window");
        })
        .detach();
    });
}
