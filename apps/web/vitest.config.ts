import { defineConfig } from 'vitest/config';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
  plugins: [svelte()],
  test: {
    environment: 'happy-dom',
    globals: true,
    include: ['src/**/*.spec.ts'],
    passWithNoTests: true,
    setupFiles: ['src/test-setup.ts'],
  },
  resolve: {
    conditions: ['browser'],
    alias: {
      $lib: '/src/lib',
      '$app/environment': '/src/lib/sync/__mocks__/app-environment.ts',
      '$app/state': '/src/lib/sync/__mocks__/app-state.ts',
    },
  },
});
