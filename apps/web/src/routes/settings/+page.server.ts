import { fail, redirect } from '@sveltejs/kit';
import type { ServerLoadEvent, RequestEvent } from '@sveltejs/kit';
import { PUBLIC_API_BASE_URL } from '$env/static/public';

export const load = async ({ fetch }: ServerLoadEvent) => {
  const res = await fetch('/api/auth/me');
  const profile = res.ok ? await res.json() as { display_name?: string; email?: string } : {};
  return { displayName: profile.display_name ?? '', email: profile.email ?? '' };
};

export const actions = {
  updateProfile: async ({ request, fetch }: RequestEvent) => {
    const data = await request.formData();
    const display_name = data.get('display_name') as string;
    const email = data.get('email') as string;

    if (!display_name && !email) {
      return fail(400, { updateError: 'At least one field is required.' });
    }

    let res: Response;
    try {
      res = await fetch(`${PUBLIC_API_BASE_URL}/api/auth/profile`, {
        method: 'PATCH',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ display_name, email }),
      });
    } catch {
      return fail(503, { updateError: 'Server unavailable. Please try again.' });
    }

    if (res.ok) {
      return { updateSuccess: true };
    }

    const body = await res.json().catch(() => ({})) as { error?: string };
    return fail(res.status, { updateError: body.error ?? 'Failed to update profile.' });
  },

  changePassword: async ({ request, fetch }: RequestEvent) => {
    const data = await request.formData();
    const old_password = data.get('old_password') as string;
    const new_password = data.get('new_password') as string;
    const confirm_password = data.get('confirm_password') as string;

    if (!old_password || !new_password || !confirm_password) {
      return fail(400, { passwordError: 'All password fields are required.' });
    }
    if (new_password !== confirm_password) {
      return fail(400, { passwordError: 'New passwords do not match.' });
    }
    if (new_password.length < 8) {
      return fail(400, { passwordError: 'New password must be at least 8 characters.' });
    }

    let res: Response;
    try {
      res = await fetch(`${PUBLIC_API_BASE_URL}/api/auth/password`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ old_password, new_password }),
      });
    } catch {
      return fail(503, { passwordError: 'Server unavailable. Please try again.' });
    }

    if (res.ok) {
      return { passwordSuccess: true };
    }

    const body = await res.json().catch(() => ({})) as { error?: string };
    return fail(res.status, { passwordError: body.error ?? 'Failed to change password.' });
  },

  logout: async ({ fetch, cookies }: RequestEvent) => {
    try {
      await fetch(`${PUBLIC_API_BASE_URL}/api/auth/logout`, { method: 'POST' });
    } catch {
      // Best-effort — clear cookies regardless
    }

    cookies.delete('access_token', { path: '/' });
    cookies.delete('refresh_token', { path: '/api/auth/' });

    throw redirect(303, '/auth/login');
  },
};
