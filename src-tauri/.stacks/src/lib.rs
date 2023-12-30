use tauri::{
    SystemTray,
    Wry,
};

mod internals;
mod utils;

/// Provides pre-defined settings for a stacks tauri app
/// Maybe later we will create a custom builder
pub fn create_builder() -> tauri::Builder<Wry> {
    tauri::Builder::default()
        .system_tray(SystemTray::new())
        .on_system_tray_event(internals::tray::get_event_handler)
        .on_window_event(internals::window::get_event_handler)
}
