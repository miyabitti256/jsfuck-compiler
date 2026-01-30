import { defineConfig } from 'vite';

export default defineConfig({
  build: {
    target: 'esnext' // Wasm Top-level await対応のため
  },
  base: './', // GitHub Pages用（相対パス）
});
