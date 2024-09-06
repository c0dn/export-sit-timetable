use crate::helper::{log_to_front, LogLevel};
use crate::models::{ScrapOptions, ScrapResult};
use crate::scrap::{extract_timetable_from_html, start_scrap};
use std::time::Duration;
use tauri::AppHandle;
use tokio::time::sleep;

#[tauri::command]
pub async fn handle_credentials(
    app: AppHandle,
    username: String,
    password: String,
    options: ScrapOptions,
) -> Result<ScrapResult, String> {
    let html = start_scrap(&app, &username, &password, options)
        .await
        .map_err(|e| {
            log_to_front(&e.to_string(), LogLevel::Error, &app, false);
            e.to_string()
        })?;
    let r = extract_timetable_from_html(html, &app).map_err(|e| {
        log_to_front(&e.to_string(), LogLevel::Error, &app, false);
        e.to_string()
    })?;
    log_to_front("Done!", LogLevel::Info, &app, true);
    Ok(r)
}
