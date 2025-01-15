import {defineConfig} from "vite";
import vue from "@vitejs/plugin-vue";
import vuetify from "vite-plugin-vuetify";

// https://vitejs.dev/config/
export default defineConfig(async () => ({
    plugins: [
        vue(),
        vuetify(),
    ],

    // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
    //
    // 1. prevent vite from obscuring rust errors
    clearScreen: false,
    // 2. tauri expects a fixed port, fail if that port is not available
    server: {
        port: 1420,
        strictPort: true,
        watch: {
            // 3. tell vite to ignore watching `src-tauri`
            ignored: ["**/src-tauri/**"],
        },
        proxy: { // 配置反向代理开发环境
            '/api': {
                target: 'http://127.0.0.1:3030',
                changeOrigin: true, // 跨域
                // rewrite: path => path.replace('/api', ''),
            }
        },
    },
}));
