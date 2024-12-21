import { defineConfig } from "vite";
import { resolve } from "path";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import wasmPack from "vite-plugin-wasm-pack";

export default defineConfig({
  base: "/npw/",
  build: {
    rollupOptions: {
      input: {
        index: resolve(__dirname, "index.html"),
        npw: resolve(__dirname, "npw.html"),
      },
    },
  },
  plugins: [svelte(), wasmPack(["../backend"], ["route-snapper"])]
})
