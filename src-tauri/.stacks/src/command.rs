use tauri::Runtime;

#[tauri::command]
pub fn run_buddy<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<(), String> {
    println!("command called");
    return Ok(());
}

#[tauri::command]
pub fn run_ssh<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<(), String> {
    println!("command called");
    return Ok(());
}

#[tauri::command]
pub fn get_projects<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<(), String> {
    println!("command called");
    return Ok(());
}
