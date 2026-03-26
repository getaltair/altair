import type { PageServerLoad } from './$types';

/**
 * Auth gate for the sync debug page.
 *
 * Mirrors the (app) layout guard. Once Better Auth session injection is
 * wired in hooks.server.ts, uncomment the redirect below.
 */
export const load: PageServerLoad = async ({ locals }) => {
	// TODO: Enable once Better Auth session injection is active.
	// if (!locals.user) {
	// 	redirect(302, '/login');
	// }

	if (!locals.user)
		console.warn('[auth] Auth gate disabled: serving /debug/sync without authentication.');

	return {};
};
