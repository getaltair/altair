import type { ServerLoadEvent } from '@sveltejs/kit';
export const load = async ({ fetch }: ServerLoadEvent) => {
  try {
    const res = await fetch('/api/health');
    if (!res.ok) return { db: 'error', powersync: 'unknown' };
    const data = await res.json() as { db?: string; powersync?: string; storage?: string };
    return { db: data.db ?? 'unknown', powersync: data.powersync ?? 'unknown', storage: data.storage };
  } catch (err) {
    console.error('[admin/health] Health check failed:', err);
    return { db: 'unreachable', powersync: 'unknown' };
  }
};
