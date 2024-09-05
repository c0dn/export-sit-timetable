import {createSignal, onCleanup, onMount} from "solid-js";
import "./App.css";
import {invoke} from "@tauri-apps/api/core";
import {listen, UnlistenFn} from "@tauri-apps/api/event";

function Home() {
    const [username, setUsername] = createSignal("");
    const [password, setPassword] = createSignal("");
    const [logs, setLogs] = createSignal<LogEvent[]>([]);

    let cleanupFunc: UnlistenFn | null = null;

    onMount(() => {
        listen<LogEvent>('logs', (event) => {
            setLogs(prev => {
                return [...prev, event.payload];
            })
        }).then(r => {
            cleanupFunc  = r
        });
    })

    onCleanup(() => {
        if (cleanupFunc) {
            cleanupFunc()
        }
    });

    const handleCredentials = async () => {
        try {
            let options: ScrapOptions = {
                debug_mode: false, filter_dropped: true, filter_waitlisted: true

            }
            await invoke("handle_credentials", { username: username(), password: password(), options: options });
            console.log(logs())
        } catch (error) {
            console.error(error);
            console.log(logs())
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
                                <input id="email" name="email" type="email" autocomplete="email" required
                                       onChange={(e) => setUsername(e.currentTarget.value)}
                                       value={username()}
                                       class="block w-full rounded-md border-0 bg-white/5 py-1.5 text-white shadow-sm ring-1 ring-inset ring-white/10 focus:ring-2 focus:ring-inset focus:ring-indigo-500 sm:text-sm sm:leading-6"/>
                            </div>
                        </div>

                        <div>
                            <div class="flex items-center justify-between">
                                <label for="password"
                                       class="block text-sm font-medium leading-6 text-white">Password</label>
                            </div>
                            <div class="mt-2">
                                <input id="password" name="password" type="password" autocomplete="current-password"
                                       onChange={(e) => setPassword(e.currentTarget.value)}
                                       value={password()}
                                       required
                                       class="block w-full rounded-md border-0 bg-white/5 py-1.5 text-white shadow-sm ring-1 ring-inset ring-white/10 focus:ring-2 focus:ring-inset focus:ring-indigo-500 sm:text-sm sm:leading-6"/>
                            </div>
                        </div>

                        <div>
                            <button type="submit"
                                    class="flex w-full justify-center rounded-md bg-indigo-500 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-400 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500">
                                Submit
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    );
}

export default Home;
