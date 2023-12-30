fn main() {
    stacks::create_builder()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}