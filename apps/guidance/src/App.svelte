<script lang="ts">
  import Home from './routes/Home.svelte';
  import Quests from './routes/Quests.svelte';
  import About from './routes/About.svelte';

  // Svelte 5 runes - state management
  let appTitle = $state('Guidance');
  let currentRoute = $state('home');

  function navigateTo(route: string) {
    currentRoute = route;
  }
</script>

<div class="app">
  <header>
    <h1>{appTitle}</h1>
    <nav>
      <button class:active={currentRoute === 'home'} onclick={() => navigateTo('home')}>
        Home
      </button>
      <button class:active={currentRoute === 'quests'} onclick={() => navigateTo('quests')}>
        Quests
      </button>
      <button class:active={currentRoute === 'about'} onclick={() => navigateTo('about')}>
        About
      </button>
    </nav>
  </header>

  <main>
    {#if currentRoute === 'home'}
      <Home />
    {:else if currentRoute === 'quests'}
      <Quests />
    {:else if currentRoute === 'about'}
      <About />
    {/if}
  </main>

  <footer>
    <p>Quest-Based Task Management for ADHD</p>
  </footer>
</div>

<style>
  .app {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
    font-family:
      system-ui,
      -apple-system,
      sans-serif;
  }

  header {
    padding: 1rem 2rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
  }

  h1 {
    margin: 0 0 1rem 0;
    font-size: 2rem;
  }

  nav {
    display: flex;
    gap: 1.5rem;
  }

  nav button {
    color: white;
    background: transparent;
    border: none;
    text-decoration: none;
    font-weight: 500;
    padding: 0.5rem 1rem;
    border-radius: 0.25rem;
    transition: background-color 0.2s;
    cursor: pointer;
    font-size: 1rem;
  }

  nav button:hover {
    background-color: rgba(255, 255, 255, 0.1);
  }

  nav button.active {
    background-color: rgba(255, 255, 255, 0.2);
  }

  main {
    flex: 1;
    padding: 2rem;
    max-width: 1200px;
    width: 100%;
    margin: 0 auto;
  }

  footer {
    padding: 1rem 2rem;
    background: #f5f5f5;
    text-align: center;
    color: #666;
  }

  footer p {
    margin: 0;
  }
</style>
