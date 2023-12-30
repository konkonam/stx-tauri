use tauri::{
    Runtime,
    SystemTray,
    Wry,
    SystemTrayEvent,
    App,
    AppHandle,
};
use serde_json::json;

mod internals;
mod utils;
mod command;
mod commands;
mod models;

use command::{
    run_buddy,
    run_ssh,
    get_projects,
};

/// Provides pre-defined settings for a stacks tauri app
/// Maybe later we will create a custom builder
pub fn create_builder() -> tauri::Builder<Wry> {
    tauri::Builder::default()
        .system_tray(SystemTray::new())
        .on_system_tray_event(internals::tray::get_event_handler)
        .on_window_event(internals::window::get_event_handler)
        .invoke_handler(tauri::generate_handler![
            run_buddy,
            run_ssh,
            get_projects,
        ])
}
