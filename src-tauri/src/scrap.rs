use crate::helper::{
    get_inner_text_from_element, log_to_front, try_parse_string_to_start_end_dt, LogLevel,
};
use crate::models::{CourseInfo, EntryType, ScrapOptions, ScrapResult, TimeTableEntry};
use chromiumoxide::error::CdpError;
use chromiumoxide::{Browser, BrowserConfig};
use chrono::Local;
use futures::StreamExt;
use scraper::{Html, Selector};
use std::fmt;
use std::time::Duration;
use tauri::{AppHandle, Url};
use tokio::time::sleep;

const CALENDER_LINK: &str = "https://in4sit.singaporetech.edu.sg/psc/CSSISSTD_4/EMPLOYEE/SA/c/SA_LEARNER_SERVICES.SSR_SSENRL_LIST.GBL?Page=SSR_SSENRL_LIST&Action=A";

const LANDING_PAGE: &str = "https://in4sit.singaporetech.edu.sg/psc/CSSISSTD/EMPLOYEE/SA/c/NUI_FRAMEWORK.PT_LANDINGPAGE.GBL";

#[derive(Debug, Clone)]
pub enum ScrapError {
    BrowserError(String),
    NavigationError(String),
    JSException(String),
    LoginFailed,
    HtmlParseError(String),
    NetworkError,
}

impl fmt::Display for ScrapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let now = Local::now();
        let now_fmt = now.format("%d/%m/%y %H:%M:%S").to_string();
        match self {
            ScrapError::BrowserError(msg) => write!(f, "[{}] Browser Error: {}", now_fmt, msg),
            ScrapError::NavigationError(msg) => {
                write!(f, "[{}] Navigation Error: {}", now_fmt, msg)
            }
            ScrapError::JSException(msg) => {
                write!(f, "[{}] JavaScript Exception: {}", now_fmt, msg)
            }
            ScrapError::NetworkError => write!(f, "[{}] Network Error", now_fmt),
            ScrapError::HtmlParseError(msg) => {
                write!(f, "[{}] HTML Parsing Error: {}", now_fmt, msg)
            }
            ScrapError::LoginFailed => write!(f, "[{}] Login Failed", now_fmt),
        }
    }
}

impl From<CdpError> for ScrapError {
    fn from(value: CdpError) -> Self {
        match value {
            CdpError::Timeout => ScrapError::NetworkError,
            CdpError::FrameNotFound(e) => {
                ScrapError::NavigationError(format!("iFrame {:?} not found", e))
            }
            CdpError::ScrollingFailed(e) => ScrapError::NavigationError(e),
            CdpError::NotFound => ScrapError::NavigationError("Selector not found".to_string()),
            CdpError::JavascriptException(e) => ScrapError::JSException(e.to_string()),
            CdpError::Url(e) => ScrapError::NavigationError(e.to_string()),
            CdpError::Chrome(e) => match e.code {
                -32000 => ScrapError::NavigationError("Selector not found".to_string()),
                _ => ScrapError::BrowserError("Generic Browser error".to_string()),
            },
            _ => {
                println!("{:?}", value);
                ScrapError::BrowserError("Generic Browser error".to_string())
            }
        }
    }
}

