import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { setup } from '@css-render/vue3-ssr'

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [vue()],

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
  ssr: {
    noExternal: ['naive-ui', 'vueuc', 'date-fns']
  },
  ssgOptions: {
    async onBeforePageRender(_, __, appCtx) {
      const { collect } = setup(appCtx.app)
        ; (appCtx as any).__collectStyle = collect
      return undefined
    },
    async onPageRendered(_, renderedHTML, appCtx) {
      return renderedHTML.replace(
        /<\/head>/,
        `${(appCtx as any).__collectStyle()}</head>`
      )
    }
  }
}));
