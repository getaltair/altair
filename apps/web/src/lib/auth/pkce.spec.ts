import { describe, it, expect } from 'vitest';
import { generateCodeVerifier, generateCodeChallenge } from './pkce';

describe('generateCodeVerifier', () => {
    it('produces 86 characters', () => {
        const verifier = generateCodeVerifier();
        expect(verifier.length).toBe(86);
    });

    it('uses only base64url-safe characters', () => {
        const verifier = generateCodeVerifier();
        expect(/^[A-Za-z0-9\-_]+$/.test(verifier)).toBe(true);
    });
});

describe('generateCodeChallenge', () => {
    it('produces deterministic output for a known input', async () => {
        const verifier = 'test-verifier-string';
        const challenge1 = await generateCodeChallenge(verifier);
        const challenge2 = await generateCodeChallenge(verifier);
        expect(challenge1).toBe(challenge2);
    });
});
