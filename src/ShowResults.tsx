import {useLocation} from "@solidjs/router";
import {JSX} from "solid-js";

export default function ShowResults() {

    interface StateValues {
        logs: LogEvent[];
        result: ScrapResult | null;
    }

    const location = useLocation<StateValues>();
    const {logs, result} = location.state || {};

    const decodeLogLevel = (level: number): JSX.Element | null => {
        if (level == 1) {
            return <p class="text-blue-400 mr-1">INFO</p>;
        } else if (level == 2) {
            return <p class="text-amber-400 mr-1">WARN</p>;
        } else if (level == 3) {
            return <p class="text-red-400 mr-1">ERROR</p>;
        } else if (level == 0) {
            return <p class="text-green-400 mr-1">DEBUG</p>;
        } else {
            return null;
        }
    }

    const determineColor = (count: number): string => {
        if (count > 0 && count <= 2) {
            return "text-amber-400";
        } else if (count > 2) {
            return "text-red-500";
        } else {
            return "";
        }
    }


    return (
        <div>
            <div class="flex min-h-full flex-col justify-center px-6 py-5">
                <h2 class="my-5 text-center text-2xl font-bold leading-9 tracking-tight text-white">Export Result</h2>
                <div class="flex flex-row-reverse">
                    <div class="w-2/3">
                        <div class="flex flex-row justify-between mb-3">
                            <h1 class="text-white text-2xl font-bold">Log</h1>
                        </div>
                        <div class="overflow-x-auto">
                            <table class="text-sm text-left text-gray-400">
                                <tbody>
                                {logs?.map((log: LogEvent) => (
                                    <tr class="border-gray-700 flex">
                                        {decodeLogLevel(log.level)}
                                        {log.message}
                                    </tr>
                                ))}
                                </tbody>
                            </table>
                        </div>
                    </div>
                    <div class="w-1/3 p-4">
                        <dl class="divide-y divide-white/10">
                            <div class="px-2 py-4 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-0">
                                <dt class="text-sm font-medium leading-6 text-white">Skipped Unknown Course</dt>
                                <dd class="mt-1 text-sm leading-6 text-gray-400 sm:col-span-2 sm:mt-0">
                                    {result ? (
                                        <p class={result.skipped_unknown_course_count > 0 ? "text-red-400" : ""}>
                                            {result.skipped_unknown_course_count}
                                        </p>
                                    ) : (
                                        <p>{Number.NaN}</p>
                                    )}
                                </dd>
                            </div>
                            <div class="px-2 py-4 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-0">
                                <dt class="text-sm font-medium leading-6 text-white">Skipped Timetable Entries</dt>
                                <dd class="mt-1 text-sm leading-6 text-gray-400 sm:col-span-2 sm:mt-0">
                                    {result ? (
                                        <p class={determineColor(result.skipped_table_entry_count)}>
                                            {result.skipped_table_entry_count}
                                        </p>
                                    ) : (
                                        <p>{Number.NaN}</p>
                                    )}
                                </dd>
                            </div>
                            <div class="px-2 py-4 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-0">
                                <dt class="text-sm font-medium leading-6 text-white">Errors Present</dt>
                                <dd class="mt-1 text-sm leading-6 text-gray-400 sm:col-span-2 sm:mt-0">
                                    {result ? (
                                        result.errors_present ? (
                                            <p class="text-red-400">Yes</p>
                                        ) : (
                                            <p class="text-green-500">No</p>
                                        )
                                    ) : (
                                        <p class="text-red-400">Yes</p>
                                    )}
                                </dd>
                            </div>
                        </dl>
                        <button type="button"
                                class="inline-flex items-center gap-x-2 rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">
                            Export ICS
                            <svg class="-mr-0.5 h-5 w-5" fill="white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg"
                                 viewBox="0 0 576 512">
                                {/*Font Awesome Free 6.6.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc*/}
                                <path
                                    d="M0 64C0 28.7 28.7 0 64 0L224 0l0 128c0 17.7 14.3 32 32 32l128 0 0 128-168 0c-13.3 0-24 10.7-24 24s10.7 24 24 24l168 0 0 112c0 35.3-28.7 64-64 64L64 512c-35.3 0-64-28.7-64-64L0 64zM384 336l0-48 110.1 0-39-39c-9.4-9.4-9.4-24.6 0-33.9s24.6-9.4 33.9 0l80 80c9.4 9.4 9.4 24.6 0 33.9l-80 80c-9.4 9.4-24.6 9.4-33.9 0s-9.4-24.6 0-33.9l39-39L384 336zm0-208l-128 0L256 0 384 128z"/>
                            </svg>
                        </button>
                    </div>
                </div>

            </div>
        </div>
    )
}