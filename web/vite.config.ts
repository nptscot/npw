import { defineConfig } from "vite";
import { resolve } from "path";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import wasmPack from "vite-plugin-wasm-pack";

export default defineConfig({
  build: {
    rollupOptions: {
      input: {
        index: resolve(__dirname, "index.html"),
        npw: resolve(__dirname, "npw.html"),
        local_storage: resolve(__dirname, "local_storage.html"),
        credits: resolve(__dirname, "credits.html"),
      },
    },
  },
  plugins: [svelte(), wasmPack(["../backend"], [])]
})
