import type { ServerLoadEvent } from '@sveltejs/kit';

export const load = async (_event: ServerLoadEvent) => {
  // Daily check-in status is determined reactively on the client via PowerSync.
  // Server load returns a safe default; the component updates when the query resolves.
  return { hasCheckedIn: false };
};
