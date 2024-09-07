use crate::handlers::{export_to_ics, get_installed_version, handle_credentials, is_update_available};
use crate::models::CourseInfo;
use tauri::Manager;
use tokio::sync::Mutex;

mod handlers;
mod helper;
mod models;
mod scrap;
mod updater;

pub struct AppState {
    pub scrapped_info: Vec<CourseInfo>,
    pub version_string: String,
}

impl AppState {
    fn new() -> Self {
        AppState {
            scrapped_info: vec![],
            version_string: "1.0.0".to_string(),
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::new();
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(app_state));
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![handle_credentials, export_to_ics, get_installed_version, is_update_available])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
