// vite.config.ts
import { defineConfig } from "file:///D:/code/rust/Automated%20testing%20tools/node_modules/vite/dist/node/index.js";
import vue from "file:///D:/code/rust/Automated%20testing%20tools/node_modules/@vitejs/plugin-vue/dist/index.mjs";
import vueJsx from "file:///D:/code/rust/Automated%20testing%20tools/node_modules/@vitejs/plugin-vue-jsx/dist/index.mjs";
import { fileURLToPath, URL } from "node:url";
var __vite_injected_original_import_meta_url = "file:///D:/code/rust/Automated%20testing%20tools/vite.config.ts";
var vite_config_default = defineConfig({
  plugins: [vue(), vueJsx()],
  resolve: {
    alias: {
      "~lib": fileURLToPath(new URL("./lib", __vite_injected_original_import_meta_url)),
      "@": fileURLToPath(new URL("./src", __vite_injected_original_import_meta_url))
    }
  },
  optimizeDeps: {
    include: []
  },
  esbuild: {
    drop: ["console", "debugger"]
  },
  // base: '/monaco-tree-editor/',
  build: {
    minify: "esbuild",
    outDir: "dist",
    target: "esnext",
    cssCodeSplit: true,
    lib: {
      entry: fileURLToPath(new URL("./lib/index.ts", __vite_injected_original_import_meta_url)),
      //path.resolve(__dirname, './lib/index.ts'),
      name: "monaco-tree-editor",
      fileName: "index",
      formats: ["es", "umd"]
    },
    rollupOptions: {
      external: ["vue"],
      output: {
        globals: {
          vue: "Vue"
        }
      }
    }
  },
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true
  }
  // 3. to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.app/v1/api/config#buildconfig.beforedevcommand
  // envPrefix: ["VITE_", "TAURI_"],
});
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcudHMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCJEOlxcXFxjb2RlXFxcXHJ1c3RcXFxcQXV0b21hdGVkIHRlc3RpbmcgdG9vbHNcIjtjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfZmlsZW5hbWUgPSBcIkQ6XFxcXGNvZGVcXFxccnVzdFxcXFxBdXRvbWF0ZWQgdGVzdGluZyB0b29sc1xcXFx2aXRlLmNvbmZpZy50c1wiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9pbXBvcnRfbWV0YV91cmwgPSBcImZpbGU6Ly8vRDovY29kZS9ydXN0L0F1dG9tYXRlZCUyMHRlc3RpbmclMjB0b29scy92aXRlLmNvbmZpZy50c1wiO2ltcG9ydCB7IGRlZmluZUNvbmZpZyB9IGZyb20gXCJ2aXRlXCI7XHJcbmltcG9ydCB2dWUgZnJvbSBcIkB2aXRlanMvcGx1Z2luLXZ1ZVwiO1xyXG5pbXBvcnQgdnVlSnN4IGZyb20gJ0B2aXRlanMvcGx1Z2luLXZ1ZS1qc3gnXHJcbmltcG9ydCB7IGZpbGVVUkxUb1BhdGgsIFVSTCB9IGZyb20gJ25vZGU6dXJsJ1xyXG5cclxuLy8gaHR0cHM6Ly92aXRlanMuZGV2L2NvbmZpZy9cclxuZXhwb3J0IGRlZmF1bHQgZGVmaW5lQ29uZmlnKHtcclxuICBwbHVnaW5zOiBbdnVlKCksdnVlSnN4KCldLFxyXG4gIHJlc29sdmU6IHtcclxuICAgIGFsaWFzOiB7XHJcbiAgICAgICd+bGliJzogZmlsZVVSTFRvUGF0aChuZXcgVVJMKCcuL2xpYicsIGltcG9ydC5tZXRhLnVybCkpLFxyXG4gICAgICAnQCc6IGZpbGVVUkxUb1BhdGgobmV3IFVSTCgnLi9zcmMnLCBpbXBvcnQubWV0YS51cmwpKSxcclxuICAgIH0sXHJcbiAgfSxcclxuICBvcHRpbWl6ZURlcHM6IHtcclxuICAgIGluY2x1ZGU6IFtdLFxyXG4gIH0sXHJcbiAgZXNidWlsZDoge1xyXG4gICAgZHJvcDogWydjb25zb2xlJywgJ2RlYnVnZ2VyJ10sXHJcbiAgfSxcclxuICAvLyBiYXNlOiAnL21vbmFjby10cmVlLWVkaXRvci8nLFxyXG4gIGJ1aWxkOiB7XHJcbiAgICBtaW5pZnk6ICdlc2J1aWxkJyxcclxuICAgIG91dERpcjogJ2Rpc3QnLFxyXG4gICAgdGFyZ2V0OiAnZXNuZXh0JyxcclxuICAgIGNzc0NvZGVTcGxpdDogdHJ1ZSxcclxuICAgIGxpYjoge1xyXG4gICAgICBlbnRyeTogZmlsZVVSTFRvUGF0aChuZXcgVVJMKCcuL2xpYi9pbmRleC50cycsIGltcG9ydC5tZXRhLnVybCkpLCAvL3BhdGgucmVzb2x2ZShfX2Rpcm5hbWUsICcuL2xpYi9pbmRleC50cycpLFxyXG4gICAgICBuYW1lOiAnbW9uYWNvLXRyZWUtZWRpdG9yJyxcclxuICAgICAgZmlsZU5hbWU6ICdpbmRleCcsXHJcbiAgICAgIGZvcm1hdHM6IFsnZXMnLCAndW1kJ10sXHJcbiAgICB9LFxyXG4gICAgcm9sbHVwT3B0aW9uczoge1xyXG4gICAgICBleHRlcm5hbDogWyd2dWUnXSxcclxuICAgICAgb3V0cHV0OiB7XHJcbiAgICAgICAgZ2xvYmFsczoge1xyXG4gICAgICAgICAgdnVlOiAnVnVlJyxcclxuICAgICAgICB9LFxyXG4gICAgICB9LFxyXG4gICAgfSxcclxuICB9LFxyXG4gIC8vIFZpdGUgb3B0aW9ucyB0YWlsb3JlZCBmb3IgVGF1cmkgZGV2ZWxvcG1lbnQgYW5kIG9ubHkgYXBwbGllZCBpbiBgdGF1cmkgZGV2YCBvciBgdGF1cmkgYnVpbGRgXHJcbiAgLy9cclxuICAvLyAxLiBwcmV2ZW50IHZpdGUgZnJvbSBvYnNjdXJpbmcgcnVzdCBlcnJvcnNcclxuICBjbGVhclNjcmVlbjogZmFsc2UsXHJcbiAgLy8gMi4gdGF1cmkgZXhwZWN0cyBhIGZpeGVkIHBvcnQsIGZhaWwgaWYgdGhhdCBwb3J0IGlzIG5vdCBhdmFpbGFibGVcclxuICBzZXJ2ZXI6IHtcclxuICAgIHBvcnQ6IDE0MjAsXHJcbiAgICBzdHJpY3RQb3J0OiB0cnVlLFxyXG4gIH0sXHJcbiAgXHJcbiAgLy8gMy4gdG8gbWFrZSB1c2Ugb2YgYFRBVVJJX0RFQlVHYCBhbmQgb3RoZXIgZW52IHZhcmlhYmxlc1xyXG4gIC8vIGh0dHBzOi8vdGF1cmkuYXBwL3YxL2FwaS9jb25maWcjYnVpbGRjb25maWcuYmVmb3JlZGV2Y29tbWFuZFxyXG5cclxuICAvLyBlbnZQcmVmaXg6IFtcIlZJVEVfXCIsIFwiVEFVUklfXCJdLFxyXG59KTtcclxuIl0sCiAgIm1hcHBpbmdzIjogIjtBQUEwUyxTQUFTLG9CQUFvQjtBQUN2VSxPQUFPLFNBQVM7QUFDaEIsT0FBTyxZQUFZO0FBQ25CLFNBQVMsZUFBZSxXQUFXO0FBSG9KLElBQU0sMkNBQTJDO0FBTXhPLElBQU8sc0JBQVEsYUFBYTtBQUFBLEVBQzFCLFNBQVMsQ0FBQyxJQUFJLEdBQUUsT0FBTyxDQUFDO0FBQUEsRUFDeEIsU0FBUztBQUFBLElBQ1AsT0FBTztBQUFBLE1BQ0wsUUFBUSxjQUFjLElBQUksSUFBSSxTQUFTLHdDQUFlLENBQUM7QUFBQSxNQUN2RCxLQUFLLGNBQWMsSUFBSSxJQUFJLFNBQVMsd0NBQWUsQ0FBQztBQUFBLElBQ3REO0FBQUEsRUFDRjtBQUFBLEVBQ0EsY0FBYztBQUFBLElBQ1osU0FBUyxDQUFDO0FBQUEsRUFDWjtBQUFBLEVBQ0EsU0FBUztBQUFBLElBQ1AsTUFBTSxDQUFDLFdBQVcsVUFBVTtBQUFBLEVBQzlCO0FBQUE7QUFBQSxFQUVBLE9BQU87QUFBQSxJQUNMLFFBQVE7QUFBQSxJQUNSLFFBQVE7QUFBQSxJQUNSLFFBQVE7QUFBQSxJQUNSLGNBQWM7QUFBQSxJQUNkLEtBQUs7QUFBQSxNQUNILE9BQU8sY0FBYyxJQUFJLElBQUksa0JBQWtCLHdDQUFlLENBQUM7QUFBQTtBQUFBLE1BQy9ELE1BQU07QUFBQSxNQUNOLFVBQVU7QUFBQSxNQUNWLFNBQVMsQ0FBQyxNQUFNLEtBQUs7QUFBQSxJQUN2QjtBQUFBLElBQ0EsZUFBZTtBQUFBLE1BQ2IsVUFBVSxDQUFDLEtBQUs7QUFBQSxNQUNoQixRQUFRO0FBQUEsUUFDTixTQUFTO0FBQUEsVUFDUCxLQUFLO0FBQUEsUUFDUDtBQUFBLE1BQ0Y7QUFBQSxJQUNGO0FBQUEsRUFDRjtBQUFBO0FBQUE7QUFBQTtBQUFBLEVBSUEsYUFBYTtBQUFBO0FBQUEsRUFFYixRQUFRO0FBQUEsSUFDTixNQUFNO0FBQUEsSUFDTixZQUFZO0FBQUEsRUFDZDtBQUFBO0FBQUE7QUFBQTtBQU1GLENBQUM7IiwKICAibmFtZXMiOiBbXQp9Cg==