pub async fn start_scrap(
    app: &AppHandle,
    username: &str,
    password: &str,
    options: ScrapOptions,
) -> Result<String, ScrapError> {
    let config = if options.debug_mode {
        BrowserConfig::builder()
            .arg("--lang=en-US")
            .with_head()
            .build()
    } else {
        BrowserConfig::builder().arg("--lang=en-US").build()
    }
    .map_err(|e| {
        log_to_front(&e, LogLevel::Error, app, true);
        ScrapError::BrowserError(e)
    })?;
    let (mut browser, mut handler) = Browser::launch(config).await?;
    let _ = tokio::spawn(async move {
        loop {
            let _event = handler.next().await.unwrap();
        }
    });
    let page = browser.new_page(LANDING_PAGE).await?;
    let page = page.wait_for_navigation().await?;
    sleep(Duration::from_secs(1)).await;
    if page.url().await? == Some(LANDING_PAGE.to_string()) {
        log_to_front(
            "Website loaded, already logged in",
            LogLevel::Info,
            app,
            true,
        );
    } else {
        log_to_front("Website loaded, logging in", LogLevel::Info, app, true);
        log_to_front("Entering email address", LogLevel::Info, app, true);
        page.find_xpath("//*[@id=\"userNameInput\"]")
            .await?
            .click()
            .await?
            .type_str(username)
            .await?;
        log_to_front("Entering password", LogLevel::Info, app, true);
        page.find_xpath("//*[@id=\"passwordInput\"]")
            .await?
            .click()
            .await?
            .type_str(password)
            .await?;
        log_to_front("Submitting sign in form", LogLevel::Info, app, true);
        page.find_xpath("//*[@id=\"submitButton\"]")
            .await?
            .click()
            .await?;
    }
    page.wait_for_navigation().await?;
    sleep(Duration::from_secs(2)).await;
    let current_url = page
        .url()
        .await?
        .ok_or(ScrapError::NavigationError("No URL strangely".to_string()))?;
    let current_url = Url::parse(&current_url)
        .map_err(|_e| ScrapError::NavigationError("URL malformed".to_string()))?;
    if current_url.host_str() != Some("in4sit.singaporetech.edu.sg") {
        log_to_front(
            "Login failed, Check credentials",
            LogLevel::Error,
            app,
            true,
        );
        Err(ScrapError::LoginFailed)
    } else {
        log_to_front("User logged in", LogLevel::Info, app, true);
        let mut retry_count = 0;
        while page.url().await? != Some(CALENDER_LINK.to_string()) && retry_count < 3 {
            page.goto(CALENDER_LINK).await?;
            page.wait_for_navigation().await?;
            sleep(Duration::from_secs(1)).await;
            retry_count += 1;
            if retry_count >= 2 {
                log_to_front(
                    "Attempting to load Calender view again",
                    LogLevel::Warn,
                    app,
                    true,
                );
            }
        }

        log_to_front("Loaded Calender view", LogLevel::Info, app, true);

        if options.filter_dropped {
            page.find_xpath("//*[@id=\"DERIVED_REGFRM1_SA_STUDYLIST_D\"]")
                .await?
                .click()
                .await?;
            log_to_front(
                "Unchecked 'Show Dropped Classes'",
                LogLevel::Info,
                app,
                true,
            );
        }

        if options.filter_waitlisted {
            page.find_xpath("//*[@id=\"DERIVED_REGFRM1_SA_STUDYLIST_W\"]")
                .await?
                .click()
                .await?;
            log_to_front(
                "Unchecked 'Show Waitlisted Classes'",
                LogLevel::Info,
                app,
                true,
            );
        }

        page.find_xpath("//*[@id=\"DERIVED_REGFRM1_SA_STUDYLIST_SHOW$14$\"]")
            .await?
            .click()
            .await?;
        page.wait_for_navigation().await?;
        sleep(Duration::from_secs(2)).await;
        log_to_front("Filtering done", LogLevel::Info, app, true);
        let html = page.content().await?;

        if options.debug_mode {
            log_to_front("Waiting for browser exit", LogLevel::Debug, app, true);
            log_to_front("Close the browser to continue", LogLevel::Debug, app, true);
            let _ = browser.wait().await;
        }
        Ok(html)
    }
}

