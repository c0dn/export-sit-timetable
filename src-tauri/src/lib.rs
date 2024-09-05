use crate::handlers::handle_credentials;

mod handlers;
mod helper;
mod models;
mod scrap;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![handle_credentials])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
