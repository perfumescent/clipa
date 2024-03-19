import {defineConfig} from "vite";
import vue from "@vitejs/plugin-vue";
import { vitePluginForArco } from '@arco-plugins/vite-vue'

// https://vitejs.dev/config/
export default defineConfig(async () => ({
    plugins: [vue(),
        vitePluginForArco({
            style: 'css'
        })],

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
    },
    css: {
        preprocessorOptions: {
            less: {
                modifyVars: {
                    // 'color-bg-1': 'rgba(255, 255, 255, 0.5)', // 修改为半透明背景
                    // 'color-bg-2': 'rgba(255, 255, 255, 0.5)', // 修改为半透明背景
                    // 'color-bg-3': 'rgba(255, 255, 255, 0.5)', // 修改为半透明背景
                    // 'color-bg-4': 'rgba(255, 255, 255, 0.5)', // 修改为半透明背景
                    // 'color-bg-5': 'rgba(255, 255, 255, 0.5)', // 修改为半透明背景
                    // 'color-bg-white': 'rgba(255, 255, 255, 0.5)', // 修改为半透明背景
                    // 'color-fill-1': 'rgba(255, 255, 255, 0.5)', // 修改为半透明背景
                    // 'color-fill-2': 'rgba(255, 255, 255, 0.5)', // 修改为半透明背景
                    'table-color-bg-body-row_hover': 'rgba(241,240,240, 1)',
                    'table-color-bg-body-row_active': 'rgba(241,240,240, 1)',
                    // 根据需要继续添加其他变量修改
                },
                javascriptEnabled: true,
            }
        }
    },
}));
