import { mount } from 'svelte';
import App from './App.svelte';

const app = mount(App, {
  // eslint-disable-next-line no-undef
  target: document.getElementById('app')!,
});

export default app;
