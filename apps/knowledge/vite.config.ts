import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

// https://vite.dev/config/
export default defineConfig({
  plugins: [svelte()],
  clearScreen: false,
  server: {
    port: 5174,
    strictPort: true,
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    target: ['es2021', 'chrome100', 'safari13'],
    // eslint-disable-next-line no-undef
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    // eslint-disable-next-line no-undef
    sourcemap: !!process.env.TAURI_DEBUG,
  },
});
