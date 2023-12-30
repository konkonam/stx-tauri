pub fn toggle_visibility(window: tauri::Window) {
    if window.is_visible().unwrap() {
        let _ = window.hide();
    } else {
        let _ = window.show();
        let _ = window.set_focus();
    }
}