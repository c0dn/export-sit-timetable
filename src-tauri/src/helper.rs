use crate::models::LogEvent;
use chrono::{DateTime, Local, NaiveDateTime};
use scraper::ElementRef;
use tauri::{AppHandle, Emitter};

pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub fn log_to_front(msg: &str, level: LogLevel, app: &AppHandle, with_ts: bool) {
    let msg = if with_ts {
        let now = Local::now();
        let now_fmt = now.format("%d/%m %H:%M:%S").to_string();
        format!("[{now_fmt}] {msg}")
    } else {
        msg.to_string()
    };
    let ev = match level {
        LogLevel::Debug => LogEvent {
            message: msg,
            level: 0,
        },
        LogLevel::Info => LogEvent {
            message: msg,
            level: 1,
        },
        LogLevel::Warn => LogEvent {
            message: msg,
            level: 2,
        },
        LogLevel::Error => LogEvent {
            message: msg,
            level: 3,
        },
    };
    app.emit("logs", ev).unwrap()
}

pub fn get_inner_text_from_element(ele: &ElementRef) -> String {
    ele.text().collect::<Vec<_>>().join("").trim().to_string()
}

pub fn try_parse_string_to_start_end_dt(
    input: &str,
) -> Result<(DateTime<Local>, DateTime<Local>), String> {
    let parts: Vec<&str> = input.split_ascii_whitespace().collect();
    if parts.len() == 5 {
        let date_part = parts[4];
        let start_time = parts[1];
        let end_time = parts[3];
        let start_datetime_str = format!("{} {}", date_part, start_time);
        let end_datetime_str = format!("{} {}", date_part, end_time);
        let dt_fmt = "%d/%m/%Y %I:%M%p";
        let start_dt = NaiveDateTime::parse_from_str(&start_datetime_str, dt_fmt).map_err(|e| {
            format!(
                "Fail to parse start dt, error: {}, value: {}, fmt: {}",
                e.to_string(),
                start_datetime_str,
                dt_fmt
            )
        })?;
        let end_dt = NaiveDateTime::parse_from_str(&end_datetime_str, dt_fmt).map_err(|e| {
            format!(
                "Fail to parse end dt, error: {}, value: {}, fmt: {}",
                e.to_string(),
                end_datetime_str,
                dt_fmt
            )
        })?;
        let start_dt = start_dt.and_local_timezone(Local).unwrap();
        let end_dt = end_dt.and_local_timezone(Local).unwrap();
        Ok((start_dt, end_dt))
    } else {
        Err(format!(
            "Fail to parse end dt, error: Unknown format, value: {}",
            input
        ))
    }
}
