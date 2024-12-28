import {defineConfig} from "vite";
import solid from "vite-plugin-solid";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
    plugins: [solid()],
    clearScreen: false,
    server: {
        host: host || false,
        port: 1420,
        strictPort: true,
        hmr: host
            ? {
                protocol: 'ws',
                host: host,
                port: 1430,
            }
            : undefined,
        watch: {
            ignored: ["**/src-tauri/**"],
        },
    },
}));
