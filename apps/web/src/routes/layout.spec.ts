import { describe, it, expect, vi, beforeEach } from 'vitest';

// ============================================================
// +layout.server.ts load function — auth guard tests
// ============================================================

describe('+layout.server.ts auth guard', () => {
  it('redirects to /auth/login when user is null', async () => {
    const { load } = await import('./+layout.server.ts');

    const mockEvent = {
      locals: { user: null },
      fetch: vi.fn(),
    };

    await expect(load(mockEvent as never)).rejects.toSatisfy((err: unknown) => {
      // SvelteKit redirect throws a Response-like object with status 303
      return (
        err instanceof Response ||
        (typeof err === 'object' && err !== null && 'status' in err && (err as { status: number }).status === 303)
      );
    });
  });

  it('returns isAdmin: true when profile.is_admin is true', async () => {
    vi.resetModules();
    const { load } = await import('./+layout.server.ts');

    const mockEvent = {
      locals: { user: { id: 'user-1', email: 'test@example.com' } },
      fetch: vi.fn().mockResolvedValue({
        ok: true,
        json: async () => ({ is_admin: true }),
      }),
    };

    const result = await load(mockEvent as never);
    expect(result).toMatchObject({ isAdmin: true });
  });

  it('returns isAdmin: false when profile.is_admin is false', async () => {
    vi.resetModules();
    const { load } = await import('./+layout.server.ts');

    const mockEvent = {
      locals: { user: { id: 'user-1', email: 'test@example.com' } },
      fetch: vi.fn().mockResolvedValue({
        ok: true,
        json: async () => ({ is_admin: false }),
      }),
    };

    const result = await load(mockEvent as never);
    expect(result).toMatchObject({ isAdmin: false });
  });

  it('returns isAdmin: false when fetch fails', async () => {
    vi.resetModules();
    const { load } = await import('./+layout.server.ts');

    const mockEvent = {
      locals: { user: { id: 'user-1', email: 'test@example.com' } },
      fetch: vi.fn().mockResolvedValue({
        ok: false,
        status: 500,
      }),
    };

    const result = await load(mockEvent as never);
    expect(result).toMatchObject({ isAdmin: false });
  });
});

// ============================================================
// Sidebar component — admin link visibility
// ============================================================

describe('Sidebar admin nav link', () => {
  beforeEach(() => {
    vi.resetModules();
  });

  it('renders Admin nav link when isAdmin=true', async () => {
    const { render } = await import('@testing-library/svelte');
    const Sidebar = (await import('../lib/components/layout/Sidebar.svelte')).default;

    const { getByText } = render(Sidebar, { props: { isAdmin: true } });
    expect(getByText('Admin')).toBeTruthy();
  });

  it('does not render Admin nav link when isAdmin=false', async () => {
    const { render } = await import('@testing-library/svelte');
    const Sidebar = (await import('../lib/components/layout/Sidebar.svelte')).default;

    const { container } = render(Sidebar, { props: { isAdmin: false } });
    // queryByText returns null if not found
    const adminLink = container.querySelector('a[href="/admin"]');
    expect(adminLink).toBeNull();
  });
});

// ============================================================
// Sidebar responsive behavior
// ============================================================

describe('Sidebar responsive layout', () => {
  it('hamburger button exists in the sidebar markup', async () => {
    const { render } = await import('@testing-library/svelte');
    const Sidebar = (await import('../lib/components/layout/Sidebar.svelte')).default;

    const { container } = render(Sidebar, { props: { isAdmin: false } });
    const hamburger = container.querySelector('.hamburger');
    expect(hamburger).toBeTruthy();
  });

  it('sidebar has CSS class "sidebar" for media query targeting', async () => {
    const { render } = await import('@testing-library/svelte');
    const Sidebar = (await import('../lib/components/layout/Sidebar.svelte')).default;

    const { container } = render(Sidebar, { props: { isAdmin: false } });
    const sidebar = container.querySelector('.sidebar');
    expect(sidebar).toBeTruthy();
  });
});
