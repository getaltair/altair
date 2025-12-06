<script lang="ts">
  import { getVersion } from '@tauri-apps/api/app';

  let version = $state('');

  async function loadVersion() {
    try {
      version = await getVersion();
    } catch {
      version = '0.1.0';
    }
  }

  loadVersion();
</script>

<div class="about">
  <h2>About Guidance</h2>

  <section>
    <h3>Purpose</h3>
    <p>
      Guidance is a quest-based task management application designed specifically for ADHD minds. It
      transforms mundane tasks into engaging quests with energy costs, making productivity feel like
      an adventure.
    </p>
  </section>

  <section>
    <h3>Technology Stack</h3>
    <ul>
      <li><strong>Desktop:</strong> Tauri 2.0 + Svelte 5</li>
      <li><strong>Database:</strong> SurrealDB (embedded + cloud)</li>
      <li><strong>Storage:</strong> S3-compatible (Minio/Backblaze B2)</li>
      <li><strong>Language:</strong> Rust + TypeScript</li>
    </ul>
  </section>

  <section>
    <h3>Part of Altair Ecosystem</h3>
    <p>Guidance is one of three apps in the Altair productivity ecosystem:</p>
    <ul>
      <li><strong>Guidance:</strong> Quest-based task management</li>
      <li><strong>Knowledge:</strong> Personal knowledge management</li>
      <li><strong>Tracking:</strong> Inventory management</li>
    </ul>
  </section>

  <section>
    <h3>Version</h3>
    <p class="version">
      {#if version}
        v{version}
      {:else}
        Loading...
      {/if}
    </p>
  </section>
</div>

<style>
  .about {
    max-width: 800px;
  }

  h2 {
    font-size: 2rem;
    margin-bottom: 1rem;
    color: #333;
  }

  h3 {
    font-size: 1.5rem;
    margin-top: 2rem;
    margin-bottom: 1rem;
    color: #667eea;
  }

  section {
    margin-bottom: 2rem;
  }

  ul {
    list-style: none;
    padding: 0;
  }

  li {
    padding: 0.5rem 0;
    padding-left: 1.5rem;
    position: relative;
  }

  li::before {
    content: '→';
    position: absolute;
    left: 0;
    color: #667eea;
    font-weight: bold;
  }

  .version {
    font-size: 1.25rem;
    font-weight: 600;
    color: #667eea;
  }

  p {
    line-height: 1.6;
    color: #4b5563;
  }
</style>
