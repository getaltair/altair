import type { Handle } from '@sveltejs/kit';

// TODO: Replace with Better Auth's svelteKitHandler once auth integration is complete.
export const handle: Handle = async ({ event, resolve }) => {
	event.locals.user = event.locals.user ?? null;
	event.locals.session = event.locals.session ?? null;
	return resolve(event);
};
