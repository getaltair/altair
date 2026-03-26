// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces

interface AuthUser {
	id: string;
	email: string;
	name: string;
	createdAt: Date;
	updatedAt: Date;
}

interface AuthSession {
	id: string;
	userId: string;
	expiresAt: Date;
}

declare global {
	namespace App {
		// interface Error {}
		// Placeholder types -- replace with Better Auth's User and Session types once auth integration is complete.
		interface Locals {
			user: AuthUser | null;
			session: AuthSession | null;
		}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}
}

export {};
