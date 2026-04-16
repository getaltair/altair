import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async (event) => {
  if (!event.locals.user) {
    throw redirect(303, '/auth/login');
  }

  try {
    const response = await event.fetch('/api/auth/me');
    if (!response.ok) {
      return { isAdmin: false, user: null };
    }
    const profile = (await response.json()) as { is_admin?: boolean; display_name?: string };
    return {
      isAdmin: profile.is_admin ?? false,
      user: { display_name: profile.display_name ?? null },
    };
  } catch {
    // Network error or malformed response — degrade gracefully
    return { isAdmin: false, user: null };
  }
};
