import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";
import path from "path";

// https://vite.dev/config/
const isProto = process.env.VITE_PROTOTYPE === "true";

export default defineConfig({
  base: isProto ? "/prototype/" : "/",
  plugins: [react(), tailwindcss()],
  resolve: isProto
    ? {
        alias: {
          "@tauri-apps/api/core": path.resolve(__dirname, "mocks/tauri-core.ts"),
          "@tauri-apps/api/event": path.resolve(__dirname, "mocks/tauri-event.ts"),
          "@tauri-apps/api/window": path.resolve(__dirname, "mocks/tauri-window.ts"),
          "@tauri-apps/api/dpi": path.resolve(__dirname, "mocks/tauri-dpi.ts"),
          "@tauri-apps/plugin-shell": path.resolve(__dirname, "mocks/plugin-shell.ts"),
          "@tauri-apps/plugin-updater": path.resolve(__dirname, "mocks/plugin-updater.ts"),
          "@tauri-apps/plugin-process": path.resolve(__dirname, "mocks/plugin-process.ts"),
        },
      }
    : {},
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true,
    // Tauri 在 `cargo build` 期间会往 src-tauri/target/doc 下生成几十万个
    // rustdoc HTML 文件。如果 Vite 监听它们，会导致无限 HMR reload，前端
    // 永远来不及挂载，Tauri 窗口就会一直停留在默认的透明背景上。
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
});
