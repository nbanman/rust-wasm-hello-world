import { defineConfig } from 'vite'
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import { fileURLToPath } from 'url';
import { dirname, resolve } from 'path';

const rootDir = dirname(fileURLToPath(import.meta.url));

export default defineConfig({
  root: '.',
  plugins: [
    wasm(),
    topLevelAwait()
  ],
  build: {
    outDir: 'dist',
    rollupOptions: {
      input: {
        main: 'index.html',
        bootstrap: 'bootstrap.js'
      }
    }
  },
  server: {
    open: 'index.html',
    fs: {
      allow: [
        '.',
        resolve(rootDir, '../pkg'),
      ]
    },
  }
});
