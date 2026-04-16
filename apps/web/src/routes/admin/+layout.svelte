<script lang="ts">
  import { page } from '$app/state';

  let { children }: { children: import('svelte').Snippet } = $props();

  const navItems = [
    { href: '/admin/users', label: 'Users' },
    { href: '/admin/households', label: 'Households' },
    { href: '/admin/health', label: 'Health' },
  ] as const;

  function isActive(href: string): boolean {
    return page.url.pathname.startsWith(href);
  }
</script>

<div class="admin-layout">
  <aside class="admin-nav">
    <h2 class="admin-nav__title">Admin</h2>
    <nav aria-label="Admin navigation">
      <ul>
        {#each navItems as item}
          <li>
            <a
              href={item.href}
              class="admin-nav__link"
              class:active={isActive(item.href)}
              aria-current={isActive(item.href) ? 'page' : undefined}
            >
              {item.label}
            </a>
          </li>
        {/each}
      </ul>
    </nav>
  </aside>

  <div class="admin-content">
    {@render children()}
  </div>
</div>

<style>
  .admin-layout {
    display: flex;
    gap: 2rem;
    max-width: 72rem;
    margin: 0 auto;
    padding: 2rem 1.5rem;
  }

  .admin-nav {
    width: 12rem;
    flex-shrink: 0;
  }

  .admin-nav__title {
    font-family: var(--font-display);
    font-size: 0.75rem;
    font-weight: 700;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--on-surface-variant);
    margin-bottom: 0.75rem;
  }

  ul {
    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .admin-nav__link {
    display: block;
    padding: 0.5rem 0.75rem;
    border-radius: 0.5rem;
    font-family: var(--font-body);
    font-size: 0.9375rem;
    font-weight: 500;
    color: var(--on-surface);
    text-decoration: none;
    transition: background-color var(--motion-standard), color var(--motion-standard);
  }

  .admin-nav__link:hover {
    background-color: var(--surface-zone);
  }

  .admin-nav__link.active {
    background-color: var(--secondary-container);
    color: var(--primary);
    font-weight: 600;
  }

  .admin-content {
    flex: 1;
    min-width: 0;
  }
</style>
