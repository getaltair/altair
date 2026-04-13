import { defineConfig } from 'vitest/config';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
  plugins: [svelte()],
  test: {
    environment: 'happy-dom',
    globals: true,
    include: ['src/**/*.spec.ts'],
    passWithNoTests: true,
  },
  resolve: {
    alias: {
      $lib: '/src/lib',
    },
  },
});
