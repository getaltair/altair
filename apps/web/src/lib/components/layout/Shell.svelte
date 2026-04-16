<script lang="ts">
  import type { Snippet } from 'svelte';
  import Sidebar from './Sidebar.svelte';
  import TopBar from './TopBar.svelte';

  interface Props {
    isAdmin: boolean;
    children: Snippet;
  }

  let { isAdmin, children }: Props = $props();
</script>

<div class="shell">
  <Sidebar {isAdmin} />

  <div class="main-area">
    <TopBar />
    <main class="content">
      {@render children()}
    </main>
  </div>
</div>

<style>
  .shell {
    display: flex;
    min-height: 100vh;
    background-color: var(--surface-base);
  }

  .main-area {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  .content {
    flex: 1;
    padding: 1.5rem;
    background-color: var(--surface-base);
  }

  @media (max-width: 767px) {
    .main-area {
      margin-left: 64px;
    }
  }
</style>
