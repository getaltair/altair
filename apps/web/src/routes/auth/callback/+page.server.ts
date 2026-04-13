import { PUBLIC_ZITADEL_CLIENT_ID, PUBLIC_ZITADEL_ISSUER } from '$env/static/public';
import type { PageServerLoadEvent } from './$types';

interface TokenResponse {
	access_token: string;
	id_token: string;
	token_type: string;
	expires_in: number;
}

interface ErrorResponse {
	error: string;
}

export async function load(
	event: PageServerLoadEvent
): Promise<TokenResponse | ErrorResponse> {
	const code = event.url.searchParams.get('code');
	const codeVerifier = event.cookies.get('code_verifier');

	if (!code) {
		return { error: 'Missing authorization code' };
	}

	if (!codeVerifier) {
		return { error: 'Missing code verifier — session may have expired' };
	}

	const body = new URLSearchParams({
		grant_type: 'authorization_code',
		code,
		redirect_uri: 'http://localhost:5173/auth/callback',
		client_id: PUBLIC_ZITADEL_CLIENT_ID,
		code_verifier: codeVerifier
	});

	let response: Response;
	try {
		response = await fetch(`${PUBLIC_ZITADEL_ISSUER}/oauth/v2/token`, {
			method: 'POST',
			headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
			body: body.toString()
		});
	} catch (err) {
		return {
			error: `Token endpoint unreachable: ${err instanceof Error ? err.message : String(err)}`
		};
	}

	if (!response.ok) {
		const text = await response.text().catch(() => '(response body unreadable)');
		return { error: `Token exchange failed (${response.status}): ${text}` };
	}

	let tokens: TokenResponse;
	try {
		tokens = await response.json();
	} catch {
		return { error: 'Token response was not valid JSON' };
	}

	if (!tokens.access_token || !tokens.id_token) {
		return { error: 'Token response missing required fields (access_token or id_token)' };
	}

	event.cookies.delete('code_verifier', { path: '/' });

	return {
		access_token: tokens.access_token,
		id_token: tokens.id_token,
		token_type: tokens.token_type,
		expires_in: tokens.expires_in
	};
}
