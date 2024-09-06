/// <reference types="vite/client" />
interface LogEvent {
    message: string;
    level: number;
}

interface ScrapOptions {
    filter_dropped: boolean;
    filter_waitlisted: boolean;
    debug_mode: boolean;
}

interface ScrapResult {
    skipped_unknown_course_count: number;
    skipped_table_entry_count: number;
    errors_present: boolean;
}