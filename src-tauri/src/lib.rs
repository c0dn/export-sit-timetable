use crate::handlers::{export_to_ics, handle_credentials};
use crate::models::CourseInfo;
use tauri::Manager;
use tokio::sync::Mutex;

mod handlers;
mod helper;
mod models;
mod scrap;

pub struct AppState {
    pub scrapped_info: Vec<CourseInfo>,
}

impl AppState {
    fn new() -> Self {
        AppState {
            scrapped_info: vec![],
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
        .invoke_handler(tauri::generate_handler![handle_credentials, export_to_ics])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
