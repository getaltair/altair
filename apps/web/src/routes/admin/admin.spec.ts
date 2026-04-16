import { describe, it, expect, vi } from 'vitest';

// ============================================================
// admin/+layout.server.ts — admin guard tests
// ============================================================

describe('admin +layout.server.ts guard', () => {
  it('redirects to /auth/login when user is unauthenticated', async () => {
    const { load } = await import('./+layout.server.ts');

    const mockEvent = {
      locals: { user: null },
      fetch: vi.fn(),
    };

    await expect(load(mockEvent as never)).rejects.toSatisfy((err: unknown) => {
      return (
        err instanceof Response ||
        (typeof err === 'object' &&
          err !== null &&
          'status' in err &&
          (err as { status: number }).status === 303 &&
          'location' in err &&
          (err as { location: string }).location === '/auth/login')
      );
    });

    expect(mockEvent.fetch).not.toHaveBeenCalled();
  });

  it('redirects to / when is_admin is false', async () => {
    vi.resetModules();
    const { load } = await import('./+layout.server.ts');

    const mockEvent = {
      locals: { user: { id: 'user-1', email: 'user@example.com' } },
      fetch: vi.fn().mockResolvedValue({
        ok: true,
        json: async () => ({ is_admin: false }),
      }),
    };

    await expect(load(mockEvent as never)).rejects.toSatisfy((err: unknown) => {
      return (
        err instanceof Response ||
        (typeof err === 'object' && err !== null && 'status' in err && (err as { status: number }).status === 303)
      );
    });
  });

  it('redirects to / when /api/auth/me returns non-ok', async () => {
    vi.resetModules();
    const { load } = await import('./+layout.server.ts');

    const mockEvent = {
      locals: { user: { id: 'user-1', email: 'user@example.com' } },
      fetch: vi.fn().mockResolvedValue({
        ok: false,
        status: 401,
      }),
    };

    await expect(load(mockEvent as never)).rejects.toSatisfy((err: unknown) => {
      return (
        err instanceof Response ||
        (typeof err === 'object' && err !== null && 'status' in err && (err as { status: number }).status === 303)
      );
    });
  });

  it('returns { isAdmin: true } when user is admin', async () => {
    vi.resetModules();
    const { load } = await import('./+layout.server.ts');

    const mockEvent = {
      locals: { user: { id: 'admin-1', email: 'admin@example.com' } },
      fetch: vi.fn().mockResolvedValue({
        ok: true,
        json: async () => ({ is_admin: true }),
      }),
    };

    const result = await load(mockEvent as never);
    expect(result).toMatchObject({ isAdmin: true });
  });
});

// ============================================================
// admin/health/+page.server.ts — load function tests
// ============================================================

describe('admin health +page.server.ts', () => {
  it('returns { db: "ok" } from mocked /api/health response', async () => {
    vi.resetModules();
    const { load } = await import('./health/+page.server.ts');

    const mockEvent = {
      fetch: vi.fn().mockResolvedValue({
        ok: true,
        json: async () => ({ db: 'ok', powersync: 'healthy' }),
      }),
    };

    const result = await load(mockEvent as never);
    expect(result).toMatchObject({ db: 'ok', powersync: 'healthy' });
  });

  it('returns { db: "error", powersync: "unknown" } when fetch returns non-ok', async () => {
    vi.resetModules();
    const { load } = await import('./health/+page.server.ts');

    const mockEvent = {
      fetch: vi.fn().mockResolvedValue({
        ok: false,
        status: 503,
      }),
    };

    const result = await load(mockEvent as never);
    expect(result).toMatchObject({ db: 'error', powersync: 'unknown' });
  });

  it('returns { db: "unreachable", powersync: "unknown" } when fetch throws', async () => {
    vi.resetModules();
    const { load } = await import('./health/+page.server.ts');

    const mockEvent = {
      fetch: vi.fn().mockRejectedValue(new Error('network error')),
    };

    const result = await load(mockEvent as never);
    expect(result).toMatchObject({ db: 'unreachable', powersync: 'unknown' });
  });

  it('uses "unknown" for missing db/powersync fields', async () => {
    vi.resetModules();
    const { load } = await import('./health/+page.server.ts');

    const mockEvent = {
      fetch: vi.fn().mockResolvedValue({
        ok: true,
        json: async () => ({ status: 'ok' }),
      }),
    };

    const result = await load(mockEvent as never);
    expect(result).toMatchObject({ db: 'unknown', powersync: 'unknown' });
  });
});
