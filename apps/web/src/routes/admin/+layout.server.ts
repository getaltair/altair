import { redirect, isRedirect } from '@sveltejs/kit';
import type { ServerLoadEvent } from '@sveltejs/kit';

export const load = async ({ locals, fetch }: ServerLoadEvent) => {
  if (!locals.user) throw redirect(303, '/auth/login');
  try {
    const res = await fetch('/api/auth/me');
    if (!res.ok) throw redirect(303, '/');
    const profile = await res.json() as { is_admin?: boolean };
    if (!profile.is_admin) throw redirect(303, '/');
  } catch (err) {
    if (isRedirect(err)) throw err;
    // Network error or malformed response — deny access
    throw redirect(303, '/');
  }
  return { isAdmin: true };
};
