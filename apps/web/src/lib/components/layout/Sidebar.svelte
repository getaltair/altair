<script lang="ts">
  import { page } from '$app/state';

  interface Props {
    isAdmin: boolean;
  }

  let { isAdmin }: Props = $props();

  let collapsed = $state(false);

  function toggleCollapsed() {
    collapsed = !collapsed;
  }

  const navItems = [
    { href: '/', label: 'Today', icon: '◆' },
    { href: '/guidance', label: 'Guidance', icon: '◎' },
    { href: '/knowledge', label: 'Knowledge', icon: '◇' },
    { href: '/tracking', label: 'Tracking', icon: '◈' },
    { href: '/search', label: 'Search', icon: '⊙' },
    { href: '/settings', label: 'Settings', icon: '⚙' },
  ] as const;

  function isActive(href: string): boolean {
    const currentPath = page.url.pathname;
    if (href === '/') {
      return currentPath === '/';
    }
    return currentPath.startsWith(href);
  }
</script>

<aside class="sidebar" class:collapsed>
  <button class="hamburger" onclick={toggleCollapsed} aria-label="Toggle navigation">
    <span></span>
    <span></span>
    <span></span>
  </button>

  <nav aria-label="Main navigation">
    <ul>
      {#each navItems as item}
        <li>
          <a
            href={item.href}
            class="nav-item"
            class:active={isActive(item.href)}
            aria-current={isActive(item.href) ? 'page' : undefined}
          >
            <span class="nav-icon" aria-hidden="true">{item.icon}</span>
            <span class="nav-label">{item.label}</span>
          </a>
        </li>
      {/each}

      {#if isAdmin}
        <li>
          <a
            href="/admin"
            class="nav-item"
            class:active={isActive('/admin')}
            aria-current={isActive('/admin') ? 'page' : undefined}
          >
            <span class="nav-icon" aria-hidden="true">⚡</span>
            <span class="nav-label">Admin</span>
          </a>
        </li>
      {/if}
    </ul>
  </nav>
</aside>

<style>
  .sidebar {
    width: 240px;
    min-height: 100vh;
    background-color: var(--surface-elevated);
    display: flex;
    flex-direction: column;
    padding: 1rem 0.75rem;
    transition: width var(--motion-standard);
    overflow: hidden;
    flex-shrink: 0;
  }

  .sidebar.collapsed {
    width: 64px;
  }

  .sidebar.collapsed .nav-label {
    display: none;
  }

  .hamburger {
    display: none;
    flex-direction: column;
    gap: 5px;
    background: none;
    border: none;
    cursor: pointer;
    padding: 0.5rem;
    margin-bottom: 1rem;
    align-self: flex-start;
  }

  .hamburger span {
    display: block;
    width: 20px;
    height: 2px;
    background-color: var(--on-surface);
    border-radius: 2px;
  }

  nav {
    flex: 1;
  }

  ul {
    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.625rem 0.75rem;
    border-radius: 9999px;
    color: var(--on-surface);
    text-decoration: none;
    font-family: var(--font-body);
    font-size: 0.9375rem;
    font-weight: 500;
    transition: background-color var(--motion-standard), color var(--motion-standard);
    white-space: nowrap;
  }

  .nav-item:hover {
    background-color: var(--surface-group);
  }

  .nav-item.active {
    background-color: var(--secondary-container);
    color: var(--primary);
    font-weight: 600;
  }

  .nav-icon {
    font-size: 1rem;
    flex-shrink: 0;
    width: 1.25rem;
    text-align: center;
  }

  /* Tablet / mobile: show hamburger, hide sidebar by default */
  @media (max-width: 767px) {
    .sidebar {
      width: 64px;
      position: fixed;
      z-index: 100;
      height: 100vh;
    }

    .sidebar.collapsed .nav-label {
      display: none;
    }

    .hamburger {
      display: flex;
    }

    .nav-label {
      display: none;
    }
  }

  /* Desktop: full sidebar visible */
  @media (min-width: 768px) {
    .hamburger {
      display: none;
    }

    .sidebar {
      position: relative;
    }

    .sidebar.collapsed .nav-label {
      display: none;
    }
  }
</style>
