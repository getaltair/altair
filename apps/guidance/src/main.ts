import './app.css';
import App from './App.svelte';
import { mount } from 'svelte';

// Mount the Svelte 5 app
const app = mount(App, {
  // eslint-disable-next-line no-undef
  target: document.getElementById('app')!,
});

export default app;