pub fn extract_timetable_from_html(
    html: String,
    app: &AppHandle,
) -> Result<ScrapResult, ScrapError> {
    log_to_front("Started processing HTML", LogLevel::Info, app, true);
    let mut results = ScrapResult {
        skipped_unknown_course_count: 0,
        skipped_table_entry_count: 0,
        errors_present: false,
    };
    let doc = Html::parse_document(&html);
    let table_selector = Selector::parse("#ACE_STDNT_ENRL_SSV2\\$0 > tbody").unwrap();
    let main_table = doc
        .select(&table_selector)
        .collect::<Vec<_>>()
        .first()
        .map_or(
            Err(ScrapError::HtmlParseError(
                "Table selector not found".to_string(),
            )),
            |v| Ok(v.to_owned()),
        )?;
    let course_tables = main_table
        .child_elements()
        .filter_map(|e| {
            if !get_inner_text_from_element(&e).is_empty() {
                Some(e)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    log_to_front(
        &format!("Have {} courses", course_tables.len()),
        LogLevel::Info,
        app,
        true,
    );

    let course_info = course_tables.iter()
        .filter_map(|e| {
            let frag = Html::parse_fragment(&e.html());
            let course_name_selector = Selector::parse("td.PAGROUPDIVIDER").unwrap();
            let course_name = frag
                .select(&course_name_selector)
                .map(|e| {
                    get_inner_text_from_element(&e)
                })
                .collect::<Vec<String>>()
                .first()
                .map(|name| name.clone())
                .or_else(|| {
                    let e = ScrapError::HtmlParseError("Course name selector, not found".to_string());
                    log_to_front(&e.to_string(), LogLevel::Error, app, false);
                    log_to_front("Course skipped, verify generated timetable.", LogLevel::Error, app, true);
                    results.skipped_unknown_course_count += 1;
                    None
                })?;
            log_to_front(&format!("Parsing {} timetable", course_name), LogLevel::Info, app, true);
            let table_entry_selector = Selector::parse("table.PSLEVEL3GRIDWBO table.PSLEVEL3GRID > tbody").unwrap();
            let course_timetable_node = frag.select(&table_entry_selector)
                .collect::<Vec<_>>()
                .get(1)
                .map(|e| e.clone())
                .or_else(|| {
                    let e = ScrapError::HtmlParseError("Timetable selector, not found".to_string());
                    log_to_front(&e.to_string(), LogLevel::Error, app, false);
                    log_to_front(&format!("{} skipped, verify generated timetable.", course_name), LogLevel::Error, app, true);
                    results.skipped_unknown_course_count += 1;
                    results.errors_present = true;
                    None
                })?;
            let mut current_entry_type = EntryType::Lecture;
            let mut current_section = "ALL".to_string();
            let course_timetable_entries = course_timetable_node
                .child_elements()
                .enumerate()
                .filter_map(|(i, e)| {
                    if i != 0 {
                        let inner_row = e.child_elements().collect::<Vec<_>>();
                        let mut location: String = String::new();
                        let mut instructor: String = String::new();
                        let mut datetime_string = String::new();

                        for (i, cell) in inner_row.iter().enumerate() {
                            if i == 2 {
                                // parse an Entry type
                                let text = get_inner_text_from_element(cell);
                                if !text.is_empty() {
                                    current_entry_type = match text.as_str() {
                                        "Quiz" => EntryType::Quiz,
                                        "Tutorial" => EntryType::Tutorial,
                                        "Laboratory" => EntryType::Lab,
                                        "Lecture" => EntryType::Lecture,
                                        "Workshop" => EntryType::Workshop,
                                        _ => {
                                            let msg = format!("Encountered unknown entry type when parsing table: {}, no matches found", text);
                                            log_to_front(&msg, LogLevel::Warn, app, true);
                                            log_to_front("Default to 'unknown', continuing...", LogLevel::Warn, app, true);
                                            results.errors_present = true;
                                            EntryType::Unknown
                                        }
                                    };
                                }
                            } else if i == 1 {
                                // Parse Class section
                                let text = get_inner_text_from_element(cell);
                                if !text.is_empty() {
                                    current_section = text;
                                }
                            } else if i == 3 {
                                // DT String, first half
                                let text = get_inner_text_from_element(cell);
                                datetime_string.push_str(&text);
                            } else if i == 4 {
                                location = get_inner_text_from_element(cell);
                            } else if i == 5 {
                                let text = get_inner_text_from_element(cell);
                                let cleaned = text.replace(".", "");
                                let cleaned = cleaned.trim();
                                instructor = cleaned.to_string();
                            } else if i == 6 {
                                // Datetime string second half
                                let text = get_inner_text_from_element(cell);
                                let dt_str_sec = text.split("-").collect::<Vec<_>>();
                                let dt_str_sec = dt_str_sec.first().unwrap();
                                let dt_str_sec = dt_str_sec.to_string();
                                datetime_string.push(' ');
                                datetime_string.push_str(&dt_str_sec);
                            }
                        }
                        if datetime_string.contains("TBA") {
                            log_to_front(&format!("Table entry skipped for {}, meeting info not available", course_name), LogLevel::Warn, app, true);
                            results.skipped_table_entry_count += 1;
                            None
                        } else {
                            let (start, end) = try_parse_string_to_start_end_dt(&datetime_string).map_err(|e| {
                                let e = ScrapError::HtmlParseError(e);
                                results.errors_present = true;
                                results.skipped_table_entry_count += 1;
                                log_to_front(&e.to_string(), LogLevel::Error, app, false);
                                log_to_front(&format!("Table entry skipped for {}, CHECK results", course_name), LogLevel::Error, app, true);
                            }).ok()?;
                            Some(TimeTableEntry {
                                entry_type: current_entry_type.clone(),
                                class_section: current_section.clone(),
                                location,
                                instructor,
                                start_datetime: start,
                                end_datetime: end,
                            })
                        }

                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            Some(CourseInfo {
                course_name,
                table_entries: course_timetable_entries,
            })
        })
        .collect::<Vec<_>>();

    Ok(results)
}
