use tauri::{Window, WindowEvent, GlobalWindowEvent};

pub fn get_event_handler(event: GlobalWindowEvent) {
    match event.event() {
        WindowEvent::Focused(focused) => {
            if !focused {
                event.window().hide().unwrap();
            }
        }
        _ => {}
    }
}

fn on_window_focus(event: GlobalWindowEvent, focused: bool) {
    if !focused {
        event.window().hide().unwrap();
    }
}