import { redirect } from '@sveltejs/kit';
import { PUBLIC_ZITADEL_CLIENT_ID, PUBLIC_ZITADEL_ISSUER } from '$env/static/public';
import { generateCodeVerifier, generateCodeChallenge } from '$lib/auth/pkce';
import type { PageServerLoadEvent } from './$types';

export async function load(event: PageServerLoadEvent): Promise<never> {
	const codeVerifier = generateCodeVerifier();
	const codeChallenge = await generateCodeChallenge(codeVerifier);

	event.cookies.set('code_verifier', codeVerifier, {
		httpOnly: true,
		sameSite: 'lax',
		path: '/',
		maxAge: 600
	});

	const params = new URLSearchParams({
		response_type: 'code',
		client_id: PUBLIC_ZITADEL_CLIENT_ID,
		redirect_uri: 'http://localhost:5173/auth/callback',
		scope: 'openid profile email',
		code_challenge: codeChallenge,
		code_challenge_method: 'S256'
	});

	const authUrl = `${PUBLIC_ZITADEL_ISSUER}/oauth/v2/authorize?${params.toString()}`;

	throw redirect(302, authUrl);
}
