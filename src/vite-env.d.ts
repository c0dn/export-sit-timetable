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

