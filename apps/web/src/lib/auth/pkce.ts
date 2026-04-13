/**
 * PKCE (Proof Key for Code Exchange) helpers for OIDC authorization code flow.
 * Uses the Web Crypto API — no external dependencies.
 */

/**
 * Generates a cryptographically random code verifier (64 bytes, base64url-encoded).
 */
export function generateCodeVerifier(): string {
	const bytes = new Uint8Array(64);
	crypto.getRandomValues(bytes);
	return base64urlEncode(bytes);
}

/**
 * Derives a code challenge from a verifier via SHA-256 hash (base64url-encoded).
 */
export async function generateCodeChallenge(verifier: string): Promise<string> {
	const encoder = new TextEncoder();
	const data = encoder.encode(verifier);
	const digest = await crypto.subtle.digest('SHA-256', data);
	return base64urlEncode(new Uint8Array(digest));
}

function base64urlEncode(bytes: Uint8Array): string {
	let str = '';
	for (const byte of bytes) {
		str += String.fromCharCode(byte);
	}
	return btoa(str).replace(/\+/g, '-').replace(/\//g, '_').replace(/=/g, '');
}
