use chrono::{DateTime, Local};
use icalendar::{Component, Event, EventLike};
use serde::{Deserialize, Serialize};

// level 0 = debug
// level 1 = info
// level 2 = warn
// level 3 = error
#[derive(Serialize, Debug, Clone)]
pub struct LogEvent {
    pub message: String,
    pub level: u8,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ScrapOptions {
    pub filter_dropped: bool,
    pub filter_waitlisted: bool,
    pub debug_mode: bool,
}

#[derive(Serialize, Debug, Clone)]
pub enum EntryType {
    Quiz,
    Tutorial,
    Lab,
    Lecture,
    Workshop,
    Unknown,
}

#[derive(Serialize, Debug, Clone)]
pub struct TimeTableEntry {
    pub entry_type: EntryType,
    pub class_section: String,
    pub location: String,
    pub instructors: Vec<String>,
    pub start_datetime: DateTime<Local>,
    pub end_datetime: DateTime<Local>,
}

#[derive(Serialize, Debug, Clone)]
pub struct CourseInfo {
    pub course_name: String,
    pub table_entries: Vec<TimeTableEntry>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ScrapResult {
    pub skipped_unknown_course_count: u8,
    pub skipped_table_entry_count: u8,
    pub errors_present: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GithubLatestReleaseRes {
    pub tag_name: String,
}

impl Into<Vec<Event>> for CourseInfo {
    fn into(self) -> Vec<Event> {
        let name = self.course_name.clone();
        self.table_entries
            .into_iter()
            .map(|e| {
                let summary = format!("{} - {} - {:?}", name, e.class_section, e.entry_type);
                let description_header = "Profs\n";
                let description = format!("{}{}", description_header, e.instructors.join("\n"));
                Event::new()
                    .summary(&summary)
                    .starts(e.start_datetime.to_utc())
                    .ends(e.end_datetime.to_utc())
                    .location(e.location.as_str())
                    .description(&description)
                    .done()
            })
            .collect()
    }
}
