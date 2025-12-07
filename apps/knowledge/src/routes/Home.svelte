<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  let name = $state('');
  let greeting = $state('');

  async function handleGreet() {
    if (name.trim()) {
      greeting = await invoke<string>('greet', { name });
    }
  }
</script>

<div class="home">
  <h2>Welcome to Knowledge</h2>
  <p class="description">
    Your personal knowledge management system. Organize your thoughts, connect ideas, and build your
    second brain.
  </p>

  <div class="demo-card">
    <h3>Test Tauri Integration</h3>
    <div class="greet-form">
      <input
        type="text"
        bind:value={name}
        placeholder="Enter your name"
        onkeypress={(e) => e.key === 'Enter' && handleGreet()}
      />
      <button onclick={handleGreet}>Greet</button>
    </div>
    {#if greeting}
      <p class="greeting">{greeting}</p>
    {/if}
  </div>

  <div class="feature-grid">
    <div class="feature">
      <h3>📝 Notes</h3>
      <p>Create and organize your notes with bidirectional links</p>
    </div>
    <div class="feature">
      <h3>🔍 Search</h3>
      <p>Find anything instantly with semantic search</p>
    </div>
    <div class="feature">
      <h3>🔗 Connections</h3>
      <p>Visualize relationships between your ideas</p>
    </div>
  </div>
</div>

<style>
  .home {
    max-width: 800px;
    margin: 0 auto;
  }

  h2 {
    font-size: 2rem;
    margin-bottom: 1rem;
    background: linear-gradient(135deg, #4f46e5 0%, #7c3aed 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .description {
    font-size: 1.1rem;
    color: #666;
    margin-bottom: 2rem;
  }

  .demo-card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.5rem;
    padding: 1.5rem;
    margin-bottom: 2rem;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }

  .demo-card h3 {
    margin-bottom: 1rem;
    color: #374151;
  }

  .greet-form {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1rem;
  }

  input {
    flex: 1;
    padding: 0.5rem 1rem;
    border: 1px solid #d1d5db;
    border-radius: 0.375rem;
    font-size: 1rem;
  }

  button {
    padding: 0.5rem 1.5rem;
    background: linear-gradient(135deg, #4f46e5 0%, #7c3aed 100%);
    color: white;
    border: none;
    border-radius: 0.375rem;
    font-weight: 500;
    cursor: pointer;
    transition: opacity 0.2s;
  }

  button:hover {
    opacity: 0.9;
  }

  .greeting {
    color: #4f46e5;
    font-weight: 500;
  }

  .feature-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1.5rem;
  }

  .feature {
    padding: 1.5rem;
    background: #f9fafb;
    border-radius: 0.5rem;
  }

  .feature h3 {
    font-size: 1.25rem;
    margin-bottom: 0.5rem;
    color: #111827;
  }

  .feature p {
    color: #6b7280;
    font-size: 0.95rem;
  }
</style>
