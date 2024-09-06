import {createMemo, createSignal, onCleanup, onMount, Show} from "solid-js";
import "./App.css";
import {invoke} from "@tauri-apps/api/core";
import {listen, UnlistenFn} from "@tauri-apps/api/event";
import {useNavigate} from "@solidjs/router";

function Home() {
    const [username, setUsername] = createSignal("");
    const [password, setPassword] = createSignal("");
    const [isLoading, setIsLoading] = createSignal(false);
    const [logs, setLogs] = createSignal<LogEvent[]>([]);
    const [debugMode, setDebugMode] = createSignal(false);
    const [filterDropped, setFilterDropped] = createSignal(true);
    const [filterWaitlisted, setFilterWaitlisted] = createSignal(true);

    const navigate = useNavigate();

    let cleanupFunc: UnlistenFn | null = null;

    onMount(() => {
        listen<LogEvent>('logs', (event) => {
            setLogs(prev => {
                return [...prev, event.payload];
            })
        }).then(r => {
            cleanupFunc = r
        });
    })

    onCleanup(() => {
        if (cleanupFunc) {
            cleanupFunc()
        }
    });

    const latestLogMessage = createMemo(() => {
        const logsArray = logs();
        return logsArray.length > 0 ? logsArray[logsArray.length - 1] : null;
    });

    const handleCredentials = async () => {
        try {
            setIsLoading(true)
            let options: ScrapOptions = {
                debug_mode: debugMode(), filter_dropped: filterDropped(), filter_waitlisted: filterWaitlisted()
            }
            let result = await invoke<ScrapResult>("handle_credentials", {
                username: username(),
                password: password(),
                options: options
            });
            setIsLoading(false)
            navigate("/results", {state: {logs: logs(), result: result}});
        } catch (error) {
            setIsLoading(false)
            navigate("/results", {state: {logs: logs(), result: null}});
        }
    }


    return (
        <div>
            <div class="flex min-h-full flex-col justify-center px-6 py-12 lg:px-8">
                <div class="sm:mx-auto sm:w-full sm:max-w-sm">
                    <h2 class="mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-white">Enter student
                        credentials</h2>
                </div>

                <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
                    <form class="space-y-6" onsubmit={(e) => {
                        e.preventDefault();
                        handleCredentials()
                    }}>
                        <div>
                            <label for="email" class="block text-sm font-medium leading-6 text-white">Email
                                address</label>
                            <div class="mt-2">
                                <input id="email" name="email" type="email" autocomplete="off" required
                                       onChange={(e) => setUsername(e.currentTarget.value)}
                                       value={username()}
                                       onfocus={(e) => e.currentTarget.removeAttribute('readonly')} readonly
                                       class="block w-full rounded-md border-0 bg-white/5 py-1.5 text-white shadow-sm ring-1 ring-inset ring-white/10 focus:ring-2 focus:ring-inset focus:ring-indigo-500 sm:text-sm sm:leading-6"/>
                            </div>
                        </div>

                        <div>
                            <div class="flex items-center justify-between">
                                <label for="password"
                                       class="block text-sm font-medium leading-6 text-white">Password</label>
                            </div>
                            <div class="mt-2">
                                <input id="password" name="password" type="password" autocomplete="off"
                                       onChange={(e) => setPassword(e.currentTarget.value)}
                                       onfocus={(e) => e.currentTarget.removeAttribute('readonly')} readonly
                                       value={password()}
                                       required
                                       class="block w-full rounded-md border-0 bg-white/5 py-1.5 text-white shadow-sm ring-1 ring-inset ring-white/10 focus:ring-2 focus:ring-inset focus:ring-indigo-500 sm:text-sm sm:leading-6"/>
                            </div>
                        </div>

                        {}

                        <fieldset>
                            <legend class="sr-only">Options</legend>
                            <div class="space-y-5">
                                <div class="relative flex items-start">
                                    <div class="flex h-6 items-center">
                                        <input id="filter-dropped" aria-describedby="filter-dropped-description"
                                               name="filter-dropped"
                                               type="checkbox"
                                               checked={filterDropped()}
                                               onChange={(e) => setFilterDropped(e.currentTarget.checked)}
                                               class="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-600"/>
                                    </div>
                                    <div class="ml-3 text-sm leading-6">
                                        <label for="filter-dropped" class="font-medium text-gray-500 mr-1">Filter dropped
                                            modules</label>
                                        <span id="filter-dropped-description" class="text-gray-200"><span
                                            class="sr-only">Filter dropped modules </span>Hide dropped modules</span>
                                    </div>
                                </div>
                                <div class="relative flex items-start">
                                    <div class="flex h-6 items-center">
                                        <input id="filter-waitlist" aria-describedby="candidates-description"
                                               name="candidates" type="checkbox"
                                               checked={filterWaitlisted()}
                                               onChange={(e) => setFilterWaitlisted(e.currentTarget.checked)}
                                               class="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-600"/>
                                    </div>
                                    <div class="ml-3 text-sm leading-6">
                                        <label for="filter-waitlist" class="font-medium text-gray-500 mr-1">Filter waitlisted
                                            modules</label>
                                        <span id="filter-waitlisted-description" class="text-gray-200"><span
                                            class="sr-only">Filter waitlisted modules </span>Hide waitlisted modules</span>
                                    </div>
                                </div>
                                <div class="relative flex items-start">
                                    <div class="flex h-6 items-center">
                                        <input id="debug" aria-describedby="debug-description" name="debug"
                                               type="checkbox"
                                               checked={debugMode()}
                                               onChange={(e) => setDebugMode(e.currentTarget.checked)}
                                               class="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-600"/>
                                    </div>
                                    <div class="ml-3 text-sm leading-6">
                                        <label for="debug" class="font-medium text-gray-500 mr-1">Debug mode</label>
                                        <span id="debug-description" class="text-gray-200">
                                            <span class="sr-only">Debug mode </span>Show browser when scrapping</span>
                                    </div>
                                </div>
                            </div>
                        </fieldset>

                        <div>
                            {latestLogMessage() && (
                                <p class="text-white text-sm">
                                    {latestLogMessage()?.level == 1 ? (
                                        <span class="text-white p-2 inline-flex items-center gap-x-1.5 rounded-md px-1.5 py-0.5 text-sm/5 font-medium sm:text-xs/5 forced-colors:outline bg-blue-500/15 group-data-[hover]:bg-blue-500/25 dark:text-blue-400 dark:group-data-[hover]:bg-blue-500/25">
                                            INFO
                                        </span>
                                    ):
                                    latestLogMessage()?.level == 2 ? (
                                        <span class="text-white p-2 inline-flex items-center gap-x-1.5 rounded-md px-1.5 py-0.5 text-sm/5 font-medium sm:text-xs/5 forced-colors:outline bg-amber-400/20 group-data-[hover]:bg-amber-400/30 dark:bg-amber-400/10 dark:text-amber-400 dark:group-data-[hover]:bg-amber-400/15">
                                            WARN
                                        </span>
                                    ):
                                    latestLogMessage()?.level == 3 ? (
                                        <span
                                            class="inline-flex p-2 text-white items-center gap-x-1.5 rounded-md px-1.5 py-0.5 text-sm/5 font-medium sm:text-xs/5 forced-colors:outline bg-red-500/15 group-data-[hover]:bg-red-500/25 dark:bg-red-500/10 dark:text-red-400 dark:group-data-[hover]:bg-red-500/20">
                                            ERROR
                                        </span>
                                    ): latestLogMessage()?.level == 0 ? (
                                        <span
                                            class="inline-flex p-2 text-white items-center gap-x-1.5 rounded-md px-1.5 py-0.5 text-sm/5 font-medium sm:text-xs/5 forced-colors:outline bg-emerald-500/15 group-data-[hover]:bg-emerald-500/25 dark:bg-emerald-500/10 dark:text-emerald-400 dark:group-data-[hover]:bg-emerald-500/20">
                                            DEBUG
                                        </span>
                                    ) : null}

                                    {latestLogMessage()?.message}
                                </p>
                            )}
                        </div>

                        <div>
                            <button type="submit"
                                    classList={{
                                        disabled: isLoading(),
                                    }}
                                    data-hs-overlay="#hs-slide-down-animation-modal"
                                    class="flex w-full justify-center rounded-md bg-indigo-500 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-400 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500">
                                <Show fallback={<span class="p-1">Submit</span>} when={isLoading()}>
                                    <div role="status">
                                        <svg aria-hidden="true"
                                             class="w-8 h-8 text-gray-200 animate-spin dark:text-gray-600 fill-white"
                                             viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
                                            <path
                                                d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
                                                fill="currentColor"/>
                                            <path
                                                d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
                                                fill="currentFill"/>
                                        </svg>
                                        <span class="sr-only">Loading...</span>
                                    </div>
                                </Show>
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    );
}

export default Home;
