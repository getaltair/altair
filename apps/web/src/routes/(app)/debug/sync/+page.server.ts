import { dev } from '$app/environment';
import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

/**
 * Auth gate for the sync debug page.
 *
 * In production, unauthenticated users are redirected to the login page.
 * In dev mode, a warning is logged but the page still loads to ease local testing.
 */
export const load: PageServerLoad = async ({ locals }) => {
	if (!dev && !locals.user) {
		redirect(302, '/login');
	}
	if (dev && !locals.user) {
		console.warn('[sync-debug] No user session in dev mode -- page will load without auth');
	}

	return {};
};
