use icalendar::{Calendar, Event};
use crate::helper::{log_to_front, LogLevel};
use crate::models::{ScrapOptions, ScrapResult};
use crate::scrap::{extract_timetable_from_html, start_scrap};
use crate::AppState;
use tauri::{AppHandle, State};
use tokio::sync::Mutex;

#[tauri::command]
pub async fn handle_credentials(
    app: AppHandle,
    state: State<'_, Mutex<AppState>>,
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
    let (r, courses_info) = extract_timetable_from_html(html, &app).map_err(|e| {
        log_to_front(&e.to_string(), LogLevel::Error, &app, false);
        e.to_string()
    })?;
    let mut state_v = state.lock().await;
    state_v.scrapped_info = courses_info;
    log_to_front("Done!", LogLevel::Info, &app, true);
    Ok(r)
}

#[tauri::command]
pub async fn export_to_ics(
    _app: AppHandle,
    state: State<'_, Mutex<AppState>>,
    path: String,
) -> Result<(), String> {
    let state_v = state.lock().await;
    let courses = state_v.scrapped_info.clone();
    let combined_events: Vec<Event> = courses
        .into_iter()
        .flat_map(|course_info| Into::<Vec<Event>>::into(course_info))
        .collect();
    let calender = Calendar::from_iter(combined_events);
    let ics_data = calender.to_string();
    tokio::fs::write(path, ics_data).await.map_err(|e| e.to_string())?;
    Ok(())
}
