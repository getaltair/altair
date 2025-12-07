<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  let name = $state('Adventurer');
  let greeting = $state('');

  async function greet() {
    greeting = await invoke('greet', { name });
  }
</script>

<div class="home">
  <h2>Welcome to Guidance</h2>
  <p>Quest-based task management designed for ADHD minds.</p>

  <div class="demo">
    <h3>Test Tauri Command</h3>
    <input bind:value={name} placeholder="Enter your name" />
    <button onclick={greet}>Greet</button>
    {#if greeting}
      <p class="greeting">{greeting}</p>
    {/if}
  </div>

  <div class="features">
    <h3>Features Coming Soon</h3>
    <ul>
      <li>Quest management with energy cost tracking</li>
      <li>Campaign organization for larger goals</li>
      <li>Quick capture for zero-friction input</li>
      <li>Gamification elements for motivation</li>
    </ul>
  </div>
</div>

<style>
  .home {
    max-width: 800px;
  }

  h2 {
    font-size: 2rem;
    margin-bottom: 0.5rem;
    color: #333;
  }

  h3 {
    font-size: 1.5rem;
    margin-top: 2rem;
    margin-bottom: 1rem;
    color: #667eea;
  }

  .demo {
    background: #f9fafb;
    padding: 1.5rem;
    border-radius: 0.5rem;
    margin-top: 2rem;
  }

  input {
    padding: 0.5rem 1rem;
    border: 1px solid #d1d5db;
    border-radius: 0.25rem;
    font-size: 1rem;
    margin-right: 1rem;
  }

  button {
    padding: 0.5rem 1.5rem;
    background: #667eea;
    color: white;
    border: none;
    border-radius: 0.25rem;
    font-size: 1rem;
    cursor: pointer;
    transition: background-color 0.2s;
  }

  button:hover {
    background: #5568d3;
  }

  .greeting {
    margin-top: 1rem;
    padding: 1rem;
    background: white;
    border-left: 4px solid #667eea;
    border-radius: 0.25rem;
    font-weight: 500;
  }

  .features ul {
    list-style: none;
    padding: 0;
  }

  .features li {
    padding: 0.75rem 0;
    border-bottom: 1px solid #e5e7eb;
  }

  .features li:last-child {
    border-bottom: none;
  }

  .features li::before {
    content: '⚡';
    margin-right: 0.5rem;
  }
</style>
