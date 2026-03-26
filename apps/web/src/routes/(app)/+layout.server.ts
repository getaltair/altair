// import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';

/**
 * Auth gate for all routes in the (app) group.
 *
 * Once Better Auth is wired, uncomment the redirect import above
 * and the guard below to enforce authentication. Once Better Auth is wired,
 * update `src/hooks.server.ts` to inject the user session into `event.locals`,
 * then uncomment the guard below.
 */
export const load: LayoutServerLoad = async ({ locals }) => {
	// TODO: Enable once Better Auth session injection is active.
	// if (!locals.user) {
	// 	redirect(302, '/login');
	// }

	if (!locals.user)
		console.warn('[auth] Auth gate disabled: serving (app) routes without authentication.');

	return {
		user: locals.user ?? null
	};
};
