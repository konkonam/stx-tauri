use tauri::{AppHandle, LogicalSize, Manager, PhysicalPosition, Window};

fn center_by_position(window: &Window, position: PhysicalPosition<f64>) {
    let scale_factor = window
        .current_monitor()
        .map(|m| m.expect("REASON").scale_factor())
        .unwrap_or(1.);

    let LogicalSize { width, .. }: LogicalSize<f64> = window.inner_size().expect("REASON").to_logical(scale_factor);

    let PhysicalPosition { x, y } = position;

    window.set_position(PhysicalPosition {
        x: x - width / 2.0,
        y: y,
    });
}

pub fn toggle_main_window(app: &AppHandle, position: PhysicalPosition<f64>) {
    let window_opt = app.get_window("main");
    if let Some(main_window) = window_opt {

        if main_window.is_visible().unwrap_or(false) {
            main_window.hide();
        } else {
            center_by_position(&main_window, position);
            main_window.show();
            main_window.set_focus();
        }
    }
}
