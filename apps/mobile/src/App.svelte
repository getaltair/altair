<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  // State using Svelte 5 runes
  let currentView = $state<'guidance' | 'knowledge' | 'tracking'>('guidance');
  let greetMsg = $state('');
  let name = $state('User');

  async function greet() {
    greetMsg = await invoke<string>('greet', { name });
  }

  function setView(view: 'guidance' | 'knowledge' | 'tracking') {
    currentView = view;
  }
</script>

<main class="mobile-app">
  <header class="header">
    <h1>Altair Mobile</h1>
    <p class="subtitle">Quest-Based Productivity</p>
  </header>

  <nav class="tab-nav">
    <button
      class="tab-button"
      class:active={currentView === 'guidance'}
      onclick={() => setView('guidance')}
    >
      🎯 Guidance
    </button>
    <button
      class="tab-button"
      class:active={currentView === 'knowledge'}
      onclick={() => setView('knowledge')}
    >
      📚 Knowledge
    </button>
    <button
      class="tab-button"
      class:active={currentView === 'tracking'}
      onclick={() => setView('tracking')}
    >
      📦 Tracking
    </button>
  </nav>

  <div class="content">
    {#if currentView === 'guidance'}
      <section class="view-section">
        <h2>Guidance</h2>
        <p>Quest management and task tracking</p>
        <div class="feature-list">
          <div class="feature-card">
            <h3>Active Quests</h3>
            <p>Your current adventures</p>
          </div>
          <div class="feature-card">
            <h3>Campaigns</h3>
            <p>Long-term goals</p>
          </div>
        </div>
      </section>
    {:else if currentView === 'knowledge'}
      <section class="view-section">
        <h2>Knowledge</h2>
        <p>Personal knowledge management</p>
        <div class="feature-list">
          <div class="feature-card">
            <h3>Notes</h3>
            <p>Your knowledge base</p>
          </div>
          <div class="feature-card">
            <h3>Wiki Links</h3>
            <p>Connected thoughts</p>
          </div>
        </div>
      </section>
    {:else}
      <section class="view-section">
        <h2>Tracking</h2>
        <p>Inventory management</p>
        <div class="feature-list">
          <div class="feature-card">
            <h3>Items</h3>
            <p>Track your possessions</p>
          </div>
          <div class="feature-card">
            <h3>Locations</h3>
            <p>Where things are stored</p>
          </div>
        </div>
      </section>
    {/if}
  </div>

  <footer class="footer">
    <div class="greet-section">
      <input type="text" bind:value={name} placeholder="Enter your name" />
      <button onclick={greet}>Greet</button>
      {#if greetMsg}
        <p class="greet-msg">{greetMsg}</p>
      {/if}
    </div>
  </footer>
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial,
      sans-serif;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    min-height: 100vh;
  }

  .mobile-app {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
    color: white;
  }

  .header {
    text-align: center;
    padding: 2rem 1rem 1rem;
    background: rgba(0, 0, 0, 0.2);
  }

  .header h1 {
    margin: 0;
    font-size: 2rem;
    font-weight: 700;
  }

  .subtitle {
    margin: 0.5rem 0 0;
    opacity: 0.9;
    font-size: 0.95rem;
  }

  .tab-nav {
    display: flex;
    justify-content: space-around;
    padding: 1rem;
    background: rgba(0, 0, 0, 0.15);
    gap: 0.5rem;
  }

  .tab-button {
    flex: 1;
    padding: 0.75rem;
    background: rgba(255, 255, 255, 0.1);
    border: 2px solid transparent;
    border-radius: 8px;
    color: white;
    font-size: 0.9rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  .tab-button.active {
    background: rgba(255, 255, 255, 0.25);
    border-color: rgba(255, 255, 255, 0.4);
    font-weight: 600;
  }

  .content {
    flex: 1;
    padding: 1.5rem;
    overflow-y: auto;
  }

  .view-section h2 {
    margin: 0 0 0.5rem;
    font-size: 1.5rem;
  }

  .view-section > p {
    margin: 0 0 1.5rem;
    opacity: 0.9;
  }

  .feature-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .feature-card {
    background: rgba(255, 255, 255, 0.15);
    border-radius: 12px;
    padding: 1.25rem;
    backdrop-filter: blur(10px);
  }

  .feature-card h3 {
    margin: 0 0 0.5rem;
    font-size: 1.1rem;
  }

  .feature-card p {
    margin: 0;
    opacity: 0.9;
    font-size: 0.9rem;
  }

  .footer {
    padding: 1rem;
    background: rgba(0, 0, 0, 0.2);
  }

  .greet-section {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    max-width: 400px;
    margin: 0 auto;
  }

  .greet-section input {
    padding: 0.75rem;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-radius: 8px;
    background: rgba(255, 255, 255, 0.1);
    color: white;
    font-size: 1rem;
  }

  .greet-section input::placeholder {
    color: rgba(255, 255, 255, 0.6);
  }

  .greet-section button {
    padding: 0.75rem;
    background: rgba(255, 255, 255, 0.2);
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-radius: 8px;
    color: white;
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
  }

  .greet-section button:active {
    background: rgba(255, 255, 255, 0.3);
  }

  .greet-msg {
    margin: 0;
    padding: 0.75rem;
    background: rgba(255, 255, 255, 0.15);
    border-radius: 8px;
    text-align: center;
  }
</style>
