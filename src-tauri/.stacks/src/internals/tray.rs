use tauri::{SystemTrayEvent, AppHandle};
use crate::utils;

pub fn get_event_handler(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::RightClick { position, .. } => {

            utils::toggle_main_window(app, position)
        },
        SystemTrayEvent::LeftClick { position, .. } => utils::toggle_main_window(app, position),
        _ => {}
    }
}
