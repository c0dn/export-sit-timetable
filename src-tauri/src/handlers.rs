use crate::helper::{log_to_front, LogLevel};
use crate::models::{ScrapOptions, ScrapResult};
use crate::scrap::{extract_timetable_from_html, start_scrap};
use crate::AppState;
use icalendar::{Calendar, Event};
use tauri::{AppHandle, State};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use tauri_plugin_shell::ShellExt;
use tokio::sync::Mutex;
use crate::updater::get_latest_release;

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
    tokio::fs::write(path, ics_data)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}


#[tauri::command]
pub async fn get_installed_version(state: State<'_, Mutex<AppState>>,) -> Result<String, ()> {
    let state_v = state.lock().await;
    let version = state_v.version_string.clone();
    Ok(version)
}

#[tauri::command]
pub async fn is_update_available(app: AppHandle,state: State<'_, Mutex<AppState>>,) -> Result<(), String> {
    let state_v = state.lock().await;
    let version = state_v.version_string.clone();
    let latest_version = get_latest_release().await?;
    if version != latest_version {
        app.dialog()
            .message(format!("An update is available, latest: {}", latest_version))
            .kind(MessageDialogKind::Info)
            .title("New Version")
            .ok_button_label("Download")
            .show(move |result| if result {
                let shell = app.shell();
                let _ = shell.open("https://github.com/c0dn/export-sit-timetable/releases", None);
            });
    }
    Ok(())
}